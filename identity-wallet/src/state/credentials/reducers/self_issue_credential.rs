use std::sync::Arc;

use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use identity_credential::sd_jwt_vc::SdJwtVcBuilder;
use identity_iota::core::{Timestamp, Url};
use jsonschema::{JSONSchema, ValidationError};
use jsonwebtoken::Algorithm;
use log::info;
use oid4vc::oid4vc_core::Sign;
use sd_jwt_payload_rework::{JsonObject, JwsSigner};
use serde_json::json;

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

pub struct SubjectWrapper(pub Arc<dyn oid4vc::oid4vc_core::Subject>);

#[derive(thiserror::Error, Debug)]
pub enum TempError {}

#[async_trait]
impl JwsSigner for SubjectWrapper {
    // FIX THIS
    type Error = TempError;

    // FIX THIS: jwt::encode?
    async fn sign(&self, header: &JsonObject, payload: &JsonObject) -> Result<Vec<u8>, Self::Error> {
        let encoded_header = URL_SAFE_NO_PAD.encode(serde_json::to_vec(header).unwrap());
        let encoded_payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(payload).unwrap());

        let message = format!("{}.{}", encoded_header, encoded_payload);

        let proof_value = Sign::sign(&*self.0, &message, "FIX THIS", Algorithm::EdDSA)
            .await
            // FIX THIS
            .unwrap();

        let signature = URL_SAFE_NO_PAD.encode(proof_value.as_slice());
        let message = [message, signature].join(".");
        Ok(message.as_bytes().to_vec())
    }
}

pub async fn self_issue_credential(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(self_issue_credential) = listen::<SelfIssueCredential>(action) {
        // Validate to be self-issued credential data from the action payload against the JsonSchema belonging to the credential type
        match self_issue_credential._type {
            SelfIssuedCredentialType::Profile => {
                let json_schema_path =
                    // This path works for the test in unime/src-tauri/tests/tests/self_issue_credential.rs
                    format! {"../../identity-wallet/resources/{}_json_schema.json", self_issue_credential._type.to_string().to_lowercase()};
                // format! {"../../../../resources/{}_json_schema.json", self_issue_credential._type.to_string().to_lowercase()};

                info!(
                    "Validating payload credential against JsonSchema from path: {}",
                    json_schema_path
                );
                json_schema_validation(json_schema_path, self_issue_credential.data.clone())?;
            }
            SelfIssuedCredentialType::Address => {
                let json_schema_path =
                    // This path works for the test in unime/src-tauri/tests/tests/self_issue_credential.rs
                    format! {"../../identity-wallet/resources/{}_json_schema.json", self_issue_credential._type.to_string().to_lowercase()};
                // format! {"../../../../resources/{}_json_schema.json", self_issue_credential._type.to_string().to_lowercase()};

                info!(
                    "Validating payload credential against JsonSchema from path: {}",
                    json_schema_path
                );
                json_schema_validation(json_schema_path, self_issue_credential.data.clone())?;
            }
        }

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
        let issuer_did = state
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
        let subjectwrapper = SubjectWrapper(subject.clone());

        let now = Timestamp::now_utc(); // TODO?: is this the right time notation?

        let credential_data = self_issue_credential.data.as_object().ok_or(AppError::Error(
            "Invalid action payload for the self_issue_credential.data field".to_string(),
        ))?;

        let sd_jwt_credential = SdJwtVcBuilder::new(credential_data)
            .map_err(|_| AppError::Error("Failed to create a SdJwtVcBuilder".to_string()))?
            .header(std::iter::once(("kid".to_string(), serde_json::Value::String(kid.clone()))).collect())
            .vct(
                "https://www.ietf.org/archive/id/draft-terbu-oauth-sd-jwt-vc-00.html"
                    .parse::<Url>()
                    .map_err(|_| AppError::Error("Failed to parse the vct into a <Url>".to_string()))?,
            ) // TODO: make this specific to the credential type chosen and coherent with the JsonSchema used.
            .iat(now)
            .iss(issuer_did)
            .require_key_binding(identity_credential::sd_jwt_v2::RequiredKeyBinding::Kid(kid))
            // .make_concealable("/address/street_address")
            // .unwrap()
            // TODO: how to implement the fn make_concealable() when fields should only be known from the JsonSchema?
            .finish::<SubjectWrapper>(&subjectwrapper, key_type)
            .await
            .map_err(|_| AppError::Error("Failed to create the self-issued sd_jwt_credential".to_string()))?;

        drop(managers);

        let signed_credential = json!(sd_jwt_credential.to_string());

        // Create and populate the VerifiableCredentialRecord
        let mut vcr = VerifiableCredentialRecord::try_from(signed_credential).map_err(|_| {
            AppError::Error("Failed to create a VerifiableCredentialRecord from self_issue_credential".to_string())
        })?;

        vcr.display_credential.data = self_issue_credential.data;
        vcr.display_credential.display_name = self_issue_credential._type.to_string();
        vcr.display_credential.issuer_name = state
            .profile_settings
            .profile
            .clone()
            .ok_or(AppError::Error(
                "No profile found to set the self-issued credential issuer name".to_string(),
            ))?
            .name;
        vcr.display_credential.metadata.date_issued = now.to_string();

        // TODO: add to stronghold
        let mut credentials = state.credentials.clone();
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

fn json_schema_validation(json_schema_path: String, data: serde_json::Value) -> Result<(), AppError> {
    let json_schema_file = std::fs::File::open(json_schema_path.clone())
        .map_err(|_| AppError::Error("Failed to find or read from JsonSchema file".to_string()))?;
    let reader = std::io::BufReader::new(json_schema_file);
    let json_schema: serde_json::Value = serde_json::from_reader(reader)
        .map_err(|_| AppError::Error("Failed to convert JsonSchema &str to serde_json::Value".to_string()))?;

    // Draft is detected automatically with fallback to Draft7
    let schema = JSONSchema::compile(&json_schema)
        .map_err(|_| AppError::Error("Failed to compile JsonSchema from serde_json::Value".to_string()))?;

    let result = schema.validate(&data);

    if result.is_err() {
        let errors: Vec<ValidationError> = result.unwrap_err().collect();
        Err(AppError::Error(format!(
            "The data is invalid according to the given JsonSchema: {:?}",
            errors
        )))
    } else {
        Ok(())
    }
}
