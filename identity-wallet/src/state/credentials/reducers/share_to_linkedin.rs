use crate::error::AppError;
use crate::state::{
    actions::{listen, Action},
    credentials::actions::share_to_linkedin::ShareToLinkedIn,
    credentials::reducers::public_link::get_or_create_public_link,
    AppState,
};
use chrono::{DateTime, Datelike};
use log::info;

#[tracing::instrument(skip_all, err)]
pub async fn share_to_linkedin(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(share_to_linkedin) = listen::<ShareToLinkedIn>(action) {
        let mut credentials = state.credentials.clone();
        let credential = credentials
            .iter_mut()
            .find(|cred| cred.id == share_to_linkedin.id)
            .ok_or(AppError::NoCredentialWithIdError(share_to_linkedin.id))?;

        info!("Sharing credential `{}` to LinkedIn", credential.display_name);

        // Build LinkedIn URL, all parameters must be URL percent-encoded, as specified by LinkedIn documentation: https://addtoprofile.linkedin.com/
        let mut linkedin_url = "https://www.linkedin.com/profile/add?startTask=CERTIFICATION_NAME".to_string();
        linkedin_url.push_str(format!("&name={}", urlencoding::encode(&credential.display_name)).as_str());
        linkedin_url.push_str(format!("&organizationName={}", urlencoding::encode(&credential.issuer_name)).as_str());

        let issue_date = DateTime::parse_from_rfc3339(&credential.metadata.date_issued)
            .map_err(|e| AppError::Error(format!("Failed to parse issue date: {}", e)))?;
        linkedin_url.push_str(format!("&issueYear={}", issue_date.year()).as_str());
        linkedin_url.push_str(format!("&issueMonth={}", issue_date.month()).as_str());

        if let Some(expiration_date_str) = &credential.metadata.expiration_date {
            let expiration_date = DateTime::parse_from_rfc3339(expiration_date_str)
                .map_err(|e| AppError::Error(format!("Failed to parse expiration date: {}", e)))?;
            linkedin_url.push_str(format!("&expirationYear={}", expiration_date.year()).as_str());
            linkedin_url.push_str(format!("&expirationMonth={}", expiration_date.month()).as_str());
        }

        let public_link = get_or_create_public_link(&state, credential).await?;

        linkedin_url.push_str(format!("&certUrl={}", urlencoding::encode(&public_link)).as_str());
        linkedin_url.push_str(format!("&certId={}", urlencoding::encode(&credential.id)).as_str());

        info!("Opening LinkedIn AddToProfile URL in browser");
        crate::log_payload!("LinkedIn AddToProfile URL: {linkedin_url}");

        // When testing, Tauri is often not initialized and the link doesn't actually need to be opened anyway.
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
                .map_err(|err| AppError::BrowserUnavailableError(format!("Failed to open URL in browser: {err}")))?;
        }

        credential.public_link = Some(public_link);
        return Ok(AppState { credentials, ..state });
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use uuid::Uuid;

    #[cfg(feature = "test_utils")]
    #[tokio::test]
    async fn share_to_linkedin_reducer_finishes_successfully() {
        let credential_id = Uuid::new_v4().to_string();
        let existing_public_link = "https://example.com/verify/existing".to_string();

        let state = AppState {
            credentials: vec![crate::state::credentials::DisplayCredential {
                id: credential_id.clone(),
                issuer_name: "Issuer".to_string(),
                display_name: "Credential".to_string(),
                metadata: crate::state::credentials::CredentialMetadata {
                    date_issued: "2026-01-01T00:00:00Z".to_string(),
                    ..Default::default()
                },
                public_link: Some(existing_public_link.clone()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let action: Action = Arc::new(ShareToLinkedIn { id: credential_id });
        let updated_state = share_to_linkedin(state, action).await.unwrap();

        assert_eq!(updated_state.credentials.len(), 1);
        assert_eq!(
            updated_state.credentials[0].public_link.as_deref(),
            Some(existing_public_link.as_str())
        );
    }
}
