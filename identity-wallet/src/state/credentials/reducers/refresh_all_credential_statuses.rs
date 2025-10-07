use log::info;
use std::sync::Arc;

use crate::{
    error::AppError,
    state::{
        actions::Action,
        credentials::{
            actions::refresh_credential_status::RefreshCredentialStatus,
            reducers::refresh_credential_status::refresh_credential_status,
        },
        AppState,
    },
};

// TODO: test for possible poor latency/performance due to many stronghold interactions when many credentials need to be refreshed.
/// Refreshes the credential status for all credentials in the state.
pub async fn refresh_all_credential_statuses(state: AppState, _action: Action) -> Result<AppState, AppError> {
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

    Ok(AppState { ..state })
}
