use crate::reducer;
use crate::state::trust_list::reducers::edit_entry::edit_trust_list_entry;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/EditTrustListEntry.ts")]
pub struct EditTrustListEntry {
    pub trust_list_id: String,
    #[ts(type = "string")]
    pub old_domain: Url,
    #[ts(type = "string")]
    pub new_domain: Url,
}

#[typetag::serde(name = "[Trust List] Edit entry")]
impl ActionTrait for EditTrustListEntry {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(edit_trust_list_entry)]
    }
}
