use crate::get_jwt_claims;
use crate::state::reducers::VERIFIABLE_CREDENTIAL;
use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect};
use crate::state::Profile;
use crate::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};
use crate::tests::{assert_state_update, setup_state_file, setup_stronghold};
use oid4vci::credential_format_profiles::CredentialFormats;

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile() {
    setup_state_file();
    setup_stronghold().await;

    let credential = VERIFIABLE_CREDENTIAL.clone();

    let credential_display = match credential {
        CredentialFormats::JwtVcJson(credential) => {
            let credential_display = serde_json::from_value::<identity_credential::credential::Credential>(
                get_jwt_claims(&credential.credential)["vc"].clone(),
            )
            .unwrap();
            credential_display
        }
        _ => unimplemented!(),
    };

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
            credentials: Some(vec![credential_display]),
            current_user_flow: Some(CurrentUserFlow::Redirect(Redirect {
                r#type: CurrentUserFlowType::Redirect,
                target: "profile".to_string(),
            })),
            ..TransferState::default()
        })],
    );
}
