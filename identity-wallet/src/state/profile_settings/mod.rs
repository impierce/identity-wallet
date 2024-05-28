pub mod actions;
pub mod reducers;

use super::FeatTrait;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use strum::EnumString;
use ts_rs::TS;

/// ProfileSettings contains all matters concerning the user profile and its settings.
#[derive(Serialize, Deserialize, Derivative, TS, Clone, PartialEq, Debug)]
#[derivative(Default)]
#[ts(export)]
#[serde(default)]
pub struct ProfileSettings {
    pub locale: Locale,
    pub profile: Option<Profile>,
    // TODO: Current simplified solution for handling a default DID method. Once we have the did-manager implemented, we
    // should probably come up with a different solution.
    #[derivative(Default(value = r#"String::from("did:jwk")"#))]
    pub preferred_did_method: String,
    #[derivative(Default(value = r#"String::from("Ed25519")"#))]
    pub preferred_key_type: String,
}

#[typetag::serde(name = "profile_settings")]
impl FeatTrait for ProfileSettings {}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq, Default)]
#[ts(export, export_to = "bindings/AppTheme.ts")]
pub enum AppTheme {
    #[default]
    #[serde(rename = "system")]
    System,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
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
