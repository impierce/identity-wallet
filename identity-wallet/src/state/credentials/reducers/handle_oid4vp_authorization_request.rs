use crate::state::credentials::reducers::vp_token_payload_prep::prepare_vp_token_object;
use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            history_event::{EventType, HistoryCredential, HistoryEvent},
            ConnectionRequest,
        },
        credentials::actions::credentials_selected::CredentialsSelected,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use identity_iota::did::CoreDID;
use log::info;
use oid4vc::oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    client_metadata::ClientMetadataResource,
};
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use oid4vc::oid4vp::dcql::dcql_query::CredentialQuery;
use oid4vc::oid4vp::oid4vp::OID4VP;

pub async fn handle_oid4vp_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("handle_presentation_request");

    if let Some(credential_uuids) = listen::<CredentialsSelected>(action).map(|payload| payload.credential_uuids) {
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

        let oid4vp_authorization_request = if let ConnectionRequest::OID4VP(oid4vp_authorization_request) =
            serde_json::from_value(serde_json::json!(state.core_utils.active_connection_request)).unwrap()
        {
            oid4vp_authorization_request
        } else {
            return Err(AppError::Error("Expected OID4VP Authorization Request".to_string()));
        };

        let mut history_credentials = vec![];
        let dcql_from_request = &oid4vp_authorization_request.body.extension.dcql_query;
        let available_credentials_map: std::collections::HashMap<String, _> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|record| (record.display_credential.id.clone(), record)) // Map to (internal_uuid_string, full_record)
            .collect();

        let mut selected_verifiable_credentials: Vec<(CredentialQuery, serde_json::Value)> = Vec::new();
        for requested_credential_query in &dcql_from_request.credentials {
            //selects from all our credential_uuids, which ones we've said ok to.
            for user_selected_uuid in &credential_uuids {
                let user_selected_uuid_str = user_selected_uuid.to_string();
                if let Some(verifiable_credential_record) = available_credentials_map.get(&user_selected_uuid_str) {
                    let credential_data = if verifiable_credential_record.display_credential.format
                        == CredentialFormats::VcSdJwt(())
                    {
                        // Handle SD-JWTs to get disclosed claims
                        let sd_jwt_vc_string = verifiable_credential_record
                            .verifiable_credential
                            .as_str()
                            .ok_or(AppError::InvalidCredentialFormatError)?
                            .to_string();

                        let sd_jwt_vc = sd_jwt_vc_string
                            .parse::<identity_credential::sd_jwt_vc::SdJwtVc>()
                            .map_err(|e| AppError::Error(format!("Failed to parse stored SD-JWT VC: {e}")))?;

                        let disclosed_object = sd_jwt_vc
                            .into_disclosed_object(&identity_credential::sd_jwt_v2::Sha256Hasher::new())
                            .map_err(|e| {
                                AppError::Error(format!("Failed to get disclosed object from SD-JWT VC: {e}"))
                            })?;
                        serde_json::json!(disclosed_object)
                    } else if verifiable_credential_record.display_credential.format == CredentialFormats::JwtVcJson(())
                    {
                        // Handle jwt-vc-json starting from vc.
                        crate::state::core_utils::helpers::get_unverified_jwt_claims(
                            &verifiable_credential_record.verifiable_credential,
                        )
                        .unwrap_or_default()
                        .get("vc")
                        .cloned()
                        .unwrap_or_else(|| {
                            log::debug!(
                                "JWT-VC-JSON missing 'vc' claim or it's not a valid JSON value: {:?}",
                                verifiable_credential_record.verifiable_credential
                            );
                            serde_json::json!({})
                        })
                    } else {
                        log::warn!(
                            "Unsupported format {:?} for evaluation. Attempting to get unverified JWT claims.",
                            verifiable_credential_record.display_credential.format
                        );
                        crate::state::core_utils::helpers::get_unverified_jwt_claims(
                            &verifiable_credential_record.verifiable_credential,
                        )
                        .unwrap_or_default()
                    };

                    let credential_query_satisfied = oid4vc::oid4vp::dcql_evaluation::evaluate_credential_query(
                        requested_credential_query,
                        &credential_data,
                    );

                    if credential_query_satisfied {
                        selected_verifiable_credentials.push((
                            requested_credential_query.clone(),
                            verifiable_credential_record.verifiable_credential.clone(),
                        ));
                        history_credentials.push(HistoryCredential::from_credential(verifiable_credential_record));
                        break;
                    }
                }
            }
        }

        if selected_verifiable_credentials.is_empty() {
            return Err(AppError::Error(
                "No credentials selected or found to fulfill the request".to_string(),
            ));
        }

        let did_method = state
            .profile_settings
            .preferred_did_methods
            .first()
            .ok_or(AppError::Error("Default DID method is missing".to_string()))?;

        let algorithm = identity_manager
            .provider_manager
            .get_matching_signing_algorithm(&oid4vp_authorization_request)
            .await
            .unwrap_or(
                *identity_manager
                    .provider_manager
                    .provider
                    .supported_signing_algorithms
                    .first()
                    .ok_or_else(|| {
                        AppError::Error(
                            "Provider manager does not contain any supported signing algorithms".to_string(),
                        )
                    })?,
            );

        let subject_did_str = identity_manager
            .subject
            .identifier(did_method, algorithm)
            .await
            .map_err(AppError::OID4VCSubjectIdentifierError)?;

        let subject_did = CoreDID::parse(subject_did_str).map_err(|_| DidParseError)?;

        let vp_token_payload = prepare_vp_token_object(
            selected_verifiable_credentials,
            &subject_did,
            &identity_manager.subject,
            &oid4vp_authorization_request,
        )
        .await?;

        let response = provider_manager
            .generate_response(&oid4vp_authorization_request, vp_token_payload)
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        info!("response generated: {:?}", response);

        if provider_manager.send_response(&response).await.is_err() {
            info!("failed to send response");
            return Err(SendAuthorizationResponseError);
        }
        info!("response successfully sent");

        let OID4VPClientMetadata {
            client_name,
            logo_uri,
            connection_url,
            client_id,
        } = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request);

        let did = CoreDID::parse(client_id).ok();

        let mut connections = state.connections;
        let previously_connected = connections.contains(connection_url.as_str(), &client_name);
        let connection = connections.update_or_insert(&connection_url, &client_name, did);

        let file_name = match logo_uri {
            Some(logo_uri) => hash(logo_uri.as_str()),
            None => "_".to_string(),
        };
        persist_asset(&file_name, &connection.id).ok();

        let mut history = state.history;
        if !previously_connected {
            history.push(HistoryEvent {
                connection_name: connection.name.clone(),
                event_type: EventType::ConnectionAdded,
                connection_id: connection.id.clone(),
                date: connection.last_interacted.clone(),
                credentials: vec![],
            });
        }
        history.push(HistoryEvent {
            connection_name: connection.name.clone(),
            event_type: EventType::CredentialsShared,
            connection_id: connection.id.clone(),
            date: connection.last_interacted.clone(),
            credentials: history_credentials,
        });

        drop(state_guard);
        return Ok(AppState {
            connections,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            history,
            ..state
        });
    }

    Ok(state)
}

