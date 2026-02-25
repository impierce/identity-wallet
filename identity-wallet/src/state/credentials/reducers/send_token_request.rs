use crate::{
    error::AppError::{self, *},
    jsonschemas::validate_credential_types,
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            helpers::{get_unverified_jwt_claims, validate_jwt_vc_json, ValueToString},
            history_event::{EventType, HistoryCredential, HistoryEvent},
            CoreUtils, DateUtils, IdentityManager,
        },
        credentials::{
            actions::authorization_code_received::CodeReceived,
            reducers::refresh_credential_status::fetch_credential_status, CredentialStatus, DisplayCredential,
            VerifiableCredentialRecord,
        },
        user_prompt::CurrentUserPrompt,
        AppState, UNIME_CLIENT_ID, UNIME_REDIRECT_URI,
    },
};
use log::{info, warn};
use oauth_tsl::{status_list::StatusType, tokens::referenced_token::StatusClaim};
use oid4vc::oid4vci::{
    credential_format_profiles::CredentialFormats,
    credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject,
    credential_response::CredentialResponseType, token_request::TokenRequest,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn send_token_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_token_request");

    if let Some((code, is_pre_authorized, wallet_state, tx_code)) = listen::<CodeReceived>(action)
        .map(|payload| (payload.code, payload.is_pre_authorized, payload.state, payload.tx_code))
    {
        if !is_pre_authorized && wallet_state.is_some() {
            if wallet_state != state.core_utils.active_wallet_state {
                return Err(AppError::Error(
                    "The state parameter in the authorization response does not match the active wallet state."
                        .to_string(),
                ));
            }
        } else if !is_pre_authorized && wallet_state.is_none() {
            return Err(AppError::Error(
                "The state parameter is missing in the authorization response.".to_string(),
            ));
        }

        let state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let identity_manager = state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?;
        let wallet = &identity_manager.wallet;

        let current_user_prompt = state
            .current_user_prompt
            .clone()
            .ok_or(MissingStateParameterError("current user prompt"))?;

        info!("current_user_prompt: {:?}", current_user_prompt);

        let credential_offer = state
            .core_utils
            .active_credential_offer
            .ok_or(AppError::Error("Missing active credential offer".to_string()))?;
        let logo_uri = match current_user_prompt {
            CurrentUserPrompt::CredentialOffer { logo_uri, .. } => logo_uri,
            _ => unreachable!(),
        };

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer.clone();

        info!("credential issuer url: {:?}", credential_issuer_url);

        // Get the credential issuer metadata.
        let credential_issuer_metadata = wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .map_err(GetCredentialIssuerMetadataError)?;

        // Check that the specified authorization servers are listed in the Credential Issuer Metadata's `authorization_servers` parameter.
        let specified_authorization_server = &credential_offer.grants.as_ref().and_then(|grants| {
            if is_pre_authorized {
                grants
                    .pre_authorized_code
                    .as_ref()
                    .and_then(|pre_auth| pre_auth.authorization_server.as_ref())
            } else {
                grants
                    .authorization_code
                    .as_ref()
                    .and_then(|auth_code| auth_code.authorization_server.as_ref())
            }
        });

        if let Some(specified_authorization_server) = specified_authorization_server {
            if !credential_issuer_metadata.authorization_servers.is_empty()
                && !credential_issuer_metadata
                    .authorization_servers
                    .contains(specified_authorization_server)
            {
                return Err(AppError::Error(format!(
                        "The specified authorization server {specified_authorization_server} is not listed in the credential issuer metadata."
                    )));
            }
        }

        // Extract the authorization server selection from the authorization_server parameter in the grant types.
        let authorization_server_url = specified_authorization_server
            .or_else(|| {
                // If no authorization server is specified, fall back to the authorization_servers in the credential issuer metadata.
                // TODO: Users should be able to select their preferred authorization server.
                credential_issuer_metadata.authorization_servers.first()
            })
            .cloned()
            // Fall back to credential issuer url if no authorization server is specified.
            .unwrap_or(credential_issuer_url.clone());

        // Get the authorization server metadata.
        let authorization_server_metadata = wallet
            .get_authorization_server_metadata(authorization_server_url.clone())
            .await
            .map_err(GetAuthorizationServerMetadataError)?;

        info!("authorization server metadata: {:?}", authorization_server_metadata);

        let token_request = if is_pre_authorized {
            TokenRequest::PreAuthorizedCode {
                pre_authorized_code: code,
                tx_code,
            }
        } else {
            let code_verifier = state
                .core_utils
                .active_code_verifier
                .and_then(|code_verifier| String::from_utf8(code_verifier).ok())
                .ok_or(AppError::Error("Missing code verifier".to_string()))?;

            TokenRequest::AuthorizationCode {
                client_id: UNIME_CLIENT_ID.to_string(),
                code,
                code_verifier: Some(code_verifier),
                redirect_uri: Some(UNIME_REDIRECT_URI.parse().unwrap()),
            }
        };

        info!("token_request: {token_request:?}");

        // Get an access token.
        let token_response = wallet
            .get_access_token(
                authorization_server_metadata
                    .token_endpoint
                    .ok_or(AppError::Error(
                        "The authorization server metadata does not contain a token endpoint.".to_string(),
                    ))?
                    .clone(),
                token_request,
            )
            .await
            .map_err(GetAccessTokenError)?;

        info!("token_response: {token_response:?}");

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
                display["name"]
                    .to_clean_string()
                    // TODO(ngdil): Remove this fallback.
                    .or_else(|| display["client_name"].to_clean_string())
                    .unwrap_or(connection_url.to_string())
            })
            .unwrap_or(connection_url.to_string());

        let mut credential_configurations_supported =
            credential_issuer_metadata.credential_configurations_supported.clone();

        let credential_configuration_ids = state
            .core_utils
            .active_credential_configuration_ids
            .ok_or_else(|| AppError::Error("Missing active credential configuration ids".to_string()))?
            .clone();

        credential_configurations_supported.retain(|credential_configuration_id, _| {
            credential_configuration_ids.contains(credential_configuration_id)
        });

        let mut credentials = vec![];
        for credential_configuration_id in credential_configuration_ids {
            let credential_configuration = credential_configurations_supported
                .get(&credential_configuration_id)
                .ok_or(UnknownCredentialConfigurationIdError(
                    credential_configuration_id.clone(),
                ))?;

            // Get a nonce if the credential issuer metadata contains a nonce endpoint.
            let nonce = if let Some(nonce_endpoint) = &credential_issuer_metadata.nonce_endpoint {
                let nonce = wallet.get_nonce(nonce_endpoint.clone()).await.map_err(|err| {
                    AppError::Error(format!("Failed to get nonce from endpoint {nonce_endpoint}: {err}"))
                })?;

                Some(nonce)
            } else {
                None
            };

            info!("nonce: {nonce:?}");

            // Determine if the token request was pre-authorized and if anonymous access is supported.
            // See: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-15.html#section-8.2.1.1-2.2.2.2
            let with_anonymous_access = is_pre_authorized
                && authorization_server_metadata
                    .pre_authorized_grant_anonymous_access_supported
                    .unwrap_or(false);

            // TODO: all code related to sending the actual credential request(s) should be moved to a separate reducer.
            // Get the credential.
            let credential_response = wallet
                .get_credential(
                    credential_issuer_metadata.clone(),
                    &token_response,
                    nonce,
                    credential_configuration_id.clone(),
                    credential_configuration,
                    with_anonymous_access,
                )
                .await
                .map_err(|err| {
                    AppError::Error(format!(
                        "Failed to get credential for configuration id \"{credential_configuration_id}\": {err}"
                    ))
                })?;

            let credential = match credential_response.credential {
                CredentialResponseType::Immediate { credentials, .. } => {
                    serde_json::json!(
                        credentials
                            // TODO: handle batch credential issuance. See: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-15.html#section-11.2.3-2.8.1
                            .first()
                            .ok_or(AppError::Error(
                                "No credentials found in the credential response.".to_string()
                            ))?
                            .credential
                    )
                }
                CredentialResponseType::Deferred { .. } => {
                    return Err(AppError::Error(
                        "Deferred credential response is not supported yet.".to_string(),
                    ))
                }
            };

            // TODO: add validation for other credential formats.
            if credential_configuration.credential_format.format() == CredentialFormats::JwtVcJson(()) {
                // Convert the received credential (as a string) into a Jwt instance for validation.
                let credential_jwt = credential
                    .as_str()
                    .ok_or(AppError::Error("Invalid JWT string.".to_string()))?;
                validate_jwt_vc_json(credential_jwt, identity_manager).await?;
            }

            credentials.push((
                credential_configuration_id,
                credential,
                credential_configuration.display.clone(),
                credential_configuration.claims.clone(),
            ));
        }

        info!("credentials: {credentials:?}");

        // Create or update the connection.
        let previously_connected = state.connections.contains(connection_url, &issuer_name);
        let mut connections = state.connections;
        let connection = connections.update_or_insert(connection_url, &issuer_name, None);

        let mut history_credentials = vec![];

        for (credential_configuration_id, credential, display, claims) in credentials.into_iter() {
            let mut verifiable_credential_record = VerifiableCredentialRecord::try_new(credential, claims)?;
            // Validate the credential against its corresponding credential JSON Schema.
            validate_credential_types(&verifiable_credential_record.display_credential.data)?;

            // The credential status is set only when the credential status claim/property can be found and is in OAuth TSL format.
            // If setting the credential status fails we currently catch the error and simply set the credential status field to None.
            // TODO: we might want to inform the user of this before accepting the credential already
            verifiable_credential_record.display_credential.credential_status =
                get_credential_status(&verifiable_credential_record, identity_manager).await;

            // Set the issuer name of the credential.
            verifiable_credential_record
                .display_credential
                .issuer_name
                .clone_from(&issuer_name);

            // Set the connection ID of the credential.
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
                .map_err(|err| AppError::Error(format!("Failed to parse credential id as UUID: {err}")))?;

            display
                .first()
                .and_then(|display| display.logo.clone())
                .map(|logo| logo.uri.clone())
                .and_then(|uri| persist_asset(&hash(uri.as_str()), key.to_string().as_str()).ok());

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

// Helpers

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
        .map(|display| display.name.clone())
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
                .map(ToString::to_string)
        })
        // Fallback to `Credential` if the credential is not a valid W3C Verifiable Credential.
        .unwrap_or("Credential".to_string())
}

