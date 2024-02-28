pub mod cancel_user_flow;
pub mod cancel_user_journey;
pub mod connection_accepted;
pub mod create_new;
pub mod credential_offers_selected;
pub mod credentials_selected;
pub mod get_state;
pub mod load_dev_profile;
pub mod qr_code_scanned;
pub mod reset;
pub mod set_locale;
pub mod toggle_dev_settings;
pub mod unlock_storage;
pub mod update_credential_metadata;
pub mod update_profile_settings;
pub mod user_data_query;

pub use cancel_user_flow::*;
pub use cancel_user_journey::*;
pub use connection_accepted::*;
pub use create_new::*;
pub use credential_offers_selected::*;
pub use credentials_selected::*;
pub use get_state::*;
pub use load_dev_profile::*;
pub use qr_code_scanned::*;
pub use reset::*;
pub use set_locale::*;
pub use unlock_storage::*;
pub use update_credential_metadata::*;
pub use update_profile_settings::*;
pub use user_data_query::*;

use super::reducers::Reducer;
use downcast_rs::{impl_downcast, DowncastSync};
use std::sync::Arc;
use ts_rs::TS;

/// Below is an example of how to add an action to the app
///
/// Example:
/// ```
/// use crate::identity_wallet::state::actions::{Action, ActionTrait, listen};
/// use crate::identity_wallet::reducer;
/// use crate::identity_wallet::state::reducers::Reducer;
/// use crate::identity_wallet::state::AppState;
/// use crate::identity_wallet::error::AppError;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize, Clone)]
/// pub struct TestExampleAction {
///     example_field: String,
///     example_field_2: bool,
/// }
///
/// #[typetag::serde(name = "[Example] Example Action")]
/// impl ActionTrait for TestExampleAction {
///     fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
///         vec![reducer!(test_example_reducer)]
///     }
/// }
///
/// pub async fn test_example_reducer(state: AppState, action: Action) -> Result<AppState, AppError> {
///     if let Some(test_example_action) = listen::<TestExampleAction>(action) {
///         // Reducer logic goes here
///         return Ok(AppState {
///             // Add changes to the state here
///             ..state
///         });
///     }
/// 
///     Ok(state)
/// }
/// ```

/// A redux-like Action that the backend knows how to handle (reduce), with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
/// For the type string, we're using ngrx style: "\[Feature\] Action name" (see https://ngrx.io/guide/store/actions)
pub type Action = Arc<dyn ActionTrait>;

/// A trait that all Actions must implement.
#[typetag::serde(tag = "type", content = "payload")]
pub trait ActionTrait: Send + std::fmt::Debug + DowncastSync {
    /// Returns the reducers that should be called when this action is dispatched.
    fn reducers<'a>(&self) -> Vec<Reducer<'a>>;
}
impl_downcast!(sync ActionTrait);

/// Downcast an Action to a specific type, if possible.
pub fn listen<T: ActionTrait + Clone>(action: Action) -> Option<T> {
    action.downcast_arc::<T>().ok().map(|action| (*action).clone())
}

// TODO: remove this once we have a better way to export the TS types.
mod bindings {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, TS)]
    #[serde(tag = "type")]
    #[ts(export, export_to = "bindings/actions/Action.ts")]
    pub enum Action {
        #[serde(rename = "[App] Get state")]
        GetState,
        #[serde(rename = "[Storage] Unlock")]
        UnlockStorage { payload: UnlockStorage },
        #[serde(rename = "[App] Reset")]
        Reset,
        #[serde(rename = "[DID] Create new")]
        CreateNew { payload: CreateNew },
        #[serde(rename = "[Settings] Set locale")]
        SetLocale { payload: SetLocale },
        #[serde(rename = "[Settings] Update profile")]
        UpdateProfileSettings { payload: UpdateProfileSettings },
        #[serde(rename = "[QR Code] Scanned")]
        QrCodeScanned { payload: QrCodeScanned },
        #[serde(rename = "[Authenticate] Connection accepted")]
        ConnectionAccepted,
        #[serde(rename = "[User Flow] Cancel")]
        CancelUserFlow {
            #[ts(optional)]
            payload: Option<CancelUserFlow>,
        },
        #[serde(rename = "[DEV] Load DEV profile")]
        LoadDevProfile { payload: DevProfile },
        #[serde(rename = "[DEV] Toggle DEV mode")]
        ToggleDevMode,
        #[serde(rename = "[Authenticate] Credentials selected")]
        CredentialsSelected { payload: CredentialsSelected },
        #[serde(rename = "[Credential Offer] Selected")]
        CredentialOffersSelected { payload: CredentialOffersSelected },
        #[serde(rename = "[Credential Metadata] Update")]
        UpdateCredentialMetadata { payload: UpdateCredentialMetadata },
        #[serde(rename = "[User Journey] Cancel")]
        CancelUserJourney,
        #[serde(rename = "[User Data] Query")]
        UserDataQuery { payload: UserDataQuery },
    }
}
