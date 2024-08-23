use crate::reducer;
use crate::state::trust_list::reducers::trust_list_edit::trust_list_edit;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListEdit.ts")]
pub struct TrustListEdit {
    pub trust_list_name: String,
    pub old_domain: String,
    pub new_domain: String,
}

#[typetag::serde(name = "[Trust List] Edit")]
impl ActionTrait for TrustListEdit {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_edit)]
    }
}
