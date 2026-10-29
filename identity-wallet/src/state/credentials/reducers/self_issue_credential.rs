use std::sync::Arc;

use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use identity_iota::core::{Timestamp, Url};
use itertools::Itertools;
use jsonwebtoken::Algorithm;
use log::{debug, info};
use oid4vc::{oid4vc_core::Sign, oid4vci::credential_format_profiles::CredentialFormats};
use sd_jwt::{JsonObject, JwsSigner, RequiredKeyBinding, SdJwtBuilder};
use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        credentials::{
            actions::self_issue_credential::{SelfIssueCredential, SelfIssuedCredentialType},
            VerifiableCredentialRecord,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

pub struct SubjectWrapper {
    pub subject: Arc<dyn oid4vc::oid4vc_core::Subject>,
    pub preferred_did_method: String,
}

#[async_trait]
impl JwsSigner for SubjectWrapper {
    type Error = AppError;

    // FIX THIS: jwt::encode?
    async fn sign(&self, header: &JsonObject, payload: &JsonObject) -> Result<Vec<u8>, Self::Error> {
        let algorithm = header
            .get("alg")
            .and_then(|alg| alg.as_str())
            .ok_or(AppError::Error("Missing `alg` in JWT header".to_string()))?
            .parse::<Algorithm>()
            .map_err(|_| AppError::Error("Unsupported algorithm in JWT header".to_string()))?;

        let encoded_header = URL_SAFE_NO_PAD.encode(serde_json::to_vec(header).unwrap());
        let encoded_payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(payload).unwrap());

        let message = format!("{}.{}", encoded_header, encoded_payload);

        let proof_value = Sign::sign(&*self.subject, &message, &self.preferred_did_method, algorithm)
            .await
            .map_err(|e| AppError::Error(format!("Failed to sign JWT for sd-jwt: {}", e)))?;

        let signature = URL_SAFE_NO_PAD.encode(proof_value.as_slice());
        let message = [message, signature].join(".");

        Ok(message.as_bytes().to_vec())
    }
}

/// A self-issued credential signed as a W3C VCDM 2.0 `vc+sd-jwt`.
pub struct SelfIssuedSdJwt {
    pub sd_jwt: String,
    /// The `type`, `issuer` and `credentialSubject` (including the holder `id`) shown by the app.
    pub data: Map<String, Value>,
}

/// Signs `credential_subject` as a self-issued `vc+sd-jwt` with every claim selectively disclosable.
pub async fn sign_self_issued_sd_jwt(
    subject: Arc<dyn oid4vc::oid4vc_core::Subject>,
    did_method: &str,
    issuer_did: &str,
    key_type: &str,
    credential_type: &str,
    mut credential_subject: Map<String, Value>,
    valid_from: Timestamp,
) -> Result<SelfIssuedSdJwt, AppError> {
    let algorithm = match key_type {
        "EdDSA" => Algorithm::EdDSA,
        "ES256" => Algorithm::ES256,
        _ => return Err(AppError::Error("Unsupported key type".to_string())),
    };
    let issuer_did: Url = issuer_did
        .parse()
        .map_err(|_| AppError::Error("Failed to parse the did into a <Url>".to_string()))?;

    let kid = subject.key_id(did_method, algorithm).await.ok_or(AppError::Error(
        "Failed to create a key id necessary to self-issue the credential".to_string(),
    ))?;

    debug!("Self-issuing VCDM 2.0 SD-JWT: issuer_did={issuer_did}, kid={kid}, alg={algorithm:?}");

    let subject_wrapper = SubjectWrapper {
        subject,
        preferred_did_method: did_method.to_string(),
    };

    let concealable_claims: Vec<String> = credential_subject.keys().cloned().collect();
    credential_subject.insert("id".to_string(), json!(issuer_did));

    let types = json!(["VerifiableCredential", credential_type]);

    // Same shape as the `vc+sd-jwt` credentials UniTrust issues, so that its verification flows can request
    // every `credentialSubject` claim on its own.
    let mut builder = SdJwtBuilder::new(json!({
        "@context": ["https://www.w3.org/ns/credentials/v2"],
        "type": types,
        "issuer": issuer_did,
        "validFrom": valid_from.to_string(),
        "credentialSubject": credential_subject,
    }))
    .and_then(|builder| builder.insert_claim("iss", issuer_did.to_string()))
    .map_err(|e| AppError::Error(format!("Failed to create the SD-JWT builder: {e}")))?
    .header("typ", "vc+sd-jwt")
    .header("kid", kid.clone())
    .require_key_binding(RequiredKeyBinding::Kid(kid));

    for claim in &concealable_claims {
        builder = builder
            .make_concealable(&format!("/credentialSubject/{claim}"))
            .map_err(|e| AppError::Error(format!("Failed to make `{claim}` selectively disclosable: {e}")))?;
    }

    let sd_jwt = builder
        .finish(&subject_wrapper, key_type)
        .await
        .map_err(|e| AppError::Error(format!("Failed to create the self-issued SD-JWT: {e}")))?
        .to_string();

    let data = Map::from_iter([
        ("type".to_string(), types),
        ("issuer".to_string(), json!(issuer_did)),
        ("credentialSubject".to_string(), Value::Object(credential_subject)),
    ]);

    Ok(SelfIssuedSdJwt { sd_jwt, data })
}

