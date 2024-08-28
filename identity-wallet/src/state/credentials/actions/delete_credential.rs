use crate::reducer;
use crate::state::credentials::reducers::delete_credential::delete_credential;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DeleteCredential.ts")]
pub struct DeleteCredential {
    #[ts(type = "string")]
    pub id: String,
}

#[typetag::serde(name = "[Credential] Delete")]
impl ActionTrait for DeleteCredential {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(delete_credential)]
    }
}
