use std::collections::HashMap;

use crate::{
    error::AppError::{self, *},
    http_client::get_http_client,
    state::{
        actions::Action,
        core_utils::{helpers::download_logo, ActiveFlow},
        credentials::reducers::handle_oid4vp_authorization_request::ClientMetadata,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{debug, info, warn};
use oid4vc::oid4vci::{
    credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject,
    credential_offer::CredentialOfferParameters,
};
use serde_json::Value;

// TODO: improving naming & docs
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

    if let Some(CurrentUserPrompt::AcceptConnection {
        client_name, logo_uri, ..
    }) = &state.current_user_prompt
    {
        Ok(AppState {
            current_user_prompt: Some(CurrentUserPrompt::CredentialOffer {
                issuer_name: client_name.clone(),
                logo_uri: logo_uri.clone(),
                credential_configurations,
                tx_code,
            }),
            ..state
        })
    } else {
        warn!("Unexpected state: No current user prompt found when reading authorization request");
        Ok(state)
    }
}

pub async fn get_oid4vci_client_metadata(
    state: &AppState,
    credential_offer: &CredentialOfferParameters,
) -> Result<ClientMetadata, AppError> {
    let state_guard = state.core_utils.managers.lock().await;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .wallet;

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.credential_issuer.clone();
    // Inner workings of `origin()` and `ascii_serialization()` are slightly unusual and basically return a "null" string when the operation failed.
    let origin = credential_issuer_url.origin().ascii_serialization();
    let connection_url = if origin == "null" {
        credential_issuer_url.to_string()
    } else {
        origin
    };

    info!("credential issuer url: {credential_issuer_url:?}");

    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await
        .ok();

    let display = credential_issuer_metadata
        .as_ref()
        .and_then(|credential_issuer_metadata| {
            credential_issuer_metadata
                .display
                .as_ref()
                .map(|display| display.first().cloned())
        })
        .flatten();

    // TODO: remove the below hard indexing
    let (issuer_name, logo_uri) = match display {
        Some(display) => {
            let issuer_name = display["name"]
                .as_str()
                .map(ToString::to_string)
                .unwrap_or(credential_issuer_url.to_string());

            let mut logo_uri = display["logo"]["uri"].as_str().map(ToString::to_string);

            if let Some(logo_uri_str) = &logo_uri {
                if download_logo(logo_uri_str).await.is_none() {
                    // If the logo download fails, we don't throw an error.
                    logo_uri = None;
                }
            } else {
                warn!("No logo URI found");
            }

            (issuer_name, logo_uri)
        }
        None => (credential_issuer_url.to_string(), None),
    };

    let did_doc = get_http_client()
        .await
        .get(format!(
            "{}/.well-known/did.json",
            credential_issuer_url.to_string().trim_end_matches('/')
        ))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let client_id = did_doc
        .get("id")
        .and_then(|id| id.as_str())
        .ok_or(AppError::DidParseError)?
        .to_string();

    Ok(ClientMetadata {
        client_name: issuer_name,
        redirect_uri: Some(credential_issuer_url.to_string()),
        connection_url,
        logo_uri,
        client_id,
    })
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
