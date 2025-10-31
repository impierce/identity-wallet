use crate::state::core_utils::helpers::get_unverified_jwt_claims;
use crate::state::credentials::create_public_link_token::create_public_link_token;
use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::{actions::share_to_linkedin::ShareToLinkedIn, VerifiableCredentialRecord},
        AppState,
    },
};
use chrono::{DateTime, Datelike};
use did_manager::Resolver;
use identity_iota::core::ToJson;
use log::{info, warn};
use serde_json::Value;
use tauri_plugin_opener::OpenerExt;
use url::Url;
use urlencoding::encode;
use uuid::Uuid;

pub async fn share_to_linkedin(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(share_to_linkedin) = listen::<ShareToLinkedIn>(action) {
        let mut credentials = state.credentials.clone();
        let credential = credentials
            .iter_mut()
            .find(|cred| cred.id == share_to_linkedin.id)
            .ok_or(AppError::NoCredentialWithIdError(share_to_linkedin.id))?;

        // Build LinkedIn URL, all parameters must be URL percent-encoded
        let mut linkedin_url = "https://www.linkedin.com/profile/add?startTask=CERTIFICATION_NAME".to_string();
        linkedin_url.push_str(format!("&name={}", encode(&credential.display_name)).as_str());
        linkedin_url.push_str(format!("&organizationName={}", encode(&credential.issuer_name)).as_str());

        let issue_date = DateTime::parse_from_rfc3339(&credential.metadata.date_issued)
            .map_err(|e| AppError::Error(e.to_string()))?;
        linkedin_url.push_str(format!("&issueYear={}", issue_date.year()).as_str());
        linkedin_url.push_str(format!("&issueMonth={}", issue_date.month()).as_str());

        if let Some(expiration_date_str) = &credential.metadata.expiration_date {
            let expiration_date =
                DateTime::parse_from_rfc3339(expiration_date_str).map_err(|e| AppError::Error(e.to_string()))?;
            linkedin_url.push_str(format!("&expirationYear={}", expiration_date.year()).as_str());
            linkedin_url.push_str(format!("&expirationMonth={}", expiration_date.month()).as_str());
        }

        // Get or create public link to the credential
        let public_link = if let Some(existing_link) = credential.public_link.clone() {
            existing_link.clone()
        } else {
            create_public_link(&state, &credential.id).await?.to_string()
        };
        linkedin_url.push_str(format!("&certUrl={}", encode(&public_link)).as_str());

        linkedin_url.push_str(format!("&certId={}", encode(&credential.id)).as_str());

        info!("Opening LinkedIn AddToProfile URL in browser: `{linkedin_url}`");
        let app_handle = state
            .core_utils
            .app_handle
            .clone()
            .ok_or(AppError::Error("Tauri app handle is not available".to_string()))?;
        app_handle
            .opener()
            .open_url(linkedin_url, None::<&str>)
            .map_err(|err| AppError::Error(format!("Failed to open URL in browser: {err}")))?;

        credential.public_link = Some(public_link);
        return Ok(AppState { credentials, ..state });
    }

    Ok(state)
}

// Helpers
pub async fn get_credential_record_from_stronghold(
    state: &AppState,
    credential_id: &str,
) -> Result<VerifiableCredentialRecord, AppError> {
    let key: Uuid = credential_id
        .parse()
        .map_err(|_| AppError::Error("Invalid credential ID format".to_string()))?;

    let managers = state.core_utils.managers.lock().await;
    let stronghold_manager = managers
        .stronghold_manager
        .as_ref()
        .ok_or_else(|| AppError::Error("Failed to retrieve stronghold manager".to_string()))?;

    let credential_opt = stronghold_manager
        .get(key)
        .map_err(|_| AppError::Error("Failed to retrieve credential from stronghold".to_string()))?;

    let credential_bytes =
        credential_opt.ok_or_else(|| AppError::Error("Credential not found in stronghold".to_string()))?;

    let credential_json = String::from_utf8(credential_bytes)
        .map_err(|_| AppError::Error("Failed to parse credential data".to_string()))?;

    let vcr: VerifiableCredentialRecord = serde_json::from_str(&credential_json)
        .map_err(|_| AppError::Error("Failed to deserialize credential record".to_string()))?;

    Ok(vcr)
}

pub async fn resolve_issuer_did_endpoint(_state: &AppState, issuer_did: &str) -> Result<String, AppError> {
    let resolver = Resolver::new().await;

    let issuer_document = resolver
        .resolve(issuer_did)
        .await
        .map_err(|_| AppError::Error("Failed to resolve issuer did".to_string()))?;

    let public_credential_endpoint = issuer_document.service().iter().find_map(|service| {
        service
            .type_()
            .contains("PublicEndpointStuff")
            .then(|| {
                info!("Found PublicEndpointPH: {service:#?}");
                service.service_endpoint()
            })
            .and_then(|service_endpoint| service_endpoint.to_json_value().ok())
            .and_then(|endpoint_value| match endpoint_value {
                Value::String(url) => Some(url),
                Value::Object(obj) => obj
                    .get("url")
                    .or_else(|| obj.get("uri"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                _ => {
                    warn!("Unexpected service endpoint format: {endpoint_value:#?}");
                    None
                }
            })
    });
    public_credential_endpoint
        .ok_or_else(|| AppError::Error("No public credential endpoint found in issuer DID document".to_string()))
}

pub async fn create_public_link(state: &AppState, credential_id: &str) -> Result<Url, AppError> {
    // Get the VerifiableCredentialRecord from the stronghold
    let vcr = get_credential_record_from_stronghold(state, credential_id).await?;
    // Generate the public link token from the credential
    let public_link_token = create_public_link_token(state, &vcr).await?;

    // Extract the issuer_did from the .aud claim of the token
    let jwt_value = serde_json::Value::String(public_link_token.clone());
    let claims = get_unverified_jwt_claims(&jwt_value)
        .map_err(|_| AppError::Error("Failed to decode public link token".to_string()))?;

    let aud = claims.get("aud").and_then(|v| v.as_str()).ok_or(AppError::Error(
        "Failed to get 'aud' claim from public link token".to_string(),
    ))?;

    let public_credential_endpoint_url = resolve_issuer_did_endpoint(state, &aud).await?;

    // Compile the Issuer public credential endpoint and the public link token into the public link.
    let public_link = format!("{}/{}", public_credential_endpoint_url, public_link_token);
    Url::parse(&public_link).map_err(|e| AppError::Error(format!("Invalid public link URL: {}", e)))
}
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_share_to_linkedin() {}
}
