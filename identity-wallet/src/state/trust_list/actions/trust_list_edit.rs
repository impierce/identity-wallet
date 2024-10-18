use crate::reducer;
use crate::state::trust_list::reducers::trust_list_edit::trust_list_edit;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListsEdit.ts")]
pub struct TrustListsEdit {
    pub trust_list_id: String,
    pub new_display_name: String,
}

#[typetag::serde(name = "[Trust Lists] Edit")]
impl ActionTrait for TrustListsEdit {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_edit)]
    }
}
