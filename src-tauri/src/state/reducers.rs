use std::sync::Arc;

use crate::crypto::stronghold::{
    create_new_stronghold, get_all_from_stronghold, get_public_key, hash_password, insert_into_stronghold,
};
use crate::did::did_key::{generate_dev_did, generate_new_did};
use crate::state::actions::Action;
use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Offer, Redirect, Selection};
use crate::state::{AppState, Profile};
use did_key::{generate, Ed25519KeyPair};
use identity_core::common::{Object, Timestamp, Url};
use identity_core::convert::FromJson;
use identity_credential::credential::{Credential, CredentialBuilder, Issuer, Jwt};
use identity_credential::presentation::JwtPresentation;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::info;
use oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vci::credential_format_profiles::CredentialFormats;
use oid4vci::credential_offer::{self, CredentialOffer, CredentialOfferQuery, CredentialsObject, Grants};
use oid4vci::token_request::{PreAuthorizedCode, TokenRequest};
use oid4vci::Wallet;
use serde_json::{json, Value};

use lazy_static::lazy_static;
use siopv2::RequestUrl;

// TODO: this is a temporary solution to store the credentials in memory.
// lazy_static! {
//     static ref VERIFIABLE_CREDENTIALS: Vec<String> =
//         vec!["eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3RnMkJkVmZRaDNQaEZLdmM0REduRkVndlJWQWpIeWlvelZSRUNNbXNiUURuI3o2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWt0ZzJCZFZmUWgzUGhGS3ZjNERHbkZFZ3ZSVkFqSHlpb3pWUkVDTW1zYlFEbiIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.qVbroJgBGplBHLl1lIr5r-FkPdGg-AEMSLw2566IYe6FsB6B51mMOO_e5dMNDfmtuYgoFP3IfV9WCbFufg3lBw".to_string()];
// }

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
    create_new_stronghold(password).await?;
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
    let subject: identity_credential::credential::Subject = serde_json::from_value(json_subject).unwrap();

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

    // let credential_university_degree: Credential = CredentialBuilder::default()
    //     .id(Url::parse("https://example.edu/credentials/3732")?)
    //     .issuer(Url::parse("did:key:a1b2c3d4e5f6")?)
    //     .type_("UniversityDegreeCredential")
    //     .subject(serde_json::from_value(json!({
    //       "id": "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY",
    //       "name": "Ferris Crabman",
    //       "degree": {
    //         "type": "BachelorDegree",
    //         "name": "Bachelor of Science and Arts",
    //       },
    //       "GPA": "4.0",
    //     }))?)
    //     .build()?;

    // // read VC from in-memory storage
    // let key = DecodingKey::from_secret(&[]);
    // let mut validation = Validation::new(Algorithm::EdDSA);
    // validation.insecure_disable_signature_validation();
    // let credential_0_as_json = decode::<serde_json::Value>(&VERIFIABLE_CREDENTIALS.first().unwrap(), &key, &validation)
    //     .unwrap()
    //     .claims;
    // let credential_0 = serde_json::from_value::<Credential>(credential_0_as_json.get("vc").unwrap().clone()).unwrap();

    // *state.credentials.lock().unwrap() = Some(vec![credential_0]);
    *state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
        r#type: CurrentUserFlowType::Redirect,
        target: "profile".to_string(),
    }));
    Ok(())
}

