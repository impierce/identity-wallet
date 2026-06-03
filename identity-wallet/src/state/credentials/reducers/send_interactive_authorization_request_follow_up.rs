use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        core_utils::{ActiveFlow, CoreUtils, Oid4vciStage},
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

        let (
            oid4vp_authorization_request,
            auth_session,
            interactive_authorization_endpoint,
            code_verifier,
            wallet_state,
            credential_offer,
            logo_uri,
        ) = match state.core_utils.active_flow.clone() {
            Some(ActiveFlow::Oid4vciOffer {
                stage:
                    Oid4vciStage::InteractiveAuthorization {
                        code_verifier,
                        wallet_state,
                        authorization_request,
                        auth_session,
                        interactive_authorization_endpoint,
                    },
                credential_offer,
                logo_uri,
            }) => (
                authorization_request,
                auth_session,
                interactive_authorization_endpoint,
                code_verifier,
                wallet_state,
                credential_offer,
                logo_uri,
            ),
            _ => {
                return Err(AppError::Error("Expected interactive OID4VCI flow context".to_string()));
            }
        };

        let auth_session = auth_session.ok_or(AppError::Error("Active auth session is missing".to_string()))?;

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
                active_flow: Some(ActiveFlow::Oid4vciOffer {
                    stage: Oid4vciStage::AuthorizationCode {
                        code_verifier,
                        wallet_state,
                    },
                    credential_offer,
                    logo_uri,
                }),
                ..state.core_utils
            },
            connections,
            history,
            ..state
        };

        let action = Arc::new(CodeReceived {
            code: code.to_string(),
            is_pre_authorized: false,
            is_interactive: true,
            state: None,
            tx_code: None,
        });

        return send_token_request(state, action).await;
    }

    Ok(state)
}
