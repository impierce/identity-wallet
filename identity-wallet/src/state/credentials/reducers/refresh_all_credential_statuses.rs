use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

// TODO: test for possible poor latency/performance due to many stronghold interactions when many credentials need to be refreshed.
/// Refreshes the credential status for all credentials in the state.
pub async fn refresh_all_credential_statuses(state: AppState, _action: Action) -> Result<AppState, AppError> {
    // let mut state = state;

    // // Collect all credential IDs.
    // let credential_ids = state
    //     .credentials
    //     .iter()
    //     .map(|credential| credential.id.clone())
    //     .collect::<Vec<_>>();

    // // Refresh each credential status one by one.
    // for credential_id in credential_ids {}

    Ok(state)

    // Ok(state)
}
