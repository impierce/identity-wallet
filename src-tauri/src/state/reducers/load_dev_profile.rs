use crate::crypto::stronghold::StrongholdManager;
use crate::state::actions::Action;
use crate::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use crate::state::{AppState, Profile};
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use did_key::{generate, Ed25519KeyPair};
use lazy_static::lazy_static;
use log::info;
use oid4vc_core::Subject;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vci::credential_format_profiles::w3c_verifiable_credentials::jwt_vc_json::JwtVcJson;
use oid4vci::credential_format_profiles::{Credential, CredentialFormats, WithCredential};
use serde_json::json;
use std::sync::Arc;

lazy_static! {
    pub static ref PERSONAL_INFORMATION: VerifiableCredentialRecord = VerifiableCredentialRecord::from(
        CredentialFormats::<WithCredential>::JwtVcJson(Credential {
            format: JwtVcJson,
            credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.Yl841U5BwWgctX5vF5Zi8SYCEQpxFqEs8_J8KrX9D_mOwL-IRmP64BeQZvnKeAdcOoYGn6CyciV51_amdPNQBw"),
        }));
    pub static ref DRIVERS_LICENSE_CREDENTIAL: VerifiableCredentialRecord = VerifiableCredentialRecord::from(
        CredentialFormats::<WithCredential>::JwtVcJson(Credential {
            format: JwtVcJson,
            credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIkRyaXZlckxpY2Vuc2VDcmVkZW50aWFsIl0sImlzc3VlciI6Imh0dHA6Ly8xOTIuMTY4LjEuMTI3OjkwOTAvIiwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wOC0xNVQwOTozMDowMFoiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjctMDgtMTVUMjM6NTk6NTlaIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6a2V5Ono2TWtnMVhYR1VxZmtoQUtVMWtWZDFQbXc2VUVqMXZ4aUxqMXhjOTFNQno1b3dOWSIsImxpY2Vuc2VDbGFzcyI6IkNsYXNzIEMiLCJpc3N1ZWRCeSI6IkNhbGlmb3JuaWEiLCJ2YWxpZGl0eSI6IlZhbGlkIn19fQ.OZCcZt5JTJcBhoLPIyrQuvZuc2dnVN65f8GvKQ3earAzJEgGMA9ZjKRNHEjI73wLwvG5MJBN7Zs_rWiNLEZ5Dg"),
        }));
}

pub async fn load_dev_profile(state: &AppState, _action: Action) -> anyhow::Result<()> {
    info!("load dev profile");

    let stronghold_manager = StrongholdManager::create("sup3rSecr3t")?;

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
    state.active_profile.lock().unwrap().replace(profile);
    state.credentials.lock().unwrap().clear();

    vec![PERSONAL_INFORMATION.clone(), DRIVERS_LICENSE_CREDENTIAL.clone()]
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
    stronghold_manager
        .values()?
        .unwrap()
        .into_iter()
        .for_each(|verifiable_credential_record| {
            state
                .credentials
                .lock()
                .unwrap()
                .push(verifiable_credential_record.display_credential);
        });

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
    let onboarding_journey: serde_json::Value = serde_json::from_str(&journey_definition).unwrap();
    *state.user_journey.lock().unwrap() = Some(onboarding_journey);

    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Redirect(Redirect {
        r#type: CurrentUserPromptType::Redirect,
        target: "me".to_string(),
    }));
    Ok(())
}
