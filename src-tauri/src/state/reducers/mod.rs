pub mod authorization;
pub mod credential_offer;

use crate::crypto::stronghold::{create_new_stronghold, get_public_key, insert_into_stronghold};
use crate::did::did_key::{generate_dev_did, generate_new_did};
use crate::get_jwt_claims;
use crate::state::actions::Action;
use crate::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use crate::state::{AppState, Profile};
use identity_core::common::{Timestamp, Url};
use identity_credential::credential::{CredentialBuilder, Issuer, Subject};
use lazy_static::lazy_static;
use log::info;
use oid4vci::credential_format_profiles::w3c_verifiable_credentials::jwt_vc_json::JwtVcJson;
use oid4vci::credential_format_profiles::{Credential, CredentialFormats, WithCredential};
use serde_json::{json, Value};
use uuid::Uuid;

// TODO: this is a temporary solution to store the credentials in memory.
lazy_static! {
    pub static ref VERIFIABLE_CREDENTIAL: CredentialFormats<WithCredential> =
        CredentialFormats::<WithCredential>::JwtVcJson(Credential {
            format: JwtVcJson,
            credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3RnMkJkVmZRaDNQaEZLdmM0REduRkVndlJWQWpIeWlvelZSRUNNbXNiUURuI3o2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.qVbroJgBGplBHLl1lIr5r-FkPdGg-AEMSLw2566IYe6FsB6B51mMOO_e5dMNDfmtuYgoFP3IfV9WCbFufg3lBw"),
        });
}

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub fn set_locale(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let locale = payload["locale"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read locale from json payload"))?;
    *state.locale.lock().unwrap() = locale.to_string();
    info!("locale set to: `{}`", locale);
    Ok(())
}

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub async fn create_did_key(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let display_name = payload["display_name"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read display_name from json payload"))?;
    let password = payload["password"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read password from json payload"))?;

    let public_key = get_public_key(password)?;
    let did_document = generate_new_did(public_key).await?;
    let profile = Profile {
        display_name: display_name.to_string(),
        primary_did: did_document.id,
    };
    *state.active_profile.lock().unwrap() = Some(profile);
    Ok(())
}

pub async fn initialize_stronghold(_state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let password = payload["password"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read password from json payload"))?;
    create_new_stronghold(password)?;
    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.active_profile.lock().unwrap() = None;
    *state.locale.lock().unwrap() = "en".to_string();
    *state.credentials.lock().unwrap() = None;
    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Redirect(Redirect {
        r#type: CurrentUserPromptType::Redirect,
        target: "welcome".to_string(),
    }));
    Ok(())
}

pub async fn load_dev_profile(state: &AppState, _action: Action) -> anyhow::Result<()> {
    let did_document = generate_dev_did().await?;
    let profile = Profile {
        display_name: "Ferris Crabman".to_string(),
        primary_did: did_document.id,
    };
    *state.active_profile.lock().unwrap() = Some(profile);

    // =====================
    // Construct a `Subject` from json
    let json_subject: Value = json!({
        "id": "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY",
        "givenName": "Ferris",
        "familyName": "Crabman",
        "email": "ferris.crabman@crabmail.com",
        "birthdate": "1985-05-21"
    });
    let subject: Subject = serde_json::from_value(json_subject).unwrap();

    // Construct an `Issuer` from json
    let json_issuer: Value = json!({
      "id": "did:example:76e12ec712ebc6f1c221ebfeb1f",
      "name": "CrabMail"
    });

    let issuer: Issuer = serde_json::from_value(json_issuer).unwrap();

    let _credential_personal_information: identity_credential::credential::Credential = CredentialBuilder::default()
        .context(Url::parse("https://www.w3.org/2018/credentials/examples/v1").unwrap())
        .id(Url::parse("http://example.org/credentials/1012").unwrap())
        .type_("PersonalInformation")
        .subject(subject)
        .issuer(issuer)
        .issuance_date(Timestamp::parse("2022-01-01T00:00:00Z").unwrap())
        .build()
        .unwrap();
    // =====================

    let _credential_university_degree: identity_credential::credential::Credential = CredentialBuilder::default()
        .id(Url::parse("https://example.edu/credentials/3732")?)
        .issuer(Url::parse("did:key:a1b2c3d4e5f6")?)
        .type_("UniversityDegreeCredential")
        .subject(serde_json::from_value(json!({
          "id": "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY",
          "name": "Ferris Crabman",
          "degree": {
            "type": "BachelorDegree",
            "name": "Bachelor of Science and Arts",
          },
          "GPA": "4.0",
        }))?)
        .build()?;

    let credential = VERIFIABLE_CREDENTIAL.clone();

    let key = Uuid::default();

    insert_into_stronghold(key, json!(credential).to_string().as_bytes().to_vec(), "my-password")?;

    let credential_display = match credential {
        CredentialFormats::JwtVcJson(credential) => {
            serde_json::from_value::<identity_credential::credential::Credential>(
                get_jwt_claims(&credential.credential)["vc"].clone(),
            )
            .unwrap()
        }
        _ => unimplemented!(),
    };

    *state.credentials.lock().unwrap() = Some(vec![(key.to_string(), credential_display)]);
    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Redirect(Redirect {
        r#type: CurrentUserPromptType::Redirect,
        target: "profile".to_string(),
    }));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{crypto::stronghold::hash_password, state::actions::ActionType, STRONGHOLD};
    use iota_stronghold::{
        procedures::{GenerateKey, KeyType, StrongholdProcedure},
        Client, KeyProvider, Location, SnapshotPath, Stronghold,
    };
    use serde_json::json;
    use tempfile::NamedTempFile;

    #[test]
    fn test_set_locale() {
        let state = AppState::default();

        assert!(set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: Some(json!({"locale": "nl"})),
            },
        )
        .is_ok());
        assert_eq!(*state.locale.lock().unwrap(), "nl".to_string());
    }

    #[test]
    fn test_set_locale_with_invalid_payload() {
        let state = AppState::default();

        // Assert that a `SetLocale` action without a payload returns an error.
        assert!(set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: None,
            },
        )
        .is_err());

        // Assert that a `SetLocale` action with an invalid payload returns an error.
        assert!(set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: Some(json!({"foo": "bar"})),
            },
        )
        .is_err());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_create_new_with_method_did_key() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();

        // create new temp stronghold for testing
        let stronghold = Stronghold::default();
        let path = STRONGHOLD.lock().unwrap().clone().to_str().unwrap().to_owned();
        let client: Client = stronghold.create_client(path.clone()).expect("cannot create client");
        let output_location = Location::counter(path.clone(), 0u8);
        client
            .execute_procedure(StrongholdProcedure::GenerateKey(GenerateKey {
                ty: KeyType::Ed25519,
                output: output_location.clone(),
            }))
            .ok();
        stronghold
            .write_client(path.clone())
            .expect("store client state into snapshot state failed");
        stronghold
            .commit_with_keyprovider(
                &SnapshotPath::from_path(format!("{path}.snapshot")),
                &KeyProvider::try_from(hash_password("s3cr3t").unwrap()).unwrap(),
            )
            .ok();

        let state = AppState::default();

        assert!(create_did_key(
            &state,
            Action {
                r#type: ActionType::CreateNew,
                payload: Some(json!({"display_name": "Ferris", "password": "s3cr3t"})),
            },
        )
        .await
        .is_ok());

        let profile = state.active_profile.lock().unwrap();
        assert_eq!(profile.as_ref().unwrap().display_name, "Ferris");
        assert!(profile.as_ref().unwrap().primary_did.starts_with("did:key:"));
    }

    #[test]
    fn test_reset_state() {
        let state = AppState {
            active_profile: Some(Profile {
                display_name: "Ferris".to_string(),
                primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            })
            .into(),
            ..AppState::default()
        };

        assert!(reset_state(
            &state,
            Action {
                r#type: ActionType::Reset,
                payload: None,
            },
        )
        .is_ok());
        assert_eq!(*state.active_profile.lock().unwrap(), None);
        assert_eq!(*state.locale.lock().unwrap(), "en".to_string());
    }
}
