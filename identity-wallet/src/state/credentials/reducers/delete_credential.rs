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
            let assets_path = ASSETS_DIR.lock().unwrap().as_path().to_owned();
            let file_path = assets_path.join(format!("{}.{}", delete_credential.id, extension));

            if file_path.exists() {
                match fs::remove_file(&file_path) {
                    Ok(_) => info!("Successfully deleted image file: `{:?}`", file_path),
                    Err(e) => warn!("Failed to delete image file: `{:?}`, reason: `{:?}`", file_path, e),
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
        credentials.retain(|credential| credential.id != delete_credential.id);

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
    use std::fs::File;
    use std::io::Write;
    use std::sync::Arc;

    use tempfile::{NamedTempFile, TempDir};

    use super::*;
    use crate::persistence::STRONGHOLD;
    use crate::state::credentials::DisplayCredential;
    use crate::state::AppState;
    use crate::stronghold::StrongholdManager;

    #[tokio::test]
    async fn test_credential_is_removed_from_appstate_and_from_stronghold_and_images_are_deleted() {
        let uuid = Uuid::new_v4();

        let credential = DisplayCredential {
            id: uuid.to_string(),
            ..Default::default()
        };

        let state = AppState {
            credentials: vec![credential],
            current_user_prompt: None,
            ..Default::default()
        };

        // Set up Stronghold
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();
        let stronghold_manager = StrongholdManager::create("sup3rSecr3t").unwrap();

        // Set up image asset
        let tmp_dir = TempDir::new().unwrap().into_path();
        *ASSETS_DIR.lock().unwrap() = tmp_dir.clone();
        let file_path = tmp_dir.join(format!("{}.{}", uuid, SUPPORTED_IMAGE_ASSET_EXTENSIONS[0]));
        let mut file = File::create(file_path.clone()).unwrap();
        file.write_all(b"some-bytes").unwrap();
        assert!(file_path.exists());

        let action = Arc::new(DeleteCredential {
            id: state.credentials[0].id.clone(),
        });

        let result = delete_credential(state.clone(), action).await.unwrap();

        // Assert AppState
        assert_eq!(result.credentials, vec![]);

        // Assert Stronghold
        assert_eq!(stronghold_manager.get(uuid).unwrap(), None);

        // Assert image asset
        assert_eq!(file_path.exists(), false);

        // Assert redirect
        assert_eq!(
            result.current_user_prompt,
            Some(CurrentUserPrompt::Redirect {
                target: "me".to_string()
            })
        );
    }
}
