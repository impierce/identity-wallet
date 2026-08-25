use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::Action,
        core_utils::{
            helpers::download_logo,
            history_event::{EventType, HistoryEvent},
            ActiveFlow,
        },
        credentials::reducers::handle_oid4vp_authorization_request::ClientMetadata,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use identity_iota::did::CoreDID;
use log::{info, warn};
use oid4vc::oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    client_metadata::ClientMetadataResource,
};
use oid4vc::siopv2::siopv2::SIOPv2;

// Sends the authorization response.
pub async fn handle_siopv2_authorization_request(state: AppState, _action: Action) -> Result<AppState, AppError> {
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
    info!("response generated: {response:?}");

    #[cfg(not(feature = "test_utils"))]
    if provider_manager.send_response(&response).await.is_err() {
        info!("failed to send response");
        return Err(SendAuthorizationResponseError);
    }
    info!("response successfully sent");

    let ClientMetadata {
        client_name,
        logo_uri,
        connection_url,
        client_id,
        ..
    } = get_siopv2_client_metadata(&siopv2_authorization_request).await?;

    if logo_uri.is_some() {
        warn!("Skipping download of client logo as it should have already been downloaded in `read_authorization_request()` and be present in /assets/tmp folder");
    }

    let did = CoreDID::parse(client_id).ok();

    let mut connections = state.connections;
    let connection = connections.update_or_insert(&connection_url, &client_name, did);

    let file_name = match logo_uri {
        Some(logo_uri) => hash(logo_uri.as_str()),
        None => "_".to_string(),
    };
    persist_asset(&file_name, &connection.id).ok();

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

// Helper

// TODO: move this functionality to the oid4vc-manager crate.
// TODO: this fn is nearly an exact copy of the fn `get_oid4vp_client_name_and_logo_uri`, find a simple way to put this into one generic helper.

pub async fn get_siopv2_client_metadata(
    siopv2_authorization_request: &AuthorizationRequest<Object<SIOPv2>>,
) -> Result<ClientMetadata, AppError> {
    // Get the connection url from the redirect url host (or use the redirect url if it does not
    // contain a host).
    let redirect_uri = siopv2_authorization_request.body.uri.uri().clone();
    let connection_url = redirect_uri.host_str().unwrap_or(redirect_uri.as_str());

    let client_id = siopv2_authorization_request.body.client_id.clone();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(match &siopv2_authorization_request.body.extension.client_metadata {
        ClientMetadataResource::ClientMetadata {
            client_name, logo_uri, ..
        } => {
            let client_name = client_name.as_ref().cloned().unwrap_or(connection_url.to_string());
            let logo_uri = logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            if let Some(logo_uri_str) = logo_uri.clone() {
                download_logo(&logo_uri_str)
                    .await
                    .ok_or(Error("Failed to download logo".to_string()))?; // should this throw an error?
            } else {
                warn!("No logo URI found");
            }

            Ok(ClientMetadata {
                client_name,
                logo_uri,
                connection_url: connection_url.to_string(),
                client_id: client_id.clone(),
                redirect_uri: Some(redirect_uri.to_string()),
            })
        }
        // TODO: support `client_metadata_uri`
        ClientMetadataResource::ClientMetadataUri(_) => Err(Error("Client metadata URI not supported".to_string())),
    }
    // Otherwise use the connection_url as the client_name.
    .unwrap_or_else(|_| ClientMetadata {
        client_name: connection_url.to_string(),
        logo_uri: None,
        connection_url: connection_url.to_string(),
        client_id,
        redirect_uri: Some(redirect_uri.to_string()),
    }))
}
