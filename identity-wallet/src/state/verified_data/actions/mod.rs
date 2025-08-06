use crate::reducer;
use crate::state::verified_data::reducers::{
    check_service_health, redeem_code, reset_email_verification, send_verification_email,
};
use crate::state::{actions::ActionTrait, Reducer};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to check the health of an external service.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/ServiceHealthCheck.ts")]
pub struct ServiceHealthCheck {
    pub service: String,
}

#[typetag::serde(name = "[Verified Data] Check service health")]
impl ActionTrait for ServiceHealthCheck {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(check_service_health)]
    }
}

/// Action to trigger sending a verification email.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/SendVerificationEmail.ts")]
pub struct SendVerificationEmail {
    pub email: String,
    pub label: String,
}

#[typetag::serde(name = "[Verified Data] Send verification email")]
impl ActionTrait for SendVerificationEmail {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_verification_email)]
    }
}

/// Action to redeem a verification code.
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

/// Action to reset the current email verification flow.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetEmailVerification;

#[typetag::serde(name = "[Verified Data] Reset email verification")]
impl ActionTrait for ResetEmailVerification {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_email_verification)]
    }
}
