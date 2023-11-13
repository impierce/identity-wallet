pub mod authorization;
pub mod credential_offer;
pub mod dev_mode;
pub mod storage;

use super::{IdentityManager, Locale};
use crate::crypto::stronghold::StrongholdManager;
use crate::state::actions::Action;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, Profile};
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use did_key::{from_existing_key, Ed25519KeyPair};
use log::info;
use oid4vc_core::Subject;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

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
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no stronghold manager found"))?;

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let name = payload["name"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read name from json payload"))?;
    let picture = payload["picture"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read picture from json payload"))?;
    let theme = payload["theme"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read theme from json payload"))?;

    let public_key = stronghold_manager.get_public_key()?;

    let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
    let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

    let provider_manager = ProviderManager::new([subject.clone()])?;
    let wallet: Wallet = Wallet::new(subject.clone());

    let profile = Profile {
        name: name.to_string(),
        picture: Some(picture.to_string()),
        theme: Some(theme.to_string()),
        primary_did: subject.identifier()?,
    };

    state.active_profile.lock().unwrap().replace(profile);
    state_guard.identity_manager.replace(IdentityManager {
        subject,
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

pub async fn update_credential_metadata(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let credential_id: Uuid = payload["id"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read credential id from json payload"))?
        .parse()?;

    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no stronghold manager found"))?;

    let mut verifiable_credential_record: VerifiableCredentialRecord = stronghold_manager
        .values()?
        .unwrap()
        .into_iter()
        .find(|record| record.display_credential.id == credential_id.to_string())
        .ok_or(anyhow::anyhow!("credential not found"))?;

    info!(
        "verifiable_credential_record (before): {:?}",
        verifiable_credential_record.display_credential.metadata
    );

    // set name if given
    verifiable_credential_record.display_credential.metadata.display.name = match payload["name"].as_str() {
        Some(name) => Some(name.to_string()),
        None => {
            info!("no name provided, using existing");
            verifiable_credential_record.display_credential.metadata.display.name
        }
    };

    // set color if given
    verifiable_credential_record.display_credential.metadata.display.color = match payload["color"].as_str() {
        Some(color) => Some(color.to_string()),
        None => {
            info!("no color provided, using existing");
            verifiable_credential_record.display_credential.metadata.display.color
        }
    };

    // set icon if given
    verifiable_credential_record.display_credential.metadata.display.icon = match payload["icon"].as_str() {
        Some(icon) => Some(icon.to_string()),
        None => {
            info!("no icon provided, using existing");
            verifiable_credential_record.display_credential.metadata.display.icon
        }
    };

    // set favorite if given
    verifiable_credential_record.display_credential.metadata.is_favorite = match payload["is_favorite"].as_bool() {
        Some(is_favorite) => is_favorite,
        None => {
            info!("no is_favorite provided, using existing");
            verifiable_credential_record.display_credential.metadata.is_favorite
        }
    };

    info!(
        "verifiable_credential_record (after): {:?}",
        verifiable_credential_record.display_credential.metadata
    );

    stronghold_manager.insert(
        credential_id,
        json!(verifiable_credential_record).to_string().as_bytes().to_vec(),
    )?;
    info!("credential metadata updated");

    *state.credentials.lock().unwrap() = stronghold_manager
        .values()?
        .unwrap()
        .into_iter()
        .map(|record| record.display_credential)
        .collect();

    Ok(())
}

pub fn update_profile_settings(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;

    let theme = match payload["theme"].as_str() {
        Some(theme) => theme.to_string(),
        None => {
            info!("no theme provided, using existing");
            state
                .active_profile
                .lock()
                .unwrap()
                .clone()
                .unwrap()
                .theme
                .clone()
                .unwrap()
        }
    };

    state
        .active_profile
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .theme
        .replace(theme);

    let name = match payload["name"].as_str() {
        Some(name) => name.to_string(),
        None => {
            info!("no name provided, using existing");
            state.active_profile.lock().unwrap().clone().unwrap().name.clone()
        }
    };

    state.active_profile.lock().unwrap().as_mut().unwrap().name = name;

    *state.current_user_prompt.lock().unwrap() = None;
    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.active_profile.lock().unwrap() = None;
    *state.locale.lock().unwrap() = Locale::default();
    state.credentials.lock().unwrap().clear();
    state.connections.lock().unwrap().clear();
    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Redirect {
        target: "welcome".to_string(),
    });
    *state.user_journey.lock().unwrap() = None;
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
                name: "Ferris".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some("system".to_string()),
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
