use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            helpers::get_unverified_jwt_claims, history_event::{EventType, HistoryCredential, HistoryEvent}, CoreUtils
        },
        credentials::{
            actions::credential_offers_selected::CredentialOffersSelected, DisplayCredential,
            VerifiableCredentialRecord,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use identity_iota::core::ToJson;
use log::{info, warn};
use oid4vc::oid4vci::{
    credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject,
    credential_offer::Grants, credential_response::CredentialResponseType, token_request::TokenRequest,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn send_credential_request(state: AppState, action: Action) -> Result<AppState, AppError> {
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

        // Create a token request with grant_type `pre_authorized_code`.
        let token_request = match credential_offer.grants.clone() {
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

                    // Get the credential.
                    let credential_response = wallet
                        .get_credential(credential_issuer_metadata, &token_response, credential_configuration)
                        .await
                        .map_err(GetCredentialError)?;

                    let credential = match credential_response.credential {
                        CredentialResponseType::Immediate { credential, .. } => credential,
                        _ => panic!("Credential was not a jwt_vc_json."),
                    };

                    // TODO: Validate credential against its corresponding credential configuration.
                    warn!("credential: {:?}", credential);
                    warn!("credential_configuration: {:?}", credential_configuration);
                    warn!("credential_configuration_json: {:?}", credential_configuration.to_json_value());
                    validate_credential_configuration(&credential, &credential_configuration)?;

                    vec![(
                        credential_configuration_id,
                        credential,
                        credential_configuration.display.clone(),
                    )]
                }
                _batch => {
                    let (credential_configuration_ids, credential_configurations): (Vec<_>, Vec<_>) =
                        credential_configurations_supported.clone().into_iter().unzip();

                    let batch_credential_response = wallet
                        .get_batch_credential(credential_issuer_metadata, &token_response, &credential_configurations)
                        .await
                        .map_err(GetBatchCredentialError)?;

                    credential_configuration_ids
                        .into_iter()
                        .zip(batch_credential_response.credential_responses.into_iter())
                        .zip(credential_configurations.into_iter())
                        .filter_map(
                            |((credential_configuration_id, credential_response), credential_configuration)| {
                                match credential_response {
                                    CredentialResponseType::Immediate { credential, .. } => Some((
                                        credential_configuration_id,
                                        credential,
                                        credential_configuration.display,
                                    )),
                                    // TODO: add support for deferred credentials.
                                    CredentialResponseType::Deferred { .. } => None,
                                }
                            },
                        )
                        .collect()
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
                ..state.core_utils
            },
            ..state
        });
    }

    Ok(state)
}

