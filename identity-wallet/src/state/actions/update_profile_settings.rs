use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::update_profile_settings;
use crate::state::AppTheme;

use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateProfileSettings.ts")]
pub struct UpdateProfileSettings {
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub picture: Option<String>,
    #[ts(optional)]
    pub theme: Option<AppTheme>,
}

#[typetag::serde(name = "[Settings] Update profile")]
impl ActionTrait for UpdateProfileSettings {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_profile_settings)]
    }
}
