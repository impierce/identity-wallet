use serde::{Deserialize, Serialize};

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::list::list_backups, Reducer},
};

/// Refreshes the list of backups held in the store.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListBackups;

#[typetag::serde(name = "[Backup] List")]
impl ActionTrait for ListBackups {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(list_backups)]
    }
}
