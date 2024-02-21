use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::{create_identity, initialize_stronghold};
use std::fmt::Formatter;
use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CreateNew.ts")]
pub struct CreateNew {
    pub name: String,
    pub picture: String,
    pub theme: String,
    pub password: String,
}

impl std::fmt::Debug for CreateNew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateNew")
            .field("name", &self.name)
            .field("picture", &self.picture)
            .field("theme", &self.theme)
            .field("password", &"*****")
            .finish()
    }
}

#[typetag::serde(name = "[DID] Create new")]
impl ActionTrait for CreateNew {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(initialize_stronghold), reducer!(create_identity)]
    }
}
