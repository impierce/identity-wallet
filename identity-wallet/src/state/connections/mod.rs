use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod actions;
pub mod reducers;

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Connection {
    pub id: String,
    pub client_name: String,
    pub url: String,
    pub verified: bool,
    pub first_interacted: String,
    pub last_interacted: String,
}
