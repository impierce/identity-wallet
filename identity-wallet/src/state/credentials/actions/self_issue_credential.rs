use crate::reducer;
use crate::state::credentials::reducers::self_issue_credential::self_issue_credential;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SelfIssueCredential.ts")]
pub struct SelfIssueCredential {
    #[ts(type = "string")]
    #[serde(rename = "type")]
    pub _type: SelfIssuedCredentialType,
    #[ts(type = "string")]
    pub data: String,
}

#[typetag::serde(name = "[Credential] Self Issue")]
impl ActionTrait for SelfIssueCredential {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(self_issue_credential)]
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, EnumString, Display)]
#[ts(export, export_to = "bindings/credentials/SelfIssuedCredentialType.ts")]
#[serde(rename_all = "snake_case")]
pub enum SelfIssuedCredentialType {
    Profile,
    Address,
}
