use crate::reducer;
use crate::state::verified_data::reducers::{redeem_code, send_verification_email};
use crate::state::{actions::ActionTrait, Reducer};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

// Initialize verification session

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

// Redeem code

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/RedeemCode.ts")]
pub struct RedeemCode {
    pub code: String,
}

#[typetag::serde(name = "[Verified Data] Redeem code")]
impl ActionTrait for RedeemCode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(redeem_code)]
    }
}
