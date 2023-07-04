use crate::crypto::stronghold::{create_new_stronghold, get_public_key, hash_password};
use crate::did::did_key::{generate_dev_did, generate_new_did};
use crate::state::actions::Action;
use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect};
use crate::state::{AppState, Profile};
use identity_core::common::{Object, Timestamp, Url};
use identity_credential::credential::{Credential, CredentialBuilder, Issuer, Jwt, Subject};
use identity_credential::presentation::JwtPresentation;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::info;
use oid4vc_manager::managers::presentation::create_presentation_submission;
use serde_json::{json, Value};

use lazy_static::lazy_static;

// TODO: this is a temporary solution to store the credentials in memory.
lazy_static! {
    static ref VERIFIABLE_CREDENTIALS: Vec<String> =
        vec!["eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3RnMkJkVmZRaDNQaEZLdmM0REduRkVndlJWQWpIeWlvelZSRUNNbXNiUURuI3o2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.qVbroJgBGplBHLl1lIr5r-FkPdGg-AEMSLw2566IYe6FsB6B51mMOO_e5dMNDfmtuYgoFP3IfV9WCbFufg3lBw".to_string()];
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
    *state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
        r#type: CurrentUserFlowType::Redirect,
        target: "profile".to_string(),
    }));
    Ok(())
}

// Reads the request url from the payload and validates it.
pub async fn read_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let request_url = payload["request_url"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read request_url from json payload"))?;

    let guard = crate::PROVIDER_MANAGER.lock().await;
    let provider = guard.as_ref().unwrap();

    let authorization_request = provider.validate_request(request_url.parse()?).await?;
    info!("validated authorization request: {:?}", authorization_request);

    // Get the indices of the verifiable credentials that can be used to fulfill the request.
    let indices: Vec<usize> = VERIFIABLE_CREDENTIALS
        .iter()
        .enumerate()
        .filter_map(|(index, verifiable_credential)| {
            // Decode the verifiable credential from the JWT without validating.
            let key = DecodingKey::from_secret(&[]);
            let mut validation = Validation::new(Algorithm::EdDSA);
            validation.insecure_disable_signature_validation();
            let verifiable_credential = decode::<serde_json::Value>(&verifiable_credential, &key, &validation)
                .unwrap()
                .claims;

            // Create presentation submission using the presentation definition and the verifiable credential.
            match create_presentation_submission(
                authorization_request
                    .presentation_definition()
                    .as_ref()
                    .expect("presentation definition not found"),
                &verifiable_credential,
            ) {
                // The verifiable credential can be used to fulfill the request.
                Ok(_presentation_submission) => Some(index),
                // The verifiable credential cannot be used to fulfill the request.
                Err(_err) => None,
            }
        })
        .collect();

    // TODO: Make sure that the frontend receives the indices of the credentials that are conforming to the presentation
    // definition. Can The UserFlow be used for this? How?

    Ok(())
}

// Sends the authorization response including the verifiable credentials.
pub async fn send_response(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let credential_index: usize = serde_json::from_value(payload["credential_index"].clone())?;

    let authorization_request = state
        .active_authorization_request
        .lock()
        .map_err(|_| anyhow::anyhow!("failed to obtain lock on active_authentication_request"))?
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no active authentication request found"))?
        .clone();

    let verifiable_credential = VERIFIABLE_CREDENTIALS
        .get(credential_index)
        .ok_or(anyhow::anyhow!("credential not found"))?;

    // Decode the verifiable credential from the JWT without validating.
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    let verifiable_credential = decode::<serde_json::Value>(&verifiable_credential, &key, &validation)
        .unwrap()
        .claims;

    // Create presentation submission using the presentation definition and the verifiable credential.
    let presentation_submission = create_presentation_submission(
        authorization_request
            .presentation_definition()
            .as_ref()
            .expect("presentation definition not found"),
        &verifiable_credential,
    )?;

    let subject_did = state
        .active_profile
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .primary_did
        .clone();

    // Create a verifiable presentation using the JWT.
    let verifiable_presentation = JwtPresentation::builder(Url::parse(subject_did).unwrap(), Object::new())
        .credential(Jwt::from(
            VERIFIABLE_CREDENTIALS
                .get(credential_index)
                .ok_or(anyhow::anyhow!("credential not found"))?
                .clone(),
        ))
        .build()
        .unwrap();

    let guard = crate::PROVIDER_MANAGER.lock().await;
    let provider = guard.as_ref().unwrap();

    let response = provider
        .generate_response(
            authorization_request,
            Default::default(),
            Some(verifiable_presentation),
            Some(presentation_submission),
        )
        .await?;
    info!("response generated: {:?}", response);

    provider.send_response(response).await?;
    info!("response successfully sent");

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
            active_authorization_request: None.into(),
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
