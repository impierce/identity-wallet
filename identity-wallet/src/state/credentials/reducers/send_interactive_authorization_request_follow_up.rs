use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        core_utils::{ConnectionRequest, CoreUtils},
        credentials::{
            actions::{authorization_code_received::CodeReceived, credentials_selected::CredentialsSelected},
            reducers::{
                handle_oid4vp_authorization_request::{
                    build_oid4vp_vp_token_and_history_credentials, update_history_and_connections,
                },
                send_token_request::send_token_request,
            },
        },
        AppState,
    },
};
use log::info;
use oid4vc::oid4vci::InteractiveAuthorizationFollowUpRequest;
use std::sync::Arc;

pub async fn send_interactive_authorization_request_follow_up(
    state: AppState,
    action: Action,
) -> Result<AppState, AppError> {
    info!("send_interactive_authorization_request_follow_up");

    if let Some(credential_uuids) = listen::<CredentialsSelected>(action)
        .and_then(|payload| payload.is_interactive.then_some(payload.credential_uuids))
    {
        let state_guard = state.core_utils.managers.lock().await;

        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;
        let identity_manager = state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?;
        let provider_manager = &identity_manager.provider_manager;

        let oid4vp_authorization_request = if let ConnectionRequest::OID4VP(oid4vp_authorization_request) =
            serde_json::from_value(serde_json::json!(state.core_utils.active_connection_request)).unwrap()
        {
            oid4vp_authorization_request
        } else {
            return Err(AppError::Error("Expected OID4VP Authorization Request".to_string()));
        };

        let (vp_token_payload, history_credentials) = build_oid4vp_vp_token_and_history_credentials(
            &state,
            stronghold_manager,
            identity_manager,
            credential_uuids,
        )
        .await?;

        let openid4vp_response = provider_manager
            .generate_response(&oid4vp_authorization_request, vp_token_payload)
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        info!("response generated: {openid4vp_response:?}");

        let auth_session = state
            .core_utils
            .active_auth_session
            .clone()
            .ok_or(AppError::Error("Active auth session is missing".to_string()))?;

        let interactive_authorization_endpoint = state
            .core_utils
            .active_interactive_authorization_endpoint
            .clone()
            .ok_or(AppError::Error(
                "Active interactive authorization endpoint is missing".to_string(),
            ))?;

        let follow_up = InteractiveAuthorizationFollowUpRequest {
            auth_session,
            openid4vp_response: Some(serde_json::json!(openid4vp_response)),
            code_verifier: None,
        };

        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

        let response = wallet
            .send_interactive_authorization_follow_up(interactive_authorization_endpoint, follow_up)
            .await
            .map_err(|e| {
                AppError::Error(format!(
                    "Failed to send interactive authorization follow-up request: {e}"
                ))
            })?;

        let code = response.authorization_code().ok_or(AppError::Error(
            "Authorization code is missing in the response".to_string(),
        ))?;

        let mut connections = state.connections;
        let mut history = state.history;

        update_history_and_connections(
            &oid4vp_authorization_request,
            history_credentials,
            &mut connections,
            &mut history,
        )
        .await;

        drop(state_guard);
        let state = AppState {
            core_utils: CoreUtils {
                active_auth_session: None,
                active_interactive_authorization_endpoint: None,
                ..state.core_utils
            },
            connections,
            history,
            ..state
        };

        let action = Arc::new(CodeReceived {
            code: code.to_string(),
            is_pre_authorized: false,
            state: None,
            tx_code: None,
        });

        return send_token_request(state, action).await;
    }

    Ok(state)
}
