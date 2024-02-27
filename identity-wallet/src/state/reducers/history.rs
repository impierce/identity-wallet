use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub enum EventType {
    #[serde(rename = "connection_added")]
    ConnectionAdded,
    #[serde(rename = "credentials_added")]
    CredentialsAdded,
    #[serde(rename = "credentials_shared")]
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
    pub image_id: String,
}
