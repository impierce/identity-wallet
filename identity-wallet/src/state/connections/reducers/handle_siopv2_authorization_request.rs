use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::Action,
        core_utils::{
            history_event::{EventType, HistoryEvent},
            ActiveFlow,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{debug, info};

/// Handles the `ConnectionAccepted` action for the SIOPv2 active flow, triggered by accepting `AcceptConnection` prompt and persists the connection.
/// Sends the SIOPv2 authorization response.
#[tracing::instrument(skip_all, err)]
pub async fn handle_siopv2_authorization_request(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let siopv2_authorization_request = match state.core_utils.active_flow.clone() {
        Some(ActiveFlow::Siopv2 { authorization_request }) => authorization_request,
        // Not a SIOPv2 flow, let other reducers handle this action.
        _ => return Ok(state),
    };

    let client_metadata = match &state.current_user_prompt {
        Some(CurrentUserPrompt::AcceptConnection { client_metadata, .. }) => client_metadata.clone(),
        _ => return Err(Error(
            "Unexpected state: No CurrentUserPrompt::AcceptConnection found when reading SIOPv2 authorization request"
                .to_string(),
        )),
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
        warn!("Failed to send SIOPv2 authorization response to redirect_uri");
        return Err(SendAuthorizationResponseError);
    }
    info!("SIOPv2 response successfully sent");

    let mut connections = state.connections;
    let connection = connections.update_or_insert(
        &client_metadata.connection_url,
        &client_metadata.client_name,
        client_metadata.client_id,
    );

    if let Some(logo_uri) = client_metadata.logo_uri {
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
        current_user_prompt: Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        }),
        history,
        ..state
    })
}
