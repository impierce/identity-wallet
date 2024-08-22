use crate::reducer;
use crate::state::trust_list::reducers::trust_list_reset::trust_list_reset;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListReset.ts")]
pub struct TrustListReset {}

#[typetag::serde(name = "[Trust List] Reset")]
impl ActionTrait for TrustListReset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_reset)]
    }
}
