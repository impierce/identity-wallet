use crate::reducer;
use crate::state::trust_list::reducers::trust_lists_reset::trust_lists_reset;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListsReset.ts")]
pub struct TrustListsReset {}

#[typetag::serde(name = "[Trust Lists] Reset")]
impl ActionTrait for TrustListsReset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_lists_reset)]
    }
}
