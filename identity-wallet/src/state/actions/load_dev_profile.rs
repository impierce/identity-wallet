use crate::reducer;
use ts_rs::TS;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::load_dev_profile;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, Default)]
#[ts(export, export_to = "bindings/actions/DevProfileType.ts")]
pub enum ProfileType {
    /// Dev profile which preloads data
    Ferris,

    /// Dev profile which recreate steps
    Turtle,

    /// No dev profile loaded
    #[default]
    None,
}


#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DevProfile.ts")]
pub struct DevProfile {
    pub profile: ProfileType,
}

#[typetag::serde(name = "[DEV] Load DEV profile")]
impl ActionTrait for DevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}
