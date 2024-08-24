use crate::reducer;
use crate::state::trust_list::reducers::add_entry::add_trust_list_entry;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/AddTrustListEntry.ts")]
pub struct AddTrustListEntry {
    pub trust_list_name: String,
    pub domain: String,
}

#[typetag::serde(name = "[Trust List] Add Entry")]
impl ActionTrait for AddTrustListEntry {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(add_trust_list_entry)]
    }
}
