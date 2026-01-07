use chrono::{DateTime, Utc};
use log::{info, warn};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        core_utils::helpers::ValueToString,
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
            .get(format!("{EMAIL_VERIFICATION_SERVICE_HOST}/healthz"))
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
                warn!("Failed to check service health: {err}");
                return Err(AppError::Error(format!(
                    "email-verification-service health could not be checked: {err}"
                )));
            }
        }
    }
    Ok(AppState {
        current_user_prompt: None,
        ..state
    })
}

/// Expected response type from email verification service.
#[derive(Deserialize, Debug)]
struct VerificationResponse {
    id: String,
    expires_at: DateTime<Utc>,
    validation_expiration_in_secs: i32,
}

pub async fn send_verification_email(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<SendVerificationEmail>(action) {
        let url = format!("{EMAIL_VERIFICATION_SERVICE_HOST}/api/verify");
        let body = json!({ "email": action.email });
        info!("[>>>] {url} {body}");
        let response = reqwest::Client::new()
            .post(url)
            .header("X-API-KEY", EMAIL_VERIFICATION_SERVICE_API_KEY)
            .json(&body)
            .send()
            .await
            .inspect_err(|err| {
                warn!("Failed to send verification request: {err}");
            })
            .map_err(|err| AppError::Error(err.to_string()))?;
        let response: VerificationResponse = response.json().await.map_err(|err| AppError::Error(err.to_string()))?;
        info!("[<<<] {response:?}");
        return Ok(AppState {
            verified_data: VerifiedData {
                email_verification: Some(EmailVerification {
                    email: action.email,
                    label: action.label,
                    verification_id: Some(response.id),
                    expires_at: Some(response.expires_at),
                    validation_expiration_in_secs: Some(response.validation_expiration_in_secs),
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
            .and_then(|email_verification| email_verification.verification_id.clone())
            .ok_or(AppError::Error(
                "Tried to redeem a code without an active email verification flow".to_string(),
            ))?;
        let url = format!("{EMAIL_VERIFICATION_SERVICE_HOST}/api/verify/{session_id}");
        let body = json!({ "code": action.code });
        info!("[>>>] {url} {body}");
        let response = reqwest::Client::new()
            .post(url)
            .header("X-API-KEY", EMAIL_VERIFICATION_SERVICE_API_KEY)
            .json(&body)
            .send()
            .await
            .map_err(|err| AppError::Error(format!("Failed to send verification code: {err}")))?;

        info!("[<<<] {response:?}");

        match response.status().as_u16() {
            200 => {
                let credential_offer_value: String = response.text().await?;
                let action = QrCodeScanned {
                    form_urlencoded: credential_offer_value,
                };
                return read_credential_offer(
                    AppState {
                        verified_data: VerifiedData {
                            email_verification: None,
                        },
                        ..state
                    },
                    std::sync::Arc::new(action),
                )
                .await;
            }
            _ => {
                let error: serde_json::Value = response.json().await?;
                warn!("Failed to redeem code: {error}");
                return Ok(AppState {
                    verified_data: VerifiedData {
                        email_verification: Some(EmailVerification {
                            error: error.get("error").and_then(|value| value.to_clean_string()),
                            ..state
                                .verified_data
                                .email_verification
                                .expect("tried to redeem a code without an active email verification flow")
                        }),
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
