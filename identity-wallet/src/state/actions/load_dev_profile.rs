use crate::reducer;
use ts_rs::TS;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::load_dev_profile;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export, export_to = "bindings/actions/DevProfileType.ts")]
pub enum ProfileType {
    /// Dev profile which preloads data
    Ferris,

    /// Dev profile which recreate steps
    Dragon,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq, PartialOrd)]
#[ts(export, export_to = "bindings/actions/DevProfileSteps.ts")]
pub enum ProfileSteps {
    /// Step 1
    CreateProfile,
    /// Step 2
    AddCredentials,
    /// Step 3
    AcceptCredentials,
    /// Step 4
    AddConnection,
    /// Step 5
    AcceptConnection,
    /// Step 6
    AddPresentation,
    /// Step 7
    ShareCredentails,
    /// Step 8
    AddFutureEngineer,
    /// Step 9
    CompleteFlow,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DevProfile.ts")]
pub struct DevProfile {
    pub profile: ProfileType,
    pub execute_steps: Option<ProfileSteps>,
}

#[typetag::serde(name = "[DEV] Load DEV profile")]
impl ActionTrait for DevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}
