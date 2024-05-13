use crate::{
    reducer,
    state::{actions::ActionTrait, did::reducers::default_method::set_default_did_method, Reducer},
};

use did_manager::DidMethod;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Set the default DID method to be used.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/SetDefaultDidMethod.ts")]
pub struct SetDefaultDidMethod {
    #[ts(type = "string")]
    pub method: DidMethod,
}

#[typetag::serde(name = "[DID] Set default method")]
impl ActionTrait for SetDefaultDidMethod {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_default_did_method)]
    }
}
