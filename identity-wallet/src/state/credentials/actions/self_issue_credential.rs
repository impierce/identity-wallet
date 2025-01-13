use crate::reducer;
use crate::state::credentials::reducers::self_issue_credential::self_issue_credential;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SelfIssueCredential.ts")]
pub struct SelfIssueCredential {
    #[ts(type = "string")]
    pub _type: String,
    #[ts(type = "any")]
    pub data: serde_json::Value,
}

#[typetag::serde(name = "[Credential] Delete")]
impl ActionTrait for SelfIssueCredential {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(self_issue_credential)]
    }
}
