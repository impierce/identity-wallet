pub mod actions;
pub mod reducers;

use super::connections::ConnectionRequest;
use crate::stronghold::StrongholdManager;

use oid4vc::oid4vc_core::Subject;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

/// CoreUtils is a struct that contains all the utils that only the rustside needs to perform its tasks.
#[derive(Default)]
pub struct CoreUtils {
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,
    pub active_connection_request: Option<ConnectionRequest>,
}
/// Managers contains both the stronghold manager and the identity manager needed to perform operations on connections & credentials.
#[derive(Default)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

/// IdentityManager contains the subject, provider_manager and wallet needed to perform operations within the oid4vc library.
pub struct IdentityManager {
    pub subject: Arc<dyn Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}
