use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState, AppStateContainer};
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me_query.json");
    let action = json_example::<Action>("tests/fixtures/actions/new_credential_sort.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_sort.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sorting() {
    todo!()
}

//   left: [DisplayCredential { id: "39373933-3863-3339-3864-646234373631", issuer_name: "Government", format: JwtVcJson(Profile { format: JwtVcJson }), data: Object {"@context": Array [String("https://www.w3.org/2018/credentials/v1"), String("https://www.w3.org/2018/credentials/examples/v1")], "type": Array [String("VerifiableCredential"), String("PersonalInformation")], "issuanceDate": String("2022-01-01T00:00:00Z"), "issuer": String("http://192.168.1.127:9090/"), "credentialSubject": Object {"id": String("did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY"), "givenName": String("Ferris"), "familyName": String("Crabman"), "email": String("ferris.crabman@crabmail.com"), "birthdate": String("1985-05-21")}}, metadata: CredentialMetadata { is_favorite: false, date_added: "", date_issued: "" }, display_name: "", display_color: None, display_icon: None }, DisplayCredential { id: "39383134-6538-3766-3963-303366323930", issuer_name: "Driver License Organisation", format: JwtVcJson(Profile { format: JwtVcJson }), data: Object {"@context": Array [String("https://www.w3.org/2018/credentials/v1"), String("https://www.w3.org/2018/credentials/examples/v1")], "type": Array [String("VerifiableCredential"), String("DriverLicenseCredential")], "issuer": String("http://192.168.1.127:9090/"), "issuanceDate": String("2022-08-15T09:30:00Z"), "expirationDate": String("2027-08-15T23:59:59Z"), "credentialSubject": Object {"id": String("did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY"), "licenseClass": String("Class C"), "issuedBy": String("California"), "validity": String("Valid")}}, metadata: CredentialMetadata { is_favorite: false, date_added: "", date_issued: "" }, display_name: "", display_color: None, display_icon: None }]
//  right: [DisplayCredential { id: "39383134-6538-3766-3963-303366323930", issuer_name: "Driver License Organisation", format: JwtVcJson(Profile { format: JwtVcJson }), data: Object {"@context": Array [String("https://www.w3.org/2018/credentials/v1"), String("https://www.w3.org/2018/credentials/examples/v1")], "type": Array [String("VerifiableCredential"), String("DriverLicenseCredential")], "issuer": String("http://192.168.1.127:9090/"), "issuanceDate": String("2022-08-15T09:30:00Z"), "expirationDate": String("2027-08-15T23:59:59Z"), "credentialSubject": Object {"id": String("did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY"), "licenseClass": String("Class C"), "issuedBy": String("California"), "validity": String("Valid")}}, metadata: CredentialMetadata { is_favorite: false, date_added: "", date_issued: "" }, display_name: "", display_color: None, display_icon: None }, DisplayCredential { id: "39373933-3863-3339-3864-646234373631", issuer_name: "Government", format: JwtVcJson(Profile { format: JwtVcJson }), data: Object {"@context": Array [String("https://www.w3.org/2018/credentials/v1"), String("https://www.w3.org/2018/credentials/examples/v1")], "type": Array [String("VerifiableCredential"), String("PersonalInformation")], "issuanceDate": String("2022-01-01T00:00:00Z"), "issuer": String("http://192.168.1.127:9090/"), "credentialSubject": Object {"id": String("did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY"), "givenName": String("Ferris"), "familyName": String("Crabman"), "email": String("ferris.crabman@crabmail.com"), "birthdate": String("1985-05-21")}}, metadata: CredentialMetadata { is_favorite: false, date_added: "", date_issued: "" }, display_name: "", display_color: None, display_icon: None }]
