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
            actions::{
                authorization_code_received::AuthorizationCodeReceived,
                credential_offers_selected::CredentialOffersSelected,
            },
            DisplayCredential, VerifiableCredentialRecord,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::info;
use oid4vc::oid4vci::{
    authorization_details::AuthorizationDetailsObject, authorization_request::AuthorizationRequest,
    credential_format_profiles::CredentialFormats,
    credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject,
    credential_offer::Grants, credential_response::CredentialResponseType, token_request::TokenRequest,
    wallet::PushedAuthorizationResponse,
};
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Client, Request,
};
use serde::Serializer;
use serde_json::json;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

pub async fn send_token_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_token_request");

    if let Some((code, client_state)) =
        listen::<AuthorizationCodeReceived>(action).map(|payload| (payload.code, payload.state))
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

        let credential_offer = state.core_utils.active_credential_offer.unwrap();
        let logo_uri = match current_user_prompt {
            CurrentUserPrompt::CredentialOffer { logo_uri, .. } => logo_uri,
            _ => unreachable!(),
        };

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer.clone();

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
                let issuer_name = display["name"]
                    .as_str()
                    .map(ToString::to_string)
                    // TODO(ngdil): Remove this fallback.
                    .or_else(|| display["client_name"].as_str().map(ToString::to_string))
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

        let code_verifier = String::from_utf8(
            state
                .core_utils
                .active_code_verifier
                .clone()
                .expect("FIXME: active_code_verifier should be set"),
        )
        .expect("FIXME: code_verifier should be utf8");

        // Create a token request with grant_type `pre_authorized_code`.
        let token_request = TokenRequest::AuthorizationCode {
            client_id: "unime-client-id".to_string(),
            code,
            code_verifier: Some(code_verifier),
            redirect_uri: Some(
                // "https://website-git-feat-assetlinks-app-site-association-impierce.vercel.app/callback"
                "unime://callback".parse().unwrap(),
            ),
        };

        info!("token_request: {:?}", token_request);

        // Get an access token.
        let token_response = wallet
            .get_access_token(authorization_server_metadata.token_endpoint.unwrap(), token_request)
            .await
            .map_err(GetAccessTokenError)?;

        info!("token_response: {:?}", token_response);

        let credential_configuration_ids = state
            .core_utils
            .active_credential_configuration_ids
            .clone()
            // FIXME
            .unwrap_or_default();

        credential_configurations_supported.retain(|credential_configuration_id, _| {
            credential_configuration_ids.contains(credential_configuration_id)
        });

        let credentials: Vec<(String, serde_json::Value, Vec<serde_json::Value>)> =
            match credential_configuration_ids.len() {
                0 => vec![],
                1 => {
                    let credential_configuration_id = credential_configuration_ids[0].clone();

                    let credential_configuration = credential_configurations_supported
                        .get(&credential_configuration_id)
                        .ok_or(UnknownCredentialConfigurationIdError(
                            credential_configuration_id.clone(),
                        ))?;

                    let nonce = if let Some(ref nonce_endpoint) = credential_issuer_metadata.nonce_endpoint {
                        let nonce = wallet
                            .get_nonce(nonce_endpoint.clone())
                            .await
                            .expect("FIXME: nonce endpoint");

                        Some(nonce)
                    } else {
                        None
                    };

                    info!("nonce: {nonce:?}");

                    // Get the credential.
                    let credential_response = wallet
                        .get_credential(
                            credential_issuer_metadata,
                            &token_response,
                            nonce,
                            credential_configuration_id.clone(),
                            credential_configuration,
                        )
                        .await
                        .map_err(GetCredentialError)?;

                    let credential = match credential_response.credential {
                        // FIXME: fix the type of the credential.
                        CredentialResponseType::Immediate { credentials, .. } => {
                            serde_json::json!(credentials[0].credential)
                        }
                        _ => panic!("Credential was not a jwt_vc_json."),
                    };

                    vec![(
                        credential_configuration_id,
                        credential,
                        credential_configuration.display.clone(),
                    )]
                }
                _batch => {
                    let (credential_configuration_ids, credential_configurations): (Vec<_>, Vec<_>) =
                        credential_configurations_supported.clone().into_iter().unzip();

                    todo!("FIXME: batch credential request");
                }
            };

        info!("credentials: {:?}", credentials);

        let mut history_credentials = vec![];

        for (credential_configuration_id, credential, display) in credentials.into_iter() {
            let mut verifiable_credential_record: VerifiableCredentialRecord = credential.try_into()?;
            verifiable_credential_record
                .display_credential
                .issuer_name
                .clone_from(&issuer_name);
            verifiable_credential_record.display_credential.connection_id = Some(connection.id.clone());

            // Set the display name of the credential.
            verifiable_credential_record.display_credential.display_name = get_credential_display_name(
                &credential_configurations_supported,
                &credential_configuration_id,
                &verifiable_credential_record,
            );

            let key: Uuid = verifiable_credential_record
                .display_credential
                .id
                .parse()
                .expect("invalid uuid");

            info!("generated hash-key: {:?}", key);

            display
                .first()
                .and_then(|display| display.get("logo"))
                .and_then(|logo| logo.get("uri").or_else(|| logo.get("url")))
                .and_then(|uri| uri.as_str())
                .and_then(|uri| persist_asset(&hash(uri), key.to_string().as_str()).ok());

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
        let mut history = state.history;
        if !history_credentials.is_empty() {
            // Only add a `ConnectionAdded` event if the connection was not previously connected.
            if !previously_connected {
                history.push(HistoryEvent {
                    connection_name: connection.name.clone(),
                    event_type: EventType::ConnectionAdded,
                    connection_id: connection.id.clone(),
                    date: connection.last_interacted.clone(),
                    credentials: vec![],
                });
            }
            history.push(HistoryEvent {
                connection_name: connection.name.clone(),
                event_type: EventType::CredentialsAdded,
                connection_id: connection.id.clone(),
                date: connection.last_interacted.clone(),
                credentials: history_credentials,
            });
        }

        drop(state_guard);
        return Ok(AppState {
            connections,
            credentials,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            history,
            core_utils: CoreUtils {
                active_credential_offer: None,
                active_credential_configuration_ids: None,
                active_code_verifier: None,
                ..state.core_utils
            },
            ..state
        });
    }

    Ok(state)
}

/// Helper function to get the display name of a credential.
fn get_credential_display_name(
    credential_configurations_supported: &HashMap<String, CredentialConfigurationsSupportedObject>,
    credential_configuration_id: &str,
    verifiable_credential_record: &VerifiableCredentialRecord,
) -> String {
    credential_configurations_supported
        .get(credential_configuration_id)
        .and_then(|credential_configuration| credential_configuration.display.first())
        // Get the name of the credential from the display property if it exists.
        .and_then(|display| display["name"].as_str())
        .or_else(|| {
            // Else, if the `type` property is a string, use it as the name of the credential.
            verifiable_credential_record.display_credential.data["type"]
                .as_str()
                .or(
                    // Or, if the `type` property is an array, use the last element as the name of the credential.
                    verifiable_credential_record.display_credential.data["type"]
                        .as_array()
                        .and_then(|types| types.last())
                        .and_then(|last_type| last_type.as_str()),
                )
        })
        .map(ToString::to_string)
        // Fallback to `Credential` if the credential is not a valid W3C Verifiable Credential.
        .unwrap_or("Credential".to_string())
}
