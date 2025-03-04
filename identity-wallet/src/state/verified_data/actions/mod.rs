use crate::reducer;
use crate::state::verified_data::reducers::send_verification_email;
use crate::state::{actions::ActionTrait, Reducer};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/SendVerificationEmail.ts")]
pub struct SendVerificationEmail {
    pub email: String,
}

#[typetag::serde(name = "[Verified Data] Send verification email")]
impl ActionTrait for SendVerificationEmail {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_verification_email)]
    }
}
