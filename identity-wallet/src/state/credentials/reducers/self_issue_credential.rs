use std::sync::Arc;

use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use identity_credential::sd_jwt_vc::{SdJwtVcBuilder, SD_JWT_VC_TYP};
use identity_iota::core::{Timestamp, Url};
use itertools::Itertools;
use jsonwebtoken::Algorithm;
use oid4vc::{oid4vc_core::Sign, oid4vci::credential_format_profiles::CredentialFormats};
use sd_jwt::{JsonObject, JwsSigner, RequiredKeyBinding};
use serde_json::json;
use uuid::Uuid;

use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        credentials::{actions::self_issue_credential::SelfIssueCredential, VerifiableCredentialRecord},
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

pub async fn self_issue_credential(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(self_issue_credential) = listen::<SelfIssueCredential>(action) {
        // TODO: autofill credentialSubject and a few other fields.

        let data = &mut self_issue_credential.data.clone();

        // Get preferred key type and convert it to jsonwebtoken::Algorithm
        let key_type = state
            .profile_settings
            .preferred_key_types
            .first()
            .ok_or(AppError::Error("Failed to get a preferred key type".to_string()))?
            .as_str();
        let algorithm = match key_type {
            "EdDSA" => jsonwebtoken::Algorithm::EdDSA,
            "ES256" => jsonwebtoken::Algorithm::ES256,
            _ => return Err(AppError::Error("Unsupported key type".to_string())),
        };

        // Get issuer DID
        let did_method = state
            .profile_settings
            .preferred_did_methods
            .first()
            .ok_or(AppError::Error("Failed to get a preferred did method".to_string()))?;
        let issuer_did: Url = state
            .dids
            .get(did_method)
            .ok_or(AppError::Error(
                "Failed to get the did for the preferred did method".to_string(),
            ))?
            .parse()
            .map_err(|_| AppError::Error("Failed to parse the did into a <Url>".to_string()))?;

        // Get kid
        let managers = state.core_utils.managers.lock().await;
        let subject = managers
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .subject
            .clone();

        let kid = subject.key_id(did_method, algorithm).await.ok_or(AppError::Error(
            "Failed to create a key id necessary to self-issue the credential".to_string(),
        ))?;

        // Wrap subject with the SubjectWrapper to get the JwsSigner implementation
        let subject_wrapper = SubjectWrapper {
            subject: subject.clone(),
            preferred_did_method: did_method.clone(),
        };

        let now = Timestamp::now_utc();

        let credential_data = data.as_object_mut().ok_or(AppError::Error(
            "Invalid action payload for the self_issue_credential.data field".to_string(),
        ))?;

        credential_data.insert("issuer".to_string(), json!(issuer_did.to_string()));

        let sd_jwt_credential = SdJwtVcBuilder::new(credential_data)
            .map_err(|_| AppError::Error("Failed to create a SdJwtVcBuilder".to_string()))?
            .headers(JsonObject::from_iter(vec![
                ("typ".to_string(), serde_json::Value::String(SD_JWT_VC_TYP.to_string())),
                ("kid".to_string(), serde_json::Value::String(kid.clone())),
            ]))
            .vct(
                "https://www.ietf.org/archive/id/draft-terbu-oauth-sd-jwt-vc-00.html"
                    .parse::<Url>()
                    .map_err(|_| AppError::Error("Failed to parse the vct into a <Url>".to_string()))?,
            ) // TODO: make this specific to the credential type chosen and coherent with the JsonSchema used.
            .iat(now)
            .iss(issuer_did)
            .require_key_binding(RequiredKeyBinding::Kid(kid))
            // TODO: how to implement the fn make_concealable(), also when fields should only be known from the JsonSchema?
            .finish::<SubjectWrapper>(&subject_wrapper, key_type)
            .await
            .map_err(|_| AppError::Error("Failed to create the self-issued sd_jwt_credential".to_string()))?;

        drop(managers);

        let signed_credential = json!(sd_jwt_credential.to_string());

        // Create and populate the VerifiableCredentialRecord
        let mut vcr = VerifiableCredentialRecord::try_new(CredentialFormats::DcSdJwt(()), signed_credential, vec![])
            .map_err(|_| {
                AppError::Error("Failed to create a VerifiableCredentialRecord from self_issue_credential".to_string())
            })?;

        vcr.display_credential.data = data.clone();
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

        let mut credentials = state.credentials.clone();
        if credentials
            .iter()
            .map(|display_credential| display_credential.id.clone())
            .contains(&vcr.display_credential.id)
        {
            // Remove the old credential from the list of credentials
            credentials.retain(|display_credential| display_credential.id != vcr.display_credential.id);
        }

        credentials.push(vcr.display_credential); // TODO: this is unsorted

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
