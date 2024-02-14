use crate::error::AppError;
use super::*;
use downcast_rs::{impl_downcast, DowncastSync};
use std::{future::Future, pin::Pin};
use ts_rs::TS;

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

// TODO: remove this once we have a better way to export the TS types.
mod bindings {
    use self::{boot::actions::{CancelUserFlow, GetState, Reset, UnlockStorage}, connections::actions::ConnectionAccepted, credentials::actions::{CredentialOffersSelected, CredentialsSelected, UpdateCredentialMetadata}, dev_mode::actions::{LoadDevProfile, SetDevMode}, extension::actions::CustomExtensionTest, profile::actions::{CreateNew, SetLocale, UpdateProfileSettings}, shared::actions::QrCodeScanned, user_data_query::actions::UserDataQuery, user_journey::actions::CancelUserJourney};

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

