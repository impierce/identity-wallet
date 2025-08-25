use crate::common::{assert_state_update::setup_state_file, json_example, test_managers};
use identity_wallet::state::credentials::reducers::refresh_credential_status::refresh_credential_status;
use identity_wallet::state::{actions::Action, AppState};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
#[serial_test::serial]
async fn test_refresh_credential_status() {
    // Spin up wiremock for the Status Provider
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ietf-oauth-token-status-list/0"))
        .respond_with(ResponseTemplate::new(200).set_body_string("eyJ0eXAiOiJzdGF0dXNsaXN0K2p3dCIsImFsZyI6IkVkRFNBIiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmFXUWlPaUpIVjBwcFJYWnhTaTFYUlZBM1RHcEpWa2wyVFd4SVNUaFlZV3BHTXpKRGNXWlpVRGQyVGpWZlVHMVZJaXdpYTNSNUlqb2lUMHRRSWl3aWVDSTZJa2xSWjNoWVRYbDRNR0pxTjFvdFVXRkxkazh5U0RWeWJVSkRSMlJPU0VWSFkwaEJSazFSWWtwTFZWVWlmUSMwIn0.eyJzdWIiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvaWV0Zi1vYXV0aC10b2tlbi1zdGF0dXMtbGlzdC8wIiwiaWF0IjoxNzU1NjEwMjcxLCJzdGF0dXNfbGlzdCI6eyJiaXRzIjoyLCJsc3QiOiJlTm9kbGNFS3hEQUlSQlgwYmlELWowSnlqNkRfX3lzN1hkakx0azBiWjk1TS1zWHRvcUdUYng5bVg2R1p0MlZXcld2SnlYVVhIbUl1N25pLVg1Tm1HTTltZTlaNlg4NFpwaE9oRXlWSGk5U2FwaS01N2h1SC12cG1ieUxhclIxNW1LNlFoaTJSSldwdTFudkZ3U2ZKempvM3VzUzJHRi04V3F2VGpidnZMajdDMGV0Um02aWtzQktkY3k4Zm5aMW5YZVY5dlpOTWxxNGx5VVlUMnhqMzFzZ01keTA1YjlmbDRIdGk2eUxWMks2cjdwRmxoUy1kMURkMEtXcWVoX2w3ek85NHhwTTBiUTZSZVJRWXJYZDhvazJYZmRQY0VjeHctbUJELS00SkttSWl2UzVXY1hibzYxdDZ0MGswdDBycFdHaW5SS214bl8zTXppWkxXVzJuWDhsVFlieTNraDRFanIyZlFLMXk3N3MydzRDbzdhVGJiV0tOWTU2NWhuMGVxR1RkYXhKems1LWwyTGh1SlFsZDcyRlppTkdDNFBkd3VtZnQzRnlWSGNzVE85OXBiN2JmbTNZWENkTWJmRmd4bmdUendCc3YtaDY4Rnk0bjVxanJXalVTU2Z1ZGhWMWVXMll6UHE5ejIyT3hsWWxSN3JmNnBRNjBQRmNiVzloNjhTQkZNQ1ZjZnVkZGVMVWJORUE5Mko0V0VEcTdJZDBSNnlYdmdrRU53VFZOeHJnZXBfTHkzbnFlTXR3NmZpWWxTVFpsdlhXUDdiRTVyMkNPd2xVUXI5TEJLd3lPNENIeVZkdnlQSkZ3eUFJZkRZcnpwdGhZRlRGNGRDTXhVS2ZCandzRlRHTFNPUXduYUNVVEl6Q2ljTzhCSUwtdjNTZ2RHei16cURieWhkV1lpZFlXNVZNUE9oVzRXaUdMSWZ3MDRyZ3ZNZ0Z4TTFrZDI3MVBqVzRuYklON3hFZ1RRMWpYczV5V1E0Mi0tTWd6WktqVXhaUDd5c1g4c0FMWkZocEZybmlkOGtzUWFmbGJxOWRDdUFzUlEwRDNtM1lINmhmUmt4ZmcwMTY0WmgtQXR4cEtlZkhUdTQ0TkJwa2JxY3dYWXIzQUhBQl9qZUdHemQwWXZlRTRKS3RRdWZrY2RrOURXVzlWWUJGVkJyNjhVaFQ4aVlCWloxTmstMTBGRWJScldaWUd6WnA1bjNmcllIck5qZFMxT1NPNGRKM2tIOFFXVjNmd3ZnRk5nYW12SDFBNE1URzlxRk83c01qVERzUnJvS2U4YkFTalFyc2NIbndhWk5leG0xS1NxTUtIUEp3TjN3YVNvUU1KTWlMN0NBUkMzMl9qcF94OEVZSDVhbFNkdDlSY0taVFZvMk9LM3NIazcyd0g4YmotY0h0ak5WSWl1U2Q1RjUyTFpybkx0NEFWUWFOR3JBTXNvUHE1Z3k3U2hHQ3g5T1BoQ0tHTHNIWUI3bHQtd09CRjN4M2pvbzFPLWFaQ3FhSmVZQzFnUElNdWd3b0ExUzdWbXJkaW9OeERFVmdWR2tXUUs3NWxSeG9kajk3ZE85RU5ncVRXYXhpS0xVRDViOFBILTlDQXRzamRLRDJOaUlTNWhKNTJkZzFVZ0theG9Sc2V6NVBILVIwZEtDR1lpc1NnSVdieU8zTlFCRUZlalhfN0pKcDdIQlF0OEF6TTdrT0ZhdUFTR3RKakdmUUJ5Z2pwZ2J1YlB0b0drVEhabjVOZzJkRzRIcWlNQWdoSTc0WURCRmJSaGd1SFVySGZDb1RMYXdnSDE4TFpRWnZ1eWdBMGFIcDBrRzdUcWU4ZnZqQWp1SVZEZzYweHBneWpDM3U1N3k4a2ozV1dScGFYRlk1Q29JcWV4UG1HYkVJSVJEckNNUGc1Q1gwS1o0N0RLdmpPT0JiUjFjc0w0UWRPaW82X3dCajVlUU5xQjRjb01sbGhnY1JEZ293Q2VzbVhFTXRDMkFfNlNZY1FZU0Y0YTJSZ2dBaXc3cFdPZHlDMGprSjZlQVNBVEozR3FRZWltNzllWE5EaHFlN0JYaXp6QndNTFUxYyJ9fQ.k0QLd3wpqijoznPNFcFYuUAq28SHCkY-naBwnCEkchTvJx7EhKHbyA2TkQqAQZQQT2QkVqTmGVoDeej36tayBg"))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Setting up the state and managers for the test.
    setup_state_file();
    let managers = test_managers(vec![]).await;
    let mut state = json_example::<AppState>("tests/fixtures/states/credential_with_status.json");
    state.core_utils.managers = managers;

    // Update the credential status URI to point to the mock server.
    state.credentials.get_mut(0).unwrap().data["credentialStatus/uri"] =
        serde_json::from_value(json!(mock_server.uri().to_string() + "/ietf-oauth-token-status-list/0")).unwrap();

    let action = json_example::<Action>("tests/fixtures/actions/refresh_credential_status.json");

    println!("2");
    refresh_credential_status(state, action).await.unwrap();

    // assert!(result.is_ok());
}

#[tokio::test]
#[serial_test::serial]
async fn test_refresh_all_credential_statuses() {
    setup_state_file();
}
