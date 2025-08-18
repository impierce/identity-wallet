use identity_wallet::state::{credentials::reducers::refresh_credential_status::get_credential_status, AppState};
use oid4vc::{oid4vc_manager::ProviderManager, oid4vci::Wallet};
use serde_json::json;
use std::sync::Arc;

use crate::common::assert_state_update::{setup_state_file, setup_stronghold};

#[tokio::test]
#[serial_test::serial]
async fn test_get_credential_status() {
    setup_state_file();
    setup_stronghold();
    
    // load state and action from fixtures
    
    // spin up wiremock for the Status Provider
    let credential_status = json!({
        "id": "https://example.com/status/123",
        "type": "StatusList",
        "uri": "https://example.com/status-list.jwt",
        "idx": 0
    });

    // let result = get_credential_status(&credential_status, &identity_manager).await;
    assert!(result.is_ok());
}
