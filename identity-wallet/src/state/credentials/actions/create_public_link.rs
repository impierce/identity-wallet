use crate::reducer;
use crate::state::credentials::reducers::public_link::create_public_link;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CreatePublicLink.ts")]
pub struct CreatePublicLink {
    pub id: String,
}

#[typetag::serde(name = "[Credential] Create public link")]
impl ActionTrait for CreatePublicLink {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(create_public_link)]
    }
}
