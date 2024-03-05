use crate::reducer;
use crate::state::connections::reducers::handle_oid4vp_authorization_request::handle_oid4vp_authorization_request;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to authenticate the selected credentials.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialsSelected.ts")]
pub struct CredentialsSelected {
    #[ts(type = "Array<string>")]
    pub credential_uuids: Vec<uuid::Uuid>,
}

#[typetag::serde(name = "[Authenticate] Credentials selected")]
impl ActionTrait for CredentialsSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_oid4vp_authorization_request)]
    }
}
