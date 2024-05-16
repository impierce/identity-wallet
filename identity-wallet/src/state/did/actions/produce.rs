use crate::{
    reducer,
    state::{actions::ActionTrait, did::reducers::produce::produce, Reducer},
};

use did_manager::DidMethod;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Produces the DID for a given method.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/ProduceDid.ts")]
pub struct ProduceDid {
    #[ts(type = "string")]
    pub method: DidMethod,
}

#[typetag::serde(name = "[DID] Produce")]
impl ActionTrait for ProduceDid {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(produce)]
    }
}
