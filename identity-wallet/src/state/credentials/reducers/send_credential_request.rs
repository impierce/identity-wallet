use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            helpers::{validate_credential_types, validate_jwt_vc_json},
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

use identity_iota::credential::Jwt;
use log::info;
use oid4vc::oid4vci::{
    authorization_details::{AuthorizationDetailsObject, CredentialConfigurationOrFormat, OpenidCredential},
    authorization_request::AuthorizationRequest,
    credential_format_profiles::CredentialFormats,
    credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject,
    credential_offer::Grants,
    credential_response::CredentialResponseType,
    pkce,
    token_request::TokenRequest,
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

#[test]
fn test() {
    // Generate a random 128-byte code verifier (must be between 43 and 128 bytes)
    let code_verifier = pkce::code_verifier(128);
    // Generate an encrypted code challenge accordingly
    let code_challenge = pkce::code_challenge(&code_verifier);

    println!("code_challenge: {}", code_challenge);
}

pub async fn send_credential_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_credential_request");

    if let Some(credential_configuration_ids) =
        listen::<CredentialOffersSelected>(action).map(|payload| payload.credential_configuration_ids)
    {
        let state_guard = state.core_utils.managers.lock().await;
        let resolver = state.core_utils.resolver().await;
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

        info!("current_user_prompt: {current_user_prompt:?}");

        let credential_offer = state.core_utils.active_credential_offer.unwrap();
        let logo_uri = match current_user_prompt {
            CurrentUserPrompt::CredentialOffer { logo_uri, .. } => logo_uri,
            _ => unreachable!(),
        };

        info!(
            "credential_offer: {}",
            serde_json::to_string_pretty(&credential_offer).unwrap()
        );

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer.clone();

        info!("credential issuer url: {credential_issuer_url:?}");

        // Get the authorization server metadata.
        let authorization_server_metadata = wallet
            .get_authorization_server_metadata(credential_issuer_url.clone())
            .await
            .map_err(GetAuthorizationServerMetadataError)?;

        info!("authorization server metadata: {authorization_server_metadata:?}");

        // Get the credential issuer metadata.
        let credential_issuer_metadata = wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .map_err(GetCredentialIssuerMetadataError)?;

        info!("credential issuer metadata: {credential_issuer_metadata:?}");

        // Get the credential issuer display.
        let display = credential_issuer_metadata
            .display
            .as_ref()
            .and_then(|display| display.first().cloned());

        info!("credential issuer display: {:?}", display);

        // Get the connection url from the credential issuer url host (or use the credential issuer url if it does not
        // contain a host).
        let connection_url = credential_issuer_url
            .host_str()
            .unwrap_or(credential_issuer_url.as_str());

        info!("connection url: {:?}", connection_url);

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

        info!("issuer name: {:?}", issuer_name);
        info!("credential configuration ids: {:?}", credential_configuration_ids);

        let mut credential_configurations_supported =
            credential_issuer_metadata.credential_configurations_supported.clone();

        credential_configurations_supported.retain(|credential_configuration_id, _| {
            credential_configuration_ids.contains(credential_configuration_id)
        });

        let authorization_details: Vec<AuthorizationDetailsObject> = credential_configurations_supported
            .iter()
            .map(
                |(credential_configuration_id, credential_configuration)| AuthorizationDetailsObject {
                    r#type: OpenidCredential::Type,
                    locations: None,
                    credential_configuration_or_format: CredentialConfigurationOrFormat::CredentialConfigurationId {
                        credential_configuration_id: credential_configuration_id.clone(),
                        parameters: None,
                    },
                    claims: Some(
                        credential_configuration
                            .claims
                            .iter()
                            .map(|claims| claims.clone().into())
                            .collect(),
                    ),
                },
            )
            .collect();

        // Create or update the connection.
        let previously_connected = state.connections.contains(connection_url, &issuer_name);
        let mut connections = state.connections;
        let connection = connections.update_or_insert(connection_url, &issuer_name, None);

        info!("connection: {:?}", connection);

        // Create a token request with grant_type `pre_authorized_code`.
        let token_request = match credential_offer.grants.clone() {
            Some(Grants {
                pre_authorized_code,
                authorization_code,
            }) => {
                if let Some(pre_authorized_code) = pre_authorized_code {
                    TokenRequest::PreAuthorizedCode {
                        pre_authorized_code: pre_authorized_code.pre_authorized_code,
                        tx_code: None,
                    }
                } else if let Some(authorization_code) = authorization_code {
                    // Generate a random 128-byte code verifier (must be between 43 and 128 bytes)
                    let code_verifier = pkce::code_verifier(128);
                    // Generate an encrypted code challenge accordingly
                    let code_challenge = pkce::code_challenge(&code_verifier);

                    let par_endpoint = authorization_server_metadata
                        .pushed_authorization_request_endpoint
                        .expect("Pushed Authorization Request endpoint is required for Authorization Code Grant Type");

                    info!("Authorization code grant type detected");
                    let par_request = build_reqwest_request_manual_body(
                        par_endpoint,
                        authorization_code.issuer_state.unwrap().clone(),
                        authorization_details,
                        code_challenge,
                    )
                    .await
                    .unwrap();

                    info!("par_request: {:?}", par_request);

                    let par_response = reqwest::Client::new()
                        .execute(par_request)
                        .await
                        .unwrap()
                        .json::<serde_json::Value>()
                        .await
                        .unwrap();

                    println!("par_response: {}", serde_json::to_string_pretty(&par_response).unwrap());

                    let par_response = serde_json::from_value(par_response).unwrap();

                    info!("par_response: {:?}", par_response);

                    let app_handle = state.core_utils.app_handle.clone().unwrap();

                    let authorization_endpoint = authorization_server_metadata
                        .authorization_endpoint
                        .expect("Authorization endpoint is required for Authorization Code Grant Type");

                    let url = build_reqwest_request_manual_body2(authorization_endpoint, par_response)
                        .await
                        .unwrap();

                    info!("Opened URL in browser: {:?}", url);
                    app_handle.opener().open_url(url, None::<&str>).unwrap();

                    drop(state_guard);
                    return Ok(AppState {
                        connections,
                        core_utils: CoreUtils {
                            active_credential_offer: Some(credential_offer),
                            active_credential_configuration_ids: Some(credential_configuration_ids),
                            active_code_verifier: Some(code_verifier),
                            ..state.core_utils
                        },
                        // current_user_prompt: Some(CurrentUserPrompt::Redirect {
                        //     target: "me".to_string(),
                        // }),
                        ..state
                    });
                } else {
                    todo!("FIXME")
                }
            }
            None => unreachable!(),
        };

        info!("token_request: {token_request:?}");

        // Get an access token.
        let token_response = wallet
            .get_access_token(authorization_server_metadata.token_endpoint.unwrap(), token_request)
            .await
            .map_err(GetAccessTokenError)?;

        info!("token_response: {token_response:?}");

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
                        .unwrap();

                    let credential = match credential_response.credential {
                        // FIXME: fix the type of the credential.
                        CredentialResponseType::Immediate { credentials, .. } => {
                            serde_json::json!(credentials[0].credential)
                        }
                        _ => panic!("Credential was not a jwt_vc_json."),
                    };

                    info!("credential: {}", serde_json::to_string_pretty(&credential).unwrap());

                    // Convert the received credential (as a string) into a Jwt instance for validation.
                    let credential_jwt = Jwt::new(
                        credential
                            .as_str()
                            .ok_or(AppError::Error("Invalid JWT string.".to_string()))?
                            .to_string(),
                    );
                    // validate_jwt_vc_json(&resolver, credential_jwt).await?;

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

        info!("credentials: {credentials:?}");

        let mut history_credentials = vec![];

        for (credential_configuration_id, credential, display) in credentials.into_iter() {
            let mut verifiable_credential_record: VerifiableCredentialRecord = credential.try_into()?;
            // Validate the credential against its corresponding credential JSON Schema.
            validate_credential_types(&verifiable_credential_record.verifiable_credential)?;

            // Set the issuer name of the credential.
            verifiable_credential_record
                .display_credential
                .issuer_name
                .clone_from(&issuer_name);

            // Set the connection id of the credential.
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

            info!("generated hash-key 2: {key:?}");

            info!(
                "record: {}",
                serde_json::to_string_pretty(&verifiable_credential_record).unwrap()
            );

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
                ..state.core_utils
            },
            ..state
        });
    }

    Ok(state)
}

