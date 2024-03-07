use crate::reducer;
use crate::state::user_journey::reducers::cancel_user_journey::cancel_user_journey;
use crate::state::{actions::ActionTrait, Reducer};

/// Action to cancel the user journey.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CancelUserJourney;

#[typetag::serde(name = "[User Journey] Cancel")]
impl ActionTrait for CancelUserJourney {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_journey)]
    }
}
