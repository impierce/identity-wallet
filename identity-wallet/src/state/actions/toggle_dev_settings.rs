use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::toggle_dev_settings; 

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ToggleDevSettings;

#[typetag::serde(name = "[DEV] Toggle DEV settings")]
impl ActionTrait for ToggleDevSettings {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(toggle_dev_settings)]
    }
}
