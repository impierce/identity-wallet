use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::load_dev_profile;

use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/LoadDevProfile.ts")]
pub struct LoadDevProfile;

#[typetag::serde(name = "[DEV] Load profile")]
impl ActionTrait for LoadDevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}
