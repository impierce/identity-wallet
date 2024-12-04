use crate::reducer;
use crate::state::trust_list::reducers::delete_trust_list::trust_list_delete;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DeleteTrustList.ts")]
pub struct DeleteTrustList {
    pub trust_list_id: String,
}

#[typetag::serde(name = "[Trust Lists] Delete")]
impl ActionTrait for DeleteTrustList {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_delete)]
    }
}