#[tracing::instrument(skip_all, err)]
pub async fn self_issue_credential(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(self_issue_credential) = listen::<SelfIssueCredential>(action) {
        let data = &self_issue_credential.data;

        let key_type = state
            .profile_settings
            .preferred_key_types
            .first()
            .ok_or(AppError::Error("Failed to get a preferred key type".to_string()))?;

        // Get issuer DID
        let did_method = state
            .profile_settings
            .preferred_did_methods
            .first()
            .ok_or(AppError::Error("Failed to get a preferred did method".to_string()))?;
        let issuer_did = state.dids.get(did_method).ok_or(AppError::Error(
            "Failed to get the did for the preferred did method".to_string(),
        ))?;

        let subject = state
            .core_utils
            .managers
            .lock()
            .await
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .subject
            .clone();

        let credential_subject = data
            .get("credentialSubject")
            .and_then(|credential_subject| credential_subject.as_object())
            .cloned()
            .ok_or(AppError::Error(
                "Invalid action payload for the self_issue_credential.data.credentialSubject field".to_string(),
            ))?;

        let credential_type = match self_issue_credential._type {
            SelfIssuedCredentialType::Profile => "NaturalPersonCredential",
            SelfIssuedCredentialType::Address => "ResidenceCredential",
        };

        let now = Timestamp::now_utc();

        let self_issued = sign_self_issued_sd_jwt(
            subject,
            did_method,
            issuer_did,
            key_type,
            credential_type,
            credential_subject,
            now,
        )
        .await?;

        // Create and populate the VerifiableCredentialRecord
        let mut vcr =
            VerifiableCredentialRecord::try_new(CredentialFormats::VcSdJwt(()), json!(self_issued.sd_jwt), vec![])
                .map_err(|_| {
                    AppError::Error(
                        "Failed to create a VerifiableCredentialRecord from self_issue_credential".to_string(),
                    )
                })?;

        let mut display_data = Map::from_iter([("name".to_string(), json!(data.get("name")))]);
        display_data.extend(self_issued.data);
        vcr.display_credential.data = Value::Object(display_data);
        vcr.display_credential.issuer_name = state
            .profile_settings
            .profile
            .as_ref()
            .ok_or(AppError::Error("No profile found".to_string()))?
            .name
            .clone();
        vcr.display_credential.display_name = data
            .get("name")
            .unwrap_or(&json!(self_issue_credential._type))
            .as_str()
            .unwrap()
            .to_owned();

        // Metadata
        vcr.display_credential.metadata.date_issued = now.to_string();
        vcr.display_credential.metadata.is_favorite = self_issue_credential.is_favorite;
        vcr.display_credential.metadata.icon = self_issue_credential.icon;

        let state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let key: Uuid = vcr.display_credential.id.parse().expect("invalid uuid");

        // Remove the old credential from the stronghold if it exists.
        stronghold_manager.remove(key).map_err(StrongholdDeletionError)?;

        stronghold_manager
            .insert(key, json!(vcr).to_string().as_bytes().to_vec())
            .map_err(StrongholdInsertionError)?;

        drop(state_guard);
        info!(
            "Self-issued credential `{}` ({key}) successfully saved to Stronghold",
            vcr.display_credential.display_name
        );

        let mut credentials = state.credentials.clone();
        if credentials
            .iter()
            .map(|display_credential| display_credential.id.clone())
            .contains(&vcr.display_credential.id)
        {
            // Remove the old credential from the list of credentials
            credentials.retain(|display_credential| display_credential.id != vcr.display_credential.id);
        }

        credentials.push(vcr.display_credential);

        let redirect_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

        return Ok(AppState {
            current_user_prompt: redirect_prompt,
            credentials,
            ..state
        });
    }

    Ok(state)
}
