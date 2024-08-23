use crate::reducer;
use crate::state::trust_list::reducers::trust_list_toggle::trust_list_toggle;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListToggle.ts")]
pub struct TrustListToggle {
    pub trust_list_name: String,
    pub domain: String,
}

#[typetag::serde(name = "[Trust List] Toggle")]
impl ActionTrait for TrustListToggle {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_toggle)]
    }
}
