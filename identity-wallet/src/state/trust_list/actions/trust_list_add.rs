use crate::reducer;
use crate::state::trust_list::reducers::trust_list_add::trust_list_add;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListAdd.ts")]
pub struct TrustListAdd {
    pub trust_list_name: String,
    pub domain: String,
}

#[typetag::serde(name = "[Trust List] Add")]
impl ActionTrait for TrustListAdd {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_add)]
    }
}
