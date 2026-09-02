use std::collections::HashMap;

use crate::{
    error::AppError::{self, *},
    state::{
        actions::Action,
        core_utils::{helpers::download_logo, ActiveFlow},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{debug, info, warn};
use oid4vc::oid4vci::credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject;

/// Sets the `CredentialOffer` prompt after the `AcceptConnetion` prompt was accepted, triggering the `ConnectionAccepted` action.
/// Accepting the prompt set in this reducer would result in the `CredentialOffersSelected` action, which is handled by `handle_credential_offer`.
pub async fn read_credential_offer(state: AppState, _action: Action) -> Result<AppState, AppError> {
    info!("read_credential_offer");

    // Sometimes reducers are connected to actions that they shouldn't execute
    // Therefore its also checked if it can parse to credential offer query
    // TODO find a better way to connect to the right reducer
    let credential_offer = match state.core_utils.active_flow.clone() {
        Some(ActiveFlow::Oid4vciOffer { credential_offer, .. }) => credential_offer,
        // Not a OID4VCI flow, let other reducers handle this action.
        _ => return Ok(state),
    };

    let state_guard = state.core_utils.managers.lock().await;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .wallet;

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.credential_issuer.clone();

    info!("credential issuer url: {credential_issuer_url:?}");

    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await
        .ok();

    info!("credential issuer metadata: {credential_issuer_metadata:?}");

    let credential_configurations: HashMap<String, CredentialConfigurationsSupportedObject> = credential_offer
        .credential_configuration_ids
        .iter()
        .filter_map(|credential_configuration_id| {
            credential_issuer_metadata
                .as_ref()
                .and_then(|credential_issuer_metadata| {
                    credential_issuer_metadata
                        .credential_configurations_supported
                        .get(credential_configuration_id)
                        .map(|credential_configuration| {
                            (credential_configuration_id.clone(), credential_configuration.clone())
                        })
                })
        })
        .collect();

    let tx_code = credential_offer
        .grants
        .as_ref()
        .and_then(|grants| grants.pre_authorized_code.clone())
        .and_then(|pre_authorized_code| pre_authorized_code.tx_code);

    download_credential_logos(&credential_configurations).await;

    drop(state_guard);

    if let Some(CurrentUserPrompt::AcceptConnection { client_metadata, .. }) = &state.current_user_prompt {
        Ok(AppState {
            current_user_prompt: Some(CurrentUserPrompt::CredentialOffer {
                issuer_name: client_metadata.client_name.clone(),
                logo_uri: client_metadata.logo_uri.clone(),
                credential_configurations,
                tx_code,
            }),
            ..state
        })
    } else {
        warn!("Unexpected state: No current user prompt found when reading credential offer");
        Ok(state)
    }
}

/// Downloads all the Credential logos.
async fn download_credential_logos(
    credential_configurations: &HashMap<String, CredentialConfigurationsSupportedObject>,
) {
    for credential_configuration in credential_configurations.values() {
        let credential_logo_uri = credential_configuration
            .credential_metadata
            .as_ref()
            .and_then(|credential_metadata| credential_metadata.display.as_ref())
            .and_then(|display| display.first())
            .and_then(|value| value.logo.as_ref().map(|logo| logo.uri.clone()));

        debug!("Credential logo URI: {credential_logo_uri:?}");

        if let Some(logo_uri_str) = credential_logo_uri {
            download_logo(logo_uri_str.as_ref()).await;
        } else {
            warn!("No logo URI found");
        }
    }
}
