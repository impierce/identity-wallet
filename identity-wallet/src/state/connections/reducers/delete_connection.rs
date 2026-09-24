use log::{info, warn};
use std::fs;

use crate::{
    error::AppError,
    persistence::{ASSETS_DIR, SUPPORTED_IMAGE_ASSET_EXTENSIONS},
    state::{
        actions::{listen, Action},
        connections::actions::delete_connection::DeleteConnection,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

/// Deletes the connection and the images associated with it's connection ID from the assets folder.
#[tracing::instrument(skip_all, err)]
pub async fn delete_connection(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(delete_connection) = listen::<DeleteConnection>(action) {
        let mut connections = state.connections.clone();

        // Delete image file in assets folder
        for extension in SUPPORTED_IMAGE_ASSET_EXTENSIONS {
            let assets_path = ASSETS_DIR.lock().unwrap().as_path().to_owned();
            let file_path = assets_path.join(format!("{}.{}", delete_connection.id, extension));

            if file_path.exists() {
                match fs::remove_file(&file_path) {
                    Ok(_) => info!("Successfully deleted image file: `{file_path:?}`"),
                    Err(e) => warn!("Failed to delete image file: `{file_path:?}`, reason: `{e:?}`"),
                }
            }
        }

        // Remove Connection from AppState
        connections.0.retain(|connection| connection.id != delete_connection.id);

        info!("Successfully deleted connection with id: `{}`", delete_connection.id);

        let redirect_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

        return Ok(AppState {
            connections,
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

    use tempfile::TempDir;

    use super::*;
    use crate::state::connections::{Connection, Connections};
    use crate::state::core_utils::history_event::{EventType, HistoryEvent};
    use crate::state::AppState;

    #[tokio::test]
    #[serial_test::serial]
    async fn test_connection_is_removed_from_appstate_and_image_is_deleted_while_history_is_preserved() {
        let connection = Connection {
            id: "conn-123".to_string(),
            name: "Test Connection".to_string(),
            url: "https://test.com".to_string(),
            did: "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            ..Default::default()
        };

        let history_event = HistoryEvent {
            connection_id: "conn-123".to_string(),
            connection_name: "Test Connection".to_string(),
            event_type: EventType::ConnectionAdded,
            date: "2026-09-24T12:00:00Z".to_string(),
            credentials: vec![],
        };

        let state = AppState {
            connections: Connections(vec![connection.clone()]),
            history: vec![history_event.clone()],
            current_user_prompt: None,
            ..Default::default()
        };

        // Set up image asset
        let tmp_dir = TempDir::new().unwrap().keep();
        *ASSETS_DIR.lock().unwrap() = tmp_dir.clone();
        let file_path = tmp_dir.join(format!("{}.{}", connection.id, SUPPORTED_IMAGE_ASSET_EXTENSIONS[0]));
        let mut file = File::create(file_path.clone()).unwrap();
        file.write_all(b"some-logo-bytes").unwrap();
        assert!(file_path.exists());

        let action = Arc::new(DeleteConnection {
            id: connection.id.clone(),
        });

        let result = delete_connection(state, action).await.unwrap();

        // Assert AppState connections
        assert!(result.connections.0.is_empty());

        // Assert image asset
        assert!(!file_path.exists());

        // Assert history is preserved
        assert_eq!(result.history.len(), 1);
        assert_eq!(result.history[0], history_event);

        // Assert redirect
        assert_eq!(
            result.current_user_prompt,
            Some(CurrentUserPrompt::Redirect {
                target: "me".to_string()
            })
        );
    }
}
