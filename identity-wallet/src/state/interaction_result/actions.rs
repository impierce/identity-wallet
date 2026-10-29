use crate::{
    reducer,
    state::{actions::ActionTrait, interaction_result::reducers::dismiss_interaction_result, Reducer},
};

use serde::{Deserialize, Serialize};

/// Action to dismiss the result pop-up of the last interaction.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DismissInteractionResult;

#[typetag::serde(name = "[Interaction Result] Dismiss")]
impl ActionTrait for DismissInteractionResult {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(dismiss_interaction_result)]
    }
}
