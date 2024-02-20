use super::dev_mode::PASSWORD;
use crate::{
    command,
    error::AppError,
    state::{
        actions::{ConnectionAccepted, CreateNew, CredentialOffersSelected, CredentialsSelected, DevProfile, ProfileSteps, QrCodeScanned, Reset},
        user_prompt::CurrentUserPrompt,
        AppState, DevMode,
    },
};
use log::{debug, info};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn load_dragon_profile(mut state: AppState, dev_profile: DevProfile) -> Result<AppState, AppError> {
    let steps = dev_profile
        .execute_step
        .expect("For dragon profile steps are expected");

    info!("Profile steps executed: {:?}", steps);

    state = reset_settings(state).await?;
    state = create_new_profile(state).await?;

    if ProfileSteps::AddCredentials <= steps {
        debug!("Add credentials step executed");
        state = add_credential(state).await?;
    }

    if ProfileSteps::AcceptCredentials <= steps {
        debug!("Accept credentials step executed");
        state = accept_credential(state).await?;
    }

    if ProfileSteps::AddConnection <= steps {
        debug!("Add connection step executed");
        state = add_connection(state).await?;
    }

    if ProfileSteps::AcceptConnection <= steps {
        debug!("Accept connection step executed");
        state = accept_connection(state).await?;
    }

    if ProfileSteps::AddPresentation <= steps {
        debug!("Add presentation step executed");
        state = add_presentation_request(state).await?;
    }

    if ProfileSteps::ShareCredentails <= steps {
        debug!("Share credentials step executed");
        state = share_credentials(state).await?;
    }

    if ProfileSteps::AddFutureEngineer <= steps {
        debug!("Add future engineer step executed");
        state = add_future_engineer(state).await?;
    }

    if ProfileSteps::CompleteFlow <= steps {
        debug!("Accept future engineer step executed");
        state = accept_future_engineer(state).await?;
    }

    state.dev_mode = DevMode::OnWithAutologin;

    Ok(state)
}


async fn create_new_profile(state: AppState) -> Result<AppState, AppError> {
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

async fn add_credential(state: AppState) -> Result<AppState, AppError> {
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

async fn accept_credential(state: AppState) -> Result<AppState, AppError> {
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

async fn add_connection(state: AppState) -> Result<AppState, AppError> {
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

async fn accept_connection(state: AppState) -> Result<AppState, AppError> {
    command::reduce(state, Arc::new(ConnectionAccepted)).await
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PresentationResponse {
    uri: String,
    request: serde_json::Value,
    #[serde(rename = "requestOptions")]
    request_options: serde_json::Value,
}

async fn add_presentation_request(state: AppState) -> Result<AppState, AppError> {
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

async fn share_credentials(state: AppState) -> Result<AppState, AppError> {
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

async fn add_future_engineer(state: AppState) -> Result<AppState, AppError> {
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

async fn accept_future_engineer(state: AppState) -> Result<AppState, AppError> {
    let cr_selected = CredentialOffersSelected { offer_indices: vec![0] };

    command::reduce(state, Arc::new(cr_selected)).await
}

async fn reset_settings(state: AppState) -> Result<AppState, AppError> {
    command::reduce(state, Arc::new(Reset)).await
}
