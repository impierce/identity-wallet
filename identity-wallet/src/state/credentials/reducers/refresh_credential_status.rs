use std::time::Duration;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        core_utils::{DateUtils, IdentityManager},
        credentials::{
            actions::refresh_credential_status::RefreshCredentialStatus, CredentialStatus, VerifiableCredentialRecord,
        },
        AppState,
    },
};
use jsonwebtoken::{decode_header, Algorithm, DecodingKey};
use log::{info, warn};
use oauth_tsl::{
    relying_party::{decompress_gzip, decrypt_status_list_token, StatusListTokenResponseType},
    status_list::{StatusList, StatusType},
};
use reqwest::{header, redirect::Policy, Client};

pub async fn refresh_credential_status(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(refresh_credential_status) = listen::<RefreshCredentialStatus>(action) {
        let state_guard = state.core_utils.managers.lock().await;
        let mut credentials = state.credentials.clone();
        let credential_id = refresh_credential_status.credential_id;

        let credential = credentials.iter_mut().find(|c| c.id == credential_id).ok_or_else(|| {
            // This should never happen, as the credential ID send by the frontend is supposed to be valid.
            warn!("No credential found with id: `{credential_id}`");
            AppError::NoCredentialWithIdError(credential_id.clone())
        })?;

        let credential_status_data = match credential.credential_status.as_mut() {
            Some(data) => data,
            None => {
                // The frontend should already be displaying the fact that there is no credentialStatus for this credential, so only a log message is enough here.
                warn!("No credential status found for credential with id: `{credential_id}`");

                // Abort the function without any changes to the state.
                // No error needed here since the credential_status is optional.
                return Ok(AppState {
                    current_user_prompt: None,
                    ..state.clone()
                });
            }
        };

        let identity_manager = state_guard
            .identity_manager
            .as_ref()
            .ok_or(AppError::MissingManagerError("identity"))?;

        let debug_timestamp_before = chrono::Local::now();

        let display_name = credential.display_name.clone();

        match fetch_credential_status(credential_status_data, identity_manager).await {
            Ok(status) => {
                info!("Successfully fetched credential status for credential with id: `{credential_id}`: `{status:?}` (previous status: `{:?}`)", credential_status_data.status);
                credential_status_data.last_checked = DateUtils::new_date_string();
                credential_status_data.status = status;
                credential_status_data.reachable = true;

                // Update the credential in Stronghold
                {
                    let stronghold_manager = state_guard
                        .stronghold_manager
                        .as_ref()
                        .ok_or(AppError::MissingManagerError("stronghold"))?;

                    let key: uuid::Uuid = credential_id.parse().map_err(AppError::InvalidUuidError)?;

                    let updated_credential = credentials
                        .iter()
                        .find(|c| c.id == credential_id)
                        .ok_or(AppError::NoCredentialWithIdError(credential_id))?;

                    let mut verifiable_credential_record = stronghold_manager
                        .remove(key)
                        .map_err(AppError::StrongholdDeletionError)?
                        .and_then(|data| serde_json::from_slice::<VerifiableCredentialRecord>(data.as_slice()).ok())
                        .ok_or(AppError::StrongholdMissingCredentialError(key))?;

                    verifiable_credential_record.display_credential = updated_credential.clone();

                    stronghold_manager
                        .insert(
                            key,
                            serde_json::json!(verifiable_credential_record)
                                .to_string()
                                .as_bytes()
                                .to_vec(),
                        )
                        .map_err(AppError::StrongholdInsertionError)?;
                }
            }
            Err(e) => {
                // TODO: check if this arm is reached when flickering happens in credential status
                // This error handling means we don't panic when the refresh_credential_status function fails.
                // Instead we don't bother the user with any of the errors and keep the old status and simply don't update it.
                // However, this is also not ideal. TODO: how to handle a status that consistently fails to refresh?
                warn!("Failed to refresh credential status for credential with id: `{credential_id}`. The current status remains unchanged: `{:?}`. Error: {e}", credential_status_data.status);
                credential_status_data.reachable = false;

                let debug_timestamp_after = chrono::Local::now();
                let debug_duration = debug_timestamp_after - debug_timestamp_before;

                return Ok(AppState {
                    debug_messages: {
                        let mut debug_messages = state.debug_messages.clone();
                        debug_messages.push_back(format!(
                            "Refreshing credential status for `{}` took {:?}ms (failed)",
                            display_name,
                            debug_duration.num_milliseconds()
                        ));
                        debug_messages
                    },
                    current_user_prompt: None,
                    ..state.clone()
                });
            }
        };

        let debug_timestamp_after = chrono::Local::now();
        let debug_duration = debug_timestamp_after - debug_timestamp_before;

        drop(state_guard);

        return Ok(AppState {
            debug_messages: {
                let mut debug_messages = state.debug_messages.clone();
                debug_messages.push_back(format!(
                    "Refreshing credential status for `{}` took {:?}ms (success)",
                    display_name,
                    debug_duration.num_milliseconds()
                ));
                debug_messages
            },
            current_user_prompt: None,
            credentials,
            ..state.to_owned()
        });
    }

    Ok(state)
}

