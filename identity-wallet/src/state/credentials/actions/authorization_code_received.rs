use crate::reducer;
use crate::state::credentials::reducers::send_token_request::send_token_request;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to authenticate the selected credentials.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/AuthorizationCodeReceived.ts")]
pub struct AuthorizationCodeReceived {
    pub code: String,
    pub state: String,
}

#[typetag::serde(name = "[Credential Offer] Authorization code received")]
impl ActionTrait for AuthorizationCodeReceived {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_token_request)]
    }
}
