use crate::reducer;
use crate::state::trust_list::reducers::toggle_entry::toggle_trust_list_entry;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/ToggleTrustListEntry.ts")]
pub struct ToggleTrustListEntry {
    pub trust_list_id: String,
    pub domain: String,
}

#[typetag::serde(name = "[Trust List] Toggle Entry")]
impl ActionTrait for ToggleTrustListEntry {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(toggle_trust_list_entry)]
    }
}
