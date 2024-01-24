use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::storage::unlock_storage;

use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UnlockStorage.ts")]
pub struct UnlockStorage {
    pub password: String,
}

#[typetag::serde(name = "[Storage] Unlock")]
impl ActionTrait for UnlockStorage {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(unlock_storage)]
    }
}
