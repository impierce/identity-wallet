pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_prompt;

use crate::{
    crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt,
    verifiable_credential_record::DisplayCredential,
};
use oid4vc_core::Subject;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use ts_rs::TS;

use self::reducers::authorization::ConnectionRequest;

pub struct IdentityManager {
    pub subject: Arc<dyn Subject>,
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
    pub active_connection_request: Mutex<Option<ConnectionRequest>>,
    pub locale: Mutex<Locale>,
    pub credentials: Mutex<Vec<DisplayCredential>>,
    pub current_user_prompt: Mutex<Option<CurrentUserPrompt>>,
    pub debug_messages: Mutex<Vec<String>>,
    pub user_journey: Mutex<Option<serde_json::Value>>,
}

/// A representation of the current state which is used for serialization.
#[derive(Clone, Debug, Deserialize, Serialize, TS, Default, PartialEq)]
#[ts(export)]
pub struct TransferState {
    pub active_profile: Option<Profile>,
    pub locale: Locale,
    pub credentials: Vec<DisplayCredential>,
    pub current_user_prompt: Option<CurrentUserPrompt>,
    pub debug_messages: Vec<String>,
    #[ts(optional, type = "object")]
    pub user_journey: Option<serde_json::Value>,
}

impl From<&AppState> for TransferState {
    fn from(state: &AppState) -> TransferState {
        TransferState {
            active_profile: state.active_profile.lock().unwrap().clone(),
            locale: state.locale.lock().unwrap().clone(),
            credentials: state.credentials.lock().unwrap().clone(),
            current_user_prompt: state.current_user_prompt.lock().unwrap().clone(),
            debug_messages: state.debug_messages.lock().unwrap().clone(),
            user_journey: state.user_journey.lock().unwrap().clone(),
        }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Locale {
    #[default]
    En,
    De,
    Nl,
}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: Option<String>,
    pub primary_did: String,
}
