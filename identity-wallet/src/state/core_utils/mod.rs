pub mod helpers;
pub mod history_event;

use rustls::RootCertStore;
use std::sync::Arc;
use url::Url;
use zeroize::Zeroizing;

use crate::command::Runtime;
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

/// CoreUtils is a struct that contains all the utils that only the rustside needs to perform its tasks.
#[derive(Clone, Default, Debug)]
pub struct CoreUtils {
    pub app_handle: Option<tauri::AppHandle<Runtime>>,
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,
    pub active_flow: Option<ActiveFlow>,
}

/// The profile password, held for as long as the profile stays unlocked.
///
/// Automatic backups re-seal an archive with the same password the user typed to
/// unlock Stronghold, and every archive draws a fresh Argon2 salt, so a cached
/// derived key would not help — the password itself has to outlive the unlock
/// reducer.
///
/// It lives in [`CoreUtils`], which is `#[serde(skip)]`: it is never written to
/// disk alongside the state and never crosses the IPC boundary to the frontend.
/// Relocking runs `get_state`, which builds a fresh [`AppState`](crate::state::AppState)
/// and drops this along with the rest of `core_utils`.
///
/// `Debug` is written by hand on purpose. `AppState` derives `Debug` all the way
/// down to here, and a derived impl would print the password into the logs.
pub struct SessionPassword(Zeroizing<String>);

impl SessionPassword {
    pub fn new(password: impl Into<String>) -> Self {
        Self(Zeroizing::new(password.into()))
    }

    /// Named to make call sites read as a deliberate handling of a secret.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SessionPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SessionPassword(*****)")
    }
}

/// Managers contains both the stronghold manager and the identity manager needed to perform operations on connections & credentials.
#[derive(Default, Debug)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
    /// Set while the profile is unlocked, so an automatic backup can seal without
    /// prompting for the password a second time. `None` for profiles that were
    /// never unlocked through [`unlock_storage`](crate::state::common::reducers::unlock_storage),
    /// such as the dev-mode profiles.
    pub backup_password: Option<SessionPassword>,
}

/// IdentityManager contains the subject, provider_manager and wallet needed to perform operations within the oid4vc library.
#[derive(Debug)]
pub struct IdentityManager {
    pub subject: Arc<Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}
