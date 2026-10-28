use std::net::IpAddr;

use async_trait::async_trait;
use openid_federation::{FederationError, FederationResult, HttpClient};
use reqwest::{Client, Url};
use tokio::sync::OnceCell;

use crate::error::AppError;
use crate::state::core_utils::tls_config;

static HTTP_CLIENT: OnceCell<Client> = OnceCell::const_new();
static PUBLIC_DESTINATION_HTTP_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub fn get_http_client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .use_preconfigured_tls(tls_config())
        .timeout(std::time::Duration::from_secs(2))
}

/// Returns a globally shared `reqwest::Client` configured with `webpki-roots` TLS.
pub async fn get_http_client() -> Client {
    HTTP_CLIENT
        .get_or_init(|| async {
            // No `Client::new()` fallback: it drops the pinned `webpki-roots` anchors (#746).
            get_http_client_builder()
                .build()
                .expect("HTTP client with pinned roots could not be built")
        })
        .await
        .clone()
}

/// Returns a globally shared `reqwest::Client` for requests to remote-supplied URLs, see
/// [`fetch_public_destination`].
///
/// Follows no redirects: [`fetch_public_destination`] follows them by hand so every hop passes
/// [`assert_public_destination`]. A default client would follow ten of them unchecked.
async fn get_public_destination_http_client() -> Result<Client, AppError> {
    PUBLIC_DESTINATION_HTTP_CLIENT
        .get_or_try_init(|| async {
            get_http_client_builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .map_err(|err| {
                    log::error!("Failed to build reqwest client for remote-supplied URLs: {err}");
                    AppError::DownloadAborted("HTTP client for remote-supplied URLs could not be built")
                })
        })
        .await
        .cloned()
}

const MAX_REDIRECTS: usize = 5;

/// Requests a remote-supplied `url`, following redirects by hand so each hop passes
/// [`assert_public_destination`] first. `reqwest`'s redirect policy is synchronous and cannot resolve a host.
pub async fn fetch_public_destination(mut url: Url) -> Result<reqwest::Response, AppError> {
    let client = get_public_destination_http_client().await?;

    for _ in 0..=MAX_REDIRECTS {
        assert_public_destination(&url).await?;

        let response = client.get(url.clone()).send().await?;
        if !response.status().is_redirection() {
            return Ok(response);
        }

        let location = response
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|location| location.to_str().ok())
            .ok_or(AppError::DownloadAborted("redirect without a usable location"))?;

        url = url
            .join(location)
            .map_err(|_| AppError::DownloadAborted("redirect to an unparseable location"))?;
    }

    Err(AppError::DownloadAborted("too many redirects"))
}

/// An `openid-federation` HTTP client that only requests public destinations, see [`fetch_public_destination`].
///
/// Entity identifiers, authority hints and federation endpoints all come from remote entity configurations, and the
/// wallet requests them while it builds a connection prompt, before the user has accepted anything.
pub struct PublicDestinationHttpClient;

#[async_trait]
impl HttpClient for PublicDestinationHttpClient {
    async fn fetch_text(&self, url: Url) -> FederationResult<String> {
        let response = fetch_public_destination(url.clone()).await.map_err(|err| match err {
            AppError::DownloadFailed(err) => FederationError::Http(err),
            AppError::DownloadAborted(reason) => {
                FederationError::EntityResolution(format!("Refused to fetch {url}: {reason}"))
            }
            err => FederationError::EntityResolution(format!("Failed to fetch {url}: {err}")),
        })?;

        if !response.status().is_success() {
            return Err(FederationError::EntityResolution(format!(
                "Failed to fetch response from {url}: HTTP {}",
                response.status()
            )));
        }

        response.text().await.map_err(FederationError::from)
    }
}

/// Allowed in debug builds so the app and its tests can reach a local issuer or `wiremock`.
fn private_destinations_allowed() -> bool {
    cfg!(debug_assertions)
}

