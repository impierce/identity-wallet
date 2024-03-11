use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::storage::unlock_storage;
use std::fmt::Formatter;
use ts_rs::TS;

/// Action to unlock the storage.
#[derive(serde::Serialize, serde::Deserialize, TS, Clone)]
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
        vec![reducer!(unlock_storage)]
    }
}
