use log::{info, warn};
use serde_json::json;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        qr_code::{actions::qrcode_scanned::QrCodeScanned, reducers::read_credential_offer::read_credential_offer},
        verified_data::{
            actions::{RedeemCode, ResetEmailVerification, SendVerificationEmail, ServiceHealthCheck},
            EmailVerification,
        },
        AppState, VerifiedData,
    },
};

const EMAIL_VERIFICATION_SERVICE_HOST: &str = "http://localhost:5177";

pub async fn check_service_health(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<ServiceHealthCheck>(action) {
        // let body = json!({ "service": action.service });
        info!("[>>>] {}", action.service);
        let response = reqwest::Client::new()
            .get(format!("{}/healthz", EMAIL_VERIFICATION_SERVICE_HOST))
            // .json(&body)
            .send()
            .await
            .inspect_err(|err| {
                warn!("Failed to check service health: {}", err);
            })
            .map_err(|err| AppError::Error(err.to_string()))?;
        // .ok();
        info!("[<<<] {}", response.status());
        return Ok(state);
    }
    Ok(state)
}

pub async fn send_verification_email(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<SendVerificationEmail>(action) {
        let body = json!({ "email": action.email });
        info!("[>>>] {}", body);
        let response = reqwest::Client::new()
            .post(format!("{}/api/verify", EMAIL_VERIFICATION_SERVICE_HOST))
            .json(&body)
            .send()
            .await
            .inspect_err(|err| {
                warn!("Failed to send verification: {}", err);
            })
            .map_err(|err| AppError::Error(err.to_string()))?;
        let json_response: serde_json::Value = response.json().await.unwrap();
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

pub async fn redeem_code(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<RedeemCode>(action) {
        let session_id = state
            .verified_data
            .email_verification
            .as_ref()
            .unwrap()
            .verification_id
            .as_ref()
            .unwrap();
        let url = format!("http://localhost:5177/api/verify/{}", session_id);
        let body = json!({ "code": action.code });
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
        let credential_offer_value: String = response.unwrap().text().await.unwrap();
        info!("[<<<] {}", credential_offer_value);

        let action = QrCodeScanned {
            form_urlencoded: credential_offer_value,
        };

        let state = read_credential_offer(state, std::sync::Arc::new(action)).await.unwrap();

        return Ok(state);
    }
    Ok(state)
}

pub async fn reset_email_verification(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(_action) = listen::<ResetEmailVerification>(action) {
        return Ok(AppState {
            verified_data: VerifiedData {
                email_verification: None,
            },
            ..state
        });
    }
    Ok(state)
}
