pub mod helpers;
pub mod history_event;

use rustls::RootCertStore;
use std::sync::Arc;
use url::Url;

use crate::command::Runtime;
use crate::state::did::{
    validate_domain_linkage::ValidationStatus, validate_linked_verifiable_presentations::LinkedVerifiableCredentialData,
};
use crate::state::user_prompt::{ClientMetadata, EcosystemProfile};
use crate::stronghold::StrongholdManager;
use crate::subject::Subject;
pub use helpers::DateUtils;

use oid4vc::{
    oid4vc_core::authorization_request::{AuthorizationRequest, Object},
    oid4vc_manager::ProviderManager,
    oid4vci::{credential_offer::CredentialOfferParameters, Wallet},
    oid4vp::oid4vp::OID4VP,
    siopv2::siopv2::SIOPv2,
};

pub fn tls_config() -> rustls::ClientConfig {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .expect("Failed to set default protocol versions")
        .with_root_certificates(root_store)
        .with_no_client_auth()
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
