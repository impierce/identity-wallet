use std::sync::Mutex;

use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::{test_managers, TEST_PASSWORD};
use identity_wallet::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, PasswordRequired, Redirect};
use identity_wallet::state::Profile;
use identity_wallet::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fs::File;
use std::path::Path;

fn json_example<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("file does not exist");
    serde_json::from_reader::<_, T>(file).expect("could not parse json")
}

#[tokio::test]
#[serial_test::serial]
async fn pretty_print_test() {
    let t_state = TransferState {
        current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
            r#type: CurrentUserPromptType::Redirect,
            target: "welcome".to_string(),
        })),
        ..TransferState::default()
    };
    let s =  serde_json::to_string_pretty(&t_state).unwrap();
    println!("{}", s);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_create_new() {
    setup_state_file();
    setup_stronghold();

    let transstate2 = json_example::<TransferState>("tests/tests/states/get_state.json");
    assert_state_update(
        // Initial state.
        AppState::default(),
        vec![
            // Get the initial state.
            Action {
                r#type: ActionType::GetState,
                payload: None,
            },
            // Create a new profile.
            Action {
                r#type: ActionType::CreateNew,
                payload: Some(json!({
                    "name": "Ferris Crabman",
                    "picture": "&#129408",
                    "theme": "system",
                    "password": TEST_PASSWORD
                })),
            },
        ],
        vec![
            // There is no profile yet, so the user is redirected to the welcome page.
            Some(TransferState {
                current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
                    r#type: CurrentUserPromptType::Redirect,
                    target: "welcome".to_string(),
                })),
                ..TransferState::default()
            }),
            // The profile was created, so the user is redirected to the profile page.
            Some(transstate2
            ),
        ],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_unlock_storage() {
    setup_state_file();
    setup_stronghold();

    assert_state_update(
        // Initial state.
        AppState {
            managers: test_managers(vec![]),
            active_profile: Mutex::new(Some(Profile {
                name: "Ferris Crabman".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some("system".to_string()),
                primary_did: "did:example:placeholder".to_string(),
            })),
            ..AppState::default()
        },
        vec![
            // Get the initial state.
            Action {
                r#type: ActionType::GetState,
                payload: None,
            },
            // Unlock the storage.
            Action {
                r#type: ActionType::UnlockStorage,
                payload: Some(json!({ "password": TEST_PASSWORD })),
            },
        ],
        vec![
            // The storage is locked, so the user is prompted to unlock it.
            Some(TransferState {
                active_profile: Some(Profile {
                    name: "Ferris Crabman".to_string(),
                    picture: Some("&#129408".to_string()),
                    theme: Some("system".to_string()),
                    primary_did: "did:example:placeholder".to_string(),
                }),
                current_user_prompt: Some(CurrentUserPrompt::PasswordRequired(PasswordRequired {
                    r#type: CurrentUserPromptType::PasswordRequired,
                })),
                ..TransferState::default()
            }),
            // The storage is unlocked, so the user is redirected to the profile page.
            Some(TransferState {
                active_profile: Some(Profile {
                    name: "Ferris Crabman".to_string(),
                    picture: Some("&#129408".to_string()),
                    theme: Some("system".to_string()),
                    primary_did: "did:example:placeholder".to_string(),
                }),
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
