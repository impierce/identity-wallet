use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        interaction_result::reducers::{continue_to_verifier, dismiss_interaction_result},
        Reducer,
    },
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

/// Action to open the `redirect_uri` a verifier answered an accepted presentation with, when the user chooses to
/// continue there. It closes the result pop-up.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContinueToVerifier;

#[typetag::serde(name = "[Interaction Result] Continue")]
impl ActionTrait for ContinueToVerifier {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(continue_to_verifier)]
    }
}
