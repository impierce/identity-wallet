use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::delete::delete_backup, Reducer},
};

/// Permanently removes one backup from the store.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/DeleteBackup.ts")]
pub struct DeleteBackup {
    /// Opaque id from the backup store.
    pub id: String,
}

#[typetag::serde(name = "[Backup] Delete")]
impl ActionTrait for DeleteBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(delete_backup)]
    }
}
