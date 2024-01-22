use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::get_state;

use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/GetState.ts")]
pub struct GetState;

#[typetag::serde(name = "[App] Get state")]
impl ActionTrait for GetState {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(get_state)]
    }
}
