pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_prompt;

use self::reducers::authorization::ConnectionRequest;
use crate::{
    crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt,
    verifiable_credential_record::DisplayCredential,
};
use derivative::Derivative;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;
use oid4vc::oid4vc_core::Subject;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc};
use strum::EnumString;
use ts_rs::TS;

/// The AppState is the main state of the application shared between the backend and the frontend.
/// We have structured the state and its operations following the redux pattern.
/// To safeguard this pattern we have introduced the FeatTrait, ActionTrait and a macro_rule for the Reducers.
/// All fields in the AppState have to implement the FeatTrait.
/// This is to ensure that the state is serializable/deserializable and cloneable among other things.
/// All actions have to implement the ActionTrait.
/// This ensures that all actions have at least one reducer, implement a debug method,
///  and are downcastable (necessary when receiving the action from the frontend)
/// The reducers are paired with the actions using our macro_rule.
/// This ensures that all reducers have the same signature and therefore follow the redux pattern and our error handling.
/// All the above goes for extensions (values) which are added to the extensions field.

/// Trait which each field of the appstate has to implement.
/// Some fields are simple values and not structs, so they don't need to implement this trait.
#[typetag::serde(tag = "feat_state_type")]
pub trait FeatTrait: Send + Sync + std::fmt::Debug + DynClone + DowncastSync {}
dyn_clone::clone_trait_object!(FeatTrait);
impl_downcast!(sync FeatTrait);

/// The container for the application state, simplifying the code to only need one mutex.
#[derive(Default, Debug)]
pub struct AppStateContainer(pub tokio::sync::Mutex<AppState>);

impl AppStateContainer {
    pub async fn insert_extension(self, key: &str, extension: Box<dyn FeatTrait>) -> Self {
        self.0.lock().await.extensions.insert(key.to_string(), extension);
        self
    }
}

/// The inner state of the application managed by Tauri. When the state is serialized in order to be sent to the
/// frontend, the `managers` and `active_connection_request` fields are skipped.
#[derive(Default, Serialize, Deserialize, Derivative, TS)]
#[derivative(Debug)]
#[ts(export)]
#[serde(default)]
pub struct AppState {
    /// This field contains the connections, containing the useable info for the frontend.
    pub connections: Vec<Connection>,
    /// This field contains the display credentials, containing the useable info for the frontend.
    pub credentials: Vec<DisplayCredential>,
    /// This field contains the query result, which is queried from credentials or connections.
    pub user_data_query: Vec<String>,
    /// This field contains utils needed for the backend to perform its tasks.
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub back_end_utils: BackEndUtils,
    /// This field contains the profile settings, including Locale.
    pub profile_settings: ProfileSettings,
    /// User prompts are a way for the backend to communicate a desired/required user interaction to the frontend.
    pub current_user_prompt: Option<CurrentUserPrompt>,
    /// Here user_journeys can be loaded from json_files or strings, to give the user a guided experience.
    #[ts(type = "object | null")]
    pub user_journey: Option<serde_json::Value>,
    /// Handled in command.rs, so no feature folder nor redux pattern needed.
    #[ts(type = "Array<string>")]
    pub debug_messages: VecDeque<String>,
    /// Extensions will bring along their own redux compliant code, in the unime folder.
    #[ts(skip)]
    pub extensions: std::collections::HashMap<String, Box<dyn FeatTrait>>,
    /// A simple boolean to enable dev mode,
    pub dev_mode_enabled: DevMode,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            back_end_utils: BackEndUtils {            
                managers: self.back_end_utils.managers.clone(),
                active_connection_request: serde_json::from_value(serde_json::json!(self.back_end_utils.active_connection_request))
                    .unwrap(),
            },
            profile_settings: self.profile_settings.clone(),
            credentials: self.credentials.clone(),
            current_user_prompt: self.current_user_prompt.clone(),
            debug_messages: self.debug_messages.clone(),
            user_journey: self.user_journey.clone(),
            connections: self.connections.clone(),
            user_data_query: self.user_data_query.clone(),
            extensions: self.extensions.clone(),
            dev_mode_enabled: self.dev_mode_enabled.clone(),
        }
    }
}