pub struct OID4VPClientMetadata {
    pub client_name: String,
    pub logo_uri: Option<String>,
    pub connection_url: String,
    pub client_id: String,
}

// TODO: move this functionality to the oid4vc-manager crate.
/// Returns (client_name, logo_uri, connection_url, client_id)
pub fn get_oid4vp_client_name_and_logo_uri(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> OID4VPClientMetadata {
    // Get the connection url from the redirect url host (or use the redirect url if it does not
    // contain a host).
    let redirect_uri = oid4vp_authorization_request.body.redirect_uri.clone();
    let connection_url = redirect_uri.host_str().unwrap_or(redirect_uri.as_str());

    let client_id = oid4vp_authorization_request.body.client_id.clone();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    match &oid4vp_authorization_request.body.extension.client_metadata {
        ClientMetadataResource::ClientMetadata {
            client_name,
            logo_uri,
            extension: _,
            other: _,
        } => {
            let client_name = client_name.as_ref().cloned().unwrap_or(connection_url.to_string());
            let logo_uri = logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            Some(OID4VPClientMetadata {
                client_name,
                logo_uri,
                connection_url: connection_url.to_string(),
                client_id: client_id.clone(),
            })
        }
        // TODO: support `client_metadata_uri`
        ClientMetadataResource::ClientMetadataUri(_) => None,
    }
    // Otherwise use the connection_url as the client_name.
    .unwrap_or(OID4VPClientMetadata {
        client_name: connection_url.to_string(),
        logo_uri: None,
        connection_url: connection_url.to_string(),
        client_id,
    })
}

#[test]
fn test_authorization_request_object() {
    let test_body: AuthorizationRequest<Object<OID4VP>> = serde_json::from_value(serde_json::json!({
        "client_id": "did:key:z6Mkm9yeuZK7inXBNjnNH3vAs9uUjqfy3mfNoKBKsKBrv8Tb",
        "redirect_uri": "https://example.com/",
        "state": null,
        "response_type": "vp_token",
        "dcql_query":
        {
      "credentials": [
        {
          "id": "CredentialQuery",
          "format": "jwt_vc_json",
          "meta": {
            "type_values": [
                ["VerifiableCredential"],
                ["PersonalInformation"],
            ]
          },
          "claims": [
              {"path": ["credentialSubject", "givenName"]},
              {"path": ["credentialSubject", "familyName"]},
              {"path": ["credentialSubject", "email"]},
              {"path": ["credentialSubject", "birthdate"]}
          ]
        }
      ]
    },
        "client_id_scheme": null,
        "response_mode": null,
        "scope": null,
        "nonce": "nonce",
        "client_metadata": {
          "vp_formats": {
            "jwt_vc_json": {
              "alg": [
                "EdDSA"
              ]
            }
          },
          "subject_syntax_types_supported": [
            "did:key"
          ]
        }
      }))
    .unwrap();
    println!("{}", test_body.to_string())
}
