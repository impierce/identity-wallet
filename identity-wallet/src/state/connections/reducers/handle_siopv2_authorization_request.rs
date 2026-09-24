use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        connections::actions::connection_accepted::ConnectionAccepted,
        core_utils::{
            history_event::{EventType, HistoryEvent},
            ActiveFlow, CoreUtils,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{debug, info};

/// Handles the `ConnectionAccepted` action for the SIOPv2 active flow, triggered by accepting `AcceptConnection` prompt and persists the connection.
/// Sends the SIOPv2 authorization response.
/// This fn completes the SIOPv2 authorization flow, therefore the active flow and pending connection data are cleared at the end and the user is redirected to the "me" page.
#[tracing::instrument(skip_all, err)]
pub async fn handle_siopv2_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(_connection_accepted) = listen::<ConnectionAccepted>(action) {
        let siopv2_authorization_request = match state.core_utils.active_flow.clone() {
            Some(ActiveFlow::Siopv2 { authorization_request }) => authorization_request,
            // Not a SIOPv2 flow, let other reducers handle this action.
            _ => return Ok(state),
        };

        let state_guard = state.core_utils.managers.lock().await;

        let provider_manager = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .provider_manager;

        info!("generating response");

        let response = provider_manager
            .generate_response(&*siopv2_authorization_request, Default::default())
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        debug!("Generated SIOPv2 authorization response: {response:?}");

        #[cfg(not(feature = "test_utils"))]
        if provider_manager.send_response(&response).await.is_err() {
            log::warn!("Failed to send SIOPv2 authorization response to redirect_uri");
            return Err(SendAuthorizationResponseError);
        }
        info!("SIOPv2 response successfully sent");

        let pending_connection_data = state.core_utils.pending_connection_data.clone().ok_or_else(|| {
            Error("Unexpected state: No pending connection data found for SIOPv2 authorization request".to_string())
        })?;
        let mut connections = state.connections;
        let connection = connections.update_last_interaction_or_insert_new(&pending_connection_data);

        if let Some(logo_uri) = pending_connection_data.client_metadata.logo_uri {
            persist_asset(&hash(logo_uri.as_str()), &connection.id).ok();
        }

        // History
        let mut history = state.history;
        history.push(HistoryEvent {
            connection_name: connection.name.clone(),
            event_type: EventType::ConnectionAdded,
            connection_id: connection.id.clone(),
            date: connection.last_interacted.clone(),
            credentials: vec![],
        });

        drop(state_guard);
        Ok(AppState {
            connections,
            core_utils: CoreUtils {
                active_flow: None,
                pending_connection_data: None,
                ..state.core_utils
            },
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            history,
            ..state
        })
    } else {
        Ok(state)
    }
}
