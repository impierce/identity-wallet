use log::info;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        backup::{
            actions::preview::PreviewBackup,
            archive, backup_store,
            preview::{state_from_archive, BackupPreview},
            store::BackupStore,
        },
        AppState,
    },
};

/// Decrypts a backup and reports what it holds, writing nothing.
///
/// Fails with [`crate::error::AppError::BackupArchiveError`] on a wrong password,
/// which is what lets the recovery screen distinguish that from a broken file.
#[tracing::instrument(skip_all, err)]
pub async fn preview_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(PreviewBackup { id, password }) = listen::<PreviewBackup>(action) {
        let bytes = backup_store().read(&id)?;
        let payload = archive::open(&password, &bytes)?;
        let archived = state_from_archive(&payload.state)?;

        let preview = BackupPreview {
            id: id.clone(),
            app_version: payload.app_version,
            profile_name: archived.profile_settings.profile.map(|profile| profile.name),
            credentials: archived.credentials.len() as u32,
            connections: archived.connections.0.len() as u32,
            assets: payload.assets.len() as u32,
        };

        info!("previewed backup `{id}`: {preview:?}");

        return Ok(AppState {
            backup_preview: Some(preview),
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
