pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_prompt;

use crate::{crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt};
use identity_credential::credential::Credential;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use siopv2::AuthorizationRequest;
use std::sync::{Arc, Mutex};
use ts_rs::TS;

pub struct IdentityManager {
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}

#[derive(Default)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

/// The inner state of the application managed by Tauri.
#[derive(Default)]
pub struct AppState {
    pub managers: tauri::async_runtime::Mutex<Managers>,
    pub active_profile: Mutex<Option<Profile>>,
    pub active_authorization_request: Mutex<Option<AuthorizationRequest>>,
    pub locale: Mutex<String>,
    pub credentials: Mutex<Vec<(String, Credential)>>,
    pub current_user_prompt: Mutex<Option<CurrentUserPrompt>>,
    pub debug_messages: Mutex<Vec<String>>,
}

/// A representation of the current state which is used for serialization.
#[derive(Clone, Debug, Deserialize, Serialize, TS, Default, PartialEq)]
#[ts(export)]
pub struct TransferState {
    pub active_profile: Option<Profile>,
    pub locale: String,
    #[ts(type = "Array<{id: string, data: object}>")]
    pub credentials: Vec<(String, Credential)>,
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
