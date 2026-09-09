use serde::{Deserialize, Serialize};

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::create_automatic::create_automatic_backup, Reducer},
};

/// Asks the backend to take a backup if one is due.
///
/// Carries no password: the only one that would work is already held for the
/// unlocked session. The frontend fires this after every unlock and lets the
/// reducer decide whether anything should happen.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAutomaticBackup;

#[typetag::serde(name = "[Backup] Create automatic")]
impl ActionTrait for CreateAutomaticBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(create_automatic_backup)]
    }
}
