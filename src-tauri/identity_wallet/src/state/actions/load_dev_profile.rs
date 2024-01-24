use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::load_dev_profile;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LoadDevProfile;

#[typetag::serde(name = "[DEV] Load profile")]
impl ActionTrait for LoadDevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}
