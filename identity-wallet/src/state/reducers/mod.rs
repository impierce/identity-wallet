pub mod authorization;
pub mod credential_offer;
pub mod dev_mode;
pub mod storage;
pub mod user_data_query;

use super::actions::actions::{listen, Action, CreateNew, CustomExtensionTest, CancelUserFlow, SetLocale, UpdateCredentialMetadata, UpdateProfileSettings};
use super::persistence::{delete_state_file, delete_stronghold, load_state};
use super::IdentityManager;
use crate::crypto::stronghold::StrongholdManager;
use crate::error::AppError::{self, *};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, Profile};
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use did_key::{from_existing_key, Ed25519KeyPair};
use log::{debug, info};
use oid4vc::oid4vc_core::Subject;
use oid4vc::oid4vc_manager::methods::key_method::KeySubject;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use serde_json::json;
use std::sync::Arc;

pub async fn test_feat_state(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(test_feat_state) = listen::<CustomExtensionTest>(action) {
        
        let mut new_state = AppState {
            ..state
        };

        new_state.extensions.insert("test".to_string(), Box::new(super::CustomExtension{
            name: "new".to_string(),
            value: if test_feat_state.test_bool { "new".to_string() } else { "old".to_string() }
        }));

        return Ok(AppState {
            ..new_state
        });
    }
    Ok(state)
}

pub async fn get_state(_state: AppState, _action: Action) -> Result<AppState, AppError> {
    println!("get_state reducer called");
    let mut state = load_state().await.unwrap_or_default();

    if state.feat_states.get("profile").is_some() {
        state.current_user_prompt = Some(CurrentUserPrompt::PasswordRequired);
    } else {
        // TODO: bug: if state is present, but empty, user will never be redirected to neither welcome or profile page
        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        });
    }

    // Overwrite dev_mode_enabled if environment variable is set
    if let Some(b) = std::env::var("DEV_MODE_ENABLED")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
    {
        state.dev_mode_enabled = b;
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

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub async fn set_locale(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(locale) = listen::<SetLocale>(action).map(|payload| payload.locale) {
        debug!("locale set to: `{:?}`", locale);
        return Ok(AppState {
            locale,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub async fn create_identity(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateNew {
        name, picture, theme, ..
    }) = listen::<CreateNew>(action)
    {
        let mut state_guard = state.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

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

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        drop(state_guard);
        return Ok(AppState {
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..state
        }.add_feat_state("profile", Box::new(profile)));
    }

    Ok(state)
}

pub async fn initialize_stronghold(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<CreateNew>(action).map(|payload| payload.password) {
        state.managers.lock().await.stronghold_manager.replace(Arc::new(
            StrongholdManager::create(&password).map_err(StrongholdCreationError)?,
        ));

        info!("stronghold initialized");
        return Ok(state);
    }

    Ok(state)
}

pub async fn update_credential_metadata(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(UpdateCredentialMetadata {
        id: credential_id,
        name,
        color,
        icon,
        is_favorite,
    }) = listen::<UpdateCredentialMetadata>(action)
    {
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

        let credentials = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|record| record.display_credential)
            .collect();

        drop(state_guard);
        return Ok(AppState { credentials, ..state });
    };

    Ok(state)
}

pub async fn update_profile_settings(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(UpdateProfileSettings { theme, name, picture }) = listen::<UpdateProfileSettings>(action) {
        let profile = state.feat_states.get("profile").ok_or(MissingStateParameterError("active profile"))?.clone()
        .downcast::<Profile>().unwrap();

        return Ok(AppState {
            ..state
            }.add_feat_state("profile", Box::new(Profile {
                name: name.unwrap_or(profile.name),
                picture,
                theme,
                ..*profile
            })));
        
    }

    Ok(state)
}

/// Completely resets the state to its default values.
pub async fn reset_state(_state: AppState, _action: Action) -> Result<AppState, AppError> {
    delete_state_file().await.ok();
    delete_stronghold().await.ok();

    Ok(AppState {
        current_user_prompt: Some(CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        actions::actions::{Action, Reset},
        Locale,
    };

    #[tokio::test]
    async fn test_set_locale() {
        let mut app_state = AppState::default();

        app_state = set_locale(app_state, Arc::new(SetLocale { locale: Locale::Nl }) as Action)
            .await
            .unwrap();

        assert_eq!(app_state.locale, Locale::Nl);
    }

    #[tokio::test]
    async fn test_reset_state() {
        let mut app_state = AppState {
            ..AppState::default()
        }.add_feat_state("profile", Box::new(Profile {
            name: "Ferris".to_string(),
            picture: Some("&#129408".to_string()),
            theme: Some("system".to_string()),
            primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
        }));

        app_state = reset_state(app_state, Arc::new(Reset)).await.unwrap();

        assert_eq!(app_state.feat_states.is_empty(), true);
        assert_eq!(app_state.locale, Locale::default());
    }
}
