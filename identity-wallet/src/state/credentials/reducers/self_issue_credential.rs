use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, SecondsFormat, Utc};
use identity_kit::{
    did::Did,
    jose::{encode_compact, JwsAlgorithm, JwsHeader, SignError, Signer},
    sd_jwt_vc::{ClaimPath, SdJwt, SdJwtBuilder},
    vc::Jwt,
};
use itertools::Itertools;
use jsonwebtoken::Algorithm;
use log::{debug, info};
use oid4vc_core::Sign;
use oid4vci::credential_format_profiles::CredentialFormats;
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

/// An identity-kit [`Signer`] for the key of `subject` that `did_method` and `algorithm` select.
pub struct SubjectSigner {
    subject: Arc<dyn oid4vc_core::Subject>,
    did_method: String,
    algorithm: Algorithm,
    alg: JwsAlgorithm,
}

impl SubjectSigner {
    pub fn new(
        subject: Arc<dyn oid4vc_core::Subject>,
        did_method: &str,
        algorithm: Algorithm,
    ) -> Result<Self, AppError> {
        let alg = match algorithm {
            Algorithm::EdDSA => JwsAlgorithm::EdDSA,
            Algorithm::ES256 => JwsAlgorithm::ES256,
            _ => return Err(AppError::Error(format!("Unsupported signing algorithm {algorithm:?}"))),
        };
        Ok(Self {
            subject,
            did_method: did_method.to_string(),
            algorithm,
            alg,
        })
    }
}

#[async_trait]
impl Signer for SubjectSigner {
    fn alg(&self) -> JwsAlgorithm {
        self.alg
    }

    fn kid(&self) -> Option<&str> {
        None
    }

