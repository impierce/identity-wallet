use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use identity_wallet::state::reducers::load_dev_profile::{DRIVERS_LICENSE_CREDENTIAL, PERSONAL_INFORMATION};
use identity_wallet::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use identity_wallet::state::Profile;
use identity_wallet::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile() {
    setup_state_file();
    setup_stronghold();

    let personal_information = PERSONAL_INFORMATION.clone().display_credential;
    let drivers_license_credential = DRIVERS_LICENSE_CREDENTIAL.clone().display_credential;

    let credentials = {
        let mut credentials = vec![personal_information, drivers_license_credential];
        credentials.sort_by(|a, b| a.id.cmp(&b.id));
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
                name: "Ferris".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some("system".to_string()),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            }),
            credentials,
            current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
                r#type: CurrentUserPromptType::Redirect,
                target: "me".to_string(),
            })),
            ..TransferState::default()
        })],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile_twice() {
    setup_state_file();
    setup_stronghold();

    let personal_information = PERSONAL_INFORMATION.clone().display_credential;
    let drivers_license_credential = DRIVERS_LICENSE_CREDENTIAL.clone().display_credential;

    let credentials = {
        let mut credentials = vec![personal_information, drivers_license_credential];
        credentials.sort_by(|a, b| a.id.cmp(&b.id));
        credentials
    };

    assert_state_update(
        AppState::default(),
        vec![
            // Load the profile twice.
            Action {
                r#type: ActionType::LoadDevProfile,
                payload: None,
            },
            Action {
                r#type: ActionType::LoadDevProfile,
                payload: None,
            },
        ],
        vec![
            // Each time the dev profile is loaded, the state should be the same.
            Some(TransferState {
                active_profile: Some(Profile {
                    display_name: "Ferris Crabman".to_string(),
                    primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
                }),
                credentials: credentials.clone(),
                current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
                    r#type: CurrentUserPromptType::Redirect,
                    target: "me".to_string(),
                })),
                ..TransferState::default()
            }),
            Some(TransferState {
                active_profile: Some(Profile {
                    display_name: "Ferris Crabman".to_string(),
                    primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
                }),
                credentials,
                current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
                    r#type: CurrentUserPromptType::Redirect,
                    target: "me".to_string(),
                })),
                ..TransferState::default()
            }),
        ],
    )
    .await;
}
