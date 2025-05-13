use crate::{
    reducer,
    state::{actions::ActionTrait, common::reducers::check_password::check_password, Reducer},
};

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use ts_rs::TS;

/// Check the password.
#[derive(Serialize, Deserialize, TS, Clone, Default)]
#[ts(export, export_to = "bindings/actions/CheckPassword.ts")]
pub struct CheckPassword {
    pub password: String,
}

impl std::fmt::Debug for CheckPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckPassword").field("password", &"*****").finish()
    }
}

#[typetag::serde(name = "[Storage] Check password")]
impl ActionTrait for CheckPassword {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(check_password)]
    }
}
