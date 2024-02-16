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

/// The app language of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default, EnumString)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Locale {
    #[default]
    En,
    De,
    Nl,
}

#[typetag::serde(name = "locale")]
impl FeatTrait for Locale {}

#[cfg(test)]
mod tests {
    use super::*;
    use super::actions::SetLocale;
    use super::reducers::set_locale;
    use crate::state::{actions::Action, AppState};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_set_locale() {
        let mut app_state = AppState::default();

        app_state = set_locale(app_state, Arc::new(SetLocale { locale: Locale::Nl }) as Action)
            .await
            .unwrap();

        assert_eq!(app_state.profile_settings.locale, Locale::Nl);
    }
}