// Helpers

/// Fetches the Status List Token from the Status Provider URI provided in the credentialStatus, and checks the given index, returning the Status.
/// There are multiple decoding and decompressing steps involved, please refer to the OAuth Token Status List specification for more details.
pub async fn fetch_credential_status(
    credential_status_data: &CredentialStatus,
    identity_manager: &IdentityManager,
) -> Result<StatusType, AppError> {
    let status_list_jwt = fetch_status_list(
        credential_status_data.uri.as_str(),
        // TODO: the response type is hardcoded to be JWT, since we can't handle CWT yet. However when we implement CWT we then need some way to discover what encoding the Status List Provider is using.
        StatusListTokenResponseType::Jwt,
    )
    .await?; // TODO: this error is caught in the caller function and shouldn't flip a status to Invalid already, so the problem is still UniCore's for flickering, let's test this anyway

    let jwt_header = decode_header(&status_list_jwt).map_err(|_| AppError::GetCredentialStatusError)?;
    let key_id = jwt_header.kid.ok_or(AppError::GetCredentialStatusError)?;

    let public_key = identity_manager
        .subject
        .public_key(&key_id)
        .await
        .map_err(|_| AppError::GetCredentialStatusError)?;
    let decoding_key = match jwt_header.alg {
        Algorithm::EdDSA => DecodingKey::from_ed_der(&public_key),
        Algorithm::ES256 => DecodingKey::from_ec_der(&public_key),
        _ => {
            warn!("Unsupported algorithm: {:?}", jwt_header.alg);
            return Err(AppError::GetCredentialStatusError);
        }
    };
    let status_list_token =
        decrypt_status_list_token(&status_list_jwt, decoding_key).map_err(|_| AppError::GetCredentialStatusError)?;
    let status_list: StatusList = status_list_token
        .claims
        .encoded_status_list
        .try_into()
        .map_err(|_| AppError::GetCredentialStatusError)?;

    let status = StatusType::try_from(
        status_list
            .get_status(credential_status_data.idx as usize)
            .map_err(|_| AppError::GetCredentialStatusError)?,
    )
    .map_err(|_| AppError::GetCredentialStatusError)?;

    info!(
        "Successfully fetched status list from `{}` with status: `{:?}`",
        credential_status_data.uri, credential_status_data.status
    );

    Ok(status)
}

/// Sends a status list request to the provided URI and returns the response body as a String.
/// The `accept_header` parameter determines the expected response format (e.g., JWT, compressed JWT).
/// If the response is gzip encoded, it will be decompressed before being returned.
pub async fn fetch_status_list(uri: &str, accept_header: StatusListTokenResponseType) -> Result<String, AppError> {
    // 3xx redirects should be followed, but infinite loops are caught after 5 redirects.
    let client = Client::builder()
        .redirect(Policy::limited(5))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(AppError::FetchCredentialListError)?;

    let res = client
        .get(uri)
        .header(header::ACCEPT, accept_header.to_string())
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(AppError::FetchCredentialListError)?;

    let is_gzipped = res
        .headers()
        .get(header::CONTENT_ENCODING)
        .is_some_and(|encoding| encoding == "gzip");

    let bytes = res.bytes().await.map_err(AppError::FetchCredentialListError)?;

    if is_gzipped {
        decompress_gzip(&bytes).map_err(|e| {
            warn!("Failed to decompress gzip status list: {}", e);
            AppError::GetCredentialStatusError
        })
    } else {
        String::from_utf8(bytes.to_vec()).map_err(|e| {
            warn!("Status list was not valid UTF-8: {}", e);
            AppError::GetCredentialStatusError
        })
    }
}
