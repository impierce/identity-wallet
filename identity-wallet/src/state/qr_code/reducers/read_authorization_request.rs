use crate::{
    error::AppError::{self, *},
    state::{
        actions::Action,
        core_utils::{ActiveFlow, CoreUtils},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use sd_jwt::{SdJwt, Sha256Hasher};
use serde_json::Value;

use identity_credential::sd_jwt_vc::SdJwtVc;
use log::{debug, info, warn};
use oid4vc::oid4vc_core::utils::jwt::get_unverified_jwt_claims;
use oid4vc::oid4vp::{dcql::dcql_query::Format, token::vp_token_validator::DecodedPresentations};
use oid4vc::{
    oid4vci::credential_format_profiles::CredentialFormats, oid4vp::dcql_evaluation::evaluate_credential_query,
};

// Reads the request url from the payload and validates it.
// TODO: improve naming & docs, this fn currently only reads OID4VP authorization requests, but the name is more generic.
pub async fn read_authorization_request(state: AppState, _action: Action) -> Result<AppState, AppError> {
    info!("read_authorization_request");

    let oid4vp_authorization_request = match state.core_utils.active_flow.clone() {
        Some(ActiveFlow::Oid4vp {
            authorization_request, ..
        }) => authorization_request,
        // Not a OID4VP flow, let other reducers handle this action.
        _ => return Ok(state),
    };

    let state_guard = state.core_utils.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?;

    let verifiable_credentials = stronghold_manager.values().map_err(StrongholdValuesError)?.unwrap();
    info!("verifiable credentials: {verifiable_credentials:?}");

    // TODO: Move most of this logic to `openid4vc` crates.
    let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
    let uuids: Vec<String> = dcql_query
        .credentials
        .iter()
        .filter_map(|credential_query_from_request| {
            verifiable_credentials.iter().find_map(|verifiable_credential_record| {
                let credential_data: Value = if credential_query_from_request.format == Format::DcSdJwt
                    && verifiable_credential_record.display_credential.format == CredentialFormats::DcSdJwt(())
                {
                    serde_json::json!(verifiable_credential_record
                        .verifiable_credential
                        .as_str()?
                        .parse::<SdJwtVc>()
                        .ok()?
                        .into_disclosed_object(&Sha256Hasher::new())
                        .ok()?)
                } else if credential_query_from_request.format == Format::VcSdJwt
                    && verifiable_credential_record.display_credential.format == CredentialFormats::VcSdJwt(())
                {
                    serde_json::json!(verifiable_credential_record
                        .verifiable_credential
                        .as_str()?
                        .parse::<SdJwt>()
                        .ok()?
                        .into_disclosed_object(&Sha256Hasher::new())
                        .ok()?)
                } else if credential_query_from_request.format == Format::JwtVcJson
                    && verifiable_credential_record.display_credential.format
                        == CredentialFormats::JwtVcJson(())
                {
                    let full_jwt_payload =
                        get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential)
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

                let credential_object = credential_data.as_object()?.clone();
                let decoded_presentations =
                    match DecodedPresentations::try_new(vec![credential_object]) {
                        Ok(decoded) => decoded,
                        Err(e) => {
                            debug!(
                                "Failed to decode credential into DecodedPresentations; id: {:?}, format: {:?}, error: {:?}",
                                verifiable_credential_record.display_credential.id,
                                verifiable_credential_record.display_credential.format,
                                e
                            );
                            return None;
                        }
                    };

                let credential_query_satisfied =
                    evaluate_credential_query(credential_query_from_request, &decoded_presentations);
                credential_query_satisfied.then_some(verifiable_credential_record.display_credential.id.clone())
            })
        })
        .collect();

    info!("uuids of VCs that can fulfill the request: {uuids:?}");

    drop(state_guard);

    if let Some(CurrentUserPrompt::AcceptConnection {
        client_name, logo_uri, ..
    }) = &state.current_user_prompt
    {
        // TODO: communicate when no credentials are available.
        if !uuids.is_empty() {
            Ok(AppState {
                core_utils: CoreUtils {
                    active_flow: Some(ActiveFlow::Oid4vp {
                        authorization_request: oid4vp_authorization_request.clone(),
                        is_interactive: false,
                    }),
                    ..state.core_utils
                },
                current_user_prompt: Some(CurrentUserPrompt::ShareCredentials {
                    client_name: client_name.clone(),
                    logo_uri: logo_uri.clone(),
                    options: uuids,
                    is_interactive: false,
                }),
                ..state
            })
        } else {
            Err(NoMatchingCredentialError)
        }
    } else {
        warn!("Unexpected state: No current user prompt found when reading authorization request");
        Ok(state)
    }
}
