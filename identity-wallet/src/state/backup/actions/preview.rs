use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::preview::preview_backup, Reducer},
};

/// Decrypts a backup and summarises it without changing anything on disk.
#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "bindings/actions/PreviewBackup.ts")]
pub struct PreviewBackup {
    /// Opaque id from the backup store.
    pub id: String,
    pub password: String,
}

impl std::fmt::Debug for PreviewBackup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreviewBackup")
            .field("id", &self.id)
            .field("password", &"*****")
            .finish()
    }
}

#[typetag::serde(name = "[Backup] Preview")]
impl ActionTrait for PreviewBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(preview_backup)]
    }
}
