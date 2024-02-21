use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::toggle_dev_mode;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ToggleDevMode;

#[typetag::serde(name = "[DEV] Toggle DEV mode")]
impl ActionTrait for ToggleDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(toggle_dev_mode)]
    }
}
