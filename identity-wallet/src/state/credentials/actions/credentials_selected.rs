use crate::reducer;
use crate::state::credentials::reducers::handle_oid4vp_authorization_request::handle_oid4vp_authorization_request;
use crate::state::credentials::reducers::send_interactive_authorization_request_follow_up::send_interactive_authorization_request_follow_up;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to authenticate the selected credentials.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialsSelected.ts")]
pub struct CredentialsSelected {
    pub selections: Vec<CredentialSelection>,
    #[serde(default)]
    pub is_interactive: bool,
}

/// The credentials the user picked for one credential query of the request.
#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq)]
#[ts(export, export_to = "bindings/actions/CredentialSelection.ts")]
pub struct CredentialSelection {
    pub query_id: String,
    #[ts(type = "Array<string>")]
    pub credential_uuids: Vec<uuid::Uuid>,
    /// The requested claims (by index) to disclose from selectively disclosable credentials. When absent, all
    /// requested claims are disclosed, or the first claim set the credential satisfies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub disclosed_claims: Option<Vec<usize>>,
}

#[typetag::serde(name = "[Authenticate] Credentials selected")]
impl ActionTrait for CredentialsSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![
            reducer!(handle_oid4vp_authorization_request),
            reducer!(send_interactive_authorization_request_follow_up),
        ]
    }
}
