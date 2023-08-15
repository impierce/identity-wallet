pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_flow;

use crate::state::user_flow::CurrentUserFlow;
use identity_credential::credential::Credential;
use serde::{Deserialize, Serialize};
use siopv2::AuthorizationRequest;
use std::sync::Mutex;
use ts_rs::TS;

/// The inner state of the application managed by Tauri.
#[derive(Default)]
pub struct AppState {
    pub active_profile: Mutex<Option<Profile>>,
    pub active_authorization_request: Mutex<Option<AuthorizationRequest>>,
    pub locale: Mutex<String>,
    pub credentials: Mutex<Option<Vec<serde_json::Value>>>,
    pub current_user_flow: Mutex<Option<CurrentUserFlow>>,
    pub debug_messages: Mutex<Vec<String>>,
    pub user_journey: Mutex<Option<serde_json::Value>>,
}

/// A representation of the current state which is used for serialization.
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TransferState {
    pub active_profile: Option<Profile>,
    pub locale: String,
    #[ts(optional, type = "Array<object>")]
    // TODO: what is the correct type here? Map<String, String>? Object? null? undefined? any? unknown?
    pub credentials: Option<Vec<serde_json::Value>>,
    pub current_user_flow: Option<CurrentUserFlow>,
    pub debug_messages: Vec<String>,
    #[ts(optional, type = "object")]
    pub user_journey: Option<serde_json::Value>,
}

impl From<&AppState> for TransferState {
    fn from(state: &AppState) -> TransferState {
        TransferState {
            active_profile: state.active_profile.lock().unwrap().clone(),
            locale: (*state.locale.lock().unwrap()).to_string(),
            credentials: state.credentials.lock().unwrap().clone(),
            current_user_flow: state.current_user_flow.lock().unwrap().clone(),
            debug_messages: state.debug_messages.lock().unwrap().clone(),
            user_journey: state.user_journey.lock().unwrap().clone(),
        }
    }
}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub struct Profile {
    pub display_name: String,
    pub primary_did: String,
}
