use crate::reducer;
use crate::state::trust_list::reducers::delete_entry::delete_trust_list_entry;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DeleteTrustListEntry.ts")]
pub struct DeleteTrustListEntry {
    pub trust_list_id: String,
    #[ts(type = "string")]
    pub domain: Url,
}

#[typetag::serde(name = "[Trust List] Delete entry")]
impl ActionTrait for DeleteTrustListEntry {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(delete_trust_list_entry)]
    }
}
