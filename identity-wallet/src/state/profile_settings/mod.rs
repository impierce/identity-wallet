pub mod actions;
pub mod reducers;

use super::FeatTrait;

use crate::state::{SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};

use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};
use ts_rs::TS;

/// ProfileSettings contains all matters concerning the user profile and its settings.
#[derive(Serialize, Deserialize, TS, Clone, PartialEq, Debug)]
#[ts(export, export_to = "bindings/profile_settings/ProfileSettings.ts")]
#[serde(default)]
pub struct ProfileSettings {
    pub locale: Locale,
    pub profile: Option<Profile>,
    pub preferred_did_methods: Vec<String>,
    pub preferred_key_types: Vec<String>,
    pub sorting_preferences: SortingPreferences,
}

#[typetag::serde(name = "profile_settings")]
impl FeatTrait for ProfileSettings {}

impl Default for ProfileSettings {
    fn default() -> Self {
        ProfileSettings {
            locale: Locale::en_US,
            profile: None,
            preferred_did_methods: SUPPORTED_DID_METHODS.iter().map(|&method| method.to_string()).collect(),
            preferred_key_types: SUPPORTED_SIGNING_ALGORITHMS
                .iter()
                .map(|algorithm| serde_json::to_string(algorithm).unwrap().replace('"', ""))
                .collect(),
            sorting_preferences: SortingPreferences {
                credentials: Preferences {
                    sort_method: CredentialSortMethod::NameAZ,
                    reverse: false,
                },
                connections: Preferences {
                    sort_method: ConnectionSortMethod::NameAZ,
                    reverse: false,
                },
            },
        }
    }
}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export, export_to = "bindings/profile_settings/Profile.ts")]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: AppTheme,
}

#[typetag::serde(name = "profile")]
impl FeatTrait for Profile {}

/// Format of a locale string: `ll_CC` - where ll is the language code (ISO 639) and CC is the country code (ISO 3166).
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default, EnumString)]
#[ts(export, export_to = "bindings/profile_settings/Locale.ts")]
#[allow(non_camel_case_types)]
#[derive(IntoStaticStr)]
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

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export, export_to = "bindings/profile_settings/SortingPreferences.ts")]
pub struct SortingPreferences {
    pub credentials: Preferences<CredentialSortMethod>,
    pub connections: Preferences<ConnectionSortMethod>,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export, export_to = "bindings/profile_settings/Preferences.ts")]
pub struct Preferences<T> {
    pub sort_method: T,
    pub reverse: bool,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, Default, PartialEq, EnumString)]
#[ts(export, export_to = "bindings/profile_settings/CredentialSortMethod.ts")]
#[serde(rename_all = "snake_case")]
pub enum CredentialSortMethod {
    #[default]
    #[serde(rename = "name_az")]
    NameAZ,
    IssueDateNewOld,
    AddedDateNewOld,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, Default, PartialEq, EnumString)]
#[ts(export, export_to = "bindings/profile_settings/ConnectionSortMethod.ts")]
#[serde(rename_all = "snake_case")]
pub enum ConnectionSortMethod {
    #[default]
    #[serde(rename = "name_az")]
    NameAZ,
    FirstInteractedNewOld,
    LastInteractedNewOld,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq, Default)]
#[ts(export, export_to = "bindings/profile_settings/AppTheme.ts")]
#[serde(rename_all = "snake_case")]
pub enum AppTheme {
    #[default]
    System,
    Dark,
    Light,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::state::AppState;
    use tests::{
        actions::{set_locale::SetLocale, update_profile_settings::UpdateProfileSettings},
        reducers::{set_locale::set_locale, update_profile_settings::update_profile_settings},
    };

    use std::sync::Arc;

    #[tokio::test]
    async fn test_set_locale() {
        let mut app_state = AppState::default();

        app_state = set_locale(app_state, Arc::new(SetLocale { locale: Locale::nl_NL }))
            .await
            .unwrap();

        assert_eq!(app_state.profile_settings.locale, Locale::nl_NL);
    }

    #[tokio::test]
    async fn test_update_profile_settings() {
        let active_profile = Profile {
            name: "Ferris".to_string(),
            picture: Some("&#129408".to_string()),
            theme: AppTheme::System,
        };

        let mut app_state = AppState {
            profile_settings: ProfileSettings {
                profile: Some(active_profile.clone()),
                ..ProfileSettings::default()
            },
            ..AppState::default()
        };

        app_state = update_profile_settings(
            app_state,
            Arc::new(UpdateProfileSettings {
                name: None,
                picture: None,
                theme: Some(AppTheme::Light),
            }),
        )
        .await
        .unwrap();

        assert_eq!(
            app_state.profile_settings.profile,
            Some(Profile {
                theme: AppTheme::Light,
                ..active_profile
            })
        );
    }
}
