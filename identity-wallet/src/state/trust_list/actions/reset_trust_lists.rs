use crate::reducer;
use crate::state::trust_list::reducers::reset_trust_lists::reset_trust_lists;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/ResetEntry.ts")]
pub struct ResetTrustLists {}

#[typetag::serde(name = "[Trust Lists] Reset")]
impl ActionTrait for ResetTrustLists {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_trust_lists)]
    }
}
