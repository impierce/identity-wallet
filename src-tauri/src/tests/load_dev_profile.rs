use identity_credential::credential::Credential;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::state::reducers::VERIFIABLE_CREDENTIALS;
use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect};
use crate::state::Profile;
use crate::tests::assert_state_update;

use crate::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};

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
        vec![Action {
            r#type: ActionType::LoadDevProfile,
            payload: None,
        }],
        vec![Some(TransferState {
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
        })],
    );
}
