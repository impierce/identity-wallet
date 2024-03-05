use crate::{
    reducer,
    state::{
        actions::ActionTrait, profile_settings::reducers::update_profile_settings::update_profile_settings, Reducer,
    },
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to update the profile settings.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateProfileSettings.ts")]
pub struct UpdateProfileSettings {
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub picture: Option<String>,
    #[ts(optional)]
    pub theme: Option<String>,
}

#[typetag::serde(name = "[Settings] Update profile")]
impl ActionTrait for UpdateProfileSettings {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_profile_settings)]
    }
}
