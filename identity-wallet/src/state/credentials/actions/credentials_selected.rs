use std::time::Duration;

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

/// How long `handle_action` waits for [`CredentialsSelected`]: the presentation is built (which may resolve DIDs) and
/// posted to the verifier, which may take up to [`MAX_POST_DURATION`](crate::http_client::MAX_POST_DURATION). That
/// limit must run out first, so a POST without an answer is reported as not sent rather than cut off by the action's
/// timeout.
const CREDENTIALS_SELECTED_TIME_LIMIT: Duration = Duration::from_secs(20);

#[typetag::serde(name = "[Authenticate] Credentials selected")]
impl ActionTrait for CredentialsSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![
            reducer!(handle_oid4vp_authorization_request),
            reducer!(send_interactive_authorization_request_follow_up),
        ]
    }

    fn time_limit(&self) -> Duration {
        CREDENTIALS_SELECTED_TIME_LIMIT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::MAX_POST_DURATION;

    #[test]
    fn the_post_to_the_verifier_ends_before_the_action_times_out() {
        // Time left for building the presentation before the POST, which may resolve DIDs.
        let before_the_post = Duration::from_secs(5);
        assert!(MAX_POST_DURATION + before_the_post <= CREDENTIALS_SELECTED_TIME_LIMIT);
    }
}
