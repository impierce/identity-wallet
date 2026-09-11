use std::collections::HashMap;

use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        connections::actions::connection_accepted::ConnectionAccepted,
        core_utils::{helpers::download_logo, ActiveFlow},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::{debug, info, warn};
use oid4vc::oid4vci::credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject;

/// Sets the `CredentialOffer` prompt after the `AcceptConnetion` prompt was accepted, triggering the `ConnectionAccepted` action.
/// Accepting the prompt set in this reducer would result in the `CredentialOffersSelected` action, which is handled by `handle_credential_offer`.
pub async fn read_credential_offer(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(_connection_accepted) = listen::<ConnectionAccepted>(action) {
        info!("read oid4vci credential offer");

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
    } else {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::{hash, ASSETS_DIR, STRONGHOLD};
    use crate::state::connections::actions::connection_accepted::ConnectionAccepted;
    use crate::state::core_utils::{CoreUtils, IdentityManager, Managers, Oid4vciStage};
    use crate::state::did::validate_domain_linkage::{ValidationResult, ValidationStatus};
    use crate::state::user_prompt::ClientMetadata;
    use crate::state::{SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
    use crate::stronghold::StrongholdManager;
    use crate::subject::subject;

    use oid4vc::oid4vc_manager::ProviderManager;
    use oid4vc::oid4vci::credential_format_profiles::{
        w3c_verifiable_credentials::jwt_vc_json, CredentialFormats, Parameters, WithParameters,
    };
    use oid4vc::oid4vci::credential_issuer::credential_configurations_supported::{
        CredentialConfigurationsSupportedDisplay, CredentialMetadata, Logo,
    };
    use oid4vc::oid4vci::credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata;
    use oid4vc::oid4vci::credential_offer::{
        CredentialConfigurationIds, CredentialOfferParameters, Grants, InputMode, PreAuthorizedCode, TxCodeConstraints,
    };
    use oid4vc::oid4vci::Wallet;

    use std::sync::Arc;
    use tempfile::{NamedTempFile, TempDir};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const OFFERED_CONFIGURATION_ID: &str = "UniversityDegreeCredential";
    const UNOFFERED_CONFIGURATION_ID: &str = "DriversLicenseCredential";
    const CLIENT_NAME: &str = "University";
    const CLIENT_LOGO_URI: &str = "https://example.com/issuer.png";

    async fn test_managers() -> Arc<tauri::async_runtime::Mutex<Managers>> {
        let stronghold_path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = stronghold_path.as_os_str().into();

        let password = "sup3rSecr3t".to_string();
        let stronghold_manager = Arc::new(StrongholdManager::create(&password).unwrap());
        let subject = subject(stronghold_manager.clone(), password).await;

        let provider_manager = ProviderManager::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .unwrap();
        let wallet = Wallet::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .unwrap();

        Arc::new(tauri::async_runtime::Mutex::new(Managers {
            stronghold_manager: Some(stronghold_manager),
            identity_manager: Some(IdentityManager {
                subject,
                provider_manager,
                wallet,
            }),
        }))
    }

    fn credential_configuration(logo_uri: Option<&str>) -> CredentialConfigurationsSupportedObject {
        CredentialConfigurationsSupportedObject {
            credential_format: CredentialFormats::<WithParameters>::JwtVcJson(Parameters {
                parameters: (jwt_vc_json::CredentialDefinition {
                    type_: vec!["VerifiableCredential".to_string(), OFFERED_CONFIGURATION_ID.to_string()],
                })
                .into(),
            }),
            credential_metadata: Some(CredentialMetadata {
                display: Some(vec![CredentialConfigurationsSupportedDisplay {
                    name: "University Credential".to_string(),
                    locale: Some("en-US".to_string()),
                    logo: logo_uri.map(|uri| Logo {
                        uri: uri.parse().unwrap(),
                        alt_text: None,
                    }),
                    description: None,
                    background_image: None,
                    background_color: None,
                    text_color: None,
                }]),
                claims: Default::default(),
            }),
            ..Default::default()
        }
    }

    /// Also advertises a configuration that is not part of the offer, so tests can assert it is filtered out.
    async fn mount_issuer_metadata(mock_server: &MockServer, credential_logo_uri: Option<&str>) {
        Mock::given(method("GET"))
            .and(path("/.well-known/openid-credential-issuer"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(CredentialIssuerMetadata {
                    credential_issuer: mock_server.uri().parse().unwrap(),
                    credential_endpoint: format!("{}/credential", mock_server.uri()).parse().unwrap(),
                    credential_configurations_supported: [
                        (
                            OFFERED_CONFIGURATION_ID.to_string(),
                            credential_configuration(credential_logo_uri),
                        ),
                        (UNOFFERED_CONFIGURATION_ID.to_string(), credential_configuration(None)),
                    ]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }),
            )
            .expect(1)
            .mount(mock_server)
            .await;
    }

    async fn mount_credential_logo(mock_server: &MockServer) {
        Mock::given(method("GET"))
            .and(path("/logo/credential.svg"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                br#"<svg xmlns="http://www.w3.org/2000/svg"/>"#.to_vec(),
                "image/svg+xml",
            ))
            .expect(1)
            .mount(mock_server)
            .await;
    }

    fn accept_connection_prompt(connection_url: &str) -> CurrentUserPrompt {
        CurrentUserPrompt::AcceptConnection {
            client_metadata: ClientMetadata {
                client_name: CLIENT_NAME.to_string(),
                logo_uri: Some(CLIENT_LOGO_URI.to_string()),
                connection_url: connection_url.to_string(),
                redirect_uri: None,
                client_id: "did:example:123".parse().unwrap(),
            },
            connection_data: None,
            domain_validation: Box::new(ValidationResult {
                status: ValidationStatus::Unknown,
                url: connection_url.parse().unwrap(),
                name: None,
                logo_uri: None,
                issuance_date: None,
                message: None,
            }),
            linked_verifiable_presentations: None,
            ecosystems: None,
        }
    }

    fn pre_authorized_grants(tx_code: Option<TxCodeConstraints>) -> Grants {
        Grants {
            authorization_code: None,
            pre_authorized_code: Some(PreAuthorizedCode {
                pre_authorized_code: "pre-authorized-code".to_string(),
                tx_code,
                interval: None,
                authorization_server: None,
            }),
        }
    }

    fn oid4vci_state(
        managers: Arc<tauri::async_runtime::Mutex<Managers>>,
        credential_issuer: &str,
        current_user_prompt: Option<CurrentUserPrompt>,
        grants: Option<Grants>,
    ) -> AppState {
        AppState {
            core_utils: CoreUtils {
                managers,
                active_flow: Some(ActiveFlow::Oid4vciOffer {
                    stage: Oid4vciStage::OfferReceived,
                    credential_offer: Box::new(CredentialOfferParameters {
                        credential_issuer: credential_issuer.parse().unwrap(),
                        credential_configuration_ids: CredentialConfigurationIds::try_new(vec![
                            OFFERED_CONFIGURATION_ID.to_string(),
                        ])
                        .unwrap(),
                        grants,
                    }),
                    logo_uri: None,
                }),
                ..Default::default()
            },
            current_user_prompt,
            ..Default::default()
        }
    }

    /// Logos are downloaded into `assets/tmp` under the hash of their URI.
    fn downloaded_logo_path(logo_uri: &str) -> std::path::PathBuf {
        ASSETS_DIR
            .lock()
            .unwrap()
            .join("tmp")
            .join(format!("{}.svg", hash(logo_uri)))
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn credential_offer_prompt_is_built_from_the_offer_and_the_issuer_metadata() {
        *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().keep();

        let mock_server = MockServer::start().await;
        let credential_logo_uri = format!("{}/logo/credential.svg", mock_server.uri());

        mount_issuer_metadata(&mock_server, Some(&credential_logo_uri)).await;
        mount_credential_logo(&mock_server).await;

        let tx_code = TxCodeConstraints {
            input_mode: Some(InputMode::Numeric),
            length: Some(6),
            description: None,
        };

        let state = oid4vci_state(
            test_managers().await,
            &mock_server.uri(),
            Some(accept_connection_prompt(&mock_server.uri())),
            Some(pre_authorized_grants(Some(tx_code.clone()))),
        );

        let result = read_credential_offer(state, Arc::new(ConnectionAccepted))
            .await
            .unwrap();

        let Some(CurrentUserPrompt::CredentialOffer {
            issuer_name,
            logo_uri,
            credential_configurations,
            tx_code: prompt_tx_code,
        }) = result.current_user_prompt
        else {
            panic!(
                "expected a `CredentialOffer` prompt, got: {:?}",
                result.current_user_prompt
            );
        };

        // The display data is carried over from the accepted connection, not re-derived from the issuer metadata.
        assert_eq!(issuer_name, CLIENT_NAME);
        assert_eq!(logo_uri.as_deref(), Some(CLIENT_LOGO_URI));

        // Only the offered configuration is kept, even though the issuer advertises more.
        assert_eq!(credential_configurations.len(), 1);
        assert!(credential_configurations.contains_key(OFFERED_CONFIGURATION_ID));

        assert_eq!(prompt_tx_code, Some(tx_code));
        assert!(downloaded_logo_path(&credential_logo_uri).exists());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn no_logo_is_downloaded_when_the_metadata_contains_none() {
        *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().keep();

        let mock_server = MockServer::start().await;
        mount_issuer_metadata(&mock_server, None).await;

        let state = oid4vci_state(
            test_managers().await,
            &mock_server.uri(),
            Some(accept_connection_prompt(&mock_server.uri())),
            None,
        );

        let result = read_credential_offer(state, Arc::new(ConnectionAccepted))
            .await
            .unwrap();

        // The `assets/tmp` folder is only created by a download, so its absence proves none was attempted.
        assert!(!ASSETS_DIR.lock().unwrap().join("tmp").exists());

        let Some(CurrentUserPrompt::CredentialOffer { tx_code, .. }) = result.current_user_prompt else {
            panic!("expected a `CredentialOffer` prompt");
        };
        assert_eq!(tx_code, None);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn state_is_unchanged_when_the_connection_was_not_accepted() {
        *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().keep();

        let mock_server = MockServer::start().await;
        mount_issuer_metadata(&mock_server, None).await;

        let state = oid4vci_state(test_managers().await, &mock_server.uri(), None, None);

        let result = read_credential_offer(state, Arc::new(ConnectionAccepted))
            .await
            .unwrap();

        assert!(result.current_user_prompt.is_none());
    }

    #[tokio::test]
    async fn state_is_unchanged_when_the_active_flow_is_not_oid4vci() {
        let result = read_credential_offer(AppState::default(), Arc::new(ConnectionAccepted))
            .await
            .unwrap();

        assert!(result.current_user_prompt.is_none());
    }
}
