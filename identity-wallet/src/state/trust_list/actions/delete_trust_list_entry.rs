use crate::reducer;
use crate::state::trust_list::reducers::delete_entry::delete_trust_list_entry;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DeleteTrustListEntry.ts")]
pub struct DeleteTrustListEntry {
    pub trust_list_id: String,
    pub domain: String,
}

#[typetag::serde(name = "[Trust List] Delete Entry")]
impl ActionTrait for DeleteTrustListEntry {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(delete_trust_list_entry)]
    }
}
