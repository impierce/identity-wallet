use identity_wallet::state::{
    actions::Action,
    core_utils::CoreUtils,
    credentials::reducers::self_issue_credential::self_issue_credential,
    profile_settings::{Profile, ProfileSettings},
    AppState, AppStateContainer,
};
use tokio::sync::Mutex;

use crate::common::{
    assert_state_update::{assert_state_update, setup_state_file, setup_stronghold},
    json_example, test_managers,
};

#[tokio::test]
#[serial_test::serial]
async fn test_self_issue_credential() {
    setup_state_file();
    setup_stronghold();

    // Set up AppState
    let mut app_state = AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]).await,
            ..Default::default()
        },
        profile_settings: ProfileSettings {
            profile: Some(Profile {
                name: "Ferris".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..AppState::default()
    };

    app_state.dids.insert(
        "did:jwk".to_string(),
        "did:example:ebfeb1f712ebc6f1c276e12ec21".to_string(),
    );

    let action = json_example::<Action>("tests/fixtures/actions/self_issue_profile.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/self_issued_profile.json");
    let result = self_issue_credential(app_state.clone(), action.clone()).await.unwrap();

    assert_state_update(
        AppStateContainer(Mutex::new(app_state)),
        vec![action],
        vec![Some(expected_state.clone())],
    )
    .await;

    // Assert Stronghold
    let managers = result.core_utils.managers.lock().await;
    let stronghold_manager = managers.stronghold_manager.as_ref().unwrap();

    let stronghold_values = stronghold_manager.values().unwrap().unwrap();
    let stronghold_value = stronghold_values.first().unwrap().clone();

    assert_eq!(stronghold_value.display_credential, result.credentials[0]);
}
