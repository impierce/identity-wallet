use std::{future::Future, pin::Pin};
use ts_rs::TS;

use crate::{
    command::cancel_user_journey,
    error::AppError,
    state::reducers::{
        authorization::{
            handle_oid4vp_authorization_request, handle_siopv2_authorization_request, read_authorization_request,
        },
        cancel_user_flow, create_identity,
        credential_offer::{read_credential_offer, send_credential_request},
        dev_mode::{load_dev_profile, set_dev_mode},
        get_state, initialize_stronghold, reset_state, set_locale,
        storage::unlock_storage,
        update_credential_metadata, update_profile_settings,
        user_data_query::{connection_query, credential_query},
    },
};

use super::{AppState, Locale};

use downcast_rs::{impl_downcast, DowncastSync};

use crate::state::{QueryTarget, SortMethod};

use super::*;

#[macro_export]
macro_rules! reducer {
    ($reducer:expr) => {
        Box::new(move |app_state, action| Box::pin(async move { $reducer(app_state, action).await }))
    };
}

/// A redux-like Action that the backend knows how to handle (reduce), with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
/// For the type string, we're using ngrx style: "\[Feature\] Action name" (see https://ngrx.io/guide/store/actions)
pub type Action = Arc<dyn ActionTrait>;
pub type Reducer<'a> =
    Box<dyn Fn(AppState, Action) -> Pin<Box<dyn Future<Output = Result<AppState, AppError>> + Send>> + Send>;

#[typetag::serde(tag = "type", content = "payload")]
pub trait ActionTrait: Send + std::fmt::Debug + DowncastSync {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>>;
}
impl_downcast!(sync ActionTrait);

pub fn listen<T: ActionTrait + Clone>(action: Action) -> Option<T> {
    action.downcast_arc::<T>().ok().map(|action| (*action).clone())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct GetState;

#[typetag::serde(name = "[App] Get state")]
impl ActionTrait for GetState {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(get_state)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct UnlockStorage {
    pub password: String,
}

#[typetag::serde(name = "[Storage] Unlock")]
impl ActionTrait for UnlockStorage {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(unlock_storage)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct Reset;

#[typetag::serde(name = "[App] Reset")]
impl ActionTrait for Reset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_state)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct CreateNew {
    pub name: String,
    pub picture: String,
    pub theme: String,
    pub password: String,
}

#[typetag::serde(name = "[DID] Create new")]
impl ActionTrait for CreateNew {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(initialize_stronghold), reducer!(create_identity)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct SetLocale {
    #[ts(type = "string")]
    pub locale: Locale,
}

#[typetag::serde(name = "[Settings] Set locale")]
impl ActionTrait for SetLocale {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_locale)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct UpdateProfileSettings {
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub picture: Option<String>,
    #[ts(optional)]
    pub theme: Option<String>,
}

#[typetag::serde(name = "[Settings] Update profile")]
impl ActionTrait for UpdateProfileSettings {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_profile_settings)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct QrCodeScanned {
    pub form_urlencoded: String,
}

#[typetag::serde(name = "[QR Code] Scanned")]
impl ActionTrait for QrCodeScanned {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(read_authorization_request), reducer!(read_credential_offer)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct ConnectionAccepted;

#[typetag::serde(name = "[Authenticate] Connection accepted")]
impl ActionTrait for ConnectionAccepted {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_siopv2_authorization_request)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct CancelUserFlow {
    #[ts(optional)]
    pub redirect: Option<String>,
}

#[typetag::serde(name = "[User Flow] Cancel")]
impl ActionTrait for CancelUserFlow {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_flow)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct SetDevMode {
    pub enabled: bool,
}

#[typetag::serde(name = "[DEV] Set dev mode")]
impl ActionTrait for SetDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_dev_mode)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct LoadDevProfile;

#[typetag::serde(name = "[DEV] Load profile")]
impl ActionTrait for LoadDevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct CredentialsSelected {
    #[ts(type = "Array<string>")]
    pub credential_uuids: Vec<uuid::Uuid>,
}

#[typetag::serde(name = "[Authenticate] Credentials selected")]
impl ActionTrait for CredentialsSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_oid4vp_authorization_request)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct CredentialOffersSelected {
    #[ts(type = "Array<string>")]
    pub offer_indices: Vec<usize>,
}

#[typetag::serde(name = "[Credential Offer] Selected")]
impl ActionTrait for CredentialOffersSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_credential_request)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct UpdateCredentialMetadata {
    #[ts(type = "string")]
    pub id: uuid::Uuid,
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub icon: Option<String>,
    #[ts(optional)]
    pub color: Option<String>,
    #[ts(optional)]
    pub is_favorite: Option<bool>,
}

#[typetag::serde(name = "[Credential Metadata] Update")]
impl ActionTrait for UpdateCredentialMetadata {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_credential_metadata)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct CancelUserJourney;

#[typetag::serde(name = "[User Journey] Cancel")]
impl ActionTrait for CancelUserJourney {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_journey)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export)]
pub struct UserDataQuery {
    pub target: QueryTarget,
    #[ts(optional)]
    pub search_term: Option<String>,
    #[ts(optional)]
    pub sort_method: Option<SortMethod>,
    #[serde(default)]
    pub sort_reverse: bool,
}

#[typetag::serde(name = "[User Data] Query")]
impl ActionTrait for UserDataQuery {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(credential_query), reducer!(connection_query)]
    }
}
