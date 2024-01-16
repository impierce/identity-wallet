pub mod authorization;
pub mod credential_offer;
pub mod dev_mode;
pub mod storage;
pub mod user_data_query;

use super::{IdentityManager, Locale};
use crate::crypto::stronghold::StrongholdManager;
use crate::error::AppError::{self, *};
use crate::state::actions::Action;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, Profile};
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use did_key::{from_existing_key, Ed25519KeyPair};
use log::{debug, info};
use oid4vc_core::Subject;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use serde_json::json;
use std::sync::Arc;

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub fn set_locale(state: &mut AppState, action: Action) -> Result<(), AppError> {
    let locale = match action {
        Action::SetLocale { locale } => locale,
        _ => return Err(InvalidActionError { action }),
    };

    state.locale = locale;
    debug!("locale set to: `{:?}`", state.locale);

    state.current_user_prompt = None;
    Ok(())
}

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub async fn create_identity(state: &mut AppState, action: Action) -> Result<(), AppError> {
    let mut state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?;

    let (name, picture, theme) = match action {
        Action::CreateNew {
            name, picture, theme, ..
        } => (name, picture, theme),
        _ => return Err(InvalidActionError { action }),
    };

    let public_key = stronghold_manager.get_public_key().map_err(StrongholdPublicKeyError)?;

    let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
    let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

    let provider_manager = ProviderManager::new([subject.clone()]).map_err(OID4VCProviderManagerError)?;
    let wallet: Wallet = Wallet::new(subject.clone());

    let profile = Profile {
        name: name.to_string(),
        picture: Some(picture.to_string()),
        theme: Some(theme.to_string()),
        primary_did: subject.identifier().map_err(OID4VCSubjectIdentifierError)?,
    };

    state.active_profile.replace(profile);
    state_guard.identity_manager.replace(IdentityManager {
        subject,
        provider_manager,
        wallet,
    });

    Ok(())
}

pub async fn initialize_stronghold(state: &mut AppState, action: Action) -> Result<(), AppError> {
    let password = match action {
        Action::CreateNew { password, .. } => password,
        _ => return Err(InvalidActionError { action }),
    };

    state.managers.lock().await.stronghold_manager.replace(Arc::new(
        StrongholdManager::create(&password).map_err(StrongholdCreationError)?,
    ));

    info!("stronghold initialized");

    Ok(())
}

pub async fn update_credential_metadata(state: &mut AppState, action: Action) -> Result<(), AppError> {
    let (credential_id, name, icon, color, is_favorite) = match action {
        Action::UpdateCredentialMetadata {
            id,
            name,
            icon,
            color,
            is_favorite,
        } => (id, name, icon, color, is_favorite),
        _ => return Err(InvalidActionError { action }),
    };

    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?;

    let mut verifiable_credential_record: VerifiableCredentialRecord = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .into_iter()
        .find(|record| record.display_credential.id == credential_id.to_string())
        .ok_or(StrongholdMissingCredentialError(credential_id))?;

    info!(
        "verifiable_credential_record (before): {:?}",
        verifiable_credential_record.display_credential.metadata
    );

    // set name if given
    if name.is_some() {
        verifiable_credential_record.display_credential.metadata.display.name = name;
    }

    // set color if given
    if color.is_some() {
        verifiable_credential_record.display_credential.metadata.display.color = color;
    }

    // set icon if given
    if icon.is_some() {
        verifiable_credential_record.display_credential.metadata.display.icon = icon;
    }

    // set favorite if given
    if let Some(is_favorite) = is_favorite {
        verifiable_credential_record.display_credential.metadata.is_favorite = is_favorite;
    }

    info!(
        "verifiable_credential_record (after): {:?}",
        verifiable_credential_record.display_credential.metadata
    );

    stronghold_manager
        .insert(
            credential_id,
            json!(verifiable_credential_record).to_string().as_bytes().to_vec(),
        )
        .map_err(StrongholdInsertionError)?;
    info!("credential metadata updated");

    state.credentials = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .into_iter()
        .map(|record| record.display_credential)
        .collect();

    Ok(())
}

pub fn update_profile_settings(state: &mut AppState, action: Action) -> Result<(), AppError> {
    let (name, picture, theme) = match action {
        Action::UpdateProfileSettings { name, picture, theme } => (name, picture, theme),
        _ => return Err(InvalidActionError { action }),
    };

    let _ = theme.map(|theme| {
        state.active_profile.as_mut().unwrap().theme.replace(theme.to_string());
        debug!("updated theme: {}", theme);
    });

    let _ = name.map(|name| {
        state.active_profile.as_mut().unwrap().name = name.to_string();
        debug!("updated name: {}", name);
    });

    let _ = picture.map(|picture| {
        state
            .active_profile
            .as_mut()
            .unwrap()
            .picture
            .replace(picture.to_string());
        debug!("updated picture: {}", picture);
    });

    state.current_user_prompt = None;
    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &mut AppState, _action: Action) -> Result<(), AppError> {
    state.active_profile = None;
    state.locale = Locale::default();
    state.credentials.clear();
    state.connections.clear();
    state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
        target: "welcome".to_string(),
    });
    state.user_journey = None;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::actions::Action;

    #[test]
    fn test_set_locale() {
        let mut state = AppState::default();

        assert!(set_locale(&mut state, Action::SetLocale { locale: Locale::Nl }).is_ok());
        assert_eq!(state.locale, Locale::Nl);
    }

    #[test]
    fn test_reset_state() {
        let mut state = AppState {
            active_profile: Some(Profile {
                name: "Ferris".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some("system".to_string()),
                primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            })
            .into(),
            ..AppState::default()
        };

        assert!(reset_state(&mut state, Action::Reset).is_ok());
        assert_eq!(state.active_profile, None);
        assert_eq!(state.locale, Locale::default());
    }
}
