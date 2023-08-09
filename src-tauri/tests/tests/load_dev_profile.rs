use crate::common::assert_state_update::assert_state_update;
use identity_credential::credential::Credential;
use identity_wallet::state::reducers::VERIFIABLE_CREDENTIALS;
use identity_wallet::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect};
use identity_wallet::state::Profile;
use identity_wallet::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile() {
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    let credential_0_as_json = decode::<serde_json::Value>(&VERIFIABLE_CREDENTIALS.first().unwrap(), &key, &validation)
        .unwrap()
        .claims;
    let credential_0 = serde_json::from_value::<Credential>(credential_0_as_json.get("vc").unwrap().clone()).unwrap();

    assert_state_update(
        AppState::default(),
        Action {
            r#type: ActionType::LoadDevProfile,
            payload: None,
        },
        TransferState {
            active_profile: Some(Profile {
                display_name: "Ferris Crabman".to_string(),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            }),
            credentials: Some(vec![credential_0]),
            current_user_flow: Some(CurrentUserFlow::Redirect(Redirect {
                r#type: CurrentUserFlowType::Redirect,
                target: "profile".to_string(),
            })),
            ..TransferState::default()
        },
    );
}
