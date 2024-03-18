pub mod actions;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Connection contains the ID and information of a connection.
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

#[typetag::serde(name = "connection")]
impl FeatTrait for Connection {}

impl Connection {
    // TODO(ngdil): Temporary solution to support NGDIL demo, replace with different unique identifier to distinguish connection
    pub fn create_connection_id(connection_name: &str) -> String {
        base64::encode_config(connection_name, base64::URL_SAFE)
    }
}
