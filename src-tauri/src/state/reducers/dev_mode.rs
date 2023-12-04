use crate::crypto::stronghold::StrongholdManager;
use crate::error::AppError::{self, *};
use crate::state::actions::Action;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, Connection, Profile};
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use crate::ASSETS_DIR;
use did_key::{generate, Ed25519KeyPair};
use lazy_static::lazy_static;
use log::info;
use oid4vc_core::Subject;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vci::credential_format_profiles::w3c_verifiable_credentials::jwt_vc_json::JwtVcJson;
use oid4vci::credential_format_profiles::{Credential, CredentialFormats, WithCredential};
use serde_json::json;
use std::fs::File;
use std::io::copy;
use std::sync::Arc;

lazy_static! {
    pub static ref PERSONAL_INFORMATION: VerifiableCredentialRecord = VerifiableCredentialRecord::from(
        CredentialFormats::<WithCredential>::JwtVcJson(Credential {
            format: JwtVcJson,
            credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.Yl841U5BwWgctX5vF5Zi8SYCEQpxFqEs8_J8KrX9D_mOwL-IRmP64BeQZvnKeAdcOoYGn6CyciV51_amdPNQBw"),
        })
    );
    pub static ref DRIVERS_LICENSE_CREDENTIAL: VerifiableCredentialRecord = VerifiableCredentialRecord::from(
        CredentialFormats::<WithCredential>::JwtVcJson(Credential {
            format: JwtVcJson,
            credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIkRyaXZlckxpY2Vuc2VDcmVkZW50aWFsIl0sImlzc3VlciI6Imh0dHA6Ly8xOTIuMTY4LjEuMTI3OjkwOTAvIiwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wOC0xNVQwOTozMDowMFoiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjctMDgtMTVUMjM6NTk6NTlaIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6a2V5Ono2TWtnMVhYR1VxZmtoQUtVMWtWZDFQbXc2VUVqMXZ4aUxqMXhjOTFNQno1b3dOWSIsImxpY2Vuc2VDbGFzcyI6IkNsYXNzIEMiLCJpc3N1ZWRCeSI6IkNhbGlmb3JuaWEiLCJ2YWxpZGl0eSI6IlZhbGlkIn19fQ.OZCcZt5JTJcBhoLPIyrQuvZuc2dnVN65f8GvKQ3earAzJEgGMA9ZjKRNHEjI73wLwvG5MJBN7Zs_rWiNLEZ5Dg"),
        })
    );
    pub static ref OPEN_BADGE: VerifiableCredentialRecord = VerifiableCredentialRecord::from(
        CredentialFormats::<WithCredential>::JwtVcJson(Credential {
            format: JwtVcJson,
            credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiLCJodHRwczovL3B1cmwuaW1zZ2xvYmFsLm9yZy9zcGVjL29iL3YzcDAvY29udGV4dC0zLjAuMi5qc29uIl0sImlkIjoiaHR0cDovL2V4YW1wbGUuZWR1L2NyZWRlbnRpYWxzLzM3MzIiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiT3BlbkJhZGdlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOnsiaWQiOiJodHRwczovL2V4YW1wbGUuZWR1L2lzc3VlcnMvNTY1MDQ5IiwidHlwZSI6WyJJc3N1ZXJQcm9maWxlIl0sIm5hbWUiOiJFeGFtcGxlIFVuaXZlcnNpdHkifSwiaXNzdWFuY2VEYXRlIjoiMjAxMC0wMS0wMVQwMDowMDowMFoiLCJuYW1lIjoiVGVhbXdvcmsgQmFkZ2UiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpleGFtcGxlOmViZmViMWY3MTJlYmM2ZjFjMjc2ZTEyZWMyMSIsInR5cGUiOlsiQWNoaWV2ZW1lbnRTdWJqZWN0Il0sImFjaGlldmVtZW50Ijp7ImlkIjoiaHR0cHM6Ly9leGFtcGxlLmNvbS9hY2hpZXZlbWVudHMvMjFzdC1jZW50dXJ5LXNraWxscy90ZWFtd29yayIsInR5cGUiOlsiQWNoaWV2ZW1lbnQiXSwiY3JpdGVyaWEiOnsibmFycmF0aXZlIjoiVGVhbSBtZW1iZXJzIGFyZSBub21pbmF0ZWQgZm9yIHRoaXMgYmFkZ2UgYnkgdGhlaXIgcGVlcnMgYW5kIHJlY29nbml6ZWQgdXBvbiByZXZpZXcgYnkgRXhhbXBsZSBDb3JwIG1hbmFnZW1lbnQuIn0sImRlc2NyaXB0aW9uIjoiVGhpcyBiYWRnZSByZWNvZ25pemVzIHRoZSBkZXZlbG9wbWVudCBvZiB0aGUgY2FwYWNpdHkgdG8gY29sbGFib3JhdGUgd2l0aGluIGEgZ3JvdXAgZW52aXJvbm1lbnQuIiwibmFtZSI6IlRlYW13b3JrIn19fX0.OZCcZt5JTJcBhoLPIyrQuvZuc2dnVN65f8GvKQ3earAzJEgGMA9ZjKRNHEjI73wLwvG5MJBN7Zs_rWiNLEZ5Dg"),
        })
    );
}

