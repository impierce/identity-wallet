use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::actions::refresh_credential_status::RefreshCredentialStatus,
        AppState,
    },
};

pub async fn refresh_credential_status(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(refresh_credential_status) = listen::<RefreshCredentialStatus>(action) {
        return Ok(AppState { ..state });
    }
    Ok(state)
}
