use crate::common::assert_state_update::{assert_state_update, setup_assets_dir, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::dev_mode::reducers::load_dev_profile::load_dev_profile;
use identity_wallet::state::AppStateContainer;
use identity_wallet::state::{actions::Action, AppState};

use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_delete_credential() {
    setup_state_file();
    setup_stronghold();
    setup_assets_dir();

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/dev_load_profile.json");

    let state = load_dev_profile(state, action).await.unwrap();

    let expected_state = json_example::<AppState>("tests/fixtures/states/delete_credential_four_credentials.json");
    let action = json_example::<Action>("tests/fixtures/actions/delete_credential_personal_information.json");

    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}