    async fn sign(&self, signing_input: &[u8]) -> Result<Vec<u8>, SignError> {
        let message = std::str::from_utf8(signing_input).map_err(SignError::new)?;
        Sign::sign(&*self.subject, message, &self.did_method, self.algorithm)
            .await
            .map_err(|err| SignError::msg(format!("{err:#}")))
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
    subject: Arc<dyn oid4vc_core::Subject>,
    did_method: &str,
    issuer_did: &str,
    key_type: &str,
    credential_type: &str,
    mut credential_subject: Map<String, Value>,
    valid_from: DateTime<Utc>,
) -> Result<SelfIssuedSdJwt, AppError> {
    let algorithm = match key_type {
        "EdDSA" => Algorithm::EdDSA,
        "ES256" => Algorithm::ES256,
        _ => return Err(AppError::Error("Unsupported key type".to_string())),
    };
    let issuer_did =
        Did::parse(issuer_did).map_err(|err| AppError::Error(format!("Failed to parse the DID: {err}")))?;

    let kid = subject.key_id(did_method, algorithm).await.ok_or(AppError::Error(
        "Failed to create a key id necessary to self-issue the credential".to_string(),
    ))?;

    debug!("Self-issuing VCDM 2.0 SD-JWT: issuer_did={issuer_did}, kid={kid}, alg={algorithm:?}");

    let signer = SubjectSigner::new(subject, did_method, algorithm)?;

    let concealable_claims: Vec<String> = credential_subject.keys().cloned().collect();
    credential_subject.insert("id".to_string(), json!(issuer_did));

    let types = json!(["VerifiableCredential", credential_type]);

    // Same shape as the `vc+sd-jwt` credentials UniTrust issues, so that its verification flows can request
    // every `credentialSubject` claim on its own.
    let claims = json!({
        "@context": ["https://www.w3.org/ns/credentials/v2"],
        "type": types,
        "issuer": issuer_did,
        "validFrom": valid_from.to_rfc3339_opts(SecondsFormat::Secs, true),
        "credentialSubject": credential_subject,
        "iss": issuer_did.to_string(),
    });
    let Value::Object(claims) = claims else {
        unreachable!("a JSON object literal is an object")
    };

    let encoded = concealable_claims
        .iter()
        .fold(SdJwtBuilder::new(claims), |builder, claim| {
            builder.conceal(ClaimPath::new(["credentialSubject", claim.as_str()]))
        })
        .encode()
        .map_err(|e| AppError::Error(format!("Failed to conceal the self-issued claims: {e}")))?;

    let mut payload = encoded.payload;
    payload.insert("cnf".to_string(), json!({ "kid": kid }));

    let header = JwsHeader::new(signer.alg()).with_typ("vc+sd-jwt").with_kid(kid);
    let payload = serde_json::to_vec(&payload).map_err(|e| AppError::Error(e.to_string()))?;
    let jwt = encode_compact(&header, &payload, &signer)
        .await
        .map_err(|e| AppError::Error(format!("Failed to create the self-issued SD-JWT: {e}")))?;
    let jwt = Jwt::new(jwt).map_err(|e| AppError::Error(format!("Failed to create the self-issued SD-JWT: {e}")))?;
    let sd_jwt = SdJwt::new(jwt, encoded.disclosures).to_string();

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

        let now = Utc::now();

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
        vcr.display_credential.metadata.date_issued = now.to_rfc3339_opts(SecondsFormat::Secs, true);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{persistence::STRONGHOLD, stronghold::StrongholdManager, subject::subject};
    use identity_kit::{
        jose::DefaultVerifier,
        sd_jwt_vc::{HashAlgorithm, KeyBinding},
    };
    use oid4vc_core::Subject as _;
    use tempfile::NamedTempFile;

    /// A self-issued SD-JWT verifies under the issuer's key, reads back as a record, and presents with a key binding
    /// JWT that verifies under the holder key its `cnf` names.
    #[tokio::test]
    #[serial_test::serial]
    async fn a_self_issued_sd_jwt_verifies_and_presents_with_key_binding() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();
        let subject = subject(Arc::new(StrongholdManager::create("sup3rSecr3t").unwrap()))
            .await
            .unwrap();

        for (did_method, key_type, algorithm) in [
            ("did:jwk", "EdDSA", Algorithm::EdDSA),
            ("did:key", "ES256", Algorithm::ES256),
        ] {
            let issuer_did = subject.identifier(did_method, algorithm).await.unwrap();
            let credential_subject = Map::from_iter([("name".to_string(), json!("Ferris"))]);
            let self_issued = sign_self_issued_sd_jwt(
                subject.clone(),
                did_method,
                &issuer_did,
                key_type,
                "NaturalPersonCredential",
                credential_subject,
                "2026-01-01T00:00:00Z".parse().unwrap(),
            )
            .await
            .unwrap();

            let sd_jwt = SdJwt::parse(&self_issued.sd_jwt).unwrap();
            let jws = sd_jwt.jwt().decode().unwrap();
            let kid = jws.kid().unwrap().to_string();
            assert_eq!(jws.header().typ.as_deref(), Some("vc+sd-jwt"));
            assert_eq!(subject.signing_algorithm(&kid).await.unwrap(), algorithm);
            let issuer_key = subject.resolve_public_key(&kid).await.unwrap();
            jws.verify(&DefaultVerifier, &issuer_key).unwrap();
            assert_eq!(sd_jwt.payload().unwrap()["cnf"], json!({ "kid": kid }));
            assert_eq!(
                sd_jwt.process().unwrap().payload["credentialSubject"],
                json!({ "id": issuer_did, "name": "Ferris" })
            );

            let record =
                VerifiableCredentialRecord::try_new(CredentialFormats::VcSdJwt(()), json!(self_issued.sd_jwt), vec![])
                    .unwrap();
            assert_eq!(record.display_credential.data["credentialSubject"]["name"], "Ferris");
            assert_eq!(record.display_credential.data["issuer"], json!(issuer_did));

            let signer = SubjectSigner::new(subject.clone(), did_method, algorithm).unwrap();
            let presented = sd_jwt
                .clone()
                .sign_key_binding(
                    &KeyBinding::new("https://verifier.example", "n-0S6_WzA2Mj", chrono::Utc::now()),
                    &signer,
                )
                .await
                .unwrap();
            let key_binding = presented.key_binding_jwt().unwrap().decode().unwrap();
            assert_eq!(key_binding.header().typ.as_deref(), Some("kb+jwt"));
            key_binding.verify(&DefaultVerifier, &issuer_key).unwrap();
            let claims: Value = key_binding.claims().unwrap();
            assert_eq!(claims["aud"], "https://verifier.example");
            assert_eq!(claims["nonce"], "n-0S6_WzA2Mj");
            assert_eq!(claims["sd_hash"], json!(sd_jwt.sd_hash(HashAlgorithm::default())));
        }
    }
}