fn validate_credential_configuration(
    credential_jwt: &serde_json::Value,
    credential_configuration: &CredentialConfigurationsSupportedObject,
) -> Result<(), AppError> {
    let credential = get_unverified_jwt_claims(credential_jwt)?.get("vc").ok_or(AppError::InvalidCredentialFormatError)?;
    
    // validate credential against credential configuration
    let credential_def = credential_configuration.credential_format.to_json();
    Ok(())
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
    use identity_iota::core::FromJson;
    use oid4vc::oid4vci::credential_format_profiles::{CredentialFormats, WithParameters};

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

    #[test]
    fn test_cred_config() {
        let cred_jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGVXpJMU5pSXNJbU55ZGlJNklsQXRNalUySWl3aWEybGtJam9pWjJsNVNqSmtaSFZIUmpGd2VsSkVUV3MzV1hKV2NGbG9MV3hWY0dOVVlrcDBYemgxT1RFeVdtSnZTU0lzSW10MGVTSTZJa1ZESWl3aWVDSTZJbUZJVHpSM0xWOTRWMHhuVVRSTlJGWlhVbXBNTWs1cmQzSnNOMGN4VlhwNWJuTnNSak14UjE5SFptc2lMQ0o1SWpvaVRXdzJNbTFsWDBOT1FsWkhXakZtYTBsUE5YWk9Xa1JGVVhoZldHTmpOMWRQVkZWbldrWlhTekZWWnlKOSMwIn0.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVoybDVTakprWkhWSFJqRndlbEpFVFdzM1dYSldjRmxvTFd4VmNHTlVZa3AwWHpoMU9URXlXbUp2U1NJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1GSVR6UjNMVjk0VjB4blVUUk5SRlpYVW1wTU1rNXJkM0pzTjBjeFZYcDVibk5zUmpNeFIxOUhabXNpTENKNUlqb2lUV3cyTW0xbFgwTk9RbFpIV2pGbWEwbFBOWFpPV2tSRlVYaGZXR05qTjFkUFZGVm5Xa1pYU3pGVlp5SjkiLCJzdWIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaU4xRnFhekpTZG1SbGNqTmxVMkZoYW1oeVNHRlZkMjgzU21oalNXNXhlbTVTU1dsVldVSmxiRFJoVFNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SWxCVVpFbENWRUUxTjNrdFMwNHpXVXBXVjNWM2JXSlBja0Z3ZFhoZmFXTjFNekpPV0RKSlRuUnlPR01pTENKNUlqb2lka2cwZG1sck9GUTRkMGx3Y0hSTk1IZFFUamRNYVV0V2RFZHZVME5GV0VaMGNIVmFTMGd3WW0xamJ5SjkiLCJleHAiOjE3NzIwMTk4NjYsIm5iZiI6MTc0MDQ4Mzg2NiwiaWF0IjoxNzQwNDgzODY2LCJqdGkiOiJodHRwczovL3d3dy5kZWZlbnNpZS5ubC9vbmRlcndlcnBlbi9kaWVuamFhciIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9vYi92M3AwL2NvbnRleHQtMy4wLjIuanNvbiJdLCJpZCI6Imh0dHBzOi8vd3d3LmRlZmVuc2llLm5sL29uZGVyd2VycGVuL2RpZW5qYWFyIiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIk9wZW5CYWRnZUNyZWRlbnRpYWwiXSwibmFtZSI6IkRpZW5qYWFyIERlZmVuc2llIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaU4xRnFhekpTZG1SbGNqTmxVMkZoYW1oeVNHRlZkMjgzU21oalNXNXhlbTVTU1dsVldVSmxiRFJoVFNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SWxCVVpFbENWRUUxTjNrdFMwNHpXVXBXVjNWM2JXSlBja0Z3ZFhoZmFXTjFNekpPV0RKSlRuUnlPR01pTENKNUlqb2lka2cwZG1sck9GUTRkMGx3Y0hSTk1IZFFUamRNYVV0V2RFZHZVME5GV0VaMGNIVmFTMGd3WW0xamJ5SjkiLCJ0eXBlIjpbIkFjaGlldmVtZW50U3ViamVjdCJdLCJhY2hpZXZlbWVudCI6eyJpZCI6Imh0dHBzOi8vd3d3LmRlZmVuc2llLm5sL29uZGVyd2VycGVuL2RpZW5qYWFyIiwidHlwZSI6IkFjaGlldmVtZW50IiwiYWxpZ25tZW50IjpbeyJ0eXBlIjoiQWxpZ25tZW50IiwidGFyZ2V0RGVzY3JpcHRpb24iOiJBbHRlciBvbmUncyBhdHRpdHVkZSBvciBiZWhhdmlvdXIgdG8gYWNjb21tb2RhdGUgbW9kaWZpY2F0aW9ucyBpbiB0aGUgd29ya3BsYWNlLiIsInRhcmdldE5hbWUiOiJBZGFwdCB0byBjaGFuZ2UiLCJ0YXJnZXRVcmwiOiJodHRwOi8vZGF0YS5ldXJvcGEuZXUvZXNjby9za2lsbC80OWRlOTk1OC0yYWE0LTRlZWYtYTg5ZC1mZTVkNWJjZDI4YzQifSx7InR5cGUiOiJBbGlnbm1lbnQiLCJ0YXJnZXREZXNjcmlwdGlvbiI6IlNraWxscyBhbmQgY29tcGV0ZW5jZXMgcmVxdWlyaW5nIGluZGl2aWR1YWxzIHRvIHVuZGVyc3RhbmQgYW5kIGNvbnRyb2wgdGhlaXIgb3duIGNhcGFiaWxpdGllcyBhbmQgbGltaXRhdGlvbnMgYW5kIHVzZSB0aGlzIHNlbGYtYXdhcmVuZXNzIHRvIG1hbmFnZSBhY3Rpdml0aWVzIGluIGEgdmFyaWV0eSBvZiBjb250ZXh0cy4gVGhleSBpbmNsdWRlIHRoZSBhYmlsaXR5IHRvIGFjdCByZWZsZWN0aXZlbHkgYW5kIHJlc3BvbnNpYmx5LCB0byBhY2NlcHQgZmVlZGJhY2ssIGFkYXB0aW5nIHRvIGNoYW5nZSBhbmQgdG8gc2VlayBvcHBvcnR1bml0aWVzIGZvciBwZXJzb25hbCBhbmQgcHJvZmVzc2lvbmFsIGRldmVsb3BtZW50LiIsInRhcmdldE5hbWUiOiJTZWxmLURpc2NpcGxpbmUiLCJ0YXJnZXRVcmwiOiJodHRwOi8vZGF0YS5ldXJvcGEuZXUvZXNjby9za2lsbC8wMjFhMjNlMS05MDdlLTQ2MjctYjA1YS01NTVmODg5Y2JiNjUifSx7InR5cGUiOiJBbGlnbm1lbnQiLCJ0YXJnZXREZXNjcmlwdGlvbiI6IkZvbGxvdyBpbnN0cnVjdGlvbnMgdG8gYWNoaWV2ZSBnb2FscyBhbmQgbWVldCBkZWFkbGluZXMuIiwidGFyZ2V0TmFtZSI6IkZvbGxvdyBJbnN0cnVjdGlvbnMiLCJ0YXJnZXRVcmwiOiJodHRwOi8vZGF0YS5ldXJvcGEuZXUvZXNjby9za2lsbC9hYjlmNTUwYy1mYThmLTRmZTgtOWY5Zi1lMDJkNDViZmY1NzAifSx7InR5cGUiOiJBbGlnbm1lbnQiLCJ0YXJnZXREZXNjcmlwdGlvbiI6IlRoZSBvcGVyYXRpb25zIG9mIHRoZSBzdXBwbHkgYW5kIGRlbWFuZCBvZiBnb29kcyBhbmQgcmVjb3VyY2VzIG9uIG1pbGl0YXJ5IGJhc2VzIGFuZCBkdXJpbmcgbWlsaXRhcnkgb3BlcmF0aW9ucyBvbiB0aGUgZmllbGQsIHRoZSBkaXNydXB0aW9uIG9mIGVuZW15IHN1cHBsaWVzLCBjb3N0IGFuYWx5c2lzLCBlcXVpcG1lbnQgZGVtYW5kcywgYW5kIG90aGVyIG1pbGl0YXJ5IGxvZ2lzdGljcyBhY3Rpdml0aWVzLiIsInRhcmdldE5hbWUiOiJNaWxpdGFyeSBMb2dpc3RpY3MiLCJ0YXJnZXRVcmwiOiJodHRwOi8vZGF0YS5ldXJvcGEuZXUvZXNjby9za2lsbC8yY2E0Yzg1Ny0xNDUwLTQwMDQtOWZkYi04NTAwZGVmNmI4ZmEifSx7InR5cGUiOiJBbGlnbm1lbnQiLCJ0YXJnZXREZXNjcmlwdGlvbiI6IlBsYW4gdGhlIHRpbWUgc2VxdWVuY2Ugb2YgZXZlbnRzLCBwcm9ncmFtbWVzIGFuZCBhY3Rpdml0aWVzLCBhcyB3ZWxsIGFzIHRoZSB3b3JrIG9mIG90aGVycy4iLCJ0YXJnZXROYW1lIjoiVGltZSBNYW5hZ2VtZW50IiwidGFyZ2V0VXJsIjoiaHR0cDovL2RhdGEuZXVyb3BhLmV1L2VzY28vc2tpbGwvZDkwMTNlMGUtZTkzNy00M2Q1LWFiNzEtMGU5MTdlZTg4MmI4In1dLCJhY2hpZXZlbWVudFR5cGUiOiJDb21wZXRlbmN5IiwiY3JpdGVyaWEiOnsibmFycmF0aXZlIjoiRm9sbG93IGEgZnVsbCB5ZWFyIG9mIHNlcnZpY2UgYXQgdGhlIER1dGNoIE1pbGl0YXJ5LiJ9LCJkZXNjcmlwdGlvbiI6Ik9uZSB5ZWFyLCB5b3Ugd29yayBhcyBzb2xkaWVyIGF0IHRoZSBNYXJpbmVzLCBBcm15LCBBaXJmb3JjZSwgb3IgTWFyZWNoYXVzc2VlLiAgWW91IGRpc2NvdmVyIHRoZSBpbm5lci13b3JraW5ncyBEdXRjaCBNaWxpdGFyeSwgbGVhcm4gZXZlcnl0aGluZyBhYm91dCB0ZWFtd29yaywgZGVhbGluZyB3aXRoIHNldGJhY2tzLCByZXNwb25zaWJpbGl0eSBhbmQgdHJ1c3RpbmcgeW91ciBvd24gc3RyZW5ndGguIFlvdSB3aWxsIGZvbGxvdyBhIHRyYWluaW5nIGluIG1pbGl0YXJ5IHNraWxscyBhbmQgbGVhcm4gdG8ga25vdyBhbmQgb3BlcmF0ZSBpbXByZXNzaXZlIHdlYXBvbiBzeXN0ZW1zLiBZb3Ugd2lsbCBiZSBjaGFsbGVuZ2VkIGJvdGggcGh5c2ljYWxseSBhbmQgbWVudGFsbHkgdG8gc3RlcCBvdXQgb2YgeW91ciBjb21mb3J0IHpvbmUuIiwibmFtZSI6IkRpZW5qYWFyIn19LCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVoybDVTakprWkhWSFJqRndlbEpFVFdzM1dYSldjRmxvTFd4VmNHTlVZa3AwWHpoMU9URXlXbUp2U1NJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1GSVR6UjNMVjk0VjB4blVUUk5SRlpYVW1wTU1rNXJkM0pzTjBjeFZYcDVibk5zUmpNeFIxOUhabXNpTENKNUlqb2lUV3cyTW0xbFgwTk9RbFpIV2pGbWEwbFBOWFpPV2tSRlVYaGZXR05qTjFkUFZGVm5Xa1pYU3pGVlp5SjkiLCJpc3N1YW5jZURhdGUiOiIyMDI1LTAyLTI1VDExOjQ0OjI2WiJ9fQ.zbCo6pJxiJ48ArcKug5_4zVQ1yvbzq-G2lakl5G2myL_6Y5jx-Bt9toHLWXA0RqCKnRaBGrOoIMVcNMizuA8dg";
        let cred_conf: CredentialFormats<WithParameters> = CredentialFormats::from_json(r#"
            {
                "format": "
                "credential_format": { 
                    "parameters": { 
                        "credential_definition": { 
                            "type_": ["VerifiableCredential", "OpenBadgeCredential"], 
                            "credential_subject": { 
                                "credential_subject": {
                                    "achievement": {
                                        "achievementType": {
                                            "value_type": "string", 
                                            "display": [{
                                                "name": "Achievement Type", 
                                                "locale": "en-US"
                                            }]
                                        }, 
                                        "criteria": {
                                            "narrative": {
                                                "value_type": "string", 
                                                "display":  [{
                                                    "locale": "en-US", 
                                                    "name": "Criteria"
                                                }]
                                            }
                                        },
                                        "alignment": {
                                            "value_type": "array", 
                                            "targetDescription": {
                                                "display": [{
                                                    "locale": "en-US", 
                                                    "name": "Description"
                                                }], 
                                                "value_type": "string"
                                            },
                                            "targetName": {
                                                "display": [{
                                                    "locale": "en-US", 
                                                    "name": "Name"
                                                }], 
                                                "value_type": "string"
                                            }
                                        }
                                    }
                                } 
                            } 
                        }, 
                        "order": null 
                    }
                }
        }
        "#
        ).unwrap();
    }
}
