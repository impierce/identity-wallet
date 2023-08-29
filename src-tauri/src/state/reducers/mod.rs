pub mod authorization;
pub mod credential_offer;
pub mod load_dev_profile;
pub mod storage;

use super::{IdentityManager, Locale};
use crate::crypto::stronghold::StrongholdManager;
// use crate::did::did_key::generate_new_did;
use crate::state::actions::Action;
use crate::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use crate::state::{AppState, Profile};
use did_key::{from_existing_key, Ed25519KeyPair};
use log::info;
use oid4vc_core::Subject;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use std::sync::Arc;

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub fn set_locale(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let locale = &payload["locale"];
    *state.locale.lock().unwrap() = serde_json::from_value::<Locale>(locale.clone())?;
    info!("locale set to: `{}`", locale);
    Ok(())
}

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub async fn create_identity(state: &AppState, action: Action) -> anyhow::Result<()> {
    let mut state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard.stronghold_manager.as_ref().unwrap();

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let display_name = payload["display_name"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read display_name from json payload"))?;

    let public_key = stronghold_manager.get_public_key()?;

    let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
    let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

    let provider_manager = ProviderManager::new([subject.clone()]).unwrap();
    let wallet: Wallet = Wallet::new(subject.clone());

    let profile = Profile {
        display_name: display_name.to_string(),
        primary_did: subject.identifier().unwrap(),
    };

    state.active_profile.lock().unwrap().replace(profile);
    state_guard.identity_manager.replace(IdentityManager {
        provider_manager,
        wallet,
    });
    Ok(())
}

pub async fn initialize_stronghold(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let password = payload["password"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read password from json payload"))?;

    state
        .managers
        .lock()
        .await
        .stronghold_manager
        .replace(Arc::new(StrongholdManager::create(password)?));

    info!("stronghold initialized");

    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.active_profile.lock().unwrap() = None;
    *state.locale.lock().unwrap() = Locale::default();
    state.credentials.lock().unwrap().clear();
    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Redirect(Redirect {
        r#type: CurrentUserPromptType::Redirect,
        target: "welcome".to_string(),
    }));
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
        assert_eq!(*state.locale.lock().unwrap(), Locale::Nl);
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
        assert_eq!(*state.locale.lock().unwrap(), Locale::default());
    }
}
