use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A redux-like Action with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
/// For the type string, we're using ngrx style: "\[Feature\] Action name" (see https://ngrx.io/guide/store/actions)
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct Action {
    pub r#type: ActionType,
    #[ts(optional, type = "object")]
    pub payload: Option<serde_json::Value>,
}

/// Actions that the backend knows how to handle (reduce).
#[derive(Serialize, Deserialize, Debug, TS, PartialEq, Clone)]
#[ts(export)]
pub enum ActionType {
    #[serde(rename = "[App] Get state")]
    GetState,
    #[serde(rename = "[App] Reset")]
    Reset,
    #[serde(rename = "[DID] Create new")]
    CreateNew,
    #[serde(rename = "[Settings] Set locale")]
    SetLocale,
    #[serde(rename = "[Settings] Update profile")]
    UpdateProfileSettings,
    #[serde(rename = "[QR Code] Scanned")]
    QrCodeScanned,
    #[serde(rename = "[User Flow] Cancel")]
    CancelUserFlow,
    #[serde(rename = "[DEV] Load profile")]
    LoadDevProfile,
    #[serde(rename = "[DEV] Set dev mode")]
    SetDevMode,
    #[serde(rename = "[Credential Offer] Read")]
    ReadCredentialOffer,
    #[serde(rename = "[Credential Offer] Selected")]
    CredentialOffersSelected,
    #[serde(rename = "[Authenticate] Read request")]
    ReadRequest,
    #[serde(rename = "[Authenticate] Connection accepted")]
    ConnectionAccepted,
    #[serde(rename = "[Authenticate] Credentials selected")]
    CredentialsSelected,
    #[serde(rename = "[Storage] Unlock")]
    UnlockStorage,
    #[serde(rename = "[Credential Metadata] Update")]
    UpdateCredentialMetadata,
    #[serde(rename = "[User Journey] Cancel")]
    CancelUserJourney,
    #[ts(skip)]
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_type_deserialization() {
        assert_eq!(
            ActionType::GetState,
            serde_json::from_str(r#""[App] Get state""#).unwrap(),
        );
        assert_eq!(
            ActionType::Unknown,
            serde_json::from_str(r#""Unknown action""#).unwrap(),
        );
    }
}
