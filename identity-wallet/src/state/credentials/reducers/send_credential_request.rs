use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            history_event::{EventType, HistoryCredential, HistoryEvent},
            CoreUtils,
        },
        credentials::{
            actions::credential_offers_selected::CredentialOffersSelected, DisplayCredential,
            VerifiableCredentialRecord,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::info;
use oid4vc::oid4vci::{
    credential_offer::Grants, credential_response::CredentialResponseType, token_request::TokenRequest,
};
use serde_json::json;
use uuid::Uuid;

pub async fn send_credential_request(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_credential_request");

    if let Some(credential_configuration_ids) =
        listen::<CredentialOffersSelected>(action).map(|payload| payload.credential_configuration_ids)
    {
        let state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

        let current_user_prompt = state
            .current_user_prompt
            .clone()
            .ok_or(MissingStateParameterError("current user prompt"))?;

        info!("current_user_prompt: {:?}", current_user_prompt);

        let credential_offer = state.core_utils.active_credential_offer.take().unwrap();
        let logo_uri = match current_user_prompt {
            CurrentUserPrompt::CredentialOffer { logo_uri, .. } => logo_uri,
            _ => unreachable!(),
        };

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer;

        info!("credential issuer url: {:?}", credential_issuer_url);

        // Get the authorization server metadata.
        let authorization_server_metadata = wallet
            .get_authorization_server_metadata(credential_issuer_url.clone())
            .await
            .map_err(GetAuthorizationServerMetadataError)?;

        info!("authorization server metadata: {:?}", authorization_server_metadata);

        // Get the credential issuer metadata.
        let credential_issuer_metadata = wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .map_err(GetCredentialIssuerMetadataError)?;

        info!("credential issuer metadata: {:?}", credential_issuer_metadata);

        // Get the credential issuer display.
        let display = credential_issuer_metadata
            .display
            .as_ref()
            .and_then(|display| display.first().cloned());

        // Get the connection url from the credential issuer url host (or use the credential issuer url if it does not
        // contain a host).
        let connection_url = credential_issuer_url
            .host_str()
            .unwrap_or(credential_issuer_url.as_str());

        // Get the credential issuer name or use the credential issuer url.
        let issuer_name = display
            .map(|display| {
                let issuer_name = display["client_name"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or(connection_url.to_string());
                issuer_name
            })
            .unwrap_or(connection_url.to_string());

        let mut credential_configurations_supported =
            credential_issuer_metadata.credential_configurations_supported.clone();

        // Create or update the connection.
        let previously_connected = state.connections.contains(connection_url, &issuer_name);
        let mut connections = state.connections;
        let connection = connections.update_or_insert(connection_url, &issuer_name, None);

        // Create a token request with grant_type `pre_authorized_code`.
        let token_request = match credential_offer.grants {
            Some(Grants {
                pre_authorized_code, ..
            }) => TokenRequest::PreAuthorizedCode {
                pre_authorized_code: pre_authorized_code.unwrap().pre_authorized_code,
                tx_code: None,
            },
            None => unreachable!(),
        };

        info!("token_request: {:?}", token_request);

        // Get an access token.
        let token_response = wallet
            .get_access_token(authorization_server_metadata.token_endpoint.unwrap(), token_request)
            .await
            .map_err(GetAccessTokenError)?;

        info!("token_response: {:?}", token_response);

        credential_configurations_supported.retain(|credential_configuration_id, _| {
            credential_configuration_ids.contains(credential_configuration_id)
        });

        let credentials: Vec<(String, serde_json::Value)> = match credential_configuration_ids.len() {
            0 => vec![],
            1 => {
                let credential_configuration_id = credential_configuration_ids[0].clone();

                let credential_configuration = credential_configurations_supported
                    .get(&credential_configuration_id)
                    .ok_or(UnknownCredentialConfigurationIdError(
                        credential_configuration_id.clone(),
                    ))?;

                // Get the credential.
                let credential_response = wallet
                    .get_credential(credential_issuer_metadata, &token_response, credential_configuration)
                    .await
                    .map_err(GetCredentialError)?;

                let credential = match credential_response.credential {
                    CredentialResponseType::Immediate { credential, .. } => credential,
                    _ => panic!("Credential was not a jwt_vc_json."),
                };

                vec![(credential_configuration_id, credential)]
            }
            _batch => {
                let (credential_configuration_ids, credential_configurations): (Vec<_>, Vec<_>) =
                    credential_configurations_supported
                        .into_iter()
                        .map(|(credential_configuration_id, credential_configuration)| {
                            (credential_configuration_id, credential_configuration)
                        })
                        .unzip();

                let batch_credential_response = wallet
                    .get_batch_credential(credential_issuer_metadata, &token_response, &credential_configurations)
                    .await
                    .map_err(GetBatchCredentialError)?;

                credential_configuration_ids
                    .into_iter()
                    .zip(batch_credential_response.credential_responses.into_iter())
                    .filter_map(
                        |(credential_configuration_id, credential_response)| match credential_response {
                            CredentialResponseType::Immediate { credential, .. } => {
                                Some((credential_configuration_id, credential))
                            }
                            // TODO: add support for deffered credentials.
                            CredentialResponseType::Deferred { .. } => None,
                        },
                    )
                    .collect()
            }
        };

        info!("credentials: {:?}", credentials);

        let mut history_credentials = vec![];

        for (credential_configuration_id, credential) in credentials.into_iter() {
            let mut verifiable_credential_record: VerifiableCredentialRecord = credential.into();
            verifiable_credential_record
                .display_credential
                .issuer_name
                .clone_from(&issuer_name);
            verifiable_credential_record.display_credential.connection_id = Some(connection.id.clone());

            let key: Uuid = verifiable_credential_record
                .display_credential
                .id
                .parse()
                .expect("invalid uuid");

            info!("generated hash-key: {:?}", key);

            persist_asset(
                format!("credential_{credential_configuration_id}").as_str(),
                key.to_string().as_str(),
            )
            .ok();

            // Remove the old credential from the stronghold if it exists.
            stronghold_manager.remove(key).map_err(StrongholdDeletionError)?;

            stronghold_manager
                .insert(key, json!(verifiable_credential_record).to_string().as_bytes().to_vec())
                .map_err(StrongholdInsertionError)?;

            // Add history event
            history_credentials.push(HistoryCredential::from_credential(&verifiable_credential_record));
        }

        let credentials: Vec<DisplayCredential> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        let file_name = match logo_uri {
            Some(logo_uri) => hash(logo_uri.as_str()),
            None => "_".to_string(),
        };
        persist_asset(&file_name, &connection.id).ok();

        // History
        if !history_credentials.is_empty() {
            // Only add a `ConnectionAdded` event if the connection was not previously connected.
            if !previously_connected {
                state.history.push(HistoryEvent {
                    connection_name: connection.name.clone(),
                    event_type: EventType::ConnectionAdded,
                    connection_id: connection.id.clone(),
                    date: connection.last_interacted.clone(),
                    credentials: vec![],
                });
            }
            state.history.push(HistoryEvent {
                connection_name: connection.name.clone(),
                event_type: EventType::CredentialsAdded,
                connection_id: connection.id.clone(),
                date: connection.last_interacted.clone(),
                credentials: history_credentials,
            });
        }

        state.connections = connections;
        state.credentials = credentials;

        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });
    }

    Ok(AppState {
        core_utils: CoreUtils {
            managers: state.core_utils.managers,
            ..Default::default()
        },
        ..state
    })
}
