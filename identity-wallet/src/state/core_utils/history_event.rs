use crate::state::credentials::VerifiableCredentialRecord;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub enum EventType {
    ConnectionAdded,
    CredentialsAdded,
    CredentialsShared,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct HistoryEvent {
    pub connection_id: String,
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

/// Implement PartialEq for HistoryEvent to allow for comparison of HistoryEvent instances in tests.
/// TODO(test): This implementation should be behind a "test" feature flag.
impl PartialEq for HistoryEvent {
    fn eq(&self, other: &Self) -> bool {
        self.connection_id == other.connection_id
            && self.connection_name == other.connection_name
            && self.event_type == other.event_type
            && self.credentials == other.credentials
    }
}
