use crate::reducer;
use crate::state::trust_list::reducers::trust_list_default::trust_list_default;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/TrustListDefault.ts")]
pub struct TrustListDefault {
    pub trust_list_id: String,
}

#[typetag::serde(name = "[Trust Lists] Default")]
impl ActionTrait for TrustListDefault {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(trust_list_default)]
    }
}
