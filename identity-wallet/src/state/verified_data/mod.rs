pub mod actions;
mod reducers;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

// TODO: rename, should be a trait that any of the verifications can implement
#[derive(Default, Debug, TS, Clone, Serialize, Deserialize)]
#[ts(export, export_to = "bindings/verified_data/VerifiedData.ts")]
pub struct VerifiedData {
    pub email_verification: Option<EmailVerification>,
}

#[derive(Debug, TS, Clone, Serialize, Deserialize)]
#[ts(export, export_to = "bindings/verified_data/EmailVerification.ts")]
pub struct EmailVerification {
    pub email: String,
    pub label: String,
    pub verification_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub validation_expiration_in_secs: Option<i32>,
    pub error: Option<String>,
}
