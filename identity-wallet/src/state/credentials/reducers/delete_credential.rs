use std::fs;

use log::{debug, info};
use uuid::Uuid;

use crate::{
    error::AppError,
    persistence::{ASSETS_DIR, SUPPORTED_LOGO_EXTENSIONS},
    state::{
        actions::{listen, Action},
        credentials::actions::delete_credential::DeleteCredential,
        AppError::StrongholdDeletionError,
        AppState,
    },
};

pub async fn delete_credential(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(delete_credential) = listen::<DeleteCredential>(action) {
        let mut credentials = state.credentials.clone();

        // Delete credential image in ASSETS folder
        for e in SUPPORTED_LOGO_EXTENSIONS {
            let asset_pathbuf = ASSETS_DIR
                .lock()
                .unwrap()
                .as_path()
                .to_owned()
                .join(&delete_credential.id);

            if asset_pathbuf.join(e).exists() {
                match fs::remove_file(&asset_pathbuf) {
                    Ok(_) => info!("Succesfully removed logo asset in path: {:?}", asset_pathbuf),
                    Err(_) => debug!("Unable to remove logo asset in path: {:?}", asset_pathbuf),
                }
            }
        }

        {
            // Delete verifiable credential from stronghold
            let managers = &state.core_utils.managers.lock().await;

            if let Some(stronghold) = &managers.stronghold_manager {
                stronghold
                    .remove(
                        Uuid::parse_str(&delete_credential.id).expect("error: could not parse credential.id to uuid"),
                    )
                    .map_err(StrongholdDeletionError)?; //);
            }
        }

        // Delete DisplayCredential from AppState
        credentials.retain(|credential| credential.id != delete_credential.id.to_string());

        info!("Succesfully deleted file from the Appstate and Stronghold and its assets from the ASSETS_DIR");

        return Ok(AppState {
            credentials,
            current_user_prompt: None,
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
    async fn test_delete_credential() {
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
