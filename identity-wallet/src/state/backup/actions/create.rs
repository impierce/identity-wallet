use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::create::create_backup, Reducer},
};

/// Writes an encrypted backup of the current profile to the backup store.
///
/// There is no path: the store decides where a backup lives and hands back an
/// opaque id, the same way the cloud provider does.
#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "bindings/actions/CreateBackup.ts")]
pub struct CreateBackup {
    pub password: String,
}

impl std::fmt::Debug for CreateBackup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateBackup").field("password", &"*****").finish()
    }
}

#[typetag::serde(name = "[Backup] Create")]
impl ActionTrait for CreateBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(create_backup)]
    }
}