/// Helper function to fetch the credential status of a newly received credential and set the fields the `credential_status` field of the DisplayCredential.
/// Currently supports only the OAuth Token Status List mechanism.
/// The function looks for the credential status info in 2 places:
/// 1. In the JWT root for the key `status` as specified in the IETF OAuth Token Status List specification.
/// 2. In the `credentialStatus` property of the credential, as specified in the W3C Verifiable Credential Data Model specification (1.1 and 2.0).
///     * How to fill in the `credentialStatus` property is not specified in the W3C VC Data Model specifications for the OAuth Token Status List mechanism.
///       We decided the most logical way is to assume this should be exactly the same as the `status` claim in the JWT root.
///       There is a discussion ongoing in the DIIP profile community about this, see: https://github.com/FIDEScommunity/DIIP/issues/60
///
/// An error is returned when:
/// 1. The credential does not contain a status claim in the JWT root or a credentialStatus property in the VC.
/// 2. The status claim/property does not use the OAuth Token Status List mechanism.
async fn get_credential_status(
    verifiable_credential_record: &VerifiableCredentialRecord,
    identity_manager: &IdentityManager,
) -> Option<CredentialStatus> {
    let status_value = get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential)
        .ok() // convert Result → Option
        .and_then(|claims| {
            claims.get("status").cloned().or_else(|| {
                verifiable_credential_record
                    .display_credential
                    .data
                    .get("credentialStatus")
                    .cloned()
            })
        });

    let status_value = match status_value {
        Some(value) => value,
        None => {
            warn!("The credential does not contain a status claim/property");
            return None;
        }
    };

    let credential_status_claim = match serde_json::from_value::<StatusClaim>(status_value.clone()) {
        Ok(claim) => claim,
        Err(_) => {
            warn!("The credential status claim/property is not in the OAuth Token Status List format: {status_value}");
            return None;
        }
    };

    // Here we initialize the credential status with UNDEFINED status and an empty last_checked field, these fields will be filled after fetching the status.
    let mut credential_status_data = CredentialStatus {
        status: StatusType::UNDEFINED,
        idx: credential_status_claim.referenced_status_list.idx,
        uri: credential_status_claim.referenced_status_list.uri,
        last_checked: String::new(),
    };

    let status = match fetch_credential_status(&credential_status_data, identity_manager).await {
        Ok(status) => status,
        Err(_) => {
            warn!("Failed to fetch credential status");
            return None;
        }
    };
    credential_status_data.status = status;
    credential_status_data.last_checked = DateUtils::new_date_string();

    info!(
        "Successfully set credential status for credential with id: `{}`",
        verifiable_credential_record.display_credential.id
    );

    Some(credential_status_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use oid4vc::oid4vci::credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedDisplay;

    #[test]
    fn display_name_is_successfully_read_from_credential_configuration() {
        let credential_configuration_id = "credential_configuration_id";

        // Credential configuration with a display name.
        let credential_configurations_supported = HashMap::from_iter(vec![(
            credential_configuration_id.to_string(),
            CredentialConfigurationsSupportedObject {
                display: vec![CredentialConfigurationsSupportedDisplay {
                    name: "Credential Name".to_string(),
                    locale: None,
                    logo: None,
                    description: None,
                    background_image: None,
                    background_color: None,
                    text_color: None,
                }],
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
