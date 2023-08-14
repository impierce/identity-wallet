pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_prompt;

use crate::state::user_prompt::CurrentUserPrompt;
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
    pub credentials: Mutex<Option<Vec<Credential>>>,
    pub current_user_prompt: Mutex<Option<CurrentUserPrompt>>,
    pub debug_messages: Mutex<Vec<String>>,
}

/// A representation of the current state which is used for serialization.
#[derive(Clone, Debug, Deserialize, Serialize, TS, Default, PartialEq)]
#[ts(export)]
pub struct TransferState {
    pub active_profile: Option<Profile>,
    pub locale: String,
    #[ts(optional, type = "object")]
    // TODO: what is the correct type here? Map<String, String>? Object? null? undefined? any? unknown?
    pub credentials: Option<Vec<Credential>>,
    pub current_user_prompt: Option<CurrentUserPrompt>,
    pub debug_messages: Vec<String>,
}

impl From<&AppState> for TransferState {
    fn from(state: &AppState) -> TransferState {
        TransferState {
            active_profile: state.active_profile.lock().unwrap().clone(),
            locale: state.locale.lock().unwrap().clone(),
            credentials: state.credentials.lock().unwrap().clone(),
            current_user_prompt: state.current_user_prompt.lock().unwrap().clone(),
            debug_messages: state.debug_messages.lock().unwrap().clone(),
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
