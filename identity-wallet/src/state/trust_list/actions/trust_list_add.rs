use crate::reducer;
use crate::state::trust_list::reducers::trust_list_add::trust_list_add;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListsAdd.ts")]
pub struct TrustListsAdd {
    pub display_name: String,
}

#[typetag::serde(name = "[Trust Lists] Add")]
impl ActionTrait for TrustListsAdd {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_add)]
    }
}
