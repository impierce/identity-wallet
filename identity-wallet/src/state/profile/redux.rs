use std::sync::Arc;
use crate::{error::AppError::{self, *}, crypto::stronghold::StrongholdManager, reducer, state::{actions::{listen, Action, ActionTrait, Reducer}, user_prompt::CurrentUserPrompt, AppState}};
use did_key::{from_existing_key, Ed25519KeyPair};
use log::{debug, info};
use oid4vc::{oid4vc_core::Subject, oid4vc_manager::{methods::key_method::KeySubject, ProviderManager}, oid4vci::Wallet};
use ts_rs::TS;
use super::{Locale, Profile};

/// Actions

/// Action to set the locale to the given value.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SetLocale.ts")]
pub struct SetLocale {
    #[ts(type = "string")]
    pub locale: Locale,
}

#[typetag::serde(name = "[Settings] Set locale")]
impl ActionTrait for SetLocale {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_locale)]
    }
}

/// Action to create a new profile.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CreateNew.ts")]
pub struct CreateNew {
    pub name: String,
    pub picture: String,
    pub theme: String,
    pub password: String,
}

#[typetag::serde(name = "[DID] Create new")]
impl ActionTrait for CreateNew {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(initialize_stronghold), reducer!(create_identity)]
    }
}

/// Action to update the profile settings.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateProfileSettings.ts")]
pub struct UpdateProfileSettings {
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub picture: Option<String>,
    #[ts(optional)]
    pub theme: Option<String>,
}

#[typetag::serde(name = "[Settings] Update profile")]
impl ActionTrait for UpdateProfileSettings {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_profile_settings)]
    }
}

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
