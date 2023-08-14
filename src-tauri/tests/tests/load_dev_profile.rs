use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use identity_wallet::get_jwt_claims;
use identity_wallet::state::reducers::VERIFIABLE_CREDENTIAL;
use identity_wallet::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use identity_wallet::state::Profile;
use identity_wallet::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};
use oid4vci::credential_format_profiles::CredentialFormats;
use uuid::Uuid;

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile() {
    setup_state_file();
    setup_stronghold();

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
            credentials: Some(vec![(Uuid::default().to_string(), credential_display)]),
            current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
                r#type: CurrentUserPromptType::Redirect,
                target: "profile".to_string(),
            })),
            ..TransferState::default()
        })],
    );
}
