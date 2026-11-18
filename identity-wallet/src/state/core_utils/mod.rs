pub mod helpers;
pub mod history_event;

use rustls::RootCertStore;
use std::sync::Arc;
use url::Url;

use crate::command::Runtime;
use crate::state::connections::EcosystemsCheck;
use crate::state::did::{
    validate_domain_linkage::ValidationStatus, validate_linked_verifiable_presentations::LinkedVerifiableCredentialData,
};
use crate::state::user_prompt::{ClientMetadata, EcosystemProfile};
use crate::stronghold::StrongholdManager;
use crate::subject::Subject;
pub use helpers::DateUtils;

use oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use oid4vc_manager::ProviderManager;
use oid4vci::{credential_offer::CredentialOfferParameters, Wallet};
use oid4vp::oid4vp::OID4VP;
use siopv2::siopv2::SIOPv2;

pub fn tls_config() -> rustls::ClientConfig {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    // Debug builds only: test CAs for local test servers (`debug_trust_anchors`). A release build has no code
    // for it, so its roots stay the pinned `webpki-roots` (#746).
    #[cfg(debug_assertions)]
    debug_trust_anchors::add(&mut root_store);

    rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .expect("Failed to set default protocol versions")
        .with_root_certificates(root_store)
        .with_no_client_auth()
}

/// Extra trust anchors for debug builds: the PEM certificates in the file `UNIME_EXTRA_CA_CERTS` names, so a debug
/// build reaches local test servers (a conformance suite, a test harness) whose certificates come from a test CA.
/// Never part of a release build, and nothing changes when the variable is not set.
#[cfg(debug_assertions)]
pub mod debug_trust_anchors {
    use rustls::pki_types::{pem::PemObject, CertificateDer};
    use rustls::RootCertStore;
    use std::path::Path;

    /// The environment variable naming a PEM file of test CA certificates.
    pub const VARIABLE: &str = "UNIME_EXTRA_CA_CERTS";

    /// Adds the certificates of the file `UNIME_EXTRA_CA_CERTS` names, with a warning that says how many; a file that
    /// cannot be read, or that holds no certificate, is reported and adds nothing. The path is not logged: it can
    /// hold a user or project name.
    pub fn add(root_store: &mut RootCertStore) {
        let Ok(path) = std::env::var(VARIABLE) else { return };
        match add_file(root_store, Path::new(&path)) {
            Ok((added, ignored)) => log::warn!(
                "Debug build: trusting {added} extra CA certificate(s) from the file {VARIABLE} names{}",
                if ignored > 0 {
                    format!(" ({ignored} could not be used)")
                } else {
                    String::new()
                }
            ),
            Err(error) => log::error!("{VARIABLE}: {error}; no extra trust anchor is added"),
        }
    }

    /// Adds the certificates of the PEM file at `path`. Returns how many were added and how many could not be used.
    fn add_file(root_store: &mut RootCertStore, path: &Path) -> Result<(usize, usize), String> {
        let pem = std::fs::read(path).map_err(|error| format!("the file cannot be read ({error})"))?;
        let certificates = certificates(&pem)?;
        if certificates.is_empty() {
            return Err("no PEM certificate in the file".to_string());
        }
        Ok(root_store.add_parsable_certificates(certificates))
    }

