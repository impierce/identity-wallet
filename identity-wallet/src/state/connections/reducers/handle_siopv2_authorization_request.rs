use crate::{
    error::AppError::{self, *},
    persistence::persist_asset,
    state::{
        actions::Action,
        connections::{reducers::get_siopv2_client_name_and_logo_uri, Connection, ConnectionRequest},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{info, warn};

// Sends the authorization response.
pub async fn handle_siopv2_authorization_request(state: AppState, _action: Action) -> Result<AppState, AppError> {
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

    let connection_time = chrono::Utc::now().to_rfc3339();

    let (client_name, logo_uri, connection_url) = get_siopv2_client_name_and_logo_uri(&siopv2_authorization_request)
        .map_err(|_| MissingAuthorizationRequestParameterError("connection_url"))?;

    if logo_uri.is_some() {
        warn!("Skipping download of client logo as it should have already been downloaded in `read_authorization_request()` and be present in /assets/tmp folder");
    }

    let mut connections = state.connections.clone();

    let result = connections
        .iter_mut()
        .find(|connection| connection.url == connection_url && connection.client_name == client_name)
        .map(|connection| {
            connection.last_interacted = connection_time.clone();
        });

    // TODO: This is a HORRIBLE solution to determine the connection_id by the non-unique "issuer name".
    // It is a TEMPORARY solution and should only be used in DEMO environments,
    // since we currently lack a unique identitfier to distinguish connections.
    let connection_id = base64::encode_config(&client_name, base64::URL_SAFE);

    persist_asset("issuer_0", &connection_id).ok();

    if result.is_none() {
        connections.push(Connection {
            id: connection_id,
            client_name,
            url: connection_url,
            verified: false,
            first_interacted: connection_time.clone(),
            last_interacted: connection_time,
        })
    };

    drop(state_guard);
    Ok(AppState {
        connections,
        current_user_prompt: Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        }),
        ..state
    })
}
