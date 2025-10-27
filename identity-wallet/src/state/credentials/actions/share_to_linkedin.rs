use crate::reducer;
use crate::state::credentials::reducers::share_to_linkedin::share_to_linkedin;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/ShareToLinkedIn.ts")]
pub struct ShareToLinkedIn {
    // #[ts(type = "string")]
    // pub id: String,
}

#[typetag::serde(name = "[Credential] Share to LinkedIn")]
impl ActionTrait for ShareToLinkedIn {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(share_to_linkedin)]
    }
}
