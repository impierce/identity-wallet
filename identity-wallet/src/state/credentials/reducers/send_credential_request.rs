use crate::oid4vci::authorization_request::CodeChallengeMethod;
use crate::state::core_utils::helpers::download_logo;
use crate::state::core_utils::{ActiveFlow, Oid4vciStage};
use crate::state::credentials::reducers::handle_oid4vp_authorization_request::{
    get_oid4vp_client_metadata, ClientMetadata,
};
use crate::state::credentials::reducers::send_token_request::send_token_request;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{UNIME_CLIENT_ID, UNIME_REDIRECT_URI};
use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        core_utils::CoreUtils,
        credentials::actions::{
            authorization_code_received::CodeReceived, credential_offers_selected::CredentialOffersSelected,
        },
        AppState,
    },
};
use identity_credential::sd_jwt_vc::SdJwtVc;
use log::{debug, info, warn};
use oid4vc::oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use oid4vc::oid4vc_core::utils::jwt::get_unverified_jwt_claims;
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use oid4vc::oid4vci::InteractionType;
use oid4vc::oid4vci::{
    authorization_details::{AuthorizationDetailsObject, OpenidCredential},
    credential_offer::Grants,
    pkce,
};
use oid4vc::oid4vp::dcql_evaluation::evaluate_credential_query;
use oid4vc::oid4vp::oid4vp::OID4VP;
use oid4vc::oid4vp::token::vp_token_validator::DecodedPresentations;
use sd_jwt::Sha256Hasher;
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

