use crate::{
    reducer,
    state::{actions::ActionTrait, common::reducers::get_state::get_state, Reducer},
};

use serde::{Deserialize, Serialize};

/// Action to retrieve the state from the storage.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetState;

#[typetag::serde(name = "[App] Get state")]
impl ActionTrait for GetState {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(get_state)]
    }
}
