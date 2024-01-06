use oid4vci::credential_offer::CredentialOfferQuery;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{Locale, UserDataQuery};

/// A redux-like Action that the backend knows how to handle (reduce), with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
/// For the type string, we're using ngrx style: "\[Feature\] Action name" (see https://ngrx.io/guide/store/actions)
#[derive(Serialize, Deserialize, Debug, TS, PartialEq, Clone)]
#[serde(tag = "type")]
#[ts(export)]
pub enum Action {
    #[serde(rename = "[App] Get state")]
    GetState,
    #[serde(rename = "[App] Reset")]
    Reset,
    #[serde(rename = "[DID] Create new")]
    CreateNew {
        name: String,
        picture: String,
        theme: String,
        password: String,
    },
    #[serde(rename = "[Settings] Set locale")]
    SetLocale {
        #[ts(type = "string")]
        locale: Locale,
    },
    #[serde(rename = "[Settings] Update profile")]
    UpdateProfileSettings {
        #[ts(optional)]
        name: Option<String>,
        #[ts(optional)]
        picture: Option<String>,
        #[ts(optional)]
        theme: Option<String>,
    },
    #[serde(rename = "[QR Code] Scanned")]
    QrCodeScanned { form_urlencoded: String },
    #[serde(rename = "[User Flow] Cancel")]
    CancelUserFlow {
        #[ts(optional)]
        redirect: Option<String>,
    },
    #[serde(rename = "[DEV] Load profile")]
    LoadDevProfile,
    #[serde(rename = "[DEV] Set dev mode")]
    SetDevMode { enabled: bool },
    #[serde(rename = "[Credential Offer] Read")]
    ReadCredentialOffer {
        #[ts(type = "object")]
        credential_offer_uri: CredentialOfferQuery,
    },
    #[serde(rename = "[Credential Offer] Selected")]
    CredentialOffersSelected {
        #[ts(type = "Array<string>")]
        offer_indices: Vec<usize>,
    },
    #[serde(rename = "[Authenticate] Read request")]
    ReadRequest { authorization_request: String },
    #[serde(rename = "[Authenticate] Connection accepted")]
    ConnectionAccepted,
    #[serde(rename = "[Authenticate] Credentials selected")]
    CredentialsSelected {
        #[ts(type = "Array<string>")]
        credential_uuids: Vec<uuid::Uuid>,
    },
    #[serde(rename = "[Storage] Unlock")]
    UnlockStorage { password: String },
    #[serde(rename = "[Credential Metadata] Update")]
    UpdateCredentialMetadata {
        #[ts(type = "string")]
        id: uuid::Uuid,
        #[ts(optional)]
        name: Option<String>,
        #[ts(optional)]
        icon: Option<String>,
        #[ts(optional)]
        color: Option<String>,
        #[ts(optional)]
        is_favorite: Option<bool>,
    },
    #[serde(rename = "[User Journey] Cancel")]
    CancelUserJourney,
    #[serde(rename = "[User Data] Query")]
    UserDataQuery(UserDataQuery),
    #[ts(skip)]
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action() {
        assert_eq!(
            Action::ConnectionAccepted,
            serde_json::from_str(include_str!(
                "../../tests/fixtures/actions/authenticate_connect_accept.json"
            ))
            .unwrap(),
        );
        assert_eq!(
            Action::CredentialsSelected {
                credential_uuids: vec!["39373933-3863-3339-3864-646234373631".parse().unwrap()]
            },
            serde_json::from_str(include_str!(
                "../../tests/fixtures/actions/authenticate_cred_selected.json"
            ))
            .unwrap(),
        );
        assert_eq!(
            Action::CreateNew {
                name: "Ferris Crabman".to_string(),
                picture: "&#129408".to_string(),
                theme: "system".to_string(),
                password: "sup3rSecr3t".to_string(),
            },
            serde_json::from_str(include_str!("../../tests/fixtures/actions/create_new.json")).unwrap(),
        );
        assert_eq!(
            Action::LoadDevProfile,
            serde_json::from_str(include_str!("../../tests/fixtures/actions/dev_load_profile.json")).unwrap(),
        );
        assert_eq!(
            Action::GetState,
            serde_json::from_str(include_str!("../../tests/fixtures/actions/get_state.json")).unwrap(),
        );
        assert_eq!(
            Action::QrCodeScanned {
                form_urlencoded: "siopv2://idtoken?response_type=id_token&client_id=did%3Akey%3Az6Mkm9yeuZK7inXBNjnNH3vAs9uUjqfy3mfNoKBKsKBrv8Tb&scope=openid&redirect_uri=https%3A%2F%2Fexample.com&nonce=nonce".to_string()
            },
            serde_json::from_str(include_str!("../../tests/fixtures/actions/qr_scanned_id_token.json")).unwrap(),
        );
        assert_eq!(
            Action::UnlockStorage {
                password: "sup3rSecr3t".to_string()
            },
            serde_json::from_str(include_str!("../../tests/fixtures/actions/unlock_storage.json")).unwrap(),
        );
    }
}
