use crate::{
    error::AppError::{self, *},
    persistence::{download_asset, hash},
    state::{
        actions::{listen, Action},
        connections::reducers::handle_siopv2_authorization_request::get_siopv2_client_name_and_logo_uri,
        core_utils::{helpers::get_unverified_jwt_claims, ConnectionRequest, CoreUtils},
        credentials::reducers::handle_oid4vp_authorization_request::{
            get_oid4vp_client_name_and_logo_uri, OID4VPClientMetadata,
        },
        did::{
            validate_domain_linkage::validate_domain_linkage,
            validate_linked_verifiable_presentations::validate_linked_verifiable_presentations,
        },
        qr_code::actions::qrcode_scanned::QrCodeScanned,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use identity_credential::{sd_jwt_v2::Sha256Hasher, sd_jwt_vc::SdJwtVc};
use log::{debug, info};
use oid4vc::oid4vp::{evaluate_input, oid4vp::OID4VP};
use oid4vc::siopv2::siopv2::SIOPv2;
use oid4vc::{
    oid4vc_core::authorization_request::{AuthorizationRequest, Object},
    oid4vci::credential_format_profiles::CredentialFormats,
};

// Reads the request url from the payload and validates it.
pub async fn read_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("read_authorization_request");

    if let Some(qr_code_scanned) = listen::<QrCodeScanned>(action)
        .map(|payload| payload.form_urlencoded)
        .filter(|s| !s.starts_with("openid-credential-offer"))
    {
        let state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;
        let provider_manager = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .provider_manager;

        let generic_authorization_request = provider_manager
            .validate_request(qr_code_scanned.clone())
            .await
            .map_err(|_| InvalidQRCodeError(qr_code_scanned))?;

        if let Result::Ok(siopv2_authorization_request) =
            AuthorizationRequest::<Object<SIOPv2>>::from_generic(&generic_authorization_request)
        {
            let redirect_uri = siopv2_authorization_request.body.redirect_uri.to_string();

            let (client_name, logo_uri, connection_url, _) =
                get_siopv2_client_name_and_logo_uri(&siopv2_authorization_request);

            info!(
                "client_name in Authorization Request Display parameter: {:?}",
                client_name
            );
            info!("logo_uri in Authorization Request Display parameter: {:?}", logo_uri);

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

            let previously_connected = state.connections.contains(&connection_url, &client_name);

            let url = url::Url::parse(&redirect_uri).map_err(|_| {
                Error(format!(
                    "`redirect_uri` could not be parsed to url::Url: `{:?}`",
                    redirect_uri.clone()
                ))
            })?;

            let did = siopv2_authorization_request.body.client_id.as_str();

            let domain_validation: Box<crate::state::did::validate_domain_linkage::ValidationResult> =
                Box::new(validate_domain_linkage(url, did).await);

            let temp: Vec<String> = state
                .trust_lists
                .0
                .iter()
                .map(|trust_list| {
                    trust_list
                        .entries
                        .iter()
                        .filter_map(|(domain, trusted)| trusted.then_some(domain.clone()))
                        .collect::<String>()
                })
                .collect();

            info!("temp: {:?}", temp);

            let linked_verifiable_presentations = validate_linked_verifiable_presentations(did)
                .await
                .into_iter()
                .flatten()
                .filter(|linked_verifiable_credential| {
                    linked_verifiable_credential.issuer_linked_domains.iter().any(|domain| {
                        info!("domain: {:?}", domain.to_string());

                        temp.contains(&domain.to_string())
                    })
                })
                .collect();

            info!("linked_verifiable_presentations: {:?}", linked_verifiable_presentations);

            drop(state_guard);

            return Ok(AppState {
                core_utils: CoreUtils {
                    active_connection_request: Some(ConnectionRequest::SIOPv2(siopv2_authorization_request.into())),
                    ..state.core_utils
                },
                current_user_prompt: Some(CurrentUserPrompt::AcceptConnection {
                    client_name,
                    logo_uri,
                    redirect_uri,
                    previously_connected,
                    domain_validation,
                    linked_verifiable_presentations,
                }),
                ..state
            });
        } else if let Result::Ok(oid4vp_authorization_request) =
            AuthorizationRequest::<Object<OID4VP>>::from_generic(&generic_authorization_request)
        {
            let verifiable_credentials = stronghold_manager.values().map_err(StrongholdValuesError)?.unwrap();
            info!("verifiable credentials: {:?}", verifiable_credentials);

            let uuids: Vec<String> = oid4vp_authorization_request
                .body
                .extension
                .presentation_definition
                .input_descriptors()
                .iter()
                .map(|input_descriptor| {
                    verifiable_credentials
                        .iter()
                        .find_map(|verifiable_credential_record| {
                            let credential = if verifiable_credential_record.display_credential.format
                                == CredentialFormats::VcSdJwt(())
                            {
                                serde_json::json!(verifiable_credential_record
                                    .verifiable_credential
                                    .as_str()?
                                    .parse::<SdJwtVc>()
                                    .ok()?
                                    .into_disclosed_object(&Sha256Hasher::new())
                                    .ok()?)
                            } else {
                                get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential).unwrap()
                            };

                            evaluate_input(input_descriptor, &credential)
                                .then_some(verifiable_credential_record.display_credential.id.clone())
                        })
                        .ok_or(NoMatchingCredentialError)
                })
                .collect::<Result<Vec<String>, AppError>>()?;

            info!("uuids of VCs that can fulfill the request: {:?}", uuids);

            let OID4VPClientMetadata {
                client_name,
                logo_uri,
                connection_url: _,
                client_id: _,
                algorithm: _,
            } = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request);

            info!("client_name in credential_offer: {:?}", client_name);
            info!("logo_uri in read_authorization_request: {:?}", logo_uri);

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

            // TODO: communicate when no credentials are available.
            if !uuids.is_empty() {
                drop(state_guard);
                return Ok(AppState {
                    core_utils: CoreUtils {
                        active_connection_request: Some(ConnectionRequest::OID4VP(oid4vp_authorization_request.into())),
                        ..state.core_utils
                    },
                    current_user_prompt: Some(CurrentUserPrompt::ShareCredentials {
                        client_name,
                        logo_uri,
                        options: uuids,
                    }),
                    ..state
                });
            }
        } else {
            return Err(InvalidAuthorizationRequest(Box::new(generic_authorization_request)));
        };
    }

    Ok(state)
}
