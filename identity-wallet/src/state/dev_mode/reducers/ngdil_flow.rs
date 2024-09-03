use crate::{
    command,
    error::AppError,
    state::{
        connections::actions::connection_accepted::ConnectionAccepted,
        credentials::actions::{
            credential_offers_selected::CredentialOffersSelected, credentials_selected::CredentialsSelected,
        },
        qr_code::actions::qrcode_scanned::QrCodeScanned,
        user_prompt::CurrentUserPrompt,
    },
    state::{dev_mode::actions::dev_profile::OID4VCSteps, AppState},
};

use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn ngdil_flow(state: AppState, ngdil_step: OID4VCSteps, auto_confirm: bool) -> Result<AppState, AppError> {
    match ngdil_step {
        OID4VCSteps::Connect => {
            let mut state = add_connection(state).await?;

            if auto_confirm {
                state = accept_connection(state).await?;
            }

            return Ok(state);
        }
        OID4VCSteps::Share => {
            let mut state = add_presentation_request(state).await?;

            if auto_confirm {
                state = share_credentials(state).await?;
            }

            return Ok(state);
        }
        OID4VCSteps::Receive => {
            let mut state = add_credential(state).await?;

            if auto_confirm {
                state = accept_credential(state).await?;
            }

            return Ok(state);
        }
        OID4VCSteps::All => {
            // For all, auto_confirm is always true
            let mut state = add_connection(state).await?;
            state = accept_connection(state).await?;
            state = add_presentation_request(state).await?;
            state = share_credentials(state).await?;
            state = add_credential(state).await?;
            state = accept_credential(state).await?;
            state = add_future_engineer(state).await?;
            state = accept_future_engineer(state).await?;

            return Ok(state);
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CredentialResponse {
    uri: String,
    pin: serde_json::Value,
    offer: serde_json::Value,
}

async fn add_credential(state: AppState) -> Result<AppState, AppError> {
    // URL from NGDIL demo
    let url = "https://staging.api.ngdil.com/api/starting-offer";

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
        credential_configuration_ids: vec![
            "National ID".to_string(),
            "School Course Certificate".to_string(),
            "Volunteer Badge".to_string(),
            "Higher Education Information Literacy Level 1".to_string(),
            "Business Innovation & Interdisciplinair Samenwerken".to_string(),
        ],
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
    let url = "https://staging.api.ngdil.com/siop";

    let payload = serde_json::json!({
        "clientMetadata": {
            "logoUri": "https://staging.client.ngdil.com/imgs/kw1c-white.png",
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
    let url = "https://staging.api.ngdil.com/api/oid4vp";

    let payload = serde_json::json!({
        "presentationStage":"dominiqueEnrolCourse",
        "clientMetadata":{"logoUri":"https://staging.client.ngdil.com/imgs/kw1c-white.png","clientName":"Koning Willem I College"}
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
    let url = "https://staging.api.ngdil.com/api/credential-offer";

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
    let cr_selected = CredentialOffersSelected {
        credential_configuration_ids: vec!["Future Engineer Certificate".to_string()],
    };

    command::reduce(state, Arc::new(cr_selected)).await
}
