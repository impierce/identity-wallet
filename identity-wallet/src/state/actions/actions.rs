use std::{future::Future, pin::Pin};
use ts_rs::TS;
use crate::{
    command::cancel_user_journey,
    error::AppError,
    state::reducers::{
        test_feat_state,
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
use downcast_rs::{impl_downcast, DowncastSync};
use crate::state::{QueryTarget, SortMethod};
use super::super::*;

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
//pub trait ActionTrait: Send + std::fmt::Debug + DowncastSync {
pub trait ActionTrait: Send + std::fmt::Debug + DowncastSync {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>>;
}
impl_downcast!(sync ActionTrait);

pub fn listen<T: ActionTrait + Clone>(action: Action) -> Option<T> {
    action.downcast_arc::<T>().ok().map(|action| (*action).clone())
}

// TODO: remove this once we have a better way to export the TS types.
mod bindings {
    use super::*;

    #[derive(Serialize, Deserialize, TS)]
    #[serde(tag = "type")]
    #[ts(export, export_to = "bindings/actions/Action.ts")]
    pub enum Action {
        #[serde(rename = "[App] Get state")]
        GetState { payload: GetState },
        #[serde(rename = "[Storage] Unlock")]
        UnlockStorage { payload: UnlockStorage },
        #[serde(rename = "[App] Reset")]
        Reset { payload: Reset },
        #[serde(rename = "[DID] Create new")]
        CreateNew { payload: CreateNew },
        #[serde(rename = "[Settings] Set locale")]
        SetLocale { payload: SetLocale },
        #[serde(rename = "[Settings] Update profile")]
        UpdateProfileSettings { payload: UpdateProfileSettings },
        #[serde(rename = "[QR Code] Scanned")]
        QrCodeScanned { payload: QrCodeScanned },
        #[serde(rename = "[Authenticate] Connection accepted")]
        ConnectionAccepted { payload: ConnectionAccepted },
        #[serde(rename = "[User Flow] Cancel")]
        CancelUserFlow { payload: CancelUserFlow },
        #[serde(rename = "[DEV] Set dev mode")]
        SetDevMode { payload: SetDevMode },
        #[serde(rename = "[DEV] Load profile")]
        LoadDevProfile { payload: LoadDevProfile },
        #[serde(rename = "[Authenticate] Credentials selected")]
        CredentialsSelected { payload: CredentialsSelected },
        #[serde(rename = "[Credential Offer] Selected")]
        CredentialOffersSelected { payload: CredentialOffersSelected },
        #[serde(rename = "[Credential Metadata] Update")]
        UpdateCredentialMetadata { payload: UpdateCredentialMetadata },
        #[serde(rename = "[User Journey] Cancel")]
        CancelUserJourney { payload: CancelUserJourney },
        #[serde(rename = "[User Data] Query")]
        UserDataQuery { payload: UserDataQuery },
        #[serde(rename = "[Test] Test")]
        CustomExtensionTest { payload: CustomExtensionTest },
        
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/GetState.ts")]
pub struct GetState;

#[typetag::serde(name = "[App] Get state")]
impl ActionTrait for GetState {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(get_state)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UnlockStorage.ts")]
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
#[ts(export, export_to = "bindings/actions/Reset.ts")]
pub struct Reset;

#[typetag::serde(name = "[App] Reset")]
impl ActionTrait for Reset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_state)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CreateNew.ts")]
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
#[ts(export, export_to = "bindings/actions/SetLocale.ts")]
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
#[ts(export, export_to = "bindings/actions/UpdateProfileSettings.ts")]
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
#[ts(export, export_to = "bindings/actions/QrCodeScanned.ts")]
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
#[ts(export, export_to = "bindings/actions/ConnectionAccepted.ts")]
pub struct ConnectionAccepted;

#[typetag::serde(name = "[Authenticate] Connection accepted")]
impl ActionTrait for ConnectionAccepted {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_siopv2_authorization_request)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CancelUserFlow.ts")]
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
#[ts(export, export_to = "bindings/actions/SetDevMode.ts")]
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
#[ts(export, export_to = "bindings/actions/LoadDevProfile.ts")]
pub struct LoadDevProfile;

#[typetag::serde(name = "[DEV] Load profile")]
impl ActionTrait for LoadDevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialsSelected.ts")]
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
#[ts(export, export_to = "bindings/actions/CredentialOffersSelected.ts")]
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
#[ts(export, export_to = "bindings/actions/UpdateCredentialMetadata.ts")]
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
#[ts(export, export_to = "bindings/actions/CancelUserJourney.ts")]
pub struct CancelUserJourney;

#[typetag::serde(name = "[User Journey] Cancel")]
impl ActionTrait for CancelUserJourney {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_journey)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UserDataQuery.ts")]
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

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/Test.ts")]
pub struct CustomExtensionTest {
    pub test_term: Option<String>,
    #[serde(default)]
    pub test_bool: bool,
}

#[typetag::serde(name = "[Test] Test")]
impl ActionTrait for CustomExtensionTest {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(test_feat_state)]
    }
}   
