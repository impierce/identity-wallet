use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            helpers::{normalize_connection_url, validate_credential_types},
            history_event::{EventType, HistoryCredential, HistoryEvent},
            ActiveFlow, CoreUtils, DateUtils, Oid4vciStage,
        },
        credentials::{
            actions::authorization_code_received::CodeReceived, holder_binding::ensure_held_binding,
            issuer_signature::verify_issuer_signature, reducers::refresh_credential_status::fetch_credential_status,
            CredentialStatus, CredentialStatusType, DisplayCredential, VerifiableCredentialRecord,
        },
        interaction_result::{InteractionKind, InteractionOutcome, InteractionResult},
        user_prompt::CurrentUserPrompt,
        AppState, UNIME_CLIENT_ID, UNIME_REDIRECT_URI,
    },
    subject::Subject,
    well_known::{get_authorization_server_metadata, get_credential_issuer_metadata},
};
use identity_kit::{did::Did, status_list::Status};
use log::{debug, info, warn};
use oid4vc_core::utils::jwt::get_unverified_jwt_claims;
use oid4vci::{
    credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject,
    credential_response::CredentialResponseType, token_request::TokenRequest,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

/// Redeems the code for credentials and reports the outcome in the result pop-up. Errors from the issuer's token and
/// credential endpoints end the flow with an [`InteractionResult`] instead of a generic error.
#[tracing::instrument(skip_all, err)]
pub async fn send_token_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    if listen::<CodeReceived>(action.clone()).is_none() {
        return Ok(state);
    }

    let issuer_name = match &state.current_user_prompt {
        Some(CurrentUserPrompt::CredentialOffer { issuer_name, .. }) => Some(issuer_name.clone()),
        _ => None,
    }
    .or_else(|| {
        state
            .core_utils
            .pending_connection_data
            .as_ref()
            .map(|pending_connection_data| pending_connection_data.client_metadata.client_name.clone())
    })
    .unwrap_or_default();
    let interaction_result = |outcome| {
        Some(InteractionResult {
            kind: InteractionKind::Issuance,
            counterparty: issuer_name.clone(),
            outcome,
        })
    };

    let previous_state = state.clone();
    match redeem_code_for_credentials(state, action).await {
        Ok(state) => Ok(AppState {
            interaction_result: interaction_result(InteractionOutcome::Approved),
            ..state
        }),
        Err(GetAccessTokenError(error) | GetCredentialError(error)) => {
            let outcome = InteractionOutcome::from_wallet_error(&error);
            warn!("Credential issuance failed: {outcome:?}");
            Ok(AppState {
                core_utils: CoreUtils {
                    active_flow: None,
                    pending_connection_data: None,
                    ..previous_state.core_utils
                },
                current_user_prompt: Some(CurrentUserPrompt::Redirect {
                    target: "me".to_string(),
                }),
                interaction_result: interaction_result(outcome),
                ..previous_state
            })
        }
        Err(error) => Err(error),
    }
}

