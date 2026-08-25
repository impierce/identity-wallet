use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        connections::reducers::handle_siopv2_authorization_request::get_siopv2_client_metadata,
        core_utils::{ActiveFlow, CoreUtils, Oid4vciStage},
        credentials::reducers::handle_oid4vp_authorization_request::{get_oid4vp_client_metadata, ClientMetadata},
        did::validate_linked_verifiable_presentations::validate_linked_verifiable_presentations,
        qr_code::{
            actions::qrcode_scanned::QrCodeScanned, reducers::read_credential_offer::get_oid4vci_client_metadata,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use log::info;
use oid4vc::siopv2::siopv2::SIOPv2;
use oid4vc::{
    oid4vc_core::authorization_request::{AuthorizationRequest, Object},
    oid4vci::credential_offer::CredentialOffer,
};
use oid4vc::{oid4vci::credential_offer::CredentialOfferParameters, oid4vp::oid4vp::OID4VP};

/// The kind of request encoded in a scanned QR-code.
///
/// SIOPv2 and OID4VP requests are classified through `AuthorizationRequest::from_generic`, while
/// OID4VCI credential offers use their own URL scheme and are parsed directly from the raw string.
#[derive(Debug, Clone)]
enum ParsedQrCode {
    Siopv2(Box<AuthorizationRequest<Object<SIOPv2>>>),
    Oid4vp(Box<AuthorizationRequest<Object<OID4VP>>>),
    Oid4vci(Box<CredentialOfferParameters>),
}

// Read and parde the the QR-code to a URL.
// Retrieve the connection data to display the "Trust connection" screen.
// Init the `ActiveFlow` enum with the rest of the retrieved data.
pub async fn accept_connection(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(qr_code_scanned) = listen::<QrCodeScanned>(action).map(|payload| payload.form_urlencoded) {
        let parsed_qr_code = parse_qr_code(&state, qr_code_scanned).await?;
        info!("QR code parsed as: {parsed_qr_code:?}");

        let (client_metadata, active_flow) =
            get_client_metadata_init_active_flow(&state, parsed_qr_code.clone()).await?;
        info!("Retrieved client metadata: {client_metadata:?}");
        info!("Initialized active flow: {active_flow:?}");

        let previously_connected = state
            .connections
            .contains(&client_metadata.connection_url, &client_metadata.client_name);

        let did = client_metadata.client_id.as_str();

        let state_guard = state.core_utils.managers.lock().await;

        let domain_validation = {
            #[cfg(not(feature = "test_utils"))]
            {
                use crate::state::did::validate_domain_linkage::validate_domain_linkage;

                let url_str = if let Some(redirect_uri) = &client_metadata.redirect_uri {
                    redirect_uri.clone()
                } else {
                    client_metadata.connection_url.clone()
                };

                let url = url::Url::parse(&url_str).map_err(|_| {
                    Error(format!(
                        "`redirect_uri` could not be parsed to url::Url: `{:?}`", // TODO: improve error message
                        url_str.clone()
                    ))
                })?;

                let resolver = &state_guard
                    .identity_manager
                    .as_ref()
                    .ok_or(MissingManagerError("identity"))?
                    .subject
                    .resolver()
                    .await;

                Box::new(validate_domain_linkage(resolver, url, did).await)
            }
            #[cfg(feature = "test_utils")]
            {
                // Skip validation during tests
                Default::default()
            }
        };

        info!("Domain validation result: {domain_validation:?}");

        let resolver = state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .subject
            .resolver()
            .await;

        let linked_verifiable_presentations = validate_linked_verifiable_presentations(&resolver, did)
            .await
            .into_iter()
            .flatten()
            .collect();

        info!("linked_verifiable_presentations: {linked_verifiable_presentations:?}");

        drop(state_guard);

        let current_user_prompt = Some(CurrentUserPrompt::AcceptConnection {
            client_name: client_metadata.client_name,
            logo_uri: client_metadata.logo_uri,
            redirect_uri: client_metadata.redirect_uri,
            previously_connected,
            domain_validation,
            linked_verifiable_presentations,
        });

        info!("Setting current user prompt to: {current_user_prompt:?}");

        Ok(AppState {
            current_user_prompt,
            core_utils: CoreUtils {
                active_flow: Some(active_flow),
                ..state.core_utils
            },
            ..state
        })
    } else {
        Ok(state)
    }
}

// Helper

// OID4VCI credential offers are handled by a dedicated reducer, so they're
// parsed directly here rather than through `provider_manager.validate_request`.
async fn parse_qr_code(state: &AppState, qr_code_scanned: String) -> Result<ParsedQrCode, AppError> {
    let state_guard = state.core_utils.managers.lock().await;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .wallet;

    if let Ok(credential_offer) = qr_code_scanned.parse::<CredentialOffer>() {
        let credential_offer: CredentialOfferParameters = match credential_offer {
            CredentialOffer::CredentialOffer(credential_offer) => *credential_offer,
            CredentialOffer::CredentialOfferUri(credential_offer_uri) => wallet
                .get_credential_offer(credential_offer_uri)
                .await
                .map_err(GetCredentialOfferError)?,
        };

        return Ok(ParsedQrCode::Oid4vci(Box::new(credential_offer)));
    }

    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .provider_manager;

    let generic_authorization_request = provider_manager
        .validate_request(qr_code_scanned.clone())
        .await
        .map_err(|_| InvalidQRCodeError(qr_code_scanned.clone()))?;

    if let Result::Ok(siopv2_authorization_request) =
        AuthorizationRequest::<Object<SIOPv2>>::from_generic(&generic_authorization_request)
    {
        Ok(ParsedQrCode::Siopv2(Box::new(siopv2_authorization_request)))
    } else if let Result::Ok(oid4vp_authorization_request) =
        AuthorizationRequest::<Object<OID4VP>>::from_generic(&generic_authorization_request)
    {
        Ok(ParsedQrCode::Oid4vp(Box::new(oid4vp_authorization_request)))
    } else {
        Err(InvalidAuthorizationRequest(Box::new(generic_authorization_request)))
    }
}

async fn get_client_metadata_init_active_flow(
    state: &AppState,
    parsed_qr_code: ParsedQrCode,
) -> Result<(ClientMetadata, ActiveFlow), AppError> {
    match parsed_qr_code {
        ParsedQrCode::Siopv2(siopv2_authorization_request) => {
            let client_metadata = get_siopv2_client_metadata(&siopv2_authorization_request).await?;
            let active_flow = ActiveFlow::Siopv2 {
                authorization_request: siopv2_authorization_request,
            };
            Ok((client_metadata, active_flow))
        }
        ParsedQrCode::Oid4vp(oid4vp_authorization_request) => {
            let client_metadata = get_oid4vp_client_metadata(&oid4vp_authorization_request).await?;
            let active_flow = ActiveFlow::Oid4vp {
                authorization_request: oid4vp_authorization_request,
                is_interactive: false,
            };
            Ok((client_metadata, active_flow))
        }
        ParsedQrCode::Oid4vci(credential_offer) => {
            let client_metadata = get_oid4vci_client_metadata(state, &credential_offer).await?;
            let active_flow = ActiveFlow::Oid4vciOffer {
                stage: Oid4vciStage::OfferReceived,
                logo_uri: client_metadata.logo_uri.clone(),
                credential_offer,
            };
            Ok((client_metadata, active_flow))
        }
    }
}
