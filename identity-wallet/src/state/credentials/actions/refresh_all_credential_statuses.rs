use crate::reducer;
use crate::state::credentials::reducers::refresh_all_credential_statuses::refresh_all_credential_statuses;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshAllCredentialStatuses;

#[typetag::serde(name = "[Credential] Refresh all statuses")]
impl ActionTrait for RefreshAllCredentialStatuses {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(refresh_all_credential_statuses)]
    }
}
