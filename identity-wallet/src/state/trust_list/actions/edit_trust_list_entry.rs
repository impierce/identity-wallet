use crate::reducer;
use crate::state::trust_list::reducers::edit_entry::edit_trust_list_entry;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/EditTrustListEntry.ts")]
pub struct EditTrustListEntry {
    pub trust_list_name: String,
    pub old_domain: String,
    pub new_domain: String,
}

#[typetag::serde(name = "[Trust List] Edit Entry")]
impl ActionTrait for EditTrustListEntry {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(edit_trust_list_entry)]
    }
}
