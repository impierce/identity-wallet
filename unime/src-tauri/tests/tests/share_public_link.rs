use identity_wallet::state::{
    actions::Action,
    core_utils::CoreUtils,
    credentials::{reducers::share_to_linkedin::share_to_linkedin, VerifiableCredentialRecord},
    profile_settings::{Profile, ProfileSettings},
    AppState, AppStateContainer,
};
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::common::{
    assert_state_update::{assert_state_update, setup_state_file, setup_stronghold},
    json_example, test_managers,
};

const CREDENTIAL_JWT: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2dFODROQ01wTWVBeDlqSzljZjVXNEc4Z2NaOXh1d0p2RzFlN3dOazhLQ2d0I3o2TWtnRTg0TkNNcE1lQXg5aks5Y2Y1VzRHOGdjWjl4dXdKdkcxZTd3Tms4S0NndCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWtnRTg0TkNNcE1lQXg5aks5Y2Y1VzRHOGdjWjl4dXdKdkcxZTd3Tms4S0NndCIsInN1YiI6ImRpZDprZXk6ejZNa2lpZXlvTE1TVnNKQVp2N0pqZTV3V1NrREV5bVVna3lGOGtiY3JqWnBYM3FkIiwianRpIjoiZjQ3YWMxMGItNThjYy00MzcyLWE1NjctMGUwMmIyYzNkNDc5IiwibmJmIjoxMjYyMzA0MDAwLCJpYXQiOjEyNjIzMDQwMDAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIl0sInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6a2V5Ono2TWtpaWV5b0xNU1ZzSkFadjdKamU1d1dTa0RFeW1VZ2t5RjhrYmNyalpwWDNxZCIsImZpcnN0X25hbWUiOiJGZXJyaXMiLCJsYXN0X25hbWUiOiJSdXN0YWNlYW4ifSwiaXNzdWVyIjoiZGlkOmtleTp6Nk1rZ0U4NE5DTXBNZUF4OWpLOWNmNVc0RzhnY1o5eHV3SnZHMWU3d05rOEtDZ3QiLCJpc3N1YW5jZURhdGUiOiIyMDEwLTAxLTAxVDAwOjAwOjAwWiIsImNyZWRlbnRpYWxTdGF0dXMiOnsiaWQiOiJodHRwczovL215LWRvbWFpbi5leGFtcGxlLm9yZy9pZXRmLW9hdXRoLXRva2VuLXN0YXR1cy1saXN0LzAiLCJ0eXBlIjoic3RhdHVzbGlzdCtqd3QiLCJpZHgiOjEyMywidXJpIjoiaHR0cHM6Ly9teS1kb21haW4uZXhhbXBsZS5vcmcvaWV0Zi1vYXV0aC10b2tlbi1zdGF0dXMtbGlzdC8wIn19LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsiaWR4IjoxMjMsInVyaSI6Imh0dHBzOi8vbXktZG9tYWluLmV4YW1wbGUub3JnL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCJ9fX0.LpNq8l-qqqCA-htsB8KZLaVoNCfxqTrsPxVmEj0dsPAGFhOqO8lXI7DU0FhNwzWedxJ1ySS_Vq7ChBW-TgY7Bw";

#[tokio::test]
#[serial_test::serial]
async fn test_share_public_link() {
    // Set up AppState
    setup_state_file();

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

    // Set up stronghold
    setup_stronghold();
    let stronghold_manager = app_state
        .core_utils
        .managers
        .lock()
        .await
        .stronghold_manager
        .as_ref()
        .unwrap()
        .clone();

    // Create the VerifiableCredentialRecord from the JWT
    let credential_jwt_value = serde_json::to_value(CREDENTIAL_JWT).unwrap();
    let mut vrc =
        VerifiableCredentialRecord::try_new(CredentialFormats::JwtVcJson(()), credential_jwt_value, Vec::new())
            .unwrap();

    // Replace the key/id with the test Uuid key from the action fixture
    let key = Uuid::parse_str("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
    vrc.display_credential.id = key.clone().to_string();

    // Insert the VerifiableCredentialRecord into Stronghold
    stronghold_manager
        .insert(key, serde_json::json!(vrc).to_string().as_bytes().to_vec())
        .unwrap();

    // Insert the DisplayCredential into the app state
    app_state.credentials.push(vrc.display_credential);

    let action = json_example::<Action>("tests/fixtures/actions/share_to_linkedin.json");
    let result = share_to_linkedin(app_state.clone(), action.clone()).await.unwrap();

    println!("Result: {:#?}", result);

    // TODO!!

    let expected_state = json_example::<AppState>("tests/fixtures/states/share_to_linkedin.json");

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