// Reads the request url from the payload and validates it.
pub async fn read_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_request");
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;

    let guard = crate::PROVIDER_MANAGER.lock().await;
    let provider = guard.as_ref().unwrap();

    info!("trying to validate request: {:?}", payload);

    let authorization_request = provider
        .validate_request(serde_json::from_value(payload).unwrap())
        .await
        .unwrap();
    info!("validated authorization request: {:?}", authorization_request);

    let verifiable_credentials = get_all_from_stronghold("my-password").await?.unwrap();

    // Get the indices of the verifiable credentials that can be used to fulfill the request.
    let indices: Vec<usize> = verifiable_credentials
        .iter()
        .enumerate()
        .filter_map(|(index, verifiable_credential)| {
            // Decode the verifiable credential from the JWT without validating.
            let key = DecodingKey::from_secret(&[]);
            let mut validation = Validation::new(Algorithm::EdDSA);
            validation.insecure_disable_signature_validation();
            let verifiable_credential =
                decode::<serde_json::Value>(std::str::from_utf8(verifiable_credential).unwrap(), &key, &validation)
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

    info!("indices of VCs that can fulfill the request: {:?}", indices);

    // TODO: Make sure that the frontend receives the indices of the credentials that are conforming to the presentation
    // definition. Can The UserFlow be used for this? How?

    *state.active_authorization_request.lock().unwrap() = Some(authorization_request);

    if let Some(index) = indices.first() {
        *state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Selection(Selection {
            r#type: CurrentUserFlowType::SelectCredentials,
            options: vec![index.to_string()],
        }));
    }

    Ok(())
}
pub async fn read_credential_offer(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_credential_offer");
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let credential_offer: CredentialOffer<CredentialFormats> = match serde_json::from_value(payload).unwrap() {
        CredentialOfferQuery::CredentialOffer(credential_offer) => credential_offer,
        _ => unreachable!(),
    };
    info!("credential offer: {:?}", credential_offer);
    *state.credential_offers.lock().unwrap() = Some(vec![credential_offer.clone()]);
    *state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Offer(Offer {
        r#type: CurrentUserFlowType::Offer,
        options: vec![serde_json::to_value(&credential_offer).unwrap()],
    }));

    Ok(())
}

pub async fn send_credential_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("send_credential_request");

    let credential_offer = state
        .credential_offers
        .lock()
        .unwrap()
        .clone()
        .unwrap()
        .first()
        .unwrap()
        .clone();

    // The credential offer contains a credential format for a university degree.
    let university_degree_credential_format = match credential_offer.credentials.get(0).unwrap().clone() {
        CredentialsObject::ByValue(credential_format) => credential_format,
        _ => unreachable!(),
    };

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.credential_issuer;

    // Create a new subject.
    let subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(
        "this-is-a-very-UNSAFE-secret-key".as_bytes(),
    )));

    // Create a new wallet.
    let wallet: Wallet<CredentialFormats> = Wallet::new(Arc::new(subject));

    // Get the authorization server metadata.
    let authorization_server_metadata = wallet
        .get_authorization_server_metadata(credential_issuer_url.clone())
        .await
        .unwrap();

    // Get the credential issuer metadata.
    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await
        .unwrap();

    // Create a token request with grant_type `pre_authorized_code`.
    let token_request = match credential_offer.grants {
        Some(Grants {
            pre_authorized_code, ..
        }) => TokenRequest::PreAuthorizedCode {
            grant_type: PreAuthorizedCode,
            pre_authorized_code: pre_authorized_code.unwrap().pre_authorized_code,
            user_pin: None,
        },
        None => unreachable!(),
    };
    info!("token_request: {:?}", token_request);

    // Get an access token.
    let token_response = wallet
        .get_access_token(authorization_server_metadata.token_endpoint, token_request)
        .await
        .unwrap();

    // Get the credential.
    let credential_response = wallet
        .get_credential(
            credential_issuer_metadata,
            &token_response,
            university_degree_credential_format,
        )
        .await
        .unwrap();

    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    let credential_0_as_json = decode::<serde_json::Value>(
        credential_response.credential.clone().unwrap().as_str().unwrap(),
        &key,
        &validation,
    )
    .unwrap()
    .claims;
    let credential_0 = serde_json::from_value::<Credential>(credential_0_as_json.get("vc").unwrap().clone()).unwrap();

    *state.credentials.lock().unwrap() = Some(vec![credential_0]);

    let buffer = serde_json::to_vec(&credential_response.credential.unwrap())?;

    insert_into_stronghold(b"key".to_vec(), buffer, "my-password").await?;

    *state.current_user_flow.lock().unwrap() = None;
    *state.credential_offers.lock().unwrap() = None;

    Ok(())
}

