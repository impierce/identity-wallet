use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use identity_wallet::get_jwt_claims;
use identity_wallet::state::reducers::load_dev_profile::{DRIVERS_LICENSE_CREDENTIAL, PERSONAL_INFORMATION};
use identity_wallet::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use identity_wallet::state::Profile;
use identity_wallet::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};
use oid4vci::credential_format_profiles::CredentialFormats;

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile() {
    setup_state_file();
    setup_stronghold();

    let (key1, personal_information) = PERSONAL_INFORMATION.clone();
    let credential_display1 = match personal_information {
        CredentialFormats::JwtVcJson(credential) => {
            let credential_display1 = serde_json::from_value::<identity_credential::credential::Credential>(
                get_jwt_claims(&credential.credential)["vc"].clone(),
            )
            .unwrap();
            credential_display1
        }
        _ => unimplemented!(),
    };

    let (key2, drivers_license_credential) = DRIVERS_LICENSE_CREDENTIAL.clone();
    let credential_display2 = match drivers_license_credential {
        CredentialFormats::JwtVcJson(credential) => {
            let credential_display2 = serde_json::from_value::<identity_credential::credential::Credential>(
                get_jwt_claims(&credential.credential)["vc"].clone(),
            )
            .unwrap();
            credential_display2
        }
        _ => unimplemented!(),
    };

    let credentials = {
        let mut credentials = vec![
            (key1.to_string(), credential_display1),
            (key2.to_string(), credential_display2),
        ];
        credentials.sort_by(|a, b| a.0.cmp(&b.0));
        credentials
    };

    assert_state_update(
        AppState::default(),
        vec![Action {
            r#type: ActionType::LoadDevProfile,
            payload: None,
        }],
        vec![Some(TransferState {
            active_profile: Some(Profile {
                display_name: "Ferris".to_string(),
                profile_picture: "&#129408".to_string(),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            }),
            credentials,
            current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
                r#type: CurrentUserPromptType::Redirect,
                target: "profile".to_string(),
            })),
            ..TransferState::default()
        })],
    );
}
