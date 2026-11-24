use log::info;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        common::actions::unlock_storage::UnlockStorage,
        credentials::reducers::refresh_credential_status::refresh_statuses,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

/// Refreshes the credential status for all credentials in the state, see [`refresh_statuses`].
#[tracing::instrument(skip_all, err)]
pub async fn refresh_all_credential_statuses(state: AppState, action: Action) -> Result<AppState, AppError> {
    let credential_ids: Vec<String> = state
        .credentials
        .iter()
        .filter(|credential| credential.credential_status.is_some())
        .map(|credential| credential.id.clone())
        .collect();

    info!(
        "Refreshing statuses for {} of {} credentials",
        credential_ids.len(),
        state.credentials.len()
    );

    let (state, summary) = refresh_statuses(state, &credential_ids).await?;

    let message = format!(
        "Refreshing the status of {} credential(s) from {} status list(s) took {}ms ({} failed)",
        summary.refreshed + summary.failed,
        summary.status_lists,
        summary.duration.as_millis(),
        summary.failed
    );
    if summary.failed == 0 {
        crate::debug_log_ok!(state.core_utils.debug_log, "{message}");
    } else {
        crate::debug_log_error!(state.core_utils.debug_log, "{message}");
    }

    // TODO: This is a temporary solution to pass the correct `current_user_prompt` to the new state.
    // The UnlockStorage action needs to end with a redirect prompt to "me", but other actions should end with the current user prompt set to None.
    // RefreshCredentialStatus therefore sets the current user prompt to None, but if this reducer is called by RefreshAllCredentialStatuses as part of an UnlockStorage action,
    // then UnlockStorage needs the redirect prompt. In the future this should be handled when `main_exec` is called.
    let redirect_prompt = listen::<UnlockStorage>(action).map(|_unlock_storage| CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    Ok(AppState {
        current_user_prompt: redirect_prompt,
        ..state
    })
}
