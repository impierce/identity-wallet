use crate::reducer;
use crate::state::credentials::reducers::refresh_credential_status::refresh_credential_status;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/RefreshCredentialStatus.ts")]
pub struct RefreshCredentialStatus {
    pub credential_id: Option<String>,
}

#[typetag::serde(name = "[Credential] Refresh Status")]
impl ActionTrait for RefreshCredentialStatus {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(refresh_credential_status)]
    }
}
