use crate::reducer;
use crate::state::credentials::reducers::refresh_all_credential_statuses::refresh_all_credential_statuses;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/RefreshAllCredentialStatuses.ts")]
pub struct RefreshAllCredentialStatuses {}

#[typetag::serde(name = "[Credential] Refresh All Statuses")]
impl ActionTrait for RefreshAllCredentialStatuses {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(refresh_all_credential_statuses)]
    }
}
