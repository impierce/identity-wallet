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

use log::debug;

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

        let debug_log = state.core_utils.debug_log.clone();
        let verifier = state
            .core_utils
            .pending_connection_data
            .as_ref()
            .map(|data| data.client_metadata.client_name.clone())
            .unwrap_or_else(|| "unknown Verifier".to_string());
        debug_log.info(format!(
            "SIOPv2 login with {verifier} (client_id: {})",
            siopv2_authorization_request.body.client_id
        ));

        let response = provider_manager
            .generate_response(&*siopv2_authorization_request, Default::default())
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        debug!("Generated SIOPv2 authorization response");
        crate::log_payload!("SIOPv2 authorization response: {response:?}");

        // Reads the response body, so the Verifier's HTTP status and error reach the Debug screen.
        #[cfg(not(feature = "test_utils"))]
        match crate::state::interaction_result::send_authorization_response(&response).await {
            crate::state::interaction_result::InteractionOutcome::Approved => {}
            outcome => {
                debug_log.error(format!("SIOPv2 login with {verifier} failed: {outcome}"));
                return Err(SendAuthorizationResponseError);
            }
        }
        debug_log.ok(format!("SIOPv2 login with {verifier}: approved"));

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
