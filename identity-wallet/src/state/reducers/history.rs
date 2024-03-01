use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub enum EventType {
    ConnectionAdded,
    CredentialsAdded,
    CredentialsShared,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub struct HistoryEvent {
    pub issuer_name: String,
    pub event_type: EventType,
    pub connection_id: Option<String>,
    pub date: String,
    pub credentials: Vec<HistoryCredential>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub struct HistoryCredential {
    pub title: String,
    pub sub_title: String,
    /// This id is also used as the image asset id
    pub id: String,
}
