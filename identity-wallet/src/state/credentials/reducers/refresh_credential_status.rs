use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        core_utils::{DateUtils, IdentityManager},
        credentials::{actions::refresh_credential_status::RefreshCredentialStatus, CredentialStatusData},
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
use url::Url;

pub async fn refresh_credential_status(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(refresh_credential_status) = listen::<RefreshCredentialStatus>(action) {
        if let Some(credential_id) = refresh_credential_status.credential_id {
            let state_guard = state.core_utils.managers.lock().await;
            let mut credentials = state.credentials.clone();

            if let Some(credential) = credentials.iter_mut().find(|c| c.id == credential_id) {
                if let Some(old_status) = credential.credential_status.as_ref().map(|s| s.status) {
                    // We ok_or() with an error here because when the if let Some() statement above is true, we must have a credentialStatus.
                    println!("credential: {:?}", credential);
                    println!("old status: {:?}", old_status);

                    let credential_status = credential
                        .data
                        .get_mut("credentialStatus")
                        .ok_or(AppError::InvalidCredentialStatusFormatError)?;
                    let new_status =
                        get_credential_status(credential_status, state_guard.identity_manager.as_ref().unwrap())
                            .await?;

                    if old_status != new_status {
                        info!(
                            "Credential {} changed credential status from {:?} to {:?}",
                            credential.id, old_status, new_status
                        );
                    }

                    credential.credential_status = Some(CredentialStatusData {
                        status: new_status,
                        last_checked: DateUtils::new_date_string(),
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
        return Ok(AppState { ..state });
    }
    Ok(state)
}

/// Represents the credential status as defined in the OAuth Token Status List specification and the DIIP profile.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TSLCredentialStatus {
    pub id: Url,
    pub type_: StatusListTyp,
    pub uri: Url,
    pub idx: usize,
}

// Helpers

/// Fetches the Status List Token from the Status Provider URI provided in the credentialStatus, and checks the given index, returning the Status.
/// There are multiple decoding and decompressing steps involved, please refer to the OAuth Token Status List specification for more details.
pub async fn get_credential_status(
    credential_status: &Value,
    identity_manager: &IdentityManager,
) -> Result<StatusType, AppError> {
    if let Ok(tsl_credential_status) = serde_json::from_value::<TSLCredentialStatus>(credential_status.clone()) {
        let status_list_gzip =
            fetch_status_list(tsl_credential_status.uri.as_str(), tsl_credential_status.type_.into()).await?;
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
        let status_list_token = decrypt_status_list_token(&status_list_jwt, decoding_key)
            .map_err(|_| AppError::GetCredentialStatusError)?;
        let status_list: StatusList = status_list_token
            .claims
            .encoded_status_list
            .try_into()
            .map_err(|_| AppError::GetCredentialStatusError)?;

        let status = StatusType::try_from(
            status_list
                .get_index(tsl_credential_status.idx)
                .map_err(|_| AppError::GetCredentialStatusError)?,
        )
        .map_err(|_| AppError::GetCredentialStatusError)?;

        Ok(status)
    } else {
        // TODO: should this error or just print a debug message saying the credentialStatus format is not supported?
        Err(AppError::InvalidCredentialStatusFormatError)
    }
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
