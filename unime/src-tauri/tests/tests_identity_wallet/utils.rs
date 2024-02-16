use identity_wallet::persistence::{download_asset, LogoType};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn when_size_is_less_than_2mb_then_download_should_start() {
    let mock_server = MockServer::start().await;

    // generate 1MB of random bytes
    let random_bytes: Vec<u8> = (0..(1_024 * 1_024)).map(|_| rand::random::<u8>()).collect();

    Mock::given(method("GET"))
        .and(path("/image.png"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(random_bytes, "image/png"))
        .expect(1)
        .mount(&mock_server)
        .await;

    assert!(download_asset(
        format!("{}/image.png", &mock_server.uri()).parse().unwrap(),
        LogoType::CredentialLogo,
        0
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn when_size_is_bigger_than_2mb_then_download_should_fail() {
    let mock_server = MockServer::start().await;

    // generate 3MB of random bytes
    let random_bytes: Vec<u8> = (0..(1_024 * 1_024 * 3)).map(|_| rand::random::<u8>()).collect();

    Mock::given(method("GET"))
        .and(path("/image.png"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(random_bytes, "image/png"))
        .expect(1)
        .mount(&mock_server)
        .await;

    assert!(download_asset(
        format!("{}/image.png", &mock_server.uri()).parse().unwrap(),
        LogoType::CredentialLogo,
        0
    )
    .await
    .is_err());
}

#[tokio::test]
async fn when_mime_type_is_supported_then_download_should_start() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/image.svg"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![], "image/svg+xml"))
        .expect(1)
        .mount(&mock_server)
        .await;

    assert!(download_asset(
        format!("{}/image.svg", &mock_server.uri()).parse().unwrap(),
        LogoType::CredentialLogo,
        0
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn when_mime_type_is_not_supported_then_download_should_fail() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/image.jpeg"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![], "image/jpeg"))
        .expect(0)
        .mount(&mock_server)
        .await;

    assert!(download_asset(
        format!("{}/image.jpeg", &mock_server.uri()).parse().unwrap(),
        LogoType::CredentialLogo,
        0
    )
    .await
    .is_err());
}
