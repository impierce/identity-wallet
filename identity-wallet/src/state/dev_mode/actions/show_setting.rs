use crate::{
    reducer,
    state::{actions::ActionTrait, dev_mode::reducers::show_setting::show_setting, Reducer},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to show the DEV mode toggle in app settings.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/dev_mode/ShowDevModeSetting.ts")]
pub struct ShowDevModeSetting {
    pub show: bool,
}

#[typetag::serde(name = "[DEV] Show DEV mode setting")]
impl ActionTrait for ShowDevModeSetting {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(show_setting)]
    }
}
