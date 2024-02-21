use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::dev_mode::load_dev_profile;
use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export, export_to = "bindings/dev/ProfileType.ts")]
pub enum ProfileType {
    /// Dev profile with predefined static data
    Ferris,

    /// Dev profile that can dynamically interact with the NGDIL demo by executing `ProfileSteps`
    Dragon,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq, PartialOrd)]
#[ts(export, export_to = "bindings/dev/ProfileSteps.ts")]
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
    pub execute_step: Option<ProfileSteps>,
}

#[typetag::serde(name = "[DEV] Load DEV profile")]
impl ActionTrait for DevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}
