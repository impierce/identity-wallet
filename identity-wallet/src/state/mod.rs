pub mod actions;
pub mod common;
pub mod connections;
pub mod core_utils;
pub mod credentials;
pub mod dev_mode;
pub mod profile_settings;
pub mod qr_code;
pub mod user_data_query;
pub mod user_journey;
pub mod user_prompt;

use self::{
    actions::Action, core_utils::CoreState, dev_mode::DevMode, profile_settings::ProfileSettings,
    user_prompt::CurrentUserPrompt,
};
use crate::state::core_utils::history_event::HistoryEvent;
use crate::state::credentials::DisplayCredential;
use crate::{error::AppError, state::connections::Connections};

use derivative::Derivative;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;
use futures::Future;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, pin::Pin};
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

/// A macro to wrap a reducer function in a Box and a Pin.
/// It checks the reducers for its signature,
///  as it should comply with our standard for reducers.
#[macro_export]
macro_rules! reducer {
    ($reducer:expr) => {
        Box::new(move |app_state, action| Box::pin(async move { $reducer(app_state, action).await }))
    };
}

/// A reducer is a function that takes the current state and an action and returns the new state.
pub type Reducer<'a> =
    Box<dyn Fn(AppState, Action) -> Pin<Box<dyn Future<Output = Result<AppState, AppError>> + Send>> + Send>;

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
    /// This field contains the connections.
    pub connections: Connections,
    /// This field contains the display credentials.
    pub credentials: Vec<DisplayCredential>,
    /// This field contains the query result, which is queried from credentials or connections.
    pub user_data_query: Vec<String>,
    /// This field contains utils needed for the backend to perform its tasks.
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub core_state: CoreState,
    /// This field contains the profile settings, including Locale.
    pub profile_settings: ProfileSettings,
    /// User prompts are a way for the backend to communicate a desired/required user interaction to the frontend.
    pub current_user_prompt: Option<CurrentUserPrompt>,
    /// Here user_journeys can be loaded from json_files or strings, to give the user a guided experience.
    #[ts(type = "any | null")]
    pub user_journey: Option<serde_json::Value>,
    /// Handled in command.rs, so no feature folder nor redux pattern needed.
    #[ts(type = "Array<string>")]
    pub debug_messages: VecDeque<String>,
    /// History events
    pub history: Vec<HistoryEvent>,
    /// Extensions will bring along their own redux compliant code, in the unime folder.
    #[ts(skip)]
    pub extensions: std::collections::HashMap<String, Box<dyn FeatTrait>>,
    /// A simple enum to set dev mode,
    pub dev_mode: DevMode,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            core_state: CoreState {
                managers: self.core_state.managers.clone(),
                active_connection_request: serde_json::from_value(serde_json::json!(
                    self.core_state.active_connection_request
                ))
                .unwrap(),
            },
            profile_settings: self.profile_settings.clone(),
            credentials: self.credentials.clone(),
            current_user_prompt: self.current_user_prompt.clone(),
            debug_messages: self.debug_messages.clone(),
            user_journey: self.user_journey.clone(),
            connections: self.connections.clone(),
            user_data_query: self.user_data_query.clone(),
            history: self.history.clone(),
            extensions: self.extensions.clone(),
            dev_mode: self.dev_mode.clone(),
        }
    }
}

impl AppState {
    pub fn insert_extension(mut self, key: &str, extension: Box<dyn FeatTrait>) -> Self {
        self.extensions.insert(key.to_string(), extension);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::profile_settings::Locale;
    use crate::state::profile_settings::Profile;
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
                }),
            },
            credentials: vec![],
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            debug_messages: Default::default(),
            user_journey: None,
            connections: Connections::new(),
            ..Default::default()
        };

        let serialized = serde_json::to_string_pretty(&state).unwrap();

        // AppState is serialized without the `managers` and `active_connection_request` fields.
        // Probably a basic json file instead of the indoc! is cleaner.
        assert_eq!(
            serialized,
            indoc! {
            r#"{
                  "connections": [],
                  "credentials": [],
                  "user_data_query": [],
                  "profile_settings": {
                    "locale": "en-US",
                    "profile": {
                      "name": "John Doe",
                      "picture": null,
                      "theme": null,
                      "primary_did": "did:example:123"
                    }
                  },
                  "current_user_prompt": {
                    "type": "redirect",
                    "target": "me"
                  },
                  "user_journey": null,
                  "debug_messages": [],
                  "history": [],
                  "extensions": {},
                  "dev_mode": "Off"
                }"#}
        );
    }
}
