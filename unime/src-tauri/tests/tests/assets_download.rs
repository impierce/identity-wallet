use identity_wallet::error::AppError;
use identity_wallet::persistence::{download_asset, ASSETS_DIR};
use tempfile::TempDir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
#[serial_test::serial]
async fn when_size_is_less_than_2_mb_then_download_should_start() {
    setup_empty_assets_dir();

    let mock_server = MockServer::start().await;

    // generate 1MB of random bytes
    let random_bytes: Vec<u8> = (0..(1_024 * 1_024)).map(|_| rand::random::<u8>()).collect();

    Mock::given(method("GET"))
        .and(path("/image.png"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(random_bytes, "image/png"))
        .expect(1)
        .mount(&mock_server)
        .await;

    assert!(
        download_asset(format!("{}/image.png", &mock_server.uri()).parse().unwrap(), "image")
            .await
            .is_ok()
    );
}

#[tokio::test]
#[serial_test::serial]
async fn when_size_is_bigger_than_2_mb_then_download_should_fail() {
    setup_empty_assets_dir();

    let mock_server = MockServer::start().await;

    // generate 3MB of random bytes
    let random_bytes: Vec<u8> = (0..(1_024 * 1_024 * 3)).map(|_| rand::random::<u8>()).collect();

    Mock::given(method("GET"))
        .and(path("/image.png"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(random_bytes, "image/png"))
        .expect(1)
        .mount(&mock_server)
        .await;

    assert!(
        download_asset(format!("{}/image.png", &mock_server.uri()).parse().unwrap(), "image")
            .await
            .is_err()
    );
}

#[tokio::test]
#[serial_test::serial]
async fn when_content_type_is_supported_then_download_should_start() {
    setup_empty_assets_dir();

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/image"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![], "image/svg+xml"))
        .expect(1)
        .mount(&mock_server)
        .await;

    assert!(
        download_asset(format!("{}/image", &mock_server.uri()).parse().unwrap(), "image")
            .await
            .is_ok()
    );
}

#[tokio::test]
#[serial_test::serial]
async fn when_content_type_is_not_supported_then_download_should_fail() {
    setup_empty_assets_dir();

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/image.png")) // file extension is ignored (even if it's supported), only content-type is checked
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![], "image/jpeg"))
        .expect(1)
        .mount(&mock_server)
        .await;

    assert!(matches!(
        download_asset(format!("{}/image.png", &mock_server.uri()).parse().unwrap(), "image").await,
        Err(AppError::DownloadAborted("content-type is not supported"))
    ));
}

fn setup_empty_assets_dir() {
    let path = TempDir::new().unwrap().into_path();
    *ASSETS_DIR.lock().unwrap() = path.as_os_str().into();
}
