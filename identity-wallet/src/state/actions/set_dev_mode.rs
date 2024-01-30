use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::set_dev_mode;

use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SetDevMode.ts")]
pub struct SetDevMode {
    pub enabled: bool,
}

#[typetag::serde(name = "[DEV] Set dev mode")]
impl ActionTrait for SetDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_dev_mode)]
    }
}