    /// The certificates of a PEM text, in order.
    pub fn certificates(pem: &[u8]) -> Result<Vec<CertificateDer<'static>>, String> {
        CertificateDer::pem_slice_iter(pem)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("the PEM cannot be read ({error})"))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// A self-signed test CA (P-256), made with `openssl req -x509` for this test only.
        const TEST_CA: &str = include_str!("debug_trust_anchors_test_ca.pem");

        #[test]
        fn reads_the_certificates_of_a_pem_file() {
            let certificates = certificates(TEST_CA.as_bytes()).unwrap();
            assert_eq!(certificates.len(), 1);
            let mut store = RootCertStore::empty();
            assert_eq!(store.add_parsable_certificates(certificates), (1, 0));
        }

        #[test]
        fn a_file_adds_its_certificates_to_the_roots_already_there() {
            let mut store = RootCertStore::empty();
            store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            let built_in = store.len();
            let test_ca =
                Path::new(env!("CARGO_MANIFEST_DIR")).join("src/state/core_utils/debug_trust_anchors_test_ca.pem");

            assert_eq!(add_file(&mut store, &test_ca), Ok((1, 0)));
            assert_eq!(store.len(), built_in + 1);
        }

        #[test]
        fn a_file_that_is_missing_or_holds_no_certificate_adds_nothing() {
            let mut store = RootCertStore::empty();
            let directory = tempfile::tempdir().unwrap();

            let missing = directory.path().join("missing.pem");
            let unreadable = add_file(&mut store, &missing).unwrap_err();
            assert!(unreadable.contains("cannot be read"), "{unreadable}");

            let no_certificate = directory.path().join("empty.pem");
            std::fs::write(&no_certificate, "no certificate here").unwrap();
            let empty = add_file(&mut store, &no_certificate).unwrap_err();
            assert!(empty.contains("no PEM certificate"), "{empty}");

            assert!(store.is_empty());
            // What is logged of a failure names neither the file nor where it is.
            let directory = directory.path().to_string_lossy();
            for reason in [unreadable, empty] {
                assert!(!reason.contains(directory.as_ref()), "{reason}");
                assert!(!reason.contains(".pem"), "{reason}");
            }
        }

        #[test]
        fn a_pem_without_certificates_gives_none_and_a_broken_one_an_error() {
            assert!(certificates(b"no certificate here").unwrap().is_empty());
            assert!(certificates(b"-----BEGIN CERTIFICATE-----\n!!!\n-----END CERTIFICATE-----\n").is_err());
        }
    }
}

/// `Oid4vciStage` represents the different stages of the OID4VCI flow that can be active within the application.
#[derive(Clone, Debug)]
pub enum Oid4vciStage {
    OfferReceived,
    PreAuthorized,
    AuthorizationCode {
        code_verifier: Vec<u8>,
        wallet_state: String,
    },
    InteractiveAuthorization {
        code_verifier: Vec<u8>,
        wallet_state: String,
        authorization_request: Box<AuthorizationRequest<Object<OID4VP>>>,
        auth_session: Option<String>,
        interactive_authorization_endpoint: Url,
    },
}

/// `ActiveFlow` represents the different types of flows that can be active within the application.
#[derive(Clone, Debug)]
pub enum ActiveFlow {
    Siopv2 {
        authorization_request: Box<AuthorizationRequest<Object<SIOPv2>>>,
    },
    Oid4vp {
        authorization_request: Box<AuthorizationRequest<Object<OID4VP>>>,
        is_interactive: bool,
    },
    Oid4vciOffer {
        stage: Oid4vciStage,
        credential_offer: Box<CredentialOfferParameters>,
        logo_uri: Option<String>,
    },
}

/// `PendingConnectionData` contains the connection data collected to populate the AcceptConnection prompt.
/// However this data remains pending until the flow is completed successfully. Only then is the connection considered fully established,
/// and then it will be added to the state. This means the data has to remain pending throughout the entire active flow of whatever type.
/// This is actually useful since some info is at times needed elsewhere during the active flow as well.
#[derive(Clone, Debug)]
pub struct PendingConnectionData {
    pub client_metadata: ClientMetadata,
    pub domain_validation: Option<(ValidationStatus, String)>,
    pub linked_verifiable_presentations: Option<Vec<LinkedVerifiableCredentialData>>,
    pub ecosystems: Option<Vec<EcosystemProfile>>,
    pub ecosystems_check: Option<EcosystemsCheck>,
}

/// CoreUtils is a struct that contains all the utils that only the rustside needs to perform its tasks.
#[derive(Clone, Default, Debug)]
pub struct CoreUtils {
    pub app_handle: Option<tauri::AppHandle<Runtime>>,
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,
    pub active_flow: Option<ActiveFlow>,
    pub pending_connection_data: Option<PendingConnectionData>,
}

/// Managers contains both the stronghold manager and the identity manager needed to perform operations on connections & credentials.
#[derive(Default, Debug)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

/// IdentityManager contains the subject, provider_manager and wallet needed to perform operations within the oid4vc library.
#[derive(Debug)]
pub struct IdentityManager {
    pub subject: Arc<Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}
