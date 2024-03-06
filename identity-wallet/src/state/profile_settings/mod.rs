pub mod create_new;
pub mod set_locale;
pub mod update_profile_settings;

use super::{AppTheme, FeatTrait};

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use strum::EnumString;
use ts_rs::TS;

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

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: Option<AppTheme>,
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
