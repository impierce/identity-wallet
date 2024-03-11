use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::verifiable_credential_record::VerifiableCredentialRecord;

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
    pub connection_id: Option<String>,
    pub connection_name: String,
    pub event_type: EventType,
    pub date: String,
    pub credentials: Vec<HistoryCredential>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub struct HistoryCredential {
    pub title: String,
    pub issuer_name: String,
    /// This id is also used as the image asset id
    pub id: String,
}

impl HistoryCredential {
    pub fn from_credential(verifiable_credential_record: &VerifiableCredentialRecord) -> Self {
            let display = &verifiable_credential_record.display_credential;

            Self {
                title: display.display_name.to_string(),
                issuer_name: display.issuer_name.to_string(),
                id: display.id.to_string(),
            }
    }
}
