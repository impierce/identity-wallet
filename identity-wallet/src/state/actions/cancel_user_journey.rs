use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::cancel_user_journey;

/// Action to cancel the user journey.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CancelUserJourney;

#[typetag::serde(name = "[User Journey] Cancel")]
impl ActionTrait for CancelUserJourney {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_journey)]
    }
}
