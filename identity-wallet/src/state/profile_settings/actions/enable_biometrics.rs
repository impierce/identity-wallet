use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    reducer,
    state::{actions::ActionTrait, profile_settings::reducers::enable_biometrics::enable_biometrics, Reducer},
};

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/EnableBiometrics.ts")]
pub struct EnableBiometrics {
    pub enable: bool,
}

#[typetag::serde(name = "[Biometrics] Enable")]
impl ActionTrait for EnableBiometrics {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(enable_biometrics)]
    }
}