async fn redeem_code_for_credentials(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some((code, is_pre_authorized, is_interactive, wallet_state, tx_code)) =
        listen::<CodeReceived>(action).map(|payload| {
            (
                payload.code,
                payload.is_pre_authorized,
                payload.is_interactive,
                payload.state,
                payload.tx_code,
            )
        })
    {
        info!(
            "Sending OID4VCI token request (pre_authorized={is_pre_authorized}, interactive={is_interactive}, has_tx_code={})",
            tx_code.is_some()
        );

        let active_oid4vci_flow = match state.core_utils.active_flow.clone() {
            Some(ActiveFlow::Oid4vciOffer {
                stage,
                credential_offer,
                logo_uri,
            }) => (stage, *credential_offer, logo_uri),
            _ => {
                return Err(AppError::FlowExpiredError(
                    "Cannot find Credential Offer in the backend state".to_string(),
                ));
            }
        };

        if !is_interactive && !is_pre_authorized && wallet_state.is_some() {
            let expected_wallet_state = match &active_oid4vci_flow.0 {
                Oid4vciStage::AuthorizationCode { wallet_state, .. } => Some(wallet_state.clone()),
                _ => None,
            };

            if wallet_state != expected_wallet_state {
                return Err(AppError::FlowExpiredError(
                    "The state parameter in the authorization response does not match the active wallet state."
                        .to_string(),
                ));
            }
        } else if !is_interactive && !is_pre_authorized && wallet_state.is_none() {
            return Err(AppError::FlowExpiredError(
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

        let (stage, credential_offer, logo_uri) = active_oid4vci_flow;

        // The credential offer contains the Credential Issuer Identifier.
        let credential_issuer = &credential_offer.credential_issuer;
        debug!("Credential issuer URL: {credential_issuer}");

        // Get the credential issuer metadata.
        let credential_issuer_metadata = get_credential_issuer_metadata(credential_issuer)
            .await
            .map_err(GetCredentialIssuerMetadataError)?;
        let credential_issuer_url = credential_issuer.url();

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
                return Err(AppError::UntrustedError(format!(
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
        let authorization_server_metadata = get_authorization_server_metadata(&authorization_server_url)
            .await
            .map_err(GetAuthorizationServerMetadataError)?;

        debug!("Authorization server metadata: {:?}", authorization_server_metadata);

        let token_request = if is_pre_authorized {
            TokenRequest::PreAuthorizedCode {
                pre_authorized_code: code,
                tx_code,
                authorization_details: None,
            }
        } else {
            let code_verifier = match stage {
                Oid4vciStage::AuthorizationCode { code_verifier, .. }
                | Oid4vciStage::InteractiveAuthorization { code_verifier, .. } => String::from_utf8(code_verifier)
                    .map_err(|_| AppError::FlowExpiredError("Missing code verifier".to_string()))?,
                _ => {
                    return Err(AppError::FlowExpiredError("Missing code verifier".to_string()));
                }
            };

            TokenRequest::AuthorizationCode {
                client_id: UNIME_CLIENT_ID.to_string(),
                code,
                code_verifier: Some(code_verifier),
                redirect_uri: Some(UNIME_REDIRECT_URI.parse().unwrap()),
                authorization_details: None,
            }
        };

        crate::log_payload!("Token request: {token_request:?}");

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

        // `TokenResponse` derives `Debug`, so the access and refresh tokens are only logged as a payload.
        debug!(
            "Received token response (token_type: {}, expires_in: {:?}, refresh_token: {}, authorization_details: {})",
            token_response.token_type,
            token_response.expires_in,
            token_response.refresh_token.is_some(),
            token_response
                .authorization_details
                .as_ref()
                .map_or(0, |authorization_details| authorization_details.len())
        );
        crate::log_payload!("Token response: {token_response:?}");

        // Get the credential issuer display.
        let display = credential_issuer_metadata
            .display
            .as_ref()
            .and_then(|display| display.first().cloned());

        let connection_url = normalize_connection_url(&credential_issuer_url);

        // Get the credential issuer name or use the normalized credential issuer url.
        let issuer_name = display
            .map(|display| {
                display["name"]
                    .as_str()
                    .map(ToString::to_string)
                    // TODO(ngdil): Remove this fallback.
                    .or_else(|| display["client_name"].as_str().map(ToString::to_string))
                    .unwrap_or(connection_url.clone())
            })
            .unwrap_or(connection_url.clone());

        let mut credential_configurations_supported =
            credential_issuer_metadata.credential_configurations_supported.clone();

        let credential_configuration_ids: Vec<String> =
            credential_offer.credential_configuration_ids.iter().cloned().collect();

        credential_configurations_supported.retain(|credential_configuration_id, _| {
            credential_configuration_ids.contains(credential_configuration_id)
        });

        // TODO: currently this is somewhat duplicate since the full ClientMetadata is not stored in the new CurrentUserPrompt::CredentialOffer state, but we should consider storing it there to avoid this duplication.
        // Create or update the connection.
        let mut pending_connection_data = state.core_utils.pending_connection_data.clone().ok_or_else(|| {
            AppError::FlowExpiredError("Expected pending connection data for OID4VCI token request".to_string())
        })?;

        // Every credential is fetched and its issuer signature verified before anything is stored, so that a credential
        // that fails leaves neither a credential nor a connection behind (ADR 0005).
        let mut received_credentials = vec![];

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

            debug!("Proof nonce present: {}", nonce.is_some());
            crate::log_payload!("Proof nonce: {nonce:?}");

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
                    GetCredentialError(err.context(format!(
                        "Failed to get credential for configuration id {credential_configuration_id}"
                    )))
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
                    return Err(AppError::UnsupportedError(
                        "Deferred credential response is not supported yet.".to_string(),
                    ))
                }
            };

            let signing_did = verify_issuer_signature(
                &credential_configuration.credential_format.format(),
                &credential,
                identity_manager,
            )
            .await?;
            debug!("Verified the issuer signature of credential configuration {credential_configuration_id}");

            // A credential bound to another wallet's DID or key is refused before anything is stored.
            ensure_held_binding(
                &credential_configuration.credential_format.format(),
                &credential,
                identity_manager.subject.as_ref(),
            )
            .await?;

            received_credentials.push((
                credential_configuration_id,
                credential_configuration,
                credential,
                signing_did,
            ));
        }

        // An issuer verified with its DID document must have signed every credential with that DID. An unverified
        // issuer is bound to the DID that signed them.
        let connection_did = connection_did(
            pending_connection_data
                .client_metadata
                .client_id
                .as_ref()
                .map(ToString::to_string),
            received_credentials
                .iter()
                .map(|(.., signing_did)| signing_did.as_str()),
        )?;
        if pending_connection_data.client_metadata.client_id.is_none() {
            info!("Binding the unverified issuer {connection_url} to the DID that signed its credentials");
            pending_connection_data.client_metadata.client_id = Some(
                Did::parse(&connection_did)
                    .map_err(|err| AppError::UntrustedError(format!("Failed to parse the signing DID: {err}")))?,
            );
        }

        // Create or update the connection. It is part of the returned state only, so it is dropped with it on an error.
        let previously_connected = state.connections.contains(&connection_did);
        let mut connections = state.connections;
        let mut history = state.history;
        let connection = connections.update_last_interaction_or_insert_new(&pending_connection_data)?;

        // Every record is built and validated before the first one is stored, so that a credential that fails leaves
        // none of the others behind (ADR 0005).
        let mut records = vec![];

        for (credential_configuration_id, credential_configuration, credential, _) in received_credentials {
            let display = credential_configuration
                .credential_metadata
                .as_ref()
                .and_then(|credential_metadata| credential_metadata.display.as_ref())
                .cloned()
                .unwrap_or_default();

            let claims = credential_configuration
                .credential_metadata
                .as_ref()
                .and_then(|credential_metadata| credential_metadata.claims.as_ref())
                .map(|claims| claims.to_vec())
                .unwrap_or_default();

            let mut verifiable_credential_record = VerifiableCredentialRecord::try_new(
                credential_configuration.credential_format.format(),
                credential.clone(),
                claims,
            )?;
            // Validate the credential against its corresponding credential JSON Schema.
            validate_credential_types(&verifiable_credential_record.verifiable_credential)?;

            // The credential status is set only when the credential status claim/property can be found and is in OAuth TSL format.
            // If setting the credential status fails we currently catch the error and simply set the credential status field to None.
            // TODO: we might want to inform the user of this before accepting the credential already
            verifiable_credential_record.display_credential.credential_status =
                get_credential_status(&verifiable_credential_record, &identity_manager.subject).await;

            // Set the issuer name of the credential.
            verifiable_credential_record.display_credential.issuer_name = issuer_name.clone();

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

            let logo_uri = display
                .first()
                .and_then(|display| display.logo.clone())
                .map(|logo| logo.uri.clone());

            records.push((key, verifiable_credential_record, logo_uri));
        }

        let mut history_credentials = vec![];

        for (key, verifiable_credential_record, logo_uri) in records {
            logo_uri.and_then(|uri| persist_asset(&hash(uri.as_str()), key.to_string().as_str()).ok());

            // Remove the old credential from the stronghold if it exists.
            stronghold_manager.remove(key).map_err(StrongholdDeletionError)?;

            stronghold_manager
                .insert(key, json!(verifiable_credential_record).to_string().as_bytes().to_vec())
                .map_err(StrongholdInsertionError)?;

            // Add history event
            history_credentials.push(HistoryCredential::from_credential(&verifiable_credential_record));

            debug!(
                "Received and stored credential `{}`",
                verifiable_credential_record.display_credential.id
            );
            crate::log_payload!(
                "Received credential: {:?}",
                verifiable_credential_record.verifiable_credential
            );
        }

        let credentials: Vec<DisplayCredential> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        if let Some(logo_uri) = logo_uri {
            persist_asset(&hash(logo_uri.as_str()), &connection.id).ok();
        }

        // History
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
                active_flow: None,
                pending_connection_data: None,
                ..state.core_utils
            },
            ..state
        });
    }

    Ok(state)
}

