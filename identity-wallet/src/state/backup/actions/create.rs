use std::{fmt::Formatter, path::PathBuf};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::create::create_backup, Reducer},
};

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "bindings/actions/CreateBackup.ts")]
pub struct CreateBackup {
    pub path: String,
    pub password: String,
}

impl std::fmt::Debug for CreateBackup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateBackup")
            .field("path", &self.path)
            .field("password", &"*****")
            .finish()
    }
}

#[typetag::serde(name = "[Backup] Create")]
impl ActionTrait for CreateBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(create_backup)]
    }
}
