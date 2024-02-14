pub mod authorization;
pub mod credential_offer;
pub mod storage;
pub mod user_data_query;

use super::actions::{listen, Action, CustomExtensionTest, CancelUserFlow, UpdateCredentialMetadata};
use super::persistence::{delete_state_file, delete_stronghold, load_state};
use crate::error::AppError::{self, *};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;
use crate::verifiable_credential_record::VerifiableCredentialRecord;
use log::info;
use serde_json::json;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        actions::{Action, Reset},
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
        }.add_feat_state("profile", Box::new(profile {
            name: "Ferris".to_string(),
            picture: Some("&#129408".to_string()),
            theme: Some("system".to_string()),
            primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
        }));

        app_state = reset_state(app_state, Arc::new(Reset)).await.unwrap();

        assert_eq!(app_state.feat_states.is_empty(), true);
        assert_eq!(app_state.extensions.is_empty(), true);
        assert_eq!(app_state.locale, Locale::default());
    }
}