// Helpers

/// The DID of the connection to an issuer whose credentials `signing_dids` signed: `verified`, the DID the issuer was
/// verified with, by its DID document, before the user accepted, which must have signed every credential; or for an
/// unverified issuer (`None`), the single DID that signed them all (ADR 0005).
fn connection_did<'a>(
    verified: Option<String>,
    mut signing_dids: impl Iterator<Item = &'a str>,
) -> Result<String, AppError> {
    let did = match verified {
        Some(did) => did,
        None => signing_dids
            .next()
            .ok_or_else(|| AppError::UntrustedError("No credential was received to verify the issuer".to_string()))?
            .to_string(),
    };
    match signing_dids.find(|signing_did| *signing_did != did) {
        Some(signing_did) => Err(AppError::UntrustedError(format!(
            "A credential is signed by {signing_did}, not by the issuer's DID {did}"
        ))),
        None => Ok(did),
    }
}

/// Helper function to get the display name of a credential.
fn get_credential_display_name(
    credential_configurations_supported: &HashMap<String, CredentialConfigurationsSupportedObject>,
    credential_configuration_id: &str,
    verifiable_credential_record: &VerifiableCredentialRecord,
) -> String {
    credential_configurations_supported
        .get(credential_configuration_id)
        .and_then(|credential_configuration| {
            credential_configuration
                .credential_metadata
                .as_ref()
                .and_then(|credential_metadata| {
                    credential_metadata.display.as_ref().and_then(|display| display.first())
                })
        })
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

/// The credential's status claim/property, read from the places [`get_credential_status`] looks in.
fn credential_status_value(verifiable_credential_record: &VerifiableCredentialRecord) -> Option<serde_json::Value> {
    get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential)
        .ok() // convert Result → Option
        .and_then(|claims| {
            claims.get("status").cloned().or_else(|| {
                verifiable_credential_record
                    .display_credential
                    .data
                    .get("credentialStatus")
                    .cloned()
            })
        })
}

