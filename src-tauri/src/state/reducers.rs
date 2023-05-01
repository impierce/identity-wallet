use crate::did::did_key::{generate_dev_did, generate_new_did};
use crate::state::{actions::Action, AppState, ClaimType, Profile};
use did_key::{generate, Ed25519KeyPair, KeyMaterial};
use lazy_static::lazy_static;
use serde_json::Value;
use siopv2::{key_method::KeySubject, Provider, RequestUrl, StandardClaims};
use tracing::info;

lazy_static! {
    pub static ref PRIVATE_KEY_BYTES: Vec<u8> = generate::<Ed25519KeyPair>(None).private_key_bytes();
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

    let did_document = generate_new_did().await?;
    let profile = Profile {
        display_name: display_name.to_string(),
        primary_did: did_document.id,
    };
    *state.active_profile.lock().unwrap() = Some(profile);
    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.active_profile.lock().unwrap() = None;
    *state.active_requested_claims.lock().unwrap() = None;
    *state.active_authentication_request.lock().unwrap() = None;
    *state.locale.lock().unwrap() = "en".to_string();
    Ok(())
}

pub async fn load_dev_profile(state: &AppState, _action: Action) -> anyhow::Result<()> {
    let did_document = generate_dev_did().await?;
    let profile = Profile {
        display_name: "Ferris".to_string(),
        primary_did: did_document.id,
    };
    *state.active_profile.lock().unwrap() = Some(profile);

    Ok(())
}

/// Reads a request from the given URL, validates it and sets the requested user claims.
pub async fn get_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let request_url = payload["request_url"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read request_url from json payload"))?;

    // Use private key to create a mock provider.
    let mock_subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(PRIVATE_KEY_BYTES.as_slice())));

    // Create a new provider and validate the request.
    let provider = Provider::new(mock_subject).await?;
    let request = provider.validate_request(request_url.parse()?).await?;

    // Collect a map of the requested claims.
    let claims = match serde_json::to_value(request.id_token_request_claims())? {
        Value::Object(obj) => Some(
            obj.into_iter()
                .map(|(k, v)| {
                    let claim_type = match v.get("essential") {
                        Some(Value::Bool(true)) => ClaimType::Required,
                        _ => ClaimType::Optional,
                    };
                    (k, claim_type)
                })
                .collect(),
        ),
        _ => None,
    };

    // Update the state with the requested claims and the authentication request.
    *state.active_requested_claims.lock().unwrap() = claims;
    *state.active_authentication_request.lock().unwrap() = Some(request);

    Ok(())
}

pub async fn send_response(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let user_claims: StandardClaims = serde_json::from_value(payload["user_claims"].clone())?;

    dbg!(&user_claims);

    let request = state
        .active_authentication_request
        .lock()
        .map_err(|_| anyhow::anyhow!("failed to obtain lock on active_authentication_request"))?
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no active authentication request found"))?
        .clone();

    dbg!(&request);

    // Use private key to create a mock provider.
    let mock_subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(PRIVATE_KEY_BYTES.as_slice())));

    let provider = Provider::new(mock_subject).await?;

    let response = provider.generate_response(request, user_claims).await?;

    info!("response generated: {:?}", response);

    provider.send_response(response).await?;

    // Reset the state parameters.
    *state.active_requested_claims.lock().unwrap() = None;
    *state.active_authentication_request.lock().unwrap() = None;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{state::actions::ActionType, UNSAFE_STORAGE};
    use serde_json::json;
    use siopv2::{
        claims::{Claim, ClaimRequests},
        request::ResponseType,
        scope::ScopeValue,
        Registration, RequestUrl, Scope,
    };
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
        *UNSAFE_STORAGE.lock().unwrap() = path.as_os_str().into();

        let state = AppState::default();

        assert!(create_did_key(
            &state,
            Action {
                r#type: ActionType::CreateNew,
                payload: Some(json!({"display_name": "Ferris"})),
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
            ..Default::default()
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

    #[tokio::test]
    async fn test_get_request_and_send_response() {
        let state = AppState::default();

        let request_url = RequestUrl::builder()
            .response_type(ResponseType::IdToken)
            .client_id("did:key:1".to_owned())
            .scope(Scope::from(vec![ScopeValue::OpenId]))
            .redirect_uri(format!("https://client.example.org/cb"))
            .response_mode("post".to_owned())
            .registration(Registration::default().with_subject_syntax_types_supported(vec!["did:key".to_owned()]))
            // Add a claim request to the request with the `name` claim.
            .claims(ClaimRequests {
                id_token: Some(StandardClaims {
                    name: Some(Claim::default()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .exp(1234567890)
            .nonce("n-0S6_WzA2Mj".to_owned())
            .build()
            .unwrap();

        assert!(get_request(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: Some(json!({ "request_url": request_url.to_string() })),
            },
        )
        .await
        .is_ok(),);

        assert!(send_response(
            &state,
            Action {
                r#type: ActionType::SendResponse,
                payload: Some(json!({ "user_claims": {
                    "name": "Ferris"
                } })),
            },
        )
        .await
        .is_ok());
    }
}
