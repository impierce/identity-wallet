use crate::reducer;
use crate::state::connections::reducers::delete_connection::delete_connection;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DeleteConnection.ts")]
pub struct DeleteConnection {
    #[ts(type = "string")]
    pub id: String,
}

#[typetag::serde(name = "[Connection] Delete")]
impl ActionTrait for DeleteConnection {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(delete_connection)]
    }
}