impl AppState {
    pub fn insert_extension(mut self, key: &str, extension: Box<dyn FeatTrait>) -> Self {
        self.extensions.insert(key.to_string(), extension);
        self
    }
}

/// BackEndUtils is a struct that contains all the utils that only the backend needs to perform its tasks.
#[derive(Default)]
pub struct BackEndUtils {
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,
    pub active_connection_request: Option<ConnectionRequest>,
}

/// Managers contains both the stronghold manager and the identity manager needed to perform operations on connections & credentials.
#[derive(Default)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

/// IdentityManager contains the subject, provider_manager and wallet needed to perform operations within the oid4vc library.
pub struct IdentityManager {
    pub subject: Arc<dyn Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}

/// ProfileSettings contains all matters concerning the user profile and its settings.
#[derive(Default, Serialize, Deserialize, Derivative, TS, Clone, PartialEq, Debug)]
#[ts(export)]
#[serde(default)]
pub struct ProfileSettings {
    pub locale: Locale,
    pub profile: Option<Profile>,
}

#[typetag::serde(name = "profile_settings")]
impl FeatTrait for ProfileSettings {}

/// Format of a locale string: `ll_CC` - where ll is the language code (ISO 639) and CC is the country code (ISO 3166).
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default, EnumString)]
#[ts(export)]
#[allow(non_camel_case_types)]
pub enum Locale {
    #[default]
    #[serde(rename = "en-US")]
    en_US,
    #[serde(rename = "en-GB")]
    en_GB,
    #[serde(rename = "de-DE")]
    de_DE,
    #[serde(rename = "nl-NL")]
    nl_NL,
}

#[typetag::serde(name = "locale")]
impl FeatTrait for Locale {}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: Option<String>,
    pub primary_did: String,
}

#[typetag::serde(name = "profile")]
impl FeatTrait for Profile {}

/// DevMode is a simple enum to enable dev mode for developers to test the app.
#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq, Default)]
#[ts(export, export_to = "bindings/DevMode.ts")]
pub enum DevMode {
    On,
    #[default]
    Off,
    OnWithAutologin,
}

#[typetag::serde(name = "dev_mode")]
impl FeatTrait for DevMode {}

/// Connection contains the ID and information of a connection.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Connection {
    pub id: String,
    pub client_name: String,
    pub url: String,
    pub verified: bool,
    pub first_interacted: String,
    pub last_interacted: String,
}

#[typetag::serde(name = "connection")]
impl FeatTrait for Connection {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_app_state_serialize() {
        let state = AppState {
            profile_settings: ProfileSettings {
                locale: Locale::en_US,
                profile: Some(Profile {
                    name: "John Doe".to_string(),
                    picture: None,
                    theme: None,
                    primary_did: "did:example:123".to_string(),
            })},
            credentials: vec![],
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            debug_messages: Default::default(),
            user_journey: None,
            connections: vec![],
            ..Default::default()
        };

        let serialized = serde_json::to_string_pretty(&state).unwrap();

        // AppState is serialized without the `managers` and `active_connection_request` fields.
        // Probably a basic json file instead of the indoc! is cleaner.
        assert_eq!(
            serialized,
            indoc! {
            r#"{
                  "active_profile": {
                    "name": "John Doe",
                    "picture": null,
                    "theme": null,
                    "primary_did": "did:example:123"
                  },
                  "locale": "en-US",
                  "credentials": [],
                  "current_user_prompt": {
                    "type": "redirect",
                    "target": "me"
                  },
                  "dev_mode": "Off",
                  "debug_messages": [],
                  "user_journey": null,
                  "connections": [],
                  "user_data_query": [],
                  "extensions": {}
                  "user_data_query": [],
                  "extensions": {}
                }"#}
        );
    }
}
