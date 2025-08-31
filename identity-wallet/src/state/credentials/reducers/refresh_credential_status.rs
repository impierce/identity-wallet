use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        common::actions::unlock_storage::UnlockStorage,
        core_utils::{DateUtils, IdentityManager},
        credentials::{
            actions::refresh_credential_status::RefreshCredentialStatus, CredentialStatusData, DisplayCredential,
        },
        AppState,
    },
};
use jsonwebtoken::{decode_header, Algorithm, DecodingKey};
use log::info;
use oauth_tsl::{
    relying_party::{decompress_gzip, decrypt_status_list_token, StatusListTokenResponseType},
    status_list::{StatusList, StatusType},
    tokens::status_list_token::StatusListTyp,
};
use reqwest::{header, redirect::Policy, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use url::Url;

pub async fn refresh_all_credential_statuses(state: AppState, action: Action) -> Result<AppState, AppError> {
    println!("{}:{}", file!(), line!());
    if let Some(_passwrod) = listen::<UnlockStorage>(action) {
        println!("{}:{}", file!(), line!());
        let mut state = state;
        println!("{}:{}", file!(), line!());
        let credentials = state.credentials.clone();

        println!("{}:{}", file!(), line!());
        for DisplayCredential { id, .. } in &credentials {
            println!("{}:{}", file!(), line!());
            let action = Arc::new(RefreshCredentialStatus {
                credential_id: Some(id.clone()),
            });

            println!("{}:{}", file!(), line!());
            state = refresh_credential_status(state.clone(), action).await?;
            // We update the state for each credential to ensure that if one fails, we still attempt to update the others.
            // This is a trade-off between performance and reliability.
            // If we wanted to optimize for performance, we could collect all the futures and run them concurrently,
            // but then a failure in one would prevent others from being updated.
            // Given that status updates are not critical operations, we prioritize reliability here.
        }

        println!("{}:{}", file!(), line!());
        return Ok(state);
    }
    println!("{}:{}", file!(), line!());
    Ok(state)
}

pub async fn refresh_credential_status(state: AppState, action: Action) -> Result<AppState, AppError> {
    println!("{}:{}", file!(), line!());
    if let Some(refresh_credential_status) = listen::<RefreshCredentialStatus>(action) {
        println!("{}:{}", file!(), line!());
        let mut credentials = state.credentials.clone();
        if let Some(credential_id) = refresh_credential_status.credential_id {
            println!("{}:{}", file!(), line!());
            let state_guard = state.core_utils.managers.lock().await;
            println!("{}:{}", file!(), line!());

            println!("{}:{}", file!(), line!());
            if let Some(credential) = credentials.iter_mut().find(|c| c.id == credential_id) {
                println!("{}:{}", file!(), line!());
                if let Some(credential_status_data) = credential.credential_status.as_ref() {
                    // We ok_or() with an error here because when the if let Some() statement above is true, we must have a credentialStatus.
                    println!("{}:{}", file!(), line!());

                    println!("{}:{}", file!(), line!());

                    let new_status =
                        get_credential_status(credential_status_data, state_guard.identity_manager.as_ref().unwrap())
                            .await?;

                    println!("{}:{}", file!(), line!());
                    if credential_status_data.status != new_status {
                        info!(
                            "Credential {} changed credential status from {:?} to {:?}",
                            credential.id, credential_status_data.status, new_status
                        );
                    }

                    println!("{}:{}", file!(), line!());
                    credential.credential_status = Some(CredentialStatusData {
                        status: new_status,
                        last_checked: DateUtils::new_date_string(),
                        ..credential_status_data.clone()
                    });

                    info!("Successfully refreshed credential with id: `{credential_id}`");
                } else {
                    info!("No credentialStatus found for credential with id: `{credential_id}`");
                    // TODO: should this return an error or not?
                }
            } else {
                info!("No credential found with id: `{credential_id}`");
            }
        }
        // If no credential ID is provided, the action is to refresh all credentials
        else {
        }
        return Ok(AppState { credentials, ..state });
    }
    Ok(state)
}

/// Represents the credential status as defined in the OAuth Token Status List specification and the DIIP profile.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TSLCredentialStatus {
    pub id: Url,
    #[serde(rename = "type")]
    pub type_: StatusListTyp,
    pub uri: Url,
    pub idx: usize,
}

// Helpers

/// Fetches the Status List Token from the Status Provider URI provided in the credentialStatus, and checks the given index, returning the Status.
/// There are multiple decoding and decompressing steps involved, please refer to the OAuth Token Status List specification for more details.
pub async fn get_credential_status(
    credential_status_data: &CredentialStatusData,
    identity_manager: &IdentityManager,
) -> Result<StatusType, AppError> {
    let status_list_gzip = fetch_status_list(
        credential_status_data.status_list_uri.as_str(),
        // FIXME
        StatusListTokenResponseType::Jwt,
    )
    .await?;
    let status_list_jwt = decompress_gzip(&status_list_gzip).map_err(|_| AppError::GetCredentialStatusError)?;

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
            panic!("Unsupported algorithm: {:?}", jwt_header.alg);
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
            .get_index(credential_status_data.status_list_index)
            .map_err(|_| AppError::GetCredentialStatusError)?,
    )
    .map_err(|_| AppError::GetCredentialStatusError)?;

    Ok(status)
}

/// Sends a status list request to the provided URI and returns the GZIP compressed JWT string as a Vec<u8>.
pub async fn fetch_status_list(uri: &str, accept_header: StatusListTokenResponseType) -> Result<Vec<u8>, AppError> {
    // 3xx redirects should be followed, but infinite loops are caught after 5 redirects.
    let client = Client::builder().redirect(Policy::limited(5)).build()?;

    let res = client
        .get(uri)
        .header(header::ACCEPT, accept_header.to_string())
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(AppError::GetCredentialStatusError);
    }

    let jwt_bytes = res.bytes().await?;
    let jwt_vec_u8 = jwt_bytes.to_vec();

    Ok(jwt_vec_u8)
}
