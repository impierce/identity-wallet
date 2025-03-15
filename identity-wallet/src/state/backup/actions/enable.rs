use serde::{Deserialize, Serialize};

use crate::{
    reducer,
    state::{actions::ActionTrait, backup::reducers::enable::enable_backup, Reducer},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnableBackup;

#[typetag::serde(name = "[Backup] Enable")]
impl ActionTrait for EnableBackup {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(enable_backup)]
    }
}
