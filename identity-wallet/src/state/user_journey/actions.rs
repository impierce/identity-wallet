use crate::{reducer, state::{actions::{ActionTrait, Reducer}, user_journey::reducers::cancel_user_journey}};
use ts_rs::TS;

/// Actions

/// Action to cancel the user journey.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CancelUserJourney.ts")]
pub struct CancelUserJourney;

#[typetag::serde(name = "[User Journey] Cancel")]
impl ActionTrait for CancelUserJourney {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_journey)]
    }
}
