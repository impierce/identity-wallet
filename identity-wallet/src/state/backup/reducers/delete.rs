use log::info;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        backup::{actions::delete::DeleteBackup, backup_store, list_backups, store::BackupStore},
        AppState,
    },
};

/// Permanently removes a backup, then refreshes the listing.
#[tracing::instrument(skip_all, err)]
pub async fn delete_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(DeleteBackup { id }) = listen::<DeleteBackup>(action) {
        backup_store().delete(&id)?;
        info!("deleted backup `{id}`");

        return Ok(AppState {
            backups: list_backups()?,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
