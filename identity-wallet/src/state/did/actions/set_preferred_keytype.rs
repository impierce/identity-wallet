use crate::{
    reducer,
    state::{actions::ActionTrait, did::reducers::preferred_keytype::set_preferred_key_type, Reducer},
};

use did_manager::DidMethod;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Set the preferred key type to be used.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/SetPreferredKeyType.ts")]
pub struct SetPreferredKeyType {
    pub key_type: String,
}

#[typetag::serde(name = "[Keys] Set preferred key type")]
impl ActionTrait for SetPreferredKeyType {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_preferred_key_type)]
    }
}
