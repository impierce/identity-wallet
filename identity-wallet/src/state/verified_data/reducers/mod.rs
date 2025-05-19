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

const EMAIL_VERIFICATION_SERVICE_HOST: &str = env!("EMAIL_VERIFICATION_SERVICE_HOST");
const EMAIL_VERIFICATION_SERVICE_API_KEY: &str = env!("EMAIL_VERIFICATION_SERVICE_API_KEY");

pub async fn check_service_health(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<ServiceHealthCheck>(action) {
        info!("[>>>] {}", action.service);
        let response = reqwest::Client::new()
            .get(format!("{}/healthz", EMAIL_VERIFICATION_SERVICE_HOST))
            .header("X-API-KEY", EMAIL_VERIFICATION_SERVICE_API_KEY)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status() == reqwest::StatusCode::OK {
                    info!("[<<<] Service is healthy: {}", resp.status());
                } else {
                    warn!("[<<<] Service returned non-OK status: {}", resp.status());
                    return Err(AppError::Error(format!(
                        "email-verification-service responded with {}",
                        resp.status()
                    )));
                }
            }
            Err(err) => {
                warn!("Failed to check service health: {}", err);
                return Err(AppError::Error(format!(
                    "email-verification-service health could not be checked: {}",
                    err
                )));
            }
        }
    }
    Ok(AppState {
        current_user_prompt: None,
        ..state
    })
}

pub async fn send_verification_email(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<SendVerificationEmail>(action) {
        let url = format!("{}/api/verify", EMAIL_VERIFICATION_SERVICE_HOST);
        let body = json!({ "email": action.email });
        info!("[>>>] {} {}", url, body);
        let response = reqwest::Client::new()
            .post(url)
            .header("X-API-KEY", EMAIL_VERIFICATION_SERVICE_API_KEY)
            .json(&body)
            .send()
            .await
            .inspect_err(|err| {
                warn!("Failed to send verification request: {}", err);
            })
            .map_err(|err| AppError::Error(err.to_string()))?;
        // TODO: handle error
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
                    label: action.label,
                    verification_id: Some(id.to_string()),
                    expires_at: Some(chrono::DateTime::parse_from_rfc3339(expires_at).unwrap().to_utc()),
                    validation_expiration_in_secs: Some(validation_expiration_in_secs),
                    error: None,
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
        let url = format!("{}/api/verify/{}", EMAIL_VERIFICATION_SERVICE_HOST, session_id);
        let body = json!({ "code": action.code });
        info!("[>>>] {} {}", url, body);
        let response = reqwest::Client::new()
            .post(url)
            .header("X-API-KEY", EMAIL_VERIFICATION_SERVICE_API_KEY)
            .json(&body)
            .send()
            .await
            .inspect_err(|err| {
                warn!("Failed to send verification code: {}", err);
            })
            .ok()
            .unwrap();

        info!("[<<<] {:?}", response);

        match response.status().as_u16() {
            200 => {
                let credential_offer_value: String = response.text().await.unwrap();
                let action = QrCodeScanned {
                    form_urlencoded: credential_offer_value,
                };
                return Ok(read_credential_offer(state, std::sync::Arc::new(action)).await.unwrap());
            }
            _ => {
                let error: serde_json::Value = response.json().await?;
                warn!("Failed to redeem code: {}", error);
                return Ok(AppState {
                    verified_data: VerifiedData {
                        email_verification: Some(EmailVerification {
                            error: Some(error.get("error").unwrap().as_str().unwrap().to_string()),
                            ..state
                                .verified_data
                                .email_verification
                                .expect("tried to redeem a code without an active email verification flow")
                        }),
                        ..state.verified_data
                    },
                    ..state
                });
            }
        }
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
