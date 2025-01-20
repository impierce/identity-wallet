use std::sync::Arc;

use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use identity_credential::sd_jwt_vc::SdJwtVcBuilder;
use identity_iota::core::{Timestamp, Url};
use jsonwebtoken::Algorithm;
use log::info;
use oid4vc::oid4vc_core::Sign;
use sd_jwt_payload_rework::{JsonObject, JwsSigner};
use serde_json::json;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::actions::self_issue_credential::{SelfIssueCredential, SelfIssuedCredentialType},
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

        let proof_value = Sign::sign(&*self.0, &message, "FIX THIS", Algorithm::default())
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
        match self_issue_credential._type {
            SelfIssuedCredentialType::Profile => {
                info!("Successfully self-issued profile credential with id: XX");
            }
            SelfIssuedCredentialType::Address => {
                info!("Successfully self-issued address credential with id: XX");
            }
        }

        {
            let now = Timestamp::from_unix(0).unwrap();

            let managers = state.core_utils.managers.lock().await;
            let subject = &managers.identity_manager.as_ref().and_then(|f| Some(f.subject.clone())).unwrap();
            let kid = subject
                .key_id(
                    "Jwk", // TODO: hardcode
                    jsonwebtoken::Algorithm::ES256, // TODO: hardcode
                )
                .await
                .unwrap();

            let subjectwrapper = SubjectWrapper(subject.clone());

            let issuer_did = "0"; //managers.identity_manager.as_ref().unwrap().subject.did.clone(); // TODO: hardcode

            let sd_jwt_credential = SdJwtVcBuilder::new(self_issue_credential.data.clone())
                .unwrap()
                .header(std::iter::once(("kid".to_string(), serde_json::Value::String(kid.clone()))).collect())
                // FIX THIS
                .vct("https://example.com/education_credential".parse::<Url>().unwrap())
                .iat(now)
                .iss(issuer_did.parse().unwrap())
                .require_key_binding(identity_credential::sd_jwt_v2::RequiredKeyBinding::Kid(
                    // FIX THIS!: how to get the holder's kid or Jwk?
                    kid,
                ))
                // .make_concealable("/address/street_address")
                // .unwrap()
                // FIX THIS!
                .finish::<SubjectWrapper>(&subjectwrapper, "ES256")
                .await
                .unwrap();

            json!(sd_jwt_credential.to_string());
        }

        let redirect_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

        return Ok(AppState {
            current_user_prompt: redirect_prompt,
            ..state
        });
    }

    Ok(state)
}

