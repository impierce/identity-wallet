use self::{connections::Connection,
    credentials::DisplayCredential,
    profile_settings::ProfileSettings, 
    shared::backend_utils::BackEndUtils, 
    user_prompt::CurrentUserPrompt};
use downcast_rs::{impl_downcast, DowncastSync};
use std::{collections::VecDeque, sync::Arc};
use serde::{Deserialize, Serialize};
use derivative::Derivative;
use dyn_clone::DynClone;
use ts_rs::TS;

pub mod common;
pub mod dev_mode;
pub mod extension;
pub mod connections;
pub mod credentials;
pub mod shared;
pub mod user_data_query;
pub mod profile_settings;
pub mod user_journey;
pub mod actions;
pub mod user_prompt;

/// Trait which each field of the appstate has to implement.
#[typetag::serde(tag = "feat_state_type")]
pub trait FeatTrait: Send + Sync + std::fmt::Debug + DynClone + DowncastSync{}
dyn_clone::clone_trait_object!(FeatTrait);
impl_downcast!(sync FeatTrait);

/// The container for the application state, simplifying the code to just one mutex.
#[derive(Default, Debug)]
pub struct AppStateContainer(pub tokio::sync::Mutex<AppState>);

impl AppStateContainer{
    pub async fn insert_extension (self, key: &str, extension: Box<dyn FeatTrait>) -> Self
    {
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
    pub dev_mode_enabled: bool,
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
            dev_mode_enabled: self.dev_mode_enabled,
        }
    }
}

impl AppState{
    pub fn insert_extension (mut self, key: &str, extension: Box<dyn FeatTrait>) -> Self
    {
        self.extensions.insert(key.to_string(), extension);
        self
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use indoc::indoc;
//     use tests::profile_settings::{Locale, Profile};

//     #[test]
//     fn test_app_state_serialize() {
//         let state = AppState {
//             current_user_prompt: Some(CurrentUserPrompt::Redirect {
//                 target: "me".to_string(),
//             }),
//             profile_settings: ProfileSettings {
//                 locale: Locale::En,
//                 profile: Some(Profile {
//                     name: "John Doe".to_string(),
//                     picture: None,
//                     theme: None,
//                     primary_did: "did:example:123".to_string(),
//             })},
//             ..Default::default()
//         };

//         let serialized = serde_json::to_string_pretty(&state).unwrap();

//         // AppState is serialized without the `managers` and `active_connection_request` fields.
//         // Probably a basic json file instead of the indoc! is cleaner.
//         assert_eq!(
//             serialized,
//             indoc! {
//             r#"{
//                   "credentials": [],
//                   "current_user_prompt": {
//                     "type": "redirect",
//                     "target": "me"
//                   },
//                   "connections": [],
//                   "user_data_query": [],
//                   "locale": "en",
//                   "profile": {
//                     "name": "John Doe",
//                     "picture": null,
//                     "theme": null,
//                     "primary_did": "did:example:123"
//                   },
//                   "user_journey": null,
//                   "debug_messages": [],
//                   "extensions": {},
//                   "dev_mode_enabled": false
//                 }"#}
//         );
//     }
// }
