use crate::state::credentials::reducers::send_token_request::send_token_request;
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
use log::info;
use oid4vc::oid4vci::{
    authorization_details::{AuthorizationDetailsObject, CredentialConfigurationOrFormat, OpenidCredential},
    credential_offer::Grants,
    pkce,
};
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

// TODO: rename this reducer to `handle_credential_offer` or similar. This should be done in an isolated PR in order to prevent
// confusing git diffs.
pub async fn send_credential_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_credential_request");
    if let Some(selected_offer) = listen::<CredentialOffersSelected>(action.clone()) {
        let credential_configuration_ids = selected_offer.credential_configuration_ids;

        let state_guard = state.core_utils.managers.lock().await;

        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

        let current_user_prompt = state
            .current_user_prompt
            .clone()
            .ok_or(MissingStateParameterError("current user prompt"))?;

        info!("current_user_prompt: {current_user_prompt:?}");

        let credential_offer = state
            .core_utils
            .active_credential_offer
            .ok_or(AppError::Error("Missing active credential offer".to_string()))?;

        // The credential offer contains a credential issuer url.
        let credential_issuer_url = credential_offer.credential_issuer.clone();

        info!("credential issuer url: {credential_issuer_url:?}");

        // Get the authorization server metadata.
        let authorization_server_metadata = wallet
            .get_authorization_server_metadata(credential_issuer_url.clone())
            .await
            .map_err(GetAuthorizationServerMetadataError)?;

        info!("authorization server metadata: {authorization_server_metadata:?}");

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

                    let tx_code = selected_offer.tx_code;

                    if tx_code_required && tx_code.is_none() {
                        return Err(AppError::Error("tx_code is required but not provided".to_string()));
                    }

                    if tx_code_required {
                        info!("tx_code is required and provided: {}", tx_code.is_some());
                    } else {
                        info!("tx_code not required for this offer");
                    }

                    let action = CodeReceived {
                        code: pre_authorized_code.pre_authorized_code.clone(),
                        is_pre_authorized: true,
                        state: None,
                        tx_code,
                    };

                    drop(state_guard);
                    return send_token_request(
                        AppState {
                            core_utils: CoreUtils {
                                active_credential_offer: Some(credential_offer),
                                active_credential_configuration_ids: Some(credential_configuration_ids),
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
                    // Generate a random 128-byte code verifier (must be between 43 and 128 bytes)
                    let code_verifier = pkce::code_verifier(128);
                    // Generate an encrypted code challenge accordingly
                    let code_challenge = pkce::code_challenge(&code_verifier);

                    let authorization_details: Vec<AuthorizationDetailsObject> = credential_configurations_supported
                        .iter()
                        .map(
                            |(credential_configuration_id, credential_configuration)| AuthorizationDetailsObject {
                                r#type: OpenidCredential::Type,
                                locations: None,
                                credential_configuration_or_format:
                                    CredentialConfigurationOrFormat::CredentialConfigurationId {
                                        credential_configuration_id: credential_configuration_id.clone(),
                                        parameters: None,
                                    },
                                claims: Some(
                                    credential_configuration
                                        .claims
                                        .iter()
                                        .map(|claims| claims.clone().into())
                                        .collect(),
                                ),
                            },
                        )
                        .collect();

                    let wallet_state = Uuid::new_v4().to_string();

                    let par_response = wallet
                        .get_pushed_authorization_response(
                            authorization_server_metadata
                                .pushed_authorization_request_endpoint
                                .ok_or(AppError::Error(
                                    "Authorization Server does not have a pushed authorirzation request endpoint"
                                        .to_string(),
                                ))?
                                .clone(),
                            UNIME_CLIENT_ID,
                            UNIME_REDIRECT_URI.parse().unwrap(),
                            wallet_state.clone(),
                            authorization_details,
                            authorization_code
                                .issuer_state
                                .ok_or(AppError::Error(
                                    "Authorization Code Grant must contain an issuer state".to_string(),
                                ))?
                                .clone(),
                            Some(code_challenge),
                            Some("S256".to_string()),
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
                            active_credential_offer: Some(credential_offer),
                            active_credential_configuration_ids: Some(credential_configuration_ids),
                            active_code_verifier: Some(code_verifier),
                            active_wallet_state: Some(wallet_state),
                            ..state.core_utils
                        },
                        ..state
                    });
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
