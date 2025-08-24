use crate::reducer;
use crate::state::credentials::reducers::send_token_request::send_token_request;
use crate::state::{actions::ActionTrait, Reducer};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to authenticate the selected credentials.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CodeReceived.ts")]
pub struct CodeReceived {
    pub code: String,
    pub is_pre_authorized: bool,
    pub state: Option<String>,
}

#[typetag::serde(name = "[Credential Offer] Code received")]
impl ActionTrait for CodeReceived {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_token_request)]
    }
}
