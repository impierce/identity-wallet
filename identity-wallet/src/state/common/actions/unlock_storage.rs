use crate::{
    reducer,
    state::{actions::ActionTrait, common::reducers::unlock_storage::unlock_storage, profile_settings::reducers::update_sorting_preference::sort_credentials, Reducer},
};

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use ts_rs::TS;

/// Action to unlock the storage.
#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UnlockStorage.ts")]
pub struct UnlockStorage {
    pub password: String,
}

impl std::fmt::Debug for UnlockStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnlockStorage").field("password", &"*****").finish()
    }
}

#[typetag::serde(name = "[Storage] Unlock")]
impl ActionTrait for UnlockStorage {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(unlock_storage), reducer!(sort_credentials)]
    }
}
