use oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    openid4vc_extension::Generic,
};
use oid4vci::credential_offer::CredentialOfferQuery;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::Locale;

/// A redux-like Action with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
/// For the type string, we're using ngrx style: "\[Feature\] Action name" (see https://ngrx.io/guide/store/actions)
// #[derive(Serialize, Deserialize, TS, Clone, Debug)]
// #[ts(export)]
// pub struct Action {
//     pub r#type: ActionType,
//     #[ts(optional, type = "object")]
//     pub payload: Option<serde_json::Value>,
// }

/// Actions that the backend knows how to handle (reduce).
#[derive(Serialize, Deserialize, Debug, TS, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "[App] Get State")]
    GetState,
    #[serde(rename = "[App] Reset")]
    Reset,
    #[serde(rename = "[Did] Create New")]
    CreateNew {
        name: String,
        picture: String,
        theme: String,
        password: String,
    },
    #[serde(rename = "[Settings] Set Locale")]
    SetLocale { locale: Locale },
    #[serde(rename = "[Settings] Update Profile")]
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
    #[serde(rename = "[Dev] Set Dev Mode")]
    SetDevMode { enabled: bool },
    #[serde(rename = "[DEV] Load Profile")]
    LoadDevProfile,
    #[serde(rename = "[Credential Offer] Read")]
    ReadCredentialOffer {
        #[ts(type = "object")]
        credential_offer_query: CredentialOfferQuery,
    },
    #[serde(rename = "[Credential Offer] Selected")]
    CredentialOffersSelected { offer_indices: Vec<usize> },
    #[serde(rename = "[Authenticate] Read Request")]
    ReadRequest {
        #[ts(type = "object")]
        authorization_request: AuthorizationRequest<Object<Generic>>,
    },
    #[serde(rename = "[Authenticate] Connection accepted")]
    ConnectionAccepted,
    #[serde(rename = "[Authenticate] Credentials Selected")]
    CredentialsSelected {
        #[ts(type = "string")]
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
    UserDataQuery,
    #[ts(skip)]
    #[serde(other)]
    Unknown,
}
