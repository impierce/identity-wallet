use log::info;
use std::sync::Arc;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        common::actions::unlock_storage::UnlockStorage,
        credentials::{
            actions::refresh_credential_status::RefreshCredentialStatus,
            reducers::refresh_credential_status::refresh_credential_status,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

// TODO: test for possible poor latency/performance due to many stronghold interactions when many credentials need to be refreshed.
/// Refreshes the credential status for all credentials in the state.
#[tracing::instrument(skip_all, err)]
pub async fn refresh_all_credential_statuses(state: AppState, action: Action) -> Result<AppState, AppError> {
    let credential_ids: Vec<String> = state
        .credentials
        .iter()
        .map(|credential| credential.id.clone())
        .collect();

    let mut state = state;

    info!("Refreshing statuses for all {} credentials", credential_ids.len());

    for credential_id in credential_ids {
        state = refresh_credential_status(state, Arc::new(RefreshCredentialStatus { credential_id })).await?;
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