pub async fn set_dev_mode(state: &AppState, action: Action) -> Result<(), AppError> {
    let enabled = match action {
        Action::SetDevMode { enabled } => enabled,
        _ => return Err(InvalidActionError { action }),
    };

    *state.dev_mode_enabled.lock().unwrap() = enabled;
    *state.current_user_prompt.lock().unwrap() = None;
    Ok(())
}

pub async fn load_dev_profile(state: &mut AppState, _action: Action) -> Result<(), AppError> {
    info!("load dev profile");

    let stronghold_manager = StrongholdManager::create("sup3rSecr3t").map_err(StrongholdCreationError)?;

    let subject = KeySubject::from_keypair(
        generate::<Ed25519KeyPair>(Some("this-is-a-very-UNSAFE-secret-key".as_bytes())),
        None,
    );

    let profile = Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: Some("system".to_string()),
        primary_did: subject.identifier().unwrap(),
    };
    state.active_profile.replace(profile);

    vec![
        PERSONAL_INFORMATION.clone(),
        DRIVERS_LICENSE_CREDENTIAL.clone(),
        OPEN_BADGE.clone(),
    ]
    .into_iter()
    .for_each(|verifiable_credential_record| {
        info!("inserting credential into stronghold");
        stronghold_manager
            .insert(
                verifiable_credential_record.display_credential.id.parse().unwrap(),
                json!(verifiable_credential_record).to_string().as_bytes().to_vec(),
            )
            .unwrap();
    });

    info!("loading credentials from stronghold");
    state.credentials = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .into_iter()
        .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
        .collect();

    load_credential_images(
        "university".to_string(),
        DRIVERS_LICENSE_CREDENTIAL.clone().display_credential.id,
        PERSONAL_INFORMATION.clone().display_credential.id,
        OPEN_BADGE.clone().display_credential.id,
    )
    .await?;

    state
        .managers
        .lock()
        .await
        .stronghold_manager
        .replace(Arc::new(stronghold_manager));

    info!("loading journey from string");
    let journey_definition = r#"
        {
            "title": "NGDIL Demo",
            "description": "Set up your profile and get started with your UniMe app.",
            "description_short": "Complete your first steps",
            "creator": "UniMe",
            "goals": [
                {
                    "id": 0,
                    "label": "Set up your profile",
                    "description": "Make your UniMe app your own by choosing a profile name and profile picture.",
                    "icon": "UserCirclePlus",
                    "faqs": [
                        { "id": 0, "title": "Will this information be shared?", "content": "No. Your profile information will never leave your device." }
                    ],
                    "prerequisites": []
                },
                {
                    "id": 1,
                    "label": "Receive your first credential",
                    "type": "hold-credential",
                    "description": "Receive your first credential from a trusted source.",
                    "icon": "FileArrowDown",
                    "faqs": [
                        { "id": 0, "title": "What is a credential?", "content": "A credential is like a digital proof that verifies something about you, such as your age, education, or memberships." }
                    ],
                    "prerequisites": []
                },
                {
                    "id": 2,
                    "label": "Use a credential to sign in to a website",
                    "type": "login",
                    "icon": "Key",
                    "faqs": [],
                    "prerequisites": []
                }
            ]
        }"#;
    // let journey_definition = std::fs::read_to_string("resources/ngdil.json")?;
    let onboarding_journey: serde_json::Value = serde_json::from_str(journey_definition).unwrap();

    if std::env::var("FEATURE_USER_JOURNEYS_ENABLED")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false)
    {
        state.user_journey = Some(onboarding_journey);
    }

    state.connections = vec![Connection {
        client_name: "NGDIL Demo".to_string(),
        url: "api.ngdil-demo.tanglelabs.io".to_string(),
        logo_uri: Some("https://demo.ngdil.com/imgs/kw1c-white.png".to_string()),
        verified: false,
        first_interacted: "2023-09-11T19:53:53.937981+00:00".to_string(),
        last_interacted: "2023-09-11T19:53:53.937981+00:00".to_string(),
    }];

    state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    state.dev_mode_enabled = true;
    Ok(())
}

async fn load_credential_images(
    issuer_id: String,
    driver_license_id: String,
    personal_information_id: String,
    badge_id: String,
) -> Result<(), AppError> {
    // Issuers
    let mut image_bytes: &[u8] = include_bytes!("../../../resources/images/issuer-university.png");
    let file_name = format!("{}.png", issuer_id);
    let mut file = File::create(ASSETS_DIR.lock().unwrap().as_path().to_owned().join(file_name))?;
    copy(&mut image_bytes, &mut file)?;

    // Credentials
    let mut image_bytes: &[u8] = include_bytes!("../../../resources/images/cuddlyferris.svg");
    let file_name = format!("{}.svg", personal_information_id);
    let mut file = File::create(ASSETS_DIR.lock().unwrap().as_path().to_owned().join(file_name))?;
    copy(&mut image_bytes, &mut file)?;

    let mut image_bytes: &[u8] = include_bytes!("../../../resources/images/credential-driver-license.png");
    let file_name = format!("{}.png", driver_license_id);
    let mut file = File::create(ASSETS_DIR.lock().unwrap().as_path().to_owned().join(file_name))?;
    copy(&mut image_bytes, &mut file)?;

    // Badges
    let mut image_bytes: &[u8] = include_bytes!("../../../resources/images/badge-university-green.png");
    let file_name = format!("{}.png", badge_id);
    let mut file = File::create(ASSETS_DIR.lock().unwrap().as_path().to_owned().join(file_name))?;
    copy(&mut image_bytes, &mut file)?;
    Ok(())
}