// TODO: rename this reducer to `handle_credential_offer` or similar. This should be done in an isolated PR in order to prevent
// confusing git diffs.
pub async fn send_credential_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_credential_request");

    if let Some(selected_offer) = listen::<CredentialOffersSelected>(action.clone()) {
        let credential_configuration_ids = selected_offer.credential_configuration_ids;

        let state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

        let (credential_offer, logo_uri) = match state.core_utils.active_flow.clone() {
            Some(ActiveFlow::Oid4vciOffer {
                credential_offer,
                logo_uri,
                ..
            }) => (*credential_offer, logo_uri),
            _ => {
                return Err(AppError::Error("Missing active OID4VCI flow context".to_string()));
            }
        };

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer.clone();

        info!("credential issuer url: {credential_issuer_url:?}");

        // Get the credential issuer metadata.
        let credential_issuer_metadata = wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .map_err(GetCredentialIssuerMetadataError)?;

        info!("credential issuer metadata: {credential_issuer_metadata:?}");

        // Get the credential issuer display.
        let display = credential_issuer_metadata
            .display
            .as_ref()
            .and_then(|display| display.first().cloned());

        info!("credential issuer display: {:?}", display);

        // Get the connection url from the credential issuer url host (or use the credential issuer url if it does not
        // contain a host).
        let connection_url = credential_issuer_url
            .host_str()
            .unwrap_or(credential_issuer_url.as_str());

        info!("connection url: {:?}", connection_url);

        // Get the credential issuer name or use the credential issuer url.
        let issuer_name = display
            .map(|display| {
                let issuer_name = display["name"]
                    .as_str()
                    .map(ToString::to_string)
                    // TODO(ngdil): Remove this fallback.
                    .or_else(|| display["client_name"].as_str().map(ToString::to_string))
                    .unwrap_or(connection_url.to_string());
                issuer_name
            })
            .unwrap_or(connection_url.to_string());

        info!("issuer name: {:?}", issuer_name);
        info!("credential configuration ids: {:?}", credential_configuration_ids);

        let mut credential_configurations_supported =
            credential_issuer_metadata.credential_configurations_supported.clone();

        credential_configurations_supported.retain(|credential_configuration_id, _| {
            credential_configuration_ids.contains(credential_configuration_id)
        });

        match credential_offer.grants.clone() {
            Some(Grants {
                pre_authorized_code,
                authorization_code,
            }) => {
                // If the Credential Offer contains a pre-authorized code grant, then dispatch the `CodeReceived` action directly.
                if let Some(pre_authorized_code) = pre_authorized_code {
                    let tx_code_required = pre_authorized_code.tx_code.is_some();

                    info!("Handling Pre-Authorized code grant. Transaction code required: `{tx_code_required}`");

                    let tx_code = selected_offer.tx_code;

                    if tx_code_required && tx_code.is_none() {
                        return Err(AppError::Error("tx_code is required but not provided".to_string()));
                    }

                    let action = CodeReceived {
                        code: pre_authorized_code.pre_authorized_code.clone(),
                        is_pre_authorized: true,
                        is_interactive: false,
                        state: None,
                        tx_code,
                    };

                    drop(state_guard);
                    return send_token_request(
                        AppState {
                            core_utils: CoreUtils {
                                active_flow: Some(ActiveFlow::Oid4vciOffer {
                                    stage: Oid4vciStage::PreAuthorized,
                                    credential_offer: Box::new(credential_offer),
                                    logo_uri,
                                }),
                                ..state.core_utils
                            },
                            ..state
                        },
                        std::sync::Arc::new(action),
                    )
                    .await;
                // TODO the code below should be moved to separate reducer(s) that handle(s) the pushed authorization request and the authorization request.
                // Else, if the Credential Offer contains an authorization code grant, then initiate the authorization
                // request. First, A Pushed Authorization Request (PAR) is sent to the authorization server. Then, the
                // `opener` plugin is used to open the authorization endpoint in the system browser. The flow will
                // continue when UniMe receives the authorization code via during redirection back to the app from the
                // browser. The frontend will then dispatch the `CodeReceived` action which will continue the flow.
                } else if let Some(authorization_code) = authorization_code {
                    let specified_authorization_server = authorization_code.authorization_server.as_ref();
                    // Check that the specified authorization servers exist in the Credential Issuer Metadata's `authorization_servers` parameter.
                    if let Some(specified_authorization_server) = specified_authorization_server {
                        if !credential_issuer_metadata.authorization_servers.is_empty()
                            && !credential_issuer_metadata
                                .authorization_servers
                                .contains(specified_authorization_server)
                        {
                            return Err(AppError::Error(format!(
                                "The specified authorization server `{specified_authorization_server}` is not an accepted authorization server."
                            )));
                        }
                    }

                    let authorization_server_url = specified_authorization_server
                        .or_else(|| credential_issuer_metadata.authorization_servers.first())
                        .cloned()
                        // Fall back to credential issuer url if no authorization server is specified.
                        .unwrap_or(credential_issuer_url);

                    // Generate a random 128-byte code verifier (must be between 43 and 128 bytes)
                    let code_verifier = pkce::code_verifier(128);
                    // Generate an encrypted code challenge accordingly
                    let code_challenge = pkce::code_challenge(&code_verifier);

                    let authorization_details: Vec<AuthorizationDetailsObject> = credential_configurations_supported
                        .keys()
                        .map(|credential_configuration_id| AuthorizationDetailsObject {
                            r#type: OpenidCredential::Type,
                            locations: None,
                            credential_configuration_id: credential_configuration_id.clone(),
                            credential_identifiers: None,
                            // Note: Technically, the Wallet can make use of the `claims` parameter to communicate
                            // to the Authorization Server that it requires certain claims to be included in the to
                            // be issued credential (see https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html#appendix-B.1-3.2).
                            // However, in practice at this point, it is unlikely that there are any Authorization
                            // Servers that support this. Since larger PAR requests (that inlcude the `claims`
                            // property) may trigger undefined server-side behaviour we choose to omit the `claims`
                            // property for now until there is a clear use case and support for it.
                            // In general, the actual use case of the `mandatory` property in the VCI metadata is
                            // questioned as can be observed in this (inactive) issue: https://github.com/openid/OpenID4VCI/issues/433
                            claims: None,
                        })
                        .collect();

                    let wallet_state = Uuid::new_v4().to_string();

                    // Get the authorization server metadata.
                    let authorization_server_metadata = wallet
                        .get_authorization_server_metadata(authorization_server_url.clone())
                        .await
                        .map_err(GetAuthorizationServerMetadataError)?;

                    if let Some(interactive_authorization_endpoint) =
                        &authorization_server_metadata.interactive_authorization_endpoint
                    {
                        let iae_response = wallet
                            .send_interactive_authorization_request(
                                interactive_authorization_endpoint.clone(),
                                UNIME_CLIENT_ID,
                                Some(UNIME_REDIRECT_URI.parse().unwrap()),
                                Some(wallet_state.clone()),
                                (!authorization_details.is_empty()).then_some(authorization_details),
                                Some(
                                    authorization_code
                                        .issuer_state
                                        .ok_or(AppError::Error(
                                            "Authorization Code Grant must contain an issuer state".to_string(),
                                        ))?
                                        .clone(),
                                ),
                                vec![InteractionType::OpenId4VpPresentation],
                                Some(code_challenge),
                                Some(CodeChallengeMethod::S256),
                            )
                            .await
                            .map_err(|err| {
                                AppError::Error(format!("Failed to get interactive authorization response: {}", err))
                            })?;

                        debug!(
                             "interactive authorization response received (auth_session_present={}, openid4vp_request_present={})",
                             iae_response.auth_session.is_some(),
                             iae_response.openid4vp_request.is_some()
                         );

                        let openid4vp_request_value = iae_response.openid4vp_request.clone().ok_or(AppError::Error(
                            "Interactive authorization response is missing `openid4vp_request`".to_string(),
                        ))?;

                        let oid4vp_authorization_request: AuthorizationRequest<Object<OID4VP>> =
                            serde_json::from_value(openid4vp_request_value)
                                .map_err(|e| AppError::Error(format!("Failed to parse openid4vp_request: {e}")))?;

                        let auth_session = iae_response.auth_session.clone();

                        let verifiable_credentials =
                            stronghold_manager.values().map_err(StrongholdValuesError)?.unwrap();

                        debug!(
                            "loaded {} verifiable credentials from stronghold",
                            verifiable_credentials.len()
                        );

                        let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;

                        let uuids: Vec<String> = dcql_query
                            .credentials
                            .iter()
                            .filter_map(|credential_query_from_request| {
                                verifiable_credentials.iter().find_map(|verifiable_credential_record| {
                                    let credential_data: serde_json::Value = if verifiable_credential_record
                                        .display_credential
                                        .format
                                        == CredentialFormats::DcSdJwt(())
                                        || verifiable_credential_record.display_credential.format
                                            == CredentialFormats::VcSdJwt(())
                                    {
                                        serde_json::json!(verifiable_credential_record
                                            .verifiable_credential
                                            .as_str()?
                                            .parse::<SdJwtVc>()
                                            .ok()?
                                            .into_disclosed_object(&Sha256Hasher::new())
                                            .ok()?)
                                    } else if verifiable_credential_record.display_credential.format
                                        == CredentialFormats::JwtVcJson(())
                                    {
                                        let full_jwt_payload = get_unverified_jwt_claims(
                                            &verifiable_credential_record.verifiable_credential,
                                        )
                                        .unwrap_or_default();
                                        // JWT_VC_JSON must be accessed from the vc values.
                                        full_jwt_payload.get("vc").cloned().unwrap_or_else(|| {
                                            debug!(
                                                "JWT-VC-JSON is missing `vc` claims or is not a valid JSON value: {:?}",
                                                full_jwt_payload
                                            );
                                            serde_json::json!({})
                                        })
                                    } else {
                                        debug!(
                                            "Unhandled credential format: {:?}",
                                            verifiable_credential_record.display_credential.format
                                        );
                                        get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential)
                                            .unwrap_or_default()
                                    };

                                    let credential_query_satisfied = evaluate_credential_query(
                                        credential_query_from_request,
                                        &DecodedPresentations::try_new(vec![credential_data.as_object()?.clone()])
                                            .ok()?,
                                    );
                                    credential_query_satisfied
                                        .then_some(verifiable_credential_record.display_credential.id.clone())
                                })
                            })
                            .collect();

                        info!("uuids of VCs that can fulfill the request: {uuids:?}");

                        let ClientMetadata {
                            client_name, logo_uri, ..
                        } = get_oid4vp_client_metadata(&oid4vp_authorization_request).await?;

                        info!("client_name in credential_offer: {client_name:?}");
                        info!("logo_uri in read_authorization_request: {logo_uri:?}");

                        if let Some(logo_uri_str) = logo_uri.clone() {
                            download_logo(&logo_uri_str).await;
                        } else {
                            warn!("No logo URI found");
                        }

                        // TODO: communicate when no credentials are available.
                        if !uuids.is_empty() {
                            drop(state_guard);

                            return Ok(AppState {
                                core_utils: CoreUtils {
                                    active_flow: Some(ActiveFlow::Oid4vciOffer {
                                        stage: Oid4vciStage::InteractiveAuthorization {
                                            code_verifier: code_verifier.clone(),
                                            wallet_state: wallet_state.clone(),
                                            authorization_request: oid4vp_authorization_request.clone().into(),
                                            auth_session: auth_session.clone(),
                                            interactive_authorization_endpoint: interactive_authorization_endpoint
                                                .clone(),
                                        },
                                        credential_offer: Box::new(credential_offer),
                                        logo_uri: logo_uri.clone(),
                                    }),
                                    ..state.core_utils
                                },
                                current_user_prompt: Some(CurrentUserPrompt::ShareCredentials {
                                    client_name,
                                    logo_uri,
                                    options: uuids,
                                    is_interactive: true,
                                }),
                                ..state
                            });
                        } else {
                            return Err(NoMatchingCredentialError);
                        }
                    } else {
                        let par_response = wallet
                            .get_pushed_authorization_response(
                                authorization_server_metadata
                                    .pushed_authorization_request_endpoint
                                    .ok_or(AppError::Error(
                                        "Authorization Server does not have a pushed authorization request endpoint"
                                            .to_string(),
                                    ))?
                                    .clone(),
                                UNIME_CLIENT_ID,
                                UNIME_REDIRECT_URI.parse().unwrap(),
                                wallet_state.clone(),
                                (!authorization_details.is_empty()).then_some(authorization_details),
                                authorization_code
                                    .issuer_state
                                    .ok_or(AppError::Error(
                                        "Authorization Code Grant must contain an issuer state".to_string(),
                                    ))?
                                    .clone(),
                                Some(code_challenge),
                                Some(CodeChallengeMethod::S256),
                            )
                            .await
                            .map_err(|err| {
                                AppError::Error(format!("Failed to get pushed authorization response: {}", err))
                            })?;

                        info!("par_response: {:?}", par_response);

                        let mut authorization_endpoint = authorization_server_metadata
                            .authorization_endpoint
                            .ok_or(AppError::Error(
                                "Authorization Server does not have an authorization endpoint".to_string(),
                            ))?
                            .clone();

                        authorization_endpoint
                            .query_pairs_mut()
                            .append_pair("client_id", UNIME_CLIENT_ID)
                            .append_pair("request_uri", &par_response.request_uri.to_string());

                        info!("Opening URL in browser: `{authorization_endpoint}`");
                        let app_handle = state
                            .core_utils
                            .app_handle
                            .clone()
                            .ok_or(AppError::Error("Tauri app handle is not available".to_string()))?;
                        app_handle
                            .opener()
                            .open_url(authorization_endpoint, None::<&str>)
                            .map_err(|err| AppError::Error(format!("Failed to open URL in browser: {err}")))?;

                        drop(state_guard);
                        return Ok(AppState {
                            core_utils: CoreUtils {
                                active_flow: Some(ActiveFlow::Oid4vciOffer {
                                    stage: Oid4vciStage::AuthorizationCode {
                                        code_verifier: code_verifier.clone(),
                                        wallet_state: wallet_state.clone(),
                                    },
                                    credential_offer: Box::new(credential_offer),
                                    logo_uri,
                                }),
                                ..state.core_utils
                            },
                            ..state
                        });
                    }
                } else {
                    return Err(AppError::Error(
                        "Credential offer does not contain a supported grant".to_string(),
                    ));
                }
            }
            None => {
                return Err(AppError::Error(
                    "Credential offer does not contain a supported grant".to_string(),
                ));
            }
        }
    }

    Ok(state)
}
