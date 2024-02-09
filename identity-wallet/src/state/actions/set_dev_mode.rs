use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::set_dev_profile;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SetDevMode;

#[typetag::serde(name = "[DEV] Set DEV mode")]
impl ActionTrait for SetDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_dev_profile)]
    }
}
