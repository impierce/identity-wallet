use crate::{
    error::AppError::{self, *},
    persistence::download_asset,
    state::{
        actions::{listen, Action},
        hash,
        qr_code::actions::qrcode_scanned::QrCodeScanned,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{debug, info};
use oid4vc::oid4vci::{
    credential_issuer::credentials_supported::CredentialsSupportedObject,
    credential_offer::{CredentialOffer, CredentialOfferQuery, CredentialsObject},
};

pub async fn read_credential_offer(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("read_credential_offer");

    // Sometimes reducers are connected to actions that they shouldn't execute
    // Therefore its also checked if it can parse to credential offer query
    // TODO find a better way to connect to the right reducer
    if let Some(credential_offer_uri) =
        listen::<QrCodeScanned>(action).and_then(|payload| payload.form_urlencoded.parse::<CredentialOfferQuery>().ok())
    {
        let state_guard = state.core_utils.managers.lock().await;
        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

        let mut credential_offer: CredentialOffer = match credential_offer_uri {
            CredentialOfferQuery::CredentialOffer(credential_offer) => credential_offer,
            CredentialOfferQuery::CredentialOfferUri(credential_offer_uri) => wallet
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

        let credentials_supported_objects: Vec<CredentialsSupportedObject> = credential_offer
            .credentials
            .iter()
            .map(|credential| {
                match credential {
                    CredentialsObject::ByReference(by_reference) => credential_issuer_metadata
                        .as_ref()
                        .and_then(|credential_issuer_metadata| {
                            credential_issuer_metadata
                                .credentials_supported
                                .iter()
                                .find(|credential_supported| {
                                    credential_supported.scope == Some(by_reference.to_owned())
                                })
                        })
                        .ok_or(MissingCredentialOfferError(by_reference.clone())),
                    _by_value => credential_issuer_metadata
                        .as_ref()
                        .and_then(|credential_issuer_metadata| credential_issuer_metadata.credentials_supported.first())
                        .ok_or(MissingCredentialOfferError("nothing found".to_string())),
                }
                .expect("TODO: handle missing")
                .to_owned()
            })
            .collect();

        // For all credentials by reference, replace them with credentials by value using the CredentialIssuerMetadata.
        for credential in credential_offer.credentials.iter_mut() {
            match credential {
                CredentialsObject::ByReference(by_reference) => {
                    *credential = CredentialsObject::ByValue(
                        credential_issuer_metadata
                            .as_ref()
                            .and_then(|credential_issuer_metadata| {
                                credential_issuer_metadata
                                    .credentials_supported
                                    .iter()
                                    .find(|credential_supported| {
                                        credential_supported.scope == Some(by_reference.to_owned())
                                    })
                                    .map(|credential_supported| credential_supported.credential_format.clone())
                            })
                            .ok_or(MissingCredentialOfferError(by_reference.clone()))?,
                    );
                }
                _by_value => (),
            }
        }

        // Get the credential issuer display if present.
        let display = credential_issuer_metadata
            .and_then(|credential_issuer_metadata| {
                credential_issuer_metadata
                    .display
                    .map(|display| display.first().cloned())
            })
            .flatten();

        // Get the credential issuer name and logo uri or use the credential issuer url.
        let (issuer_name, logo_uri) = display
            .map(|display| {
                let issuer_name = display["client_name"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or(credential_issuer_url.to_string());

                let logo_uri = display["logo_uri"].as_str().map(|s| s.to_string());
                // ===== OpenID for Verifiable Credential Issuance - draft 12 (26 November 2023) =====
                // let issuer_name = display["name"]
                //     .as_str()
                //     .map(|s| s.to_string())
                //     .unwrap_or(credential_issuer_url.to_string());
                // let logo_uri = display["logo"]
                //     .as_object()
                //     .unwrap()
                //     .get("url")
                //     .unwrap()
                //     .as_str()
                //     .map(|s| s.to_string());
                (issuer_name, logo_uri)
            })
            .unwrap_or((credential_issuer_url.to_string(), None));

        info!("issuer_name in credential_offer: {:?}", issuer_name);
        info!("logo_uri in credential_offer: {:?}", logo_uri);

        let mut display = vec![];
        for (index, credential_supported_object) in credentials_supported_objects.iter().enumerate() {
            let credential_logo_url = credential_supported_object.display.as_ref().and_then(|display| {
                display
                    .first()
                    .and_then(|value| value.get("logo").and_then(|value| value.get("url")))
            });

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
                    let _ = download_asset(credential_logo_url, format!("credential_{}", index).as_str()).await;
                }
            }

            if credential_logo_url.is_none() && logo_uri.is_none() {
                debug!("No logo found in metadata.");
            }

            display.push(
                credential_supported_object
                    .display
                    .as_ref()
                    .and_then(|display| display.first().cloned()),
            );
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
                credential_offer,
                display,
            }),
            ..state
        });
    }

    Ok(state)
}
