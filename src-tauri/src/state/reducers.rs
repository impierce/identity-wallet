use crate::crypto::stronghold::{create_new_stronghold, get_public_key, hash_password};
use crate::did::did_key::{generate_dev_did, generate_new_did};
use crate::state::actions::Action;
use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect};
use crate::state::{AppState, Profile};
use identity_core::common::{Timestamp, Url};
use identity_credential::credential::{Credential, CredentialBuilder, Issuer, Subject};
use log::info;
use serde_json::{json, Value};

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

    let public_key = get_public_key(password).await?;
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
    let password_hash = hash_password(password).await?;
    create_new_stronghold(password_hash).await?;
    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.active_profile.lock().unwrap() = None;
    *state.locale.lock().unwrap() = "en".to_string();
    *state.credentials.lock().unwrap() = None;
    *state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
        r#type: CurrentUserFlowType::Redirect,
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

    let credential_personal_information: Credential = CredentialBuilder::default()
        .context(Url::parse("https://www.w3.org/2018/credentials/examples/v1").unwrap())
        .id(Url::parse("http://example.org/credentials/1012").unwrap())
        .type_("PersonalInformation")
        .subject(subject)
        .issuer(issuer)
        .issuance_date(Timestamp::parse("2022-01-01T00:00:00Z").unwrap())
        .build()
        .unwrap();
    // =====================

    let credential_university_degree: Credential = CredentialBuilder::default()
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

    *state.credentials.lock().unwrap() = Some(vec![credential_personal_information, credential_university_degree]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{state::actions::ActionType, STRONGHOLD};
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
                &SnapshotPath::from_path(path),
                &KeyProvider::try_from(hash_password("s3cr3t").await.unwrap()).unwrap(),
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
            locale: "nl".to_string().into(),
            credentials: None.into(),
            current_user_flow: None.into(),
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
