use reqwest::Client;
use tokio::sync::OnceCell;

use crate::state::core_utils::tls_config;

static HTTP_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub fn get_http_client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .use_preconfigured_tls(tls_config())
        .timeout(std::time::Duration::from_secs(2))
}

/// Returns a globally shared `reqwest::Client` configured with `webpki-roots` TLS.
pub async fn get_http_client() -> Client {
    HTTP_CLIENT
        .get_or_init(|| async {
            get_http_client_builder().build().unwrap_or_else(|err| {
                log::error!("Failed to build reqwest client: {err}");
                Client::new()
            })
        })
        .await
        .clone()
}
