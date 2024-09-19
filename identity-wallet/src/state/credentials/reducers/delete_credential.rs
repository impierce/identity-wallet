use log::{info, warn};
use std::fs;
use uuid::Uuid;

use crate::{
    error::AppError,
    persistence::{ASSETS_DIR, SUPPORTED_IMAGE_ASSET_EXTENSIONS},
    state::{
        actions::{listen, Action},
        credentials::actions::delete_credential::DeleteCredential,
        user_prompt::CurrentUserPrompt,
        AppError::StrongholdDeletionError,
        AppState,
    },
};

pub async fn delete_credential(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(delete_credential) = listen::<DeleteCredential>(action) {
        let mut credentials = state.credentials.clone();

        // Delete image file in assets folder
        for extension in SUPPORTED_IMAGE_ASSET_EXTENSIONS {
            let asset_path = ASSETS_DIR
                .lock()
                .unwrap()
                .as_path()
                .to_owned()
                .join(&delete_credential.id);

            if asset_path.join(extension).exists() {
                match fs::remove_file(&asset_path) {
                    Ok(_) => info!("Successfully removed logo file: `{:?}`", asset_path),
                    Err(e) => warn!("Failed to remove logo file: `{:?}`, reason: `{:?}`", asset_path, e),
                }
            }
        }

        {
            // Remove credential from Stronghold
            let managers = &state.core_utils.managers.lock().await;

            if let Some(stronghold) = &managers.stronghold_manager {
                stronghold
                    .remove(Uuid::parse_str(&delete_credential.id).map_err(|e| AppError::Error(e.to_string()))?)
                    .map_err(StrongholdDeletionError)?;
            }
        }

        // Remove DisplayCredential from AppState
        credentials.retain(|credential| credential.id != delete_credential.id.to_string());

        info!("Successfully deleted credential with id: `{}`", delete_credential.id);

        let redirect_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

        return Ok(AppState {
            credentials,
            current_user_prompt: redirect_prompt,
            ..state
        });
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::state::credentials::DisplayCredential;
    use crate::state::AppState;

    #[tokio::test]
    async fn test_credential_is_removed_from_appstate() {
        let credential = DisplayCredential {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
            ..Default::default()
        };

        let state = AppState {
            credentials: vec![credential],
            current_user_prompt: None,
            ..Default::default()
        };

        let action = Arc::new(DeleteCredential {
            id: state.credentials[0].id.clone(),
        });

        let result = delete_credential(state, action).await.unwrap();

        assert_eq!(result.credentials, vec![]);
    }
}
