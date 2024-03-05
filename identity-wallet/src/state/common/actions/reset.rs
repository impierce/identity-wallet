use crate::{
    reducer,
    state::{actions::ActionTrait, common::reducers::reset_state::reset_state, Reducer},
};

use serde::{Deserialize, Serialize};

/// Action to completely purge and reset the app state.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reset;

#[typetag::serde(name = "[App] Reset")]
impl ActionTrait for Reset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_state)]
    }
}
