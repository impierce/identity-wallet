use std::collections::HashMap;

use crate::state::{actions::Action, AppState, ClaimType, Profile};
use did_key::{generate, Ed25519KeyPair, KeyMaterial};
use lazy_static::lazy_static;
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
pub fn create_did_key(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let display_name = payload["display_name"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read display_name from json payload"))?;
    let mock_profile = Profile {
        display_name: display_name.to_string(),
        primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
    };
    *state.active_profile.lock().unwrap() = Some(mock_profile);
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

// TODO: actually validate the incoming payload.
pub async fn get_request(state: &AppState, _action: Action) -> anyhow::Result<()> {
    // For now, initializes a static set of claims for mocking purposes.
    let mut claims = HashMap::new();
    claims.insert("name".to_string(), ClaimType::Optional);
    claims.insert("email".to_string(), ClaimType::Optional);

    // Update the state with the requested claims.
    *state.active_requested_claims.lock().unwrap() = Some(claims);

    Ok(())
}

// TODO: actually send the generated response to the RP.
pub async fn send_response(state: &AppState, _action: Action) -> anyhow::Result<()> {
    // Reset the state parameters.
    *state.active_requested_claims.lock().unwrap() = None;
    *state.active_authentication_request.lock().unwrap() = None;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::actions::ActionType;
    use serde_json::json;

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

    #[test]
    fn test_create_did_key() {
        let state = AppState::default();

        assert!(create_did_key(
            &state,
            Action {
                r#type: ActionType::CreateNew,
                payload: Some(json!({"display_name": "Ferris"})),
            },
        )
        .is_ok());
        assert_eq!(
            *state.active_profile.lock().unwrap(),
            Some(Profile {
                display_name: "Ferris".to_string(),
                primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            })
        );
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
}