async fn build_reqwest_request_manual_body(
    par_endpoint: url::Url,
    issuer_state: String,
    authorization_details: Vec<AuthorizationDetailsObject>,
    code_challenge: String,
) -> Result<Request, reqwest::Error> {
    // 1. Define and serialize your PushedAuthorizationRequest data
    let par_data = AuthorizationRequest {
        response_type: "code".to_string(),
        state: Some("test_state".to_string()),
        client_id: "unime-client-id".to_string(),
        redirect_uri: Some(
            // "https://website-git-feat-assetlinks-app-site-association-impierce.vercel.app/callback"
            "unime://callback".parse().unwrap(),
        ),
        code_challenge: Some(code_challenge),
        code_challenge_method: Some("S256".to_string()),
        scope: "openid profile".to_string().into(),
        issuer_state: Some(issuer_state),
        authorization_details,
    };
    let body_string = crate::oid4vci::to_form_urlencoded_string(&par_data).unwrap();

    // 2. Create a reqwest::Client
    let client = Client::new();

    // 4. Build the request
    let request = client
        .post(par_endpoint)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(body_string)
        .build()?;

    Ok(request)
}

async fn build_reqwest_request_manual_body2(
    authorization_endpoint: url::Url,
    par_response: PushedAuthorizationResponse,
) -> Result<String, reqwest::Error> {
    let encoded = serde_urlencoded::to_string(&json!({
        "client_id": "unime-client-id",
        "request_uri": par_response.request_uri.to_string()
    }))
    .unwrap();

    let request_url = format!("{authorization_endpoint}?{encoded}");

    Ok(request_url)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name_is_successfully_read_from_credential_configuration() {
        let credential_configuration_id = "credential_configuration_id";

        // Credential configuration with a display name.
        let credential_configurations_supported = HashMap::from_iter(vec![(
            credential_configuration_id.to_string(),
            CredentialConfigurationsSupportedObject {
                display: vec![json!({"name": "Credential Name"})],
                ..Default::default()
            },
        )]);

        // Credential with a `type` property. The `type` property is a string and it should be ignored in favor of the
        // display name from the credential configuration.
        let verifiable_credential_record = VerifiableCredentialRecord {
            verifiable_credential: Default::default(),
            display_credential: DisplayCredential {
                data: json!({
                    "type": "Credential Type"
                }),
                ..Default::default()
            },
        };

        // Get the display name of the credential.
        let display_name = get_credential_display_name(
            &credential_configurations_supported,
            credential_configuration_id,
            &verifiable_credential_record,
        );

        // Assert that the display name is equal to the display name from the credential configuration.
        assert_eq!(display_name, "Credential Name");
    }

    #[test]
    fn display_name_is_successfully_read_from_credential_type() {
        let credential_configuration_id = "credential_configuration_id";

        // Credential configuration without a display name. The `type` property should be used to get the display name.
        let credential_configurations_supported = HashMap::from_iter(vec![(
            credential_configuration_id.to_string(),
            CredentialConfigurationsSupportedObject {
                display: vec![],
                ..Default::default()
            },
        )]);

        // Credential with a `type` property. The `type` property is a string and it should be used as the display name.
        let verifiable_credential_record = VerifiableCredentialRecord {
            verifiable_credential: Default::default(),
            display_credential: DisplayCredential {
                data: json!({
                    "type": "Credential Type"
                }),
                ..Default::default()
            },
        };

        // Get the display name of the credential.
        let display_name = get_credential_display_name(
            &credential_configurations_supported,
            credential_configuration_id,
            &verifiable_credential_record,
        );

        // Assert that the display name is equal to the `type` property of the credential.
        assert_eq!(display_name, "Credential Type");
    }

    #[test]
    fn display_name_is_successfully_read_from_credential_type_array() {
        let credential_configuration_id = "credential_configuration_id";

        // Credential configuration without a display name. The `type` property should be used to get the display name.
        let credential_configurations_supported = HashMap::from_iter(vec![(
            credential_configuration_id.to_string(),
            CredentialConfigurationsSupportedObject {
                display: vec![],
                ..Default::default()
            },
        )]);

        // Credential with a `type` property. The `type` property is an array and the last element should be used as the
        // display name.
        let verifiable_credential_record = VerifiableCredentialRecord {
            verifiable_credential: Default::default(),
            display_credential: DisplayCredential {
                data: json!({
                    "type": ["Credential Type 1", "Credential Type 2"]
                }),
                ..Default::default()
            },
        };

        // Get the display name of the credential.
        let display_name = get_credential_display_name(
            &credential_configurations_supported,
            credential_configuration_id,
            &verifiable_credential_record,
        );

        // Assert that the display name is equal to the last element of the `type` property of the credential.
        assert_eq!(display_name, "Credential Type 2");
    }
}
