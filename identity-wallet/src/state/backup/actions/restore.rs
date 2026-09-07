use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::restore::restore_backup, Reducer},
};

/// Replaces the current profile with the contents of a stored backup.
///
/// This is destructive: the running state, Stronghold snapshot and downloaded
/// assets are overwritten by whatever the archive holds.
#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "bindings/actions/RestoreBackup.ts")]
pub struct RestoreBackup {
    /// Opaque id from the backup store.
    pub id: String,
    pub password: String,
}

impl std::fmt::Debug for RestoreBackup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RestoreBackup")
            .field("id", &self.id)
            .field("password", &"*****")
            .finish()
    }
}

#[typetag::serde(name = "[Backup] Restore")]
impl ActionTrait for RestoreBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(restore_backup)]
    }
}
