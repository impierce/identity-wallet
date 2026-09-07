use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        backup::{actions::list::ListBackups, list_backups as read_backups},
        AppState,
    },
};

/// Refreshes [`AppState::backups`] from the store.
#[tracing::instrument(skip_all, err)]
pub async fn list_backups(state: AppState, action: Action) -> Result<AppState, AppError> {
    if listen::<ListBackups>(action).is_some() {
        return Ok(AppState {
            backups: read_backups()?,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
