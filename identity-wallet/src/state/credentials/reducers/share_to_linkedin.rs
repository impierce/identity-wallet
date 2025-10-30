use std::str::FromStr;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::{
            actions::share_to_linkedin::ShareToLinkedIn, create_public_link_token::create_public_link_token,
        },
        AppState,
    },
};

use chrono::{DateTime, Datelike};
use log::info;
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

        // When testing Tauri is often not initialized and the link doesn't actually need to be opened anyway.
        #[cfg(not(feature = "test_utils"))]
        {
            use tauri_plugin_opener::OpenerExt;

            let app_handle = state
                .core_utils
                .app_handle
                .clone()
                .ok_or(AppError::Error("Tauri app handle is not available".to_string()))?;
            app_handle
                .opener()
                .open_url(linkedin_url, None::<&str>)
                .map_err(|err| AppError::Error(format!("Failed to open URL in browser: {err}")))?;
        }

        credential.public_link = Some(public_link);
        return Ok(AppState { credentials, ..state });
    }

    Ok(state)
}

// Helpers

pub async fn create_public_link(state: &AppState, credential_id: &str) -> Result<Url, AppError> {
    let uuid = Uuid::from_str(credential_id).map_err(|e| AppError::Error(e.to_string()))?;
    let public_link_token = create_public_link_token(state, uuid).await?;

    // TODO: Find Issuer public credential endpoint through DID linkedServices, get the DID from the `aud` claim of the token.

    // Compile the Issuer public credential endpoint and the public link token into the public link.
    let public_link = format!("{}/{}", "https://example.org", public_link_token);

    // placeholder return
    Ok(Url::parse(&public_link).unwrap())
}
