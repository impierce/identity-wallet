use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::reducers::load_dev_profile::{DRIVERS_LICENSE_CREDENTIAL, PERSONAL_INFORMATION};
use identity_wallet::state::{
    actions::Action,
    AppState, TransferState,
};

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile() {
    setup_state_file();
    setup_stronghold();

    // let personal_information = PERSONAL_INFORMATION.clone().display_credential;
    // let drivers_license_credential = DRIVERS_LICENSE_CREDENTIAL.clone().display_credential;

    // let credentials = {
    //     let mut credentials = vec![personal_information, drivers_license_credential];
    //     credentials.sort_by(|a, b| a.id.cmp(&b.id));
    //     credentials
    // };

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state = json_example::<TransferState>("tests/tests/fixtures-load_dev_profile/states/load_dev_profile.json");
    let action = json_example::<Action>("tests/tests/fixtures-load_dev_profile/actions/load_dev_profile.json");
    assert_state_update(
        AppState::default(),
        vec![action],
        vec![Some(state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile_twice() {
    setup_state_file();
    setup_stronghold();

    // let personal_information = PERSONAL_INFORMATION.clone().display_credential;
    // let drivers_license_credential = DRIVERS_LICENSE_CREDENTIAL.clone().display_credential;

    // let credentials = {
    //     let mut credentials = vec![personal_information, drivers_license_credential];
    //     credentials.sort_by(|a, b| a.id.cmp(&b.id));
    //     credentials
    // };

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state = json_example::<TransferState>("tests/tests/fixtures-load_dev_profile/states/load_dev_profile.json");
    let action = json_example::<Action>("tests/tests/fixtures-load_dev_profile/actions/load_dev_profile.json");
    assert_state_update(
        AppState::default(),
        vec![
            // Load the profile twice.
            action.clone(),
            action
        ],
        vec![
            // Each time the dev profile is loaded, the state should be the same.
            Some(state.clone()),
            Some(state),
        ],
    )
    .await;
}
