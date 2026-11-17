use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        core_utils::{ActiveFlow, CoreUtils, Oid4vciStage, PendingConnectionData},
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
use log::{debug, info};
use oid4vci::InteractiveAuthorizationFollowUpRequest;
use std::sync::Arc;

/// Handles the interactive `CredentialsSelected` action, which is triggered after accepting the `ShareCredentials` prompt set by `handle_credential_offer`.
/// This reducer is directly chained to the `send_token_request` reducer via the return, retrieving the credentials.
#[tracing::instrument(skip_all, err)]
pub async fn send_interactive_authorization_request_follow_up(
    state: AppState,
    action: Action,
) -> Result<AppState, AppError> {
    if let Some(selections) =
        listen::<CredentialsSelected>(action).and_then(|payload| payload.is_interactive.then_some(payload.selections))
    {
        info!(
            "Sending interactive authorization follow-up with {} credential(s)",
            selections
                .iter()
                .map(|selection| selection.credential_uuids.len())
                .sum::<usize>()
        );

        // Checked before anything is sent, as `handle_credential_offer` did before the prompt.
        refuse_presentation_to_unverified_issuer(state.core_utils.pending_connection_data.as_ref())?;

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
                return Err(AppError::FlowExpiredError(
                    "Cannot find active interactive OID4VCI flow context in the backend state".to_string(),
                ));
            }
        };

        let auth_session =
            auth_session.ok_or(AppError::FlowExpiredError("Active auth session is missing".to_string()))?;

        let (vp_token_payload, history_credentials, last_shared) =
            build_oid4vp_vp_token_and_history_credentials(&state, stronghold_manager, identity_manager, selections)
                .await?;

        let openid4vp_response = provider_manager
            .generate_response(&oid4vp_authorization_request, vp_token_payload)
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        debug!("openid4vp response generated");

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

        let pending_connection_data = state.core_utils.pending_connection_data.clone().ok_or_else(|| {
            AppError::FlowExpiredError(
                "Expected pending connection data for interactive OID4VP authorization".to_string(),
            )
        })?;
        let mut connections = state.connections;
        let mut history = state.history;

        update_history_and_connections(
            history_credentials,
            last_shared,
            &pending_connection_data,
            &mut connections,
            &mut history,
        )
        .await?;

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
                pending_connection_data: Some(pending_connection_data),
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

        // This return chains the current reducer to the next reducer `send_token_request`, normally this is done via
        // the `ActionTrait`, but in this case that requires complicating the state as well as the reducer quite a bit,
        // this simplifies the flow significantly.
        return send_token_request(state, action).await;
    }

    Ok(state)
}

/// An Interactive Authorization asks the user for a presentation. UniMe shares no credentials with an issuer whose
/// identity is not verified, one that publishes no DID document: its DID is only known once its credential is received,
/// so the share could be recorded on no connection (ADR 0005).
pub(crate) fn refuse_presentation_to_unverified_issuer(
    pending_connection_data: Option<&PendingConnectionData>,
) -> Result<(), AppError> {
    match pending_connection_data {
        Some(pending) if pending.client_metadata.client_id.is_none() => Err(UntrustedError(
            "The issuer asks for credentials in an interactive authorization, but its identity is not verified"
                .to_string(),
        )),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::user_prompt::ClientMetadata;

    fn pending(client_id: Option<&str>) -> PendingConnectionData {
        PendingConnectionData {
            client_metadata: ClientMetadata {
                client_name: "Issuer".to_string(),
                redirect_uri: None,
                connection_url: "issuer.example".to_string(),
                logo_uri: None,
                client_id: client_id.map(|did| did.parse().unwrap()),
            },
            domain_validation: None,
            linked_verifiable_presentations: None,
            ecosystems: None,
            ecosystems_check: None,
        }
    }

    #[test]
    fn no_credentials_are_presented_to_an_issuer_whose_identity_is_not_verified() {
        assert!(matches!(
            refuse_presentation_to_unverified_issuer(Some(&pending(None))),
            Err(UntrustedError(_))
        ));
        assert!(refuse_presentation_to_unverified_issuer(Some(&pending(Some("did:web:issuer.example")))).is_ok());
    }
}
