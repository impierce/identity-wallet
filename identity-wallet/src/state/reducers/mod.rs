pub mod authorization;
pub mod credential_offer;
pub mod dev_mode;
pub mod dynamic_dev_profile;
pub mod storage;
pub mod user_data_query;

use super::actions::{listen, CancelUserFlow, SetLocale, UpdateCredentialMetadata, UpdateProfileSettings};
use super::{IdentityManager, ProfileSettings};
use crate::persistence::{clear_all_assets, delete_state_file, delete_stronghold, load_state};
use crate::stronghold::StrongholdManager;
use crate::error::AppError::{self, *};
use crate::state::actions::{Action, CreateNew};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, DevMode, Profile};
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use did_key::{from_existing_key, Ed25519KeyPair};
use futures::Future;
use log::{debug, info};
use oid4vc::oid4vc_core::Subject;
use oid4vc::oid4vc_manager::methods::key_method::KeySubject;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use serde_json::json;
use std::pin::Pin;
use std::sync::Arc;

/// A macro to wrap a reducer function in a Box and a Pin.
/// It checks the reducers for its signature,
///  as it should comply with our standard for reducers.
#[macro_export]
macro_rules! reducer {
    ($reducer:expr) => {
        Box::new(move |app_state, action| Box::pin(async move { $reducer(app_state, action).await }))
    };
}

/// A reducer is a function that takes the current state and an action and returns the new state.
pub type Reducer<'a> =
    Box<dyn Fn(AppState, Action) -> Pin<Box<dyn Future<Output = Result<AppState, AppError>> + Send>> + Send>;

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
            profile_settings: ProfileSettings {
                locale,
                ..state.profile_settings
            },
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
        let mut state_guard = state.core_state.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let public_key = stronghold_manager.get_public_key().map_err(StrongholdPublicKeyError)?;

        let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
        let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

        let provider_manager = ProviderManager::new([subject.clone()]).map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(subject.clone());

        let profile_settings = ProfileSettings {
            profile: Some(Profile {
                name: name.to_string(),
                picture: Some(picture.to_string()),
                theme: Some(theme.to_string()),
                primary_did: subject.identifier().map_err(OID4VCSubjectIdentifierError)?,
            }),
            ..Default::default()
        };

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        drop(state_guard);
        return Ok(AppState {
            profile_settings,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..state
        });
    }

    Ok(state)
}

pub async fn initialize_stronghold(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<CreateNew>(action).map(|payload| payload.password) {
        state
            .core_state
            .managers
            .lock()
            .await
            .stronghold_manager
            .replace(Arc::new(
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
        let state_guard = state.core_state.managers.lock().await;
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
        return Ok(AppState {
            credentials,
            current_user_prompt: None,
            ..state
        });
    };

    Ok(state)
}

pub async fn update_profile_settings(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(UpdateProfileSettings { theme, name, picture }) = listen::<UpdateProfileSettings>(action) {
        if let Some(profile) = state.profile_settings.profile.clone() {
            return Ok(AppState {
                profile_settings: ProfileSettings {
                    profile: Some(Profile {
                        name: name.unwrap_or(profile.name),
                        picture: picture.or(profile.picture),
                        theme: theme.or(profile.theme),
                        ..profile
                    }),
                    ..state.profile_settings
                },
                current_user_prompt: None,
                ..state
            });
        }
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

pub async fn cancel_user_journey(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        user_journey: None,
        current_user_prompt: None,
        ..state
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        actions::{CancelUserJourney, Reset},
        Locale,
    };

    #[tokio::test]
    async fn test_cancel_user_flow() {
        let current_user_prompt = Some(CurrentUserPrompt::ShareCredentials {
            client_name: "Impierce Technologies".to_string(),
            logo_uri: Some("logo.png".to_string()),
            options: vec![],
        });

        let mut app_state = AppState {
            current_user_prompt: current_user_prompt.clone(),
            ..AppState::default()
        };

        app_state = cancel_user_flow(app_state, Arc::new(CancelUserFlow { redirect: None }))
            .await
            .unwrap();

        assert_eq!(app_state.current_user_prompt, None);

        let mut app_state = AppState {
            current_user_prompt,
            ..AppState::default()
        };

        app_state = cancel_user_flow(
            app_state,
            Arc::new(CancelUserFlow {
                redirect: Some("welcome".to_string()),
            }),
        )
        .await
        .unwrap();

        assert_eq!(
            app_state.current_user_prompt,
            Some(CurrentUserPrompt::Redirect {
                target: "welcome".to_string(),
            })
        );
    }

    #[tokio::test]
    async fn test_set_locale() {
        let mut app_state = AppState::default();

        app_state = set_locale(app_state, Arc::new(SetLocale { locale: Locale::nl_NL }))
            .await
            .unwrap();

        assert_eq!(app_state.profile_settings.locale, Locale::nl_NL);
    }

    #[tokio::test]
    async fn test_update_profile_settings() {
        let active_profile = Profile {
            name: "Ferris".to_string(),
            picture: Some("&#129408".to_string()),
            theme: Some("system".to_string()),
            primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
        };

        let mut app_state = AppState {
            profile_settings: ProfileSettings {
                profile: Some(active_profile.clone()),
                ..ProfileSettings::default()
            },
            ..AppState::default()
        };

        app_state = update_profile_settings(
            app_state,
            Arc::new(UpdateProfileSettings {
                name: None,
                picture: None,
                theme: Some("light".to_string()),
            }),
        )
        .await
        .unwrap();

        assert_eq!(
            app_state.profile_settings.profile,
            Some(Profile {
                theme: Some("light".to_string()),
                ..active_profile
            })
        );
    }

    #[tokio::test]
    async fn test_cancel_user_journey() {
        let mut app_state = AppState {
            user_journey: Some(json!("Some Journey")),
            ..AppState::default()
        };

        app_state = cancel_user_journey(app_state, Arc::new(CancelUserJourney))
            .await
            .unwrap();

        assert_eq!(app_state.user_journey, None);
    }

    #[tokio::test]
    async fn test_reset_state() {
        let mut app_state = AppState {
            profile_settings: ProfileSettings {
                profile: Some(Profile {
                    name: "Ferris".to_string(),
                    picture: Some("&#129408".to_string()),
                    theme: Some("system".to_string()),
                    primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        app_state = reset_state(app_state, Arc::new(Reset)).await.unwrap();

        assert_eq!(app_state.profile_settings.profile, None);
        assert_eq!(app_state.profile_settings.locale, Locale::default());
    }
}
