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

    // This line is needed to pass the correct `current_user_prompt` to the new state.
    // The `current_user_prompt` differs depending on where this reducer will be used (currently only in UnlockStorage).
    // So UnlockStorage needs the redirect prompt, most likely future use of this reducer in other locations won't.
    let redirect_prompt = listen::<UnlockStorage>(action).map(|_unlock_storage| CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    Ok(AppState {
        current_user_prompt: redirect_prompt,
        ..state
    })
}
