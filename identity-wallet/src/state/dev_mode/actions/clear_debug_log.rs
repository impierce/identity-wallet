use crate::{
    reducer,
    state::{actions::ActionTrait, dev_mode::reducers::clear_debug_log::clear_debug_log, Reducer},
};

use serde::{Deserialize, Serialize};

/// Action to clear the debug log.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearDebugLog;

#[typetag::serde(name = "[DEV] Clear debug log")]
impl ActionTrait for ClearDebugLog {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(clear_debug_log)]
    }
}
