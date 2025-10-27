use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::actions::share_to_linkedin::ShareToLinkedIn,
        AppState,
    },
};

use chrono::{DateTime, Datelike};
use urlencoding::encode;

pub async fn share_to_linkedin(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(share_to_linkedin) = listen::<ShareToLinkedIn>(action) {
        let mut credentials = state.credentials.clone();
        let mut credential = credentials
            .iter_mut()
            .find(|cred| cred.id == share_to_linkedin.id)
            .ok_or(AppError::NoCredentialWithIdError(share_to_linkedin.id))?;

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

        // &certUrl=https%3A%2F%2Fdocs.microsoft.com%2Fen-us%2Flearn%2Fcertifications%2Fd365-functional-consultant-sales
        // &certId=1234

        // Change credential metadata to include public link info.

        // return Ok(AppState {
        //     credentials,
        //     current_user_prompt: redirect_prompt,
        //     ..state
        // });
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_share_to_linkedin() {}
}
