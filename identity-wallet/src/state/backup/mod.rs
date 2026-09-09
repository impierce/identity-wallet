pub mod actions;
pub mod archive;
pub mod preview;
pub mod reducers;
pub mod retention;
pub mod store;

use crate::persistence::BACKUPS_DIR;
use crate::{
    error::AppError,
    state::backup::store::{BackupFile, BackupStore, LocalBackupStore},
};

/// The [`BackupStore`] this build uses. Desktop and mobile both get the local
/// filesystem for now; this is where the cloud provider is swapped in.
pub fn backup_store() -> impl BackupStore {
    LocalBackupStore::new(BACKUPS_DIR.lock().unwrap().clone())
}

/// Lists stored backups, newest first.
///
/// The store returns them unordered by contract, so ordering is applied here
/// rather than being relied upon from the provider.
pub fn list_backups() -> Result<Vec<BackupFile>, AppError> {
    let mut backups = backup_store().list()?;
    backups.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(backups)
}
