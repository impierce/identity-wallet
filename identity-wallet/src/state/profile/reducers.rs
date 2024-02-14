use super::{actions::{CreateNew, SetLocale, UpdateProfileSettings}, Profile};
use crate::{crypto::stronghold::StrongholdManager, error::AppError::{self, *}, state::{actions::{listen, Action}, user_prompt::CurrentUserPrompt, AppState, IdentityManager}};
use oid4vc::{oid4vc_core::Subject, oid4vc_manager::{methods::key_method::KeySubject, ProviderManager}, oid4vci::Wallet};
use did_key::{from_existing_key, Ed25519KeyPair};
use log::{debug, info};
use std::sync::Arc;

/// Reducers

/// Reducer to set the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
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

/// Reducer to create a new profile with a new DID (using the did:key method) and set it as the active profile.
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
            profile: Some(profile),
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..state
        });
    }

    Ok(state)
}

/// Reducer to initialize the stronghold manager.
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

/// Reducer to update the profile settings.
pub async fn update_profile_settings(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(UpdateProfileSettings { theme, name, picture }) = listen::<UpdateProfileSettings>(action) {
        let profile = state.profile.ok_or(MissingStateParameterError("active profile"))?.clone();

        return Ok(AppState {
            profile: Some(Profile {
                name: name.unwrap_or(profile.name),
                picture,
                theme,
                ..profile
            }),
            ..state
            });
    }

    Ok(state)
}
