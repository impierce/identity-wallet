use crate::state::FeatTrait;
use serde::{Deserialize, Serialize};
use derivative::Derivative;
use strum::EnumString;
use ts_rs::TS;

pub mod actions;
pub mod reducers;

/// The profile settings of the current user.
#[derive(Default, Serialize, Deserialize, Derivative, TS, Clone, PartialEq, Debug)]
#[ts(export)]
#[serde(default)]
pub struct ProfileSettings {
    /// Locale is a separate field from the profile only because of onboarding, 
    /// where the user needs to be able to choose the language before anything else.
    pub locale: Locale,
    pub profile: Option<Profile>,
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
    pub theme: Option<String>,
    pub primary_did: String,
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

#[cfg(test)]
mod tests {
    use crate::state::profile_settings::actions::UpdateProfileSettings;
    use tests::reducers::update_profile_settings;
    use super::*;
    use crate::state::AppState;
    use super::actions::SetLocale;
    use super::reducers::set_locale;
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
            theme: Some("system".to_string()),
            primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
        };

        let mut app_state = AppState {
            profile_settings: ProfileSettings{
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
                theme: Some("light".to_string()),
            }),
        )
        .await
        .unwrap();

        assert_eq!(
            app_state.profile_settings.profile,
            Some(Profile {
                theme: Some("light".to_string()),
                ..active_profile
            })
        );
    }
}
