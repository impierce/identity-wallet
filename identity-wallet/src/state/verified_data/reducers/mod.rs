use log::{info, warn};
use serde_json::json;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        AppState, VerifiedData,
    },
};

use super::{actions::SendVerificationEmail, EmailVerification};

pub async fn send_verification_email(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<SendVerificationEmail>(action) {
        let url = "http://localhost:5177/api/verify";
        let body = json!({ "email": action.email });
        info!("[>>>] {}", body);
        let response = reqwest::Client::new()
            .post(url)
            .json(&body)
            .send()
            .await
            .inspect_err(|err| {
                warn!("Failed to send verification: {}", err);
            })
            .ok();
        let json_response: serde_json::Value = response.unwrap().json().await.unwrap();
        info!("[<<<] {}", json_response);
        let id = json_response.get("id").unwrap().as_str().unwrap();
        let expires_at = json_response.get("expires_at").unwrap().as_str().unwrap();
        let validation_expiration_in_secs = json_response
            .get("validation_expiration_in_secs")
            .unwrap()
            .as_i64()
            .unwrap() as i32;
        return Ok(AppState {
            verified_data: VerifiedData {
                email_verification: Some(EmailVerification {
                    email: action.email,
                    verification_id: Some(id.to_string()),
                    expires_at: Some(chrono::DateTime::parse_from_rfc3339(expires_at).unwrap().to_utc()),
                    validation_expiration_in_secs: Some(validation_expiration_in_secs),
                }),
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
