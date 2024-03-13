use crate::{
    reducer,
    state::{actions::ActionTrait, dev_mode::reducers::toggle_dev_mode::toggle_dev_mode, Reducer},
};

use serde::{Deserialize, Serialize};

/// Action to toggle the DEV mode.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleDevMode;

#[typetag::serde(name = "[DEV] Toggle DEV mode")]
impl ActionTrait for ToggleDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(toggle_dev_mode)]
    }
}
