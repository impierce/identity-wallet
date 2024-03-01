use super::actions::{CancelUserFlow, UnlockStorage};
use crate::{
    error::AppError::{self, *},
    persistence::{clear_all_assets, delete_state_file, delete_stronghold, load_state},
    state::{
        actions::listen, actions::Action, core_utils::IdentityManager, dev_mode::DevMode,
        user_prompt::CurrentUserPrompt, AppState,
    },
    stronghold::StrongholdManager,
};

use did_key::{from_existing_key, Ed25519KeyPair};
use log::{debug, info};
use oid4vc::oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

pub async fn get_state(_state: AppState, _action: Action) -> Result<AppState, AppError> {
    debug!("get_state reducer called");
    let mut state = load_state().await.unwrap_or_default();

    if state.profile_settings.profile.is_some() {
        state.current_user_prompt = Some(CurrentUserPrompt::PasswordRequired);
    } else {
        // TODO: bug: if state is present, but empty, user will never be redirected to neither welcome or profile page
        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        });
    }

    // Overwrite dev_mode_enabled if environment variable is set
    if let Some(dev_mode) = std::env::var("DEV_MODE_ENABLED")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
    {
        if dev_mode {
            if state.dev_mode == DevMode::Off {
                state.dev_mode = DevMode::On;
            }
        } else {
            state.dev_mode = DevMode::Off;
        }
    }

    Ok(state)
}

pub async fn unlock_storage(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<UnlockStorage>(action).map(|payload| payload.password) {
        let mut state_guard = state.core_utils.managers.lock().await;

        let stronghold_manager = Arc::new(StrongholdManager::load(&password).map_err(StrongholdLoadingError)?);

        let public_key = stronghold_manager.get_public_key().map_err(StrongholdPublicKeyError)?;

        let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
        let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

        let provider_manager = ProviderManager::new([subject.clone()]).map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(subject.clone());

        info!("loading credentials from stronghold");
        let credentials = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        state_guard.stronghold_manager.replace(stronghold_manager);

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        info!("storage unlocked");

        drop(state_guard);
        return Ok(AppState {
            credentials,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..state
        });
    }

    Ok(state)
}

pub async fn cancel_user_flow(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(cancel_user_flow) = listen::<CancelUserFlow>(action) {
        return Ok(AppState {
            current_user_prompt: cancel_user_flow
                .redirect
                .map(|target| CurrentUserPrompt::Redirect { target }),
            ..state
        });
    }

    Ok(state)
}

/// Completely resets the state to its default values.
pub async fn reset_state(state: AppState, _action: Action) -> Result<AppState, AppError> {
    delete_state_file().await.ok();
    delete_stronghold().await.ok();
    clear_all_assets().ok();

    Ok(AppState {
        current_user_prompt: Some(CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        }),
        // Keep maintaing dev_mode state
        dev_mode: state.dev_mode,
        ..Default::default()
    })
}