/// Rejects URLs a remote party should not be able to make the wallet request: anything not
/// `http(s)`, and anything resolving to a non-routable address (CWE-918).
///
/// Resolving here cannot bind the address the socket ends up on, so this removes the trivial cases
/// rather than closing the race.
pub async fn assert_public_destination(url: &Url) -> Result<(), AppError> {
    if !matches!(url.scheme(), "http" | "https") {
        return Err(AppError::DownloadAborted("URL scheme is not http(s)"));
    }

    if private_destinations_allowed() {
        return Ok(());
    }

    let host = url
        .host_str()
        .ok_or(AppError::DownloadAborted("URL has no host to resolve"))?;
    let port = url.port_or_known_default().unwrap_or(443);

    let addresses = tokio::net::lookup_host((host, port))
        .await
        .map_err(|_| AppError::DownloadAborted("URL host could not be resolved"))?;

    let mut resolved = false;
    for address in addresses {
        resolved = true;
        if !is_public(&address.ip()) {
            log::warn!("Refusing to request {url}: {} is not a public address", address.ip());
            return Err(AppError::DownloadAborted("URL resolves to a non-public address"));
        }
    }

    if !resolved {
        return Err(AppError::DownloadAborted("URL host resolved to no addresses"));
    }

    Ok(())
}

/// Whether an address is publicly routable. Spelled out because `IpAddr::is_global` is unstable.
fn is_public(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(address) => {
            let [a, b, c, _] = address.octets();
            !(address.is_private()
                || address.is_loopback()
                || address.is_link_local()
                || address.is_multicast()
                || address.is_unspecified()
                || address.is_documentation()
                // "This network" (0.0.0.0/8).
                || a == 0
                // Carrier-grade NAT (100.64.0.0/10).
                || (a == 100 && (64..128).contains(&b))
                // IETF protocol assignments (192.0.0.0/24).
                || (a == 192 && b == 0 && c == 0)
                // Benchmarking (198.18.0.0/15).
                || (a == 198 && (b == 18 || b == 19))
                // Reserved, including the broadcast address (240.0.0.0/4).
                || a >= 240)
        }
        IpAddr::V6(address) => {
            // An IPv4-mapped address is only as public as the IPv4 address inside it.
            if let Some(mapped) = address.to_ipv4_mapped() {
                return is_public(&IpAddr::V4(mapped));
            }
            let first = address.segments()[0];
            !(address.is_loopback()
                || address.is_unspecified()
                || address.is_multicast()
                // Unique local addresses (fc00::/7).
                || (first & 0xfe00) == 0xfc00
                // Link-local unicast (fe80::/10).
                || (first & 0xffc0) == 0xfe80
                // Documentation (2001:db8::/32).
                || (first == 0x2001 && address.segments()[1] == 0x0db8))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn ip(address: &str) -> IpAddr {
        address.parse().unwrap()
    }

    #[test]
    fn public_addresses_are_accepted() {
        for address in ["1.1.1.1", "93.184.216.34", "2606:4700:4700::1111"] {
            assert!(is_public(&ip(address)), "{address} should be public");
        }
    }

    #[test]
    fn non_routable_addresses_are_rejected() {
        for address in [
            "127.0.0.1",
            "0.0.0.0",
            "10.0.0.1",
            "172.16.0.1",
            "192.168.1.1",
            "169.254.169.254",
            "100.64.0.1",
            "192.0.0.1",
            "198.18.0.1",
            "224.0.0.1",
            "255.255.255.255",
            "::1",
            "::",
            "fc00::1",
            "fe80::1",
            "::ffff:127.0.0.1",
            "::ffff:192.168.0.1",
        ] {
            assert!(!is_public(&ip(address)), "{address} should not be public");
        }
    }

    #[tokio::test]
    async fn non_http_schemes_are_rejected() {
        let url = Url::parse("file:///etc/passwd").unwrap();
        assert!(assert_public_destination(&url).await.is_err());
    }

    #[tokio::test]
    async fn public_destination_http_client_follows_redirects() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/moved"))
            .respond_with(ResponseTemplate::new(302).insert_header("location", "/entity-configuration"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/entity-configuration"))
            .respond_with(ResponseTemplate::new(200).set_body_string("entity-configuration"))
            .mount(&server)
            .await;

        let url = Url::parse(&format!("{}/moved", server.uri())).unwrap();

        assert_eq!(
            PublicDestinationHttpClient.fetch_text(url).await.unwrap(),
            "entity-configuration"
        );
    }

    #[tokio::test]
    async fn public_destination_http_client_refuses_redirect_loops() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/loop"))
            .respond_with(ResponseTemplate::new(302).insert_header("location", "/loop"))
            .mount(&server)
            .await;

        let url = Url::parse(&format!("{}/loop", server.uri())).unwrap();

        assert!(PublicDestinationHttpClient.fetch_text(url).await.is_err());
    }

    #[tokio::test]
    async fn public_destination_http_client_rejects_non_http_schemes() {
        let url = Url::parse("file:///etc/passwd").unwrap();
        assert!(PublicDestinationHttpClient.fetch_text(url).await.is_err());
    }
}
