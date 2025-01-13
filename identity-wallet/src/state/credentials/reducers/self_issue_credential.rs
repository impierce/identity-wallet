use log::{info, warn};

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::actions::self_issue_credential::SelfIssueCredential,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

pub async fn self_issue_credential(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(self_issue_credential) = listen::<SelfIssueCredential>(action) {

        // info!("Successfully self-issued credential with id: `{}`", self_issue_credential);

        let redirect_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

        return Ok(AppState {
            current_user_prompt: redirect_prompt,
            ..state
        });
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
}