/// Whether the credential has a status claim/property that uses the OAuth Token Status List mechanism, i.e. one with a
/// `status_list` member, even a malformed one. When it does, a `None` from [`get_credential_status`] means that the
/// status could not be checked.
pub fn has_token_status_list_claim(verifiable_credential_record: &VerifiableCredentialRecord) -> bool {
    credential_status_value(verifiable_credential_record)
        .is_some_and(|status_value| status_value.get("status_list").is_some())
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
pub async fn get_credential_status(
    verifiable_credential_record: &VerifiableCredentialRecord,
    subject: &Subject,
) -> Option<CredentialStatus> {
    let status_value = match credential_status_value(verifiable_credential_record) {
        Some(value) => value,
        None => {
            warn!("The credential does not contain a status claim/property");
            return None;
        }
    };

    let Some((idx, uri)) = serde_json::from_value::<Status>(status_value)
        .ok()
        .and_then(|status| status.status_list)
        .and_then(|reference| Some((reference.idx, reference.uri.parse().ok()?)))
    else {
        warn!("The credential status claim/property is not in the OAuth Token Status List format");
        return None;
    };

    // Here we initialize the credential status with UNDEFINED status and an empty last_checked field, these fields will be filled after fetching the status.
    let mut credential_status_data = CredentialStatus {
        status: CredentialStatusType::Undefined,
        idx,
        uri,
        last_checked: String::new(),
    };

    let status = match fetch_credential_status(&credential_status_data, subject).await {
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
    use oid4vci::credential_issuer::credential_configurations_supported::{
        CredentialConfigurationsSupportedDisplay, CredentialMetadata,
    };

    #[test]
    fn display_name_is_successfully_read_from_credential_configuration() {
        let credential_configuration_id = "credential_configuration_id";

        // Credential configuration with a display name.
        let credential_configurations_supported = HashMap::from_iter(vec![(
            credential_configuration_id.to_string(),
            CredentialConfigurationsSupportedObject {
                credential_metadata: Some(CredentialMetadata {
                    display: Some(vec![CredentialConfigurationsSupportedDisplay {
                        name: "Credential Name".to_string(),
                        locale: None,
                        logo: None,
                        description: None,
                        background_image: None,
                        background_color: None,
                        text_color: None,
                    }]),
                    claims: Some(vec![]),
                }),
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
            CredentialConfigurationsSupportedObject::default(),
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
            CredentialConfigurationsSupportedObject::default(),
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

/// Issuance from a mock issuer whose credentials a did:jwk signs, with and without a DID document (ADR 0005).
#[cfg(test)]
mod issuance_tests {
    use super::*;
    use crate::persistence::STRONGHOLD;
    use crate::state::core_utils::{IdentityManager, Managers, PendingConnectionData};
    use crate::state::did::validate_domain_linkage::ValidationStatus;
    use crate::state::user_prompt::ClientMetadata;
    use crate::state::{SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
    use crate::stronghold::StrongholdManager;
    use crate::subject::subject;

    use identity_kit::did::DidMethod;
    use identity_kit::identity::{Identity, IdentityKind, KeyStoreIdentity};
    use identity_kit::jose::{encode_compact, JwsAlgorithm, JwsHeader, KeyType};
    use identity_kit::keys::{InMemoryKeyStore, KeySpec, KeyStore};
    use identity_kit::methods::DidJwk;
    use identity_kit::sd_jwt_vc::{ClaimPath, SdJwt, SdJwtBuilder};
    use oid4vc_manager::ProviderManager;
    use oid4vci::credential_offer::CredentialOfferParameters;
    use oid4vci::Wallet;
    use std::sync::Arc;
    use tempfile::NamedTempFile;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const CONFIGURATION_ID: &str = "pid";

    /// An issuer that signs with a did:jwk held in memory.
    struct TestIssuer {
        store: Arc<InMemoryKeyStore>,
        key: identity_kit::keys::KeyId,
        identity: KeyStoreIdentity,
        did: String,
        kid: String,
    }

    impl TestIssuer {
        async fn new() -> Self {
            let store = Arc::new(InMemoryKeyStore::new());
            let key = store
                .generate(KeySpec::new(KeyType::P256, JwsAlgorithm::ES256))
                .await
                .unwrap();
            let identity = KeyStoreIdentity::new(store.clone()).with_producer(DidJwk);
            identity
                .bind(did_jwk(), JwsAlgorithm::ES256, key.clone())
                .await
                .unwrap();
            let did = identity
                .identifier(&did_jwk(), JwsAlgorithm::ES256)
                .await
                .unwrap()
                .to_string();
            let kid = format!("{did}#0");
            TestIssuer {
                store,
                key,
                identity,
                did,
                kid,
            }
        }

        /// The issuer's public key as a JWK.
        async fn public_jwk(&self) -> serde_json::Value {
            serde_json::to_value(self.store.public_jwk(&self.key).await.unwrap()).unwrap()
        }

        /// A `dc+sd-jwt` naming `iss` and `kid`, with `given_name` selectively disclosable, signed by this issuer.
        async fn sd_jwt(&self, iss: &str, kid: &str) -> String {
            self.sd_jwt_with_typ(iss, kid, "dc+sd-jwt").await
        }

        /// As [`Self::sd_jwt`], with the `typ` header `typ`.
        async fn sd_jwt_with_typ(&self, iss: &str, kid: &str, typ: &str) -> String {
            let claims = json!({
                "vct": "https://example.com/pid",
                "iss": iss,
                "iat": chrono::Utc::now().timestamp(),
                "given_name": "Ferris",
            });
            let encoded = SdJwtBuilder::new(claims.as_object().unwrap().clone())
                .conceal(ClaimPath::new(["given_name"]))
                .encode()
                .unwrap();
            let header = JwsHeader::new(JwsAlgorithm::ES256).with_typ(typ).with_kid(kid);
            let signer = self.identity.signer(&did_jwk(), JwsAlgorithm::ES256).await.unwrap();
            let jwt = encode_compact(&header, &serde_json::to_vec(&encoded.payload).unwrap(), signer.as_ref())
                .await
                .unwrap();
            SdJwt::new(identity_kit::vc::Jwt::new(jwt).unwrap(), encoded.disclosures).to_string()
        }
    }

    fn did_jwk() -> IdentityKind {
        IdentityKind::Did(DidMethod::parse("did:jwk").unwrap())
    }

    /// Serves the metadata, token and credential endpoints of an issuer handing out `credential`, and no DID document.
    async fn mount_issuer(credential: &str) -> MockServer {
        let server = MockServer::start().await;
        mount_issuer_on(&server, credential).await;
        server
    }

    async fn mount_issuer_on(server: &MockServer, credential: &str) {
        let uri = server.uri();
        Mock::given(method("GET"))
            .and(path("/.well-known/openid-credential-issuer"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "credential_issuer": uri,
                "credential_endpoint": format!("{uri}/credential"),
                "credential_configurations_supported": {
                    CONFIGURATION_ID: { "format": "dc+sd-jwt", "vct": "https://example.com/pid" }
                }
            })))
            .mount(server)
            .await;
        Mock::given(method("GET"))
            .and(path("/.well-known/oauth-authorization-server"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "issuer": uri,
                "token_endpoint": format!("{uri}/token"),
                "pre-authorized_grant_anonymous_access_supported": true
            })))
            .mount(server)
            .await;
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "access_token": "access", "token_type": "bearer" })),
            )
            .mount(server)
            .await;
        Mock::given(method("POST"))
            .and(path("/credential"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "credentials": [{ "credential": credential }] })),
            )
            .mount(server)
            .await;
    }

    async fn test_managers() -> Arc<tauri::async_runtime::Mutex<Managers>> {
        let stronghold_path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = stronghold_path.as_os_str().into();
        let stronghold_manager = Arc::new(StrongholdManager::create("sup3rSecr3t").unwrap());
        let subject = subject(stronghold_manager.clone()).await.unwrap();
        let provider_manager = ProviderManager::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .unwrap()
        .with_http_client(crate::http_client::get_openid4vc_http_client());
        let wallet = Wallet::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .unwrap()
        .with_http_client(crate::http_client::get_openid4vc_http_client());
        Arc::new(tauri::async_runtime::Mutex::new(Managers {
            stronghold_manager: Some(stronghold_manager),
            identity_manager: Some(IdentityManager {
                subject,
                provider_manager,
                wallet,
            }),
        }))
    }

    /// The state after the user accepted an offer from `server`, verified with the DID `client_id` of its DID document
    /// (`None`: it publishes none).
    fn accepted_offer(
        state: AppState,
        managers: Arc<tauri::async_runtime::Mutex<Managers>>,
        server: &MockServer,
        client_id: Option<&str>,
    ) -> AppState {
        accepted_offer_of(state, managers, server, client_id, &[CONFIGURATION_ID])
    }

    /// As [`accepted_offer`], for an offer of the credential configurations `configuration_ids`.
    fn accepted_offer_of(
        state: AppState,
        managers: Arc<tauri::async_runtime::Mutex<Managers>>,
        server: &MockServer,
        client_id: Option<&str>,
        configuration_ids: &[&str],
    ) -> AppState {
        let credential_offer: CredentialOfferParameters = serde_json::from_value(json!({
            "credential_issuer": server.uri(),
            "credential_configuration_ids": configuration_ids,
            "grants": { "urn:ietf:params:oauth:grant-type:pre-authorized_code": { "pre-authorized_code": "code" } }
        }))
        .unwrap();
        AppState {
            core_utils: CoreUtils {
                managers,
                active_flow: Some(ActiveFlow::Oid4vciOffer {
                    stage: Oid4vciStage::PreAuthorized,
                    credential_offer: Box::new(credential_offer),
                    logo_uri: None,
                }),
                pending_connection_data: Some(PendingConnectionData {
                    client_metadata: ClientMetadata {
                        client_name: "Issuer".to_string(),
                        logo_uri: None,
                        connection_url: server.uri(),
                        redirect_uri: Some(server.uri()),
                        client_id: client_id.map(|did| did.parse().unwrap()),
                    },
                    domain_validation: Some((ValidationStatus::Unknown, DateUtils::new_date_string())),
                    linked_verifiable_presentations: None,
                    ecosystems: None,
                    ecosystems_check: None,
                }),
                ..CoreUtils::default()
            },
            ..state
        }
    }

    fn code_received() -> Action {
        Arc::new(CodeReceived {
            code: "code".to_string(),
            is_pre_authorized: true,
            is_interactive: false,
            state: None,
            tx_code: None,
        })
    }

    async fn stored_credentials(managers: &Arc<tauri::async_runtime::Mutex<Managers>>) -> usize {
        let managers = managers.lock().await;
        let stronghold_manager = managers.stronghold_manager.as_ref().unwrap();
        stronghold_manager.values().unwrap().unwrap_or_default().len()
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn an_unverified_issuer_is_bound_to_the_did_that_signed_its_credential() {
        let issuer = TestIssuer::new().await;
        let server = mount_issuer(&issuer.sd_jwt(&issuer.did, &issuer.kid).await).await;
        let managers = test_managers().await;

        let state = accepted_offer(AppState::default(), managers.clone(), &server, None);
        let state = send_token_request(state, code_received()).await.unwrap();

        assert_eq!(state.connections.0.len(), 1);
        let connection = state.connections.0[0].clone();
        assert_eq!(connection.did, issuer.did);
        assert_eq!(state.credentials.len(), 1);
        assert_eq!(state.credentials[0].connection_id.as_ref(), Some(&connection.id));
        assert_eq!(stored_credentials(&managers).await, 1);

        // A second issuance from the same issuer finds the connection by the DID that signs.
        let state = accepted_offer(state, managers.clone(), &server, None);
        let state = send_token_request(state, code_received()).await.unwrap();
        assert_eq!(state.connections.0.len(), 1);
        assert_eq!(state.connections.0[0].id, connection.id);
        assert_eq!(state.credentials.len(), 2);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn a_credential_whose_signature_does_not_verify_is_refused_and_leaves_nothing_behind() {
        let issuer = TestIssuer::new().await;
        let impostor = TestIssuer::new().await;
        // Names the issuer's DID and key, but is signed with another key.
        let server = mount_issuer(&impostor.sd_jwt(&issuer.did, &issuer.kid).await).await;
        let managers = test_managers().await;

        let state = accepted_offer(AppState::default(), managers.clone(), &server, None);
        let result = send_token_request(state, code_received()).await;

        assert!(
            matches!(&result, Err(AppError::UntrustedError(reason)) if reason.contains("The credential's signature is invalid")),
            "{result:?}"
        );
        assert_eq!(stored_credentials(&managers).await, 0);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn a_credential_that_names_another_issuer_than_its_signer_is_refused() {
        let issuer = TestIssuer::new().await;
        let server = mount_issuer(&issuer.sd_jwt("did:web:other.example", &issuer.kid).await).await;
        let managers = test_managers().await;

        let state = accepted_offer(AppState::default(), managers.clone(), &server, None);
        let result = send_token_request(state, code_received()).await;

        assert!(
            matches!(&result, Err(AppError::UntrustedError(reason)) if reason.contains("its `iss` is not did:jwk")),
            "{result:?}"
        );
        assert_eq!(stored_credentials(&managers).await, 0);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn an_issuer_verified_with_its_did_document_must_sign_with_that_did() {
        let issuer = TestIssuer::new().await;
        let server = mount_issuer(&issuer.sd_jwt(&issuer.did, &issuer.kid).await).await;
        let managers = test_managers().await;

        let state = accepted_offer(
            AppState::default(),
            managers.clone(),
            &server,
            Some("did:web:issuer.example"),
        );
        let result = send_token_request(state, code_received()).await;

        assert!(
            matches!(&result, Err(AppError::UntrustedError(reason)) if reason.contains("not by the issuer's DID did:web:issuer.example")),
            "{result:?}"
        );
        assert_eq!(stored_credentials(&managers).await, 0);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn a_credential_signed_by_the_did_of_the_issuers_did_document_is_stored() {
        let issuer = TestIssuer::new().await;
        let server = mount_issuer(&issuer.sd_jwt(&issuer.did, &issuer.kid).await).await;
        let managers = test_managers().await;

        let state = accepted_offer(AppState::default(), managers.clone(), &server, Some(&issuer.did));
        let state = send_token_request(state, code_received()).await.unwrap();

        assert_eq!(state.connections.0.len(), 1);
        assert_eq!(state.connections.0[0].did, issuer.did);
        assert_eq!(stored_credentials(&managers).await, 1);
    }

    /// A credential's signature is verified only with a key listed under `verificationMethod`, as every other signature
    /// the wallet verifies. A did:web whose key is embedded only in `authentication` is refused; with the same key listed,
    /// the credential is stored.
    #[tokio::test]
    #[serial_test::serial]
    async fn a_credential_signed_with_a_key_embedded_only_in_authentication_is_refused() {
        let issuer = TestIssuer::new().await;
        let public_jwk = issuer.public_jwk().await;

        for listed in [false, true] {
            let server = MockServer::start().await;
            let did = format!("did:web:localhost%3A{}", server.address().port());
            let kid = format!("{did}#key-0");
            let method_json = json!({
                "id": kid,
                "type": "JsonWebKey2020",
                "controller": did,
                "publicKeyJwk": public_jwk,
            });
            let document = if listed {
                json!({ "id": did, "verificationMethod": [method_json], "assertionMethod": [kid] })
            } else {
                json!({ "id": did, "authentication": [method_json] })
            };
            Mock::given(method("GET"))
                .and(path("/.well-known/did.json"))
                .respond_with(ResponseTemplate::new(200).set_body_json(document))
                .mount(&server)
                .await;
            mount_issuer_on(&server, &issuer.sd_jwt(&did, &kid).await).await;
            let managers = test_managers().await;

            let state = accepted_offer(AppState::default(), managers.clone(), &server, None);
            let result = send_token_request(state, code_received()).await;

            if listed {
                let state = result.unwrap();
                assert_eq!(state.connections.0[0].did, did);
                assert_eq!(stored_credentials(&managers).await, 1);
            } else {
                // The key is in the document, embedded, so only the scope of the lookup refuses it.
                let subject = managers.lock().await.identity_manager.as_ref().unwrap().subject.clone();
                assert!(subject.resolve_public_key(&kid).await.is_ok());
                assert!(
                    matches!(&result, Err(AppError::UntrustedError(reason)) if reason.contains("The credential's signature is invalid")),
                    "{result:?}"
                );
                assert_eq!(stored_credentials(&managers).await, 0);
            }
        }
    }

    /// Every credential record is built and validated before the first is stored: when the second of two credentials
    /// is refused, the first, valid and already verified, is not stored either.
    #[tokio::test]
    #[serial_test::serial]
    async fn a_refused_credential_leaves_the_other_credentials_of_the_offer_unstored() {
        let issuer = TestIssuer::new().await;
        let server = MockServer::start().await;
        let uri = server.uri();
        Mock::given(method("GET"))
            .and(path("/.well-known/openid-credential-issuer"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "credential_issuer": uri,
                "credential_endpoint": format!("{uri}/credential"),
                "credential_configurations_supported": {
                    "a_valid": { "format": "dc+sd-jwt", "vct": "https://example.com/pid" },
                    "b_wrong_typ": { "format": "dc+sd-jwt", "vct": "https://example.com/pid" }
                }
            })))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/.well-known/oauth-authorization-server"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "issuer": uri,
                "token_endpoint": format!("{uri}/token"),
                "pre-authorized_grant_anonymous_access_supported": true
            })))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "access_token": "access", "token_type": "bearer" })),
            )
            .mount(&server)
            .await;
        // Both are signed by the issuer; the second carries a `typ` the record refuses.
        for (configuration_id, typ) in [("a_valid", "dc+sd-jwt"), ("b_wrong_typ", "vc+sd-jwt")] {
            let credential = issuer.sd_jwt_with_typ(&issuer.did, &issuer.kid, typ).await;
            Mock::given(method("POST"))
                .and(path("/credential"))
                .and(wiremock::matchers::body_partial_json(
                    json!({ "credential_configuration_id": configuration_id }),
                ))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(json!({ "credentials": [{ "credential": credential }] })),
                )
                .mount(&server)
                .await;
        }
        let managers = test_managers().await;

        let state = accepted_offer_of(
            AppState::default(),
            managers.clone(),
            &server,
            None,
            &["a_valid", "b_wrong_typ"],
        );
        let result = send_token_request(state, code_received()).await;

        assert!(result.is_err(), "{result:?}");
        assert_eq!(stored_credentials(&managers).await, 0);
    }

    #[test]
    fn the_connection_did_is_the_verified_did_or_else_the_one_signing_did() {
        let a = "did:jwk:a";
        let b = "did:jwk:b";
        assert_eq!(connection_did(Some(a.to_string()), [a, a].into_iter()).unwrap(), a);
        assert!(connection_did(Some(a.to_string()), [a, b].into_iter()).is_err());
        assert_eq!(connection_did(None, [b, b].into_iter()).unwrap(), b);
        assert!(connection_did(None, [a, b].into_iter()).is_err());
        assert!(connection_did(None, std::iter::empty()).is_err());
    }
}
