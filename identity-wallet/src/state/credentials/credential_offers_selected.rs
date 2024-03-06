use crate::error::AppError;
use crate::persistence::persist_asset;
use crate::reducer;
use crate::state::actions::{listen, Action};
use crate::state::credentials::verifiable_credential_record::VerifiableCredentialRecord;
use crate::state::credentials::DisplayCredential;
use crate::state::history_event::{EventType, HistoryCredential, HistoryEvent};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;
use crate::state::{actions::ActionTrait, Reducer};

use log::info;
use oid4vc::oid4vci::credential_format_profiles::{CredentialFormats, WithCredential, WithParameters};
use oid4vc::oid4vci::credential_offer::{CredentialsObject, Grants};
use oid4vc::oid4vci::credential_response::CredentialResponseType;
use oid4vc::oid4vci::token_request::TokenRequest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use uuid::Uuid;

/// Action to handle the selection of credential offers.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialOffersSelected.ts")]
pub struct CredentialOffersSelected {
    #[ts(type = "Array<string>")]
    pub offer_indices: Vec<usize>,
}

#[typetag::serde(name = "[Credential Offer] Selected")]
impl ActionTrait for CredentialOffersSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_credential_request)]
    }
}

pub async fn send_credential_request(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_credential_request");

    if let Some(offer_indices) = listen::<CredentialOffersSelected>(action).map(|payload| payload.offer_indices) {
        let state_guard = state.core_utils.managers.lock().await;

        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(AppError::MissingManagerError("stronghold"))?;

        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(AppError::MissingManagerError("identity"))?
            .wallet;

        let current_user_prompt = state
            .current_user_prompt
            .clone()
            .ok_or(AppError::MissingStateParameterError("current user prompt"))?;

        info!("current_user_prompt: {:?}", current_user_prompt);

        let credential_offer = match current_user_prompt {
            CurrentUserPrompt::CredentialOffer { credential_offer, .. } => credential_offer,
            _ => unreachable!(),
        };

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer;

        info!("credential issuer url: {:?}", credential_issuer_url);

        // Get the authorization server metadata.
        let authorization_server_metadata = wallet
            .get_authorization_server_metadata(credential_issuer_url.clone())
            .await
            .map_err(AppError::GetAuthorizationServerMetadataError)?;

        info!("authorization server metadata: {:?}", authorization_server_metadata);

        // Get the credential issuer metadata.
        let credential_issuer_metadata = wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .map_err(AppError::GetCredentialIssuerMetadataError)?;

        info!("credential issuer metadata: {:?}", credential_issuer_metadata);

        // Get the credential issuer display.
        let display = credential_issuer_metadata
            .display
            .as_ref()
            .and_then(|display| display.first().cloned());

        // Get the credential issuer name or use the credential issuer url.
        let issuer_name = display
            .map(|display| {
                let issuer_name = display["client_name"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or(credential_issuer_url.to_string());
                issuer_name
            })
            .unwrap_or(credential_issuer_url.to_string());

        let credential_offer_formats = offer_indices
            .into_iter()
            .map(|offer_index| match credential_offer.credentials.get(offer_index) {
                Some(CredentialsObject::ByValue(credential_format)) => credential_format.to_owned(),
                // Unreachable because we replace all `CredentialsObject::ByReference` with `CredentialsObject::ByValue` in `read_credential_offer`.
                _ => unreachable!(),
            })
            .collect::<Vec<CredentialFormats<WithParameters>>>();

        info!("credential_offer_formats: {:?}", credential_offer_formats);

        // Create a token request with grant_type `pre_authorized_code`.
        let token_request = match credential_offer.grants {
            Some(Grants {
                pre_authorized_code, ..
            }) => TokenRequest::PreAuthorizedCode {
                pre_authorized_code: pre_authorized_code.unwrap().pre_authorized_code,
                user_pin: None,
            },
            None => unreachable!(),
        };
        info!("token_request: {:?}", token_request);

        // Get an access token.
        let token_response = wallet
            .get_access_token(authorization_server_metadata.token_endpoint.unwrap(), token_request)
            .await
            .map_err(AppError::GetAccessTokenError)?;

        info!("token_response: {:?}", token_response);

        let credentials: Vec<CredentialFormats<WithCredential>> = match credential_offer_formats.len() {
            0 => vec![],
            1 => {
                // Get the credential.
                let credential_response = wallet
                    .get_credential(
                        credential_issuer_metadata,
                        &token_response,
                        credential_offer_formats[0].to_owned(),
                    )
                    .await
                    .map_err(AppError::GetCredentialError)?;

                let credential = match credential_response.credential {
                    CredentialResponseType::Immediate(credential) => credential,
                    _ => panic!("Credential was not a jwt_vc_json."),
                };

                vec![credential]
            }
            _batch => {
                let batch_credential_response = wallet
                    .get_batch_credential(credential_issuer_metadata, &token_response, credential_offer_formats)
                    .await
                    .map_err(AppError::GetBatchCredentialError)?;

                batch_credential_response
                    .credential_responses
                    .into_iter()
                    .map(|credential_response| match credential_response {
                        CredentialResponseType::Immediate(credential) => credential,
                        _ => panic!("Credential was not a jwt_vc_json."),
                    })
                    .collect()
            }
        };

        info!("credentials: {:?}", credentials);

        let mut history_credentials = vec![];

        for (i, credential) in credentials.into_iter().enumerate() {
            let mut verifiable_credential_record: VerifiableCredentialRecord = credential.into();
            verifiable_credential_record.display_credential.issuer_name = issuer_name.clone();

            let key: Uuid = verifiable_credential_record
                .display_credential
                .id
                .parse()
                .expect("invalid uuid");

            info!("generated hash-key: {:?}", key);

            persist_asset(format!("credential_{}", i).as_str(), key.to_string().as_str()).ok();

            // Remove the old credential from the stronghold if it exists.
            stronghold_manager
                .remove(key)
                .map_err(AppError::StrongholdDeletionError)?;

            stronghold_manager
                .insert(key, json!(verifiable_credential_record).to_string().as_bytes().to_vec())
                .map_err(AppError::StrongholdInsertionError)?;

            // Add history event
            history_credentials.push(HistoryCredential::from_credential(&verifiable_credential_record));
        }

        let credentials: Vec<DisplayCredential> = stronghold_manager
            .values()
            .map_err(AppError::StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        // History
        if !history_credentials.is_empty() {
            state.history.push(HistoryEvent {
                connection_name: issuer_name,
                event_type: EventType::CredentialsAdded,
                date: credentials[0].metadata.date_added.clone(),
                connection_id: None,
                credentials: history_credentials,
            });
        }

        state.credentials = credentials;

        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });
    }

    Ok(state)
}
