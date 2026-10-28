use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        connections::actions::connection_accepted::ConnectionAccepted,
        core_utils::{ActiveFlow, CoreUtils},
        credentials::credential_query_matching::{ensure_shareable, match_credential_queries, with_last_shared},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use log::{info, warn};

/// Reads the active OID4VP request from the active flow after the `AcceptConnection` prompt is accepted and the `ConnectionAccepted` action is send back from the Frontend.
/// This function validates the request, and sets the next CurrentUserPrompt to non-interactive `ShareCredentials`.
/// Non-interactive `CredentialsSelected` is handled by `handle_oid4vp_authorization_request`.
pub async fn read_oid4vp_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(_connection_accepted) = listen::<ConnectionAccepted>(action) {
        info!("read oid4vp authorization request");

        let oid4vp_authorization_request = match state.core_utils.active_flow.clone() {
            Some(ActiveFlow::Oid4vp {
                authorization_request, ..
            }) => authorization_request,
            // Not a OID4VP flow, let other reducers handle this action.
            _ => return Ok(state),
        };

        let state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let verifiable_credentials = stronghold_manager.values().map_err(StrongholdValuesError)?.unwrap();
        info!("verifiable credentials: {verifiable_credentials:?}");

        let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
        let requested_credentials = match_credential_queries(dcql_query, &verifiable_credentials);
        info!("credentials that can fulfill the request: {requested_credentials:?}");

        drop(state_guard);
        let pending_connection_data = state.core_utils.pending_connection_data.clone();

        if let Some(pending_connection_data) = &pending_connection_data {
            let client_metadata = pending_connection_data.client_metadata.clone();
            let client_id = client_metadata.client_id.to_string();
            let connection = state
                .connections
                .0
                .iter()
                .find(|connection| connection.did == client_id);
            let requested_credentials = with_last_shared(requested_credentials, connection);
            ensure_shareable(&requested_credentials)?;
            Ok(AppState {
                core_utils: CoreUtils {
                    active_flow: Some(ActiveFlow::Oid4vp {
                        authorization_request: oid4vp_authorization_request.clone(),
                        is_interactive: false,
                    }),
                    ..state.core_utils
                },
                current_user_prompt: Some(CurrentUserPrompt::ShareCredentials {
                    client_name: client_metadata.client_name,
                    logo_uri: client_metadata.logo_uri,
                    requested_credentials,
                    is_interactive: false,
                }),
                ..state
            })
        } else {
            warn!("Unexpected state: No pending connection data found when reading authorization request");
            Ok(state)
        }
    } else {
        Ok(state)
    }
}
