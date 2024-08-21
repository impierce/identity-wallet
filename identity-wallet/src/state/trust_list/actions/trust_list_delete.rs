use crate::reducer;
use crate::state::trust_list::reducers::trust_list_delete::trust_list_delete;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListDelete.ts")]
pub struct TrustListDelete {
    pub domain: String,
}

#[typetag::serde(name = "[Trust List] Delete")]
impl ActionTrait for TrustListDelete {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_delete)]
    }
}
