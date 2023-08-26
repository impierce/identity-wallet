use crate::crypto::stronghold::StrongholdManager;
use crate::did::did_key::generate_dev_did;
use crate::state::actions::Action;
use crate::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use crate::state::{AppState, Profile};
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use lazy_static::lazy_static;
use log::info;
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

    let did_document = generate_dev_did().await?;
    let profile = Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: Some("system".to_string()),
        primary_did: did_document.id,
    };
    *state.active_profile.lock().unwrap() = Some(profile);

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

    // TODO: read onboarding journey from json file (non-inline)
    // // add default user onboarding journey
    // let onboarding_journey: Value = json!(
    // {
    //     "title": "Onboarding",
    //     "description": "Set up your profile and get started with your UniMe app.",
    //     "description_short": "Complete your first steps",
    //     "creator": "UniMe",
    //     "goals": [
    //         {
    //             "id": 0,
    //             "label": "Set up your profile",
    //             "description": "",
    //             "faqs": [],
    //             "prerequisites": []
    //         },
    //         {
    //             "id": 1,
    //             "label": "Add a self-signed credential",
    //             "description": "",
    //             "faqs": [],
    //             "prerequisites": []
    //         },
    //         {
    //             "id": 2,
    //             "label": "Get your email address verified",
    //             "description": "We can send you a verification link to your provided email address. When you confirm the link in the email, we will issue a credential that proves you own that email address.",
    //             "faqs": [
    //                 { "id": 0, "title": "What can I do with my email?", "content": "Something." },
    //                 { "id": 1, "title": "Why should I add my email?", "content": "Because." },
    //                 { "id": 2, "title": "Is it verified?", "content": "Absolutely." }
    //             ],
    //             "prerequisites": [
    //                 { "type": "goal", "data": 0 },
    //                 { "type": "goal", "data": 1 }
    //             ]
    //         },
    //         { "id": 3, "label": "Log in in to a website", "faqs": [], "prerequisites": [] },
    //         { "id": 4, "label": "Check your history", "faqs": [], "prerequisites": [] }
    //     ]
    // }
    // );
    // *state.user_journey.lock().unwrap() = Some(onboarding_journey);

    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Redirect(Redirect {
        r#type: CurrentUserPromptType::Redirect,
        target: "me".to_string(),
    }));
    Ok(())
}
