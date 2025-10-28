use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::{actions::share_to_linkedin::ShareToLinkedIn, DisplayCredential},
        AppState,
    },
};

use chrono::{DateTime, Datelike};
use log::info;
use tauri_plugin_opener::OpenerExt;
use url::Url;
use urlencoding::encode;

pub async fn share_to_linkedin(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(share_to_linkedin) = listen::<ShareToLinkedIn>(action) {
        let mut credentials = state.credentials.clone();
        let mut credential = credentials
            .iter_mut()
            .find(|cred| cred.id == share_to_linkedin.id)
            .ok_or(AppError::NoCredentialWithIdError(share_to_linkedin.id))?;

        // Build LinkedIn URL, all parameters must be URL percent-encoded
        let mut linkedin_url = encode("https://www.linkedin.com/profile/add?startTask=CERTIFICATION_NAME").into_owned();
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

        // Get or create public link
        let public_link = if let Some(existing_link) = credential.public_link.clone() {
            existing_link.clone()
        } else {
            create_public_link(credential).await?.to_string()
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

        // return Ok(AppState {
        //     credentials,
        //     current_user_prompt: redirect_prompt,
        //     ..state
        // });
    }

    Ok(state)
}

pub async fn create_public_link(credential: &DisplayCredential) -> Result<Url, AppError> {
    // placeholder return
    Ok(Url::parse("https://example.com").unwrap())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_share_to_linkedin() {}
}
