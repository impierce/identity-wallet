use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::enable::enable_backup, Reducer},
};

/// Turns automatic backups on or off.
///
/// Only the preference: turning them on does not take a backup, and turning them
/// off does not delete the ones already stored. Both of those are separate,
/// explicit actions.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/EnableBackup.ts")]
pub struct EnableBackup {
    pub enable: bool,
}

#[typetag::serde(name = "[Backup] Enable")]
impl ActionTrait for EnableBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(enable_backup)]
    }
}