// Sends the authorization response including the verifiable credentials.
pub async fn send_response(state: &AppState, action: Action) -> anyhow::Result<()> {
    // let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let payload = match action.payload {
        Some(payload) => payload,
        None => {
            info!("||DEBUG|| unable to read payload");
            *state.debug_messages.lock().unwrap() = vec!["unable to read payload".into()];
            return Err(anyhow::anyhow!("unable to read payload"));
        }
    };
    let credential_index: usize = serde_json::from_value(payload["credential_index"].clone())?;

    // let authorization_request = state
    //     .active_authorization_request
    //     .lock()
    //     .map_err(|_| anyhow::anyhow!("failed to obtain lock on active_authentication_request"))?
    //     .as_ref()
    //     .ok_or_else(|| anyhow::anyhow!("no active authentication request found"))?
    //     .clone();
    let authorization_request = match state
        .active_authorization_request
        .lock()
        .map_err(|_| anyhow::anyhow!("failed to obtain lock on active_authentication_request"))?
        .as_ref()
    {
        Some(authorization_request) => authorization_request.clone(),
        None => {
            info!("||DEBUG|| no active Authentication Request found");
            *state.debug_messages.lock().unwrap() = vec!["no active Authentication Request found".into()];
            return Err(anyhow::anyhow!("no active authentication request found"));
        }
    };

    info!("||DEBUG|| credential not found");
    *state.debug_messages.lock().unwrap() = vec!["credential not found".into()];

    let verifiable_credentials = get_all_from_stronghold("my-password").await?.unwrap();

    // let verifiable_credential = VERIFIABLE_CREDENTIALS
    //     .get(credential_index)
    //     .ok_or(anyhow::anyhow!("credential not found"))?;
    let verifiable_credential = match verifiable_credentials.get(credential_index) {
        Some(verifiable_credential) => std::str::from_utf8(verifiable_credential).unwrap(),
        None => {
            info!("||DEBUG|| credential not found");
            *state.debug_messages.lock().unwrap() = vec!["credential not found".into()];
            return Err(anyhow::anyhow!("credential not found"));
        }
    };

    info!("||DEBUG|| decoding the verifiable credential");
    *state.debug_messages.lock().unwrap() = vec!["decoding the verifiable credential".into()];
    // Decode the verifiable credential from the JWT without validating.
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    let verifiable_credential = decode::<serde_json::Value>(&verifiable_credential, &key, &validation)
        .unwrap()
        .claims;

    // Create presentation submission using the presentation definition and the verifiable credential.
    // let presentation_submission = create_presentation_submission(
    //     authorization_request
    //         .presentation_definition()
    //         .as_ref()
    //         .expect("presentation definition not found"),
    //     &verifiable_credential,
    // )?;
    let presentation_submission = create_presentation_submission(
        match authorization_request.presentation_definition().as_ref() {
            Some(presentation_definition) => presentation_definition,
            None => {
                info!("||DEBUG|| presentation definition not found");
                *state.debug_messages.lock().unwrap() = vec!["presentation definition not found".into()];
                return Err(anyhow::anyhow!("presentation definition not found"));
            }
        },
        &verifiable_credential,
    )?;

    info!("||DEBUG|| get the subject did");
    *state.debug_messages.lock().unwrap() = vec!["get the subject did".into()];
    let subject_did = state
        .active_profile
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .primary_did
        .clone();

    info!("||DEBUG|| credential not found");
    *state.debug_messages.lock().unwrap() = vec!["credential not found".into()];
    // Create a verifiable presentation using the JWT.
    let verifiable_presentation = JwtPresentation::builder(Url::parse(subject_did).unwrap(), Object::new())
        .credential(Jwt::from(
            std::str::from_utf8(
                verifiable_credentials
                    .get(credential_index)
                    .ok_or(anyhow::anyhow!("credential not found"))?,
            )
            .unwrap()
            .to_owned(),
        ))
        .build()
        .unwrap();

    info!("||DEBUG|| get the provider");
    *state.debug_messages.lock().unwrap() = vec!["get the provider".into()];
    let guard = crate::PROVIDER_MANAGER.lock().await;
    let provider = guard.as_ref().unwrap();

    info!("||DEBUG|| generating response");
    *state.debug_messages.lock().unwrap() = vec!["generating response".into()];
    let response = provider.generate_response(
        authorization_request,
        Default::default(),
        Some(verifiable_presentation),
        Some(presentation_submission),
    )?;
    info!("||DEBUG|| response generated: {:?}", response);

    provider.send_response(response).await?;
    info!("||DEBUG|| response successfully sent");

    *state.current_user_flow.lock().unwrap() = None;

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
    #[serial_test::serial]
    async fn test_create_new_with_method_did_key() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        dbg!(&path);
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
            credential_offers: None.into(),
            credentials: None.into(),
            current_user_flow: None.into(),
            debug_messages: vec![].into(),
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
