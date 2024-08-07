use std::collections::HashMap;

use crate::{
    error::AppError::{self, *},
    persistence::{download_asset, hash},
    state::{
        actions::{listen, Action},
        core_utils::CoreUtils,
        qr_code::actions::qrcode_scanned::QrCodeScanned,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{debug, info};
use oid4vc::oid4vci::{
    credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject,
    credential_offer::{CredentialOffer, CredentialOfferParameters},
};

pub async fn read_credential_offer(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("read_credential_offer");

    // Sometimes reducers are connected to actions that they shouldn't execute
    // Therefore its also checked if it can parse to credential offer query
    // TODO find a better way to connect to the right reducer
    if let Some(credential_offer_uri) =
        listen::<QrCodeScanned>(action).and_then(|payload| payload.form_urlencoded.parse::<CredentialOffer>().ok())
    {
        let state_guard = state.core_utils.managers.lock().await;
        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

        let credential_offer: CredentialOfferParameters = match credential_offer_uri {
            CredentialOffer::CredentialOffer(credential_offer) => *credential_offer,
            CredentialOffer::CredentialOfferUri(credential_offer_uri) => wallet
                .get_credential_offer(credential_offer_uri)
                .await
                .map_err(GetCredentialOfferError)?,
        };

        info!("credential offer: {:?}", credential_offer);

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer.clone();

        info!("credential issuer url: {:?}", credential_issuer_url);

        let credential_issuer_metadata = wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .ok();

        info!("credential issuer metadata: {:?}", credential_issuer_metadata);

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

        // Get the credential issuer display if present.
        let display = credential_issuer_metadata
            .as_ref()
            .and_then(|credential_issuer_metadata| {
                credential_issuer_metadata
                    .display
                    .as_ref()
                    .map(|display| display.first().cloned())
            })
            .flatten();

        // Get the credential issuer name and logo uri or use the credential issuer url.
        let (issuer_name, logo_uri) = display
            .map(|display| {
                let issuer_name = display["name"]
                    .as_str()
                    // TODO(NGDIL): remove this NGDIL specific logic once: https://staging.api.ngdil.com/.well-known/openid-credential-issuer is fixed.
                    .or_else(|| display["client_name"].as_str())
                    .map(ToString::to_string)
                    .unwrap_or(credential_issuer_url.to_string());

                let logo_uri = display["logo"]["uri"]
                    .as_str()
                    // TODO(NGDIL): remove this NGDIL specific logic once: https://staging.api.ngdil.com/.well-known/openid-credential-issuer is fixed.
                    .or_else(|| display["logo_uri"].as_str())
                    .map(ToString::to_string);

                (issuer_name, logo_uri)
            })
            .unwrap_or((credential_issuer_url.to_string(), None));

        info!("issuer_name in credential_offer: {:?}", issuer_name);
        info!("logo_uri in credential_offer: {:?}", logo_uri);

        for (credential_configuration_id, credential_configuration) in credential_configurations.iter() {
            let credential_logo_url = credential_configuration
                .display
                .first()
                .and_then(|value| value.get("logo").and_then(|value| value.get("url")));

            info!("credential_logo_url: {:?}", credential_logo_url);

            if let Some(credential_logo_url) = credential_logo_url {
                debug!(
                    "{}",
                    format!("Downloading credential logo from url: {}", credential_logo_url)
                );
                if let Some(credential_logo_url) = credential_logo_url
                    .as_str()
                    .and_then(|s| s.parse::<reqwest::Url>().ok())
                {
                    let _ = download_asset(
                        credential_logo_url,
                        format!("credential_{}", credential_configuration_id).as_str(),
                    )
                    .await;
                }
            }

            if credential_logo_url.is_none() && logo_uri.is_none() {
                debug!("No logo found in metadata.");
            }
        }

        if logo_uri.is_some() {
            debug!(
                "{}",
                format!(
                    "Downloading client logo from url: {}",
                    logo_uri.as_ref().unwrap().as_str()
                )
            );
            if let Some(logo_uri) = logo_uri.as_ref().and_then(|s| s.parse::<reqwest::Url>().ok()) {
                let _ = download_asset(logo_uri.clone(), &hash(logo_uri.as_str())).await;
            }
        }

        drop(state_guard);
        return Ok(AppState {
            current_user_prompt: Some(CurrentUserPrompt::CredentialOffer {
                issuer_name,
                logo_uri,
                credential_configurations,
            }),
            core_utils: CoreUtils {
                active_credential_offer: Some(credential_offer),
                ..state.core_utils
            },
            ..state
        });
    }

    Ok(state)
}
