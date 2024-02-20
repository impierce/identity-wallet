use super::dev_mode::PASSWORD;
use crate::{
    command,
    error::AppError,
    state::{
        actions::{ConnectionAccepted, CreateNew, CredentialOffersSelected, CredentialsSelected, QrCodeScanned, Reset},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub(super) async fn create_new_profile(state: AppState) -> Result<AppState, AppError> {
    let create_new = CreateNew {
        name: "Shenron".to_string(),
        picture: "&#x1F432".to_string(),
        theme: "dark".to_string(),
        password: PASSWORD.to_string(),
    };

    command::reduce(state, Arc::new(create_new)).await
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CredentialResponse {
    uri: String,
    pin: serde_json::Value,
    offer: serde_json::Value,
}

pub(super) async fn add_credential(state: AppState) -> Result<AppState, AppError> {
    // URL from NGDIL demo
    let url = "https://api.demo.ngdil.com/api/starting-offer";

    let payload = serde_json::json!({
        "credentials": [
            "National ID",
            "School Course Certificate",
            "Volunteer Badge",
            "Higher Education Information Literacy Level 1",
            "Business Innovation & Interdisciplinair Samenwerken"
        ]
    });

    let response: CredentialResponse = reqwest::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?
        .json()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?;

    let qr_code = QrCodeScanned {
        form_urlencoded: response.uri,
    };

    command::reduce(state, Arc::new(qr_code)).await
}

pub(super) async fn accept_credential(state: AppState) -> Result<AppState, AppError> {
    let cr_selected = CredentialOffersSelected {
        offer_indices: vec![0, 1, 2, 3, 4],
    };

    command::reduce(state, Arc::new(cr_selected)).await
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ConnectionResponse {
    uri: String,
    request: serde_json::Value,
    #[serde(rename = "requestOptions")]
    request_options: serde_json::Value,
}

pub(super) async fn add_connection(state: AppState) -> Result<AppState, AppError> {
    // URL from NGDIL demo
    let url = "https://api.demo.ngdil.com/siop";

    let payload = serde_json::json!({
        "clientMetadata": {
            "logoUri": "https://demo.ngdil.com/imgs/kw1c-white.png",
            "clientName": "Koning Willem I College"
        }
    });

    let response: ConnectionResponse = reqwest::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?
        .json()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?;

    let qr_code = QrCodeScanned {
        form_urlencoded: response.uri,
    };

    command::reduce(state, Arc::new(qr_code)).await
}

pub(super) async fn accept_connection(state: AppState) -> Result<AppState, AppError> {
    command::reduce(state, Arc::new(ConnectionAccepted)).await
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PresentationResponse {
    uri: String,
    request: serde_json::Value,
    #[serde(rename = "requestOptions")]
    request_options: serde_json::Value,
}

pub(super) async fn add_presentation_request(state: AppState) -> Result<AppState, AppError> {
    // URL from NGDIL demo
    let url = "https://api.demo.ngdil.com/api/oid4vp";

    let payload = serde_json::json!({
        "presentationStage":"dominiqueEnrolCourse",
        "clientMetadata":{"logoUri":"https://demo.ngdil.com/imgs/kw1c-white.png","clientName":"Koning Willem I College"}
    });

    let response: PresentationResponse = reqwest::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?
        .json()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?;

    let qr_code = QrCodeScanned {
        form_urlencoded: response.uri,
    };

    command::reduce(state, Arc::new(qr_code)).await
}

pub(super) async fn share_credentials(state: AppState) -> Result<AppState, AppError> {
    if let Some(CurrentUserPrompt::ShareCredentials {
        client_name: _,
        logo_uri: _,
        options,
    }) = &state.current_user_prompt
    {
        let credential_uuids: Vec<Uuid> = options
            .iter()
            .map(|uuid_str| Uuid::parse_str(uuid_str).unwrap())
            .collect();

        let cr_selected = CredentialsSelected { credential_uuids };

        command::reduce(state, Arc::new(cr_selected)).await
    } else {
        Err(AppError::Error("Presentation not accepted".to_string()))
    }
}

pub(super) async fn add_future_engineer(state: AppState) -> Result<AppState, AppError> {
    let url = "https://api.demo.ngdil.com/api/credential-offer";

    let payload = json!({"credential":"Future Engineer","issuer":"kw1c"});

    let response: CredentialResponse = reqwest::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?
        .json()
        .await
        .map_err(|err| AppError::Error(err.to_string()))?;

    let qr_code = QrCodeScanned {
        form_urlencoded: response.uri,
    };

    command::reduce(state, Arc::new(qr_code)).await
}

pub(super) async fn accept_future_engineer(state: AppState) -> Result<AppState, AppError> {
    let cr_selected = CredentialOffersSelected { offer_indices: vec![0] };

    command::reduce(state, Arc::new(cr_selected)).await
}

pub(super) async fn reset_settings(state: AppState) -> Result<AppState, AppError> {
    command::reduce(state, Arc::new(Reset)).await
}
