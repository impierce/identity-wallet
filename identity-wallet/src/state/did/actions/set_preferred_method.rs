use crate::{
    reducer,
    state::{actions::ActionTrait, did::reducers::preferred_method::set_preferred_did_method, Reducer},
};

use did_manager::DidMethod;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Set the preferred DID method to be used.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/SetPreferredDidMethod.ts")]
pub struct SetPreferredDidMethod {
    #[ts(type = "string")]
    pub method: DidMethod,
}

#[typetag::serde(name = "[DID] Set preferred method")]
impl ActionTrait for SetPreferredDidMethod {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_preferred_did_method)]
    }
}
