use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::Action,
        core_utils::{
            history_event::{EventType, HistoryEvent},
            ConnectionRequest,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{info, warn};
use oid4vc::oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    client_metadata::ClientMetadataResource,
};
use oid4vc::siopv2::siopv2::SIOPv2;

// Sends the authorization response.
pub async fn handle_siopv2_authorization_request(mut state: AppState, _action: Action) -> Result<AppState, AppError> {
    let state_guard = state.core_utils.managers.lock().await;

    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .provider_manager;

    let siopv2_authorization_request =
        match serde_json::from_value(serde_json::json!(state.core_utils.active_connection_request)).unwrap() {
            Some(ConnectionRequest::SIOPv2(siopv2_authorization_request)) => siopv2_authorization_request,
            _ => unreachable!(),
        };

    info!("generating response");

    let response = provider_manager
        .generate_response(&*siopv2_authorization_request, Default::default())
        .map_err(GenerateAuthorizationResponseError)?;
    info!("response generated: {:?}", response);

    if provider_manager.send_response(&response).await.is_err() {
        info!("failed to send response");
        return Err(SendAuthorizationResponseError);
    }
    info!("response successfully sent");

    let (client_name, logo_uri, connection_url) = get_siopv2_client_name_and_logo_uri(&siopv2_authorization_request);

    if logo_uri.is_some() {
        warn!("Skipping download of client logo as it should have already been downloaded in `read_authorization_request()` and be present in /assets/tmp folder");
    }

    let mut connections = state.connections;
    let connection = connections.update_or_insert(&connection_url, &client_name);

    let file_name = match logo_uri {
        Some(logo_uri) => hash(logo_uri.as_str()),
        None => "_".to_string(),
    };
    persist_asset(&file_name, &connection.id).ok();

    // History
    state.history.push(HistoryEvent {
        connection_name: connection.name.clone(),
        event_type: EventType::ConnectionAdded,
        connection_id: connection.id.clone(),
        date: connection.last_interacted.clone(),
        credentials: vec![],
    });

    state.connections = connections;
    state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    drop(state_guard);

    Ok(state)
}

// Helper

// TODO: move this functionality to the oid4vc-manager crate.
pub fn get_siopv2_client_name_and_logo_uri(
    siopv2_authorization_request: &AuthorizationRequest<Object<SIOPv2>>,
) -> (String, Option<String>, String) {
    // Get the connection url from the redirect url host (or use the redirect url if it does not
    // contain a host).
    let redirect_uri = siopv2_authorization_request.body.redirect_uri.clone();
    let connection_url = redirect_uri.host_str().unwrap_or(redirect_uri.as_str());

    // Get the client_name and logo_uri from the client_metadata if it exists.
    siopv2_authorization_request
        .body
        .extension
        .client_metadata
        .as_ref()
        .and_then(|client_metadata| match client_metadata {
            ClientMetadataResource::ClientMetadata {
                client_name, logo_uri, ..
            } => {
                let client_name = client_name.as_ref().cloned().unwrap_or(connection_url.to_string());
                let logo_uri = logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());
                Some((client_name, logo_uri, connection_url.to_string()))
            }
            _ => None,
        })
        // Otherwise use the connection_url as the client_name.
        .unwrap_or((connection_url.to_string(), None, connection_url.to_string()))
}
