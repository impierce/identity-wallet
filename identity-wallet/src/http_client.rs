use std::net::IpAddr;

use reqwest::{Client, Url};
use tokio::sync::OnceCell;

use crate::error::AppError;
use crate::state::core_utils::tls_config;

static HTTP_CLIENT: OnceCell<Client> = OnceCell::const_new();
static ASSET_HTTP_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub fn get_http_client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .use_preconfigured_tls(tls_config())
        .timeout(std::time::Duration::from_secs(2))
}

/// Returns a globally shared `reqwest::Client` configured with `webpki-roots` TLS.
pub async fn get_http_client() -> Client {
    HTTP_CLIENT
        .get_or_init(|| async {
            // Deliberately no `Client::new()` fallback: it would quietly drop the pinned
            // `webpki-roots` trust anchors that `tls_config()` installs and leave the wallet
            // trusting whatever the platform happens to, which is the thing #746 set out to stop.
            // `Client::new()` panics on the same TLS-initialisation failures anyway.
            get_http_client_builder()
                .build()
                .expect("HTTP client with pinned roots could not be built")
        })
        .await
        .clone()
}

/// Returns a globally shared `reqwest::Client` for downloading assets from URLs supplied by a
/// remote party. It never follows redirects on its own: every hop has to pass
/// [`assert_public_destination`] first, which the caller does while following them by hand.
///
/// Failing here is not the same as falling back to a default client: a default client follows up
/// to ten redirects by itself, which would carry every hop after the first past
/// [`assert_public_destination`] and hand a remote party the SSRF this policy exists to prevent.
pub async fn get_asset_http_client() -> Result<Client, AppError> {
    ASSET_HTTP_CLIENT
        .get_or_try_init(|| async {
            get_http_client_builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .map_err(|err| {
                    log::error!("Failed to build reqwest client for assets: {err}");
                    AppError::DownloadAborted("asset HTTP client could not be built")
                })
        })
        .await
        .cloned()
}

/// Whether requests to loopback, private and link-local addresses are allowed.
///
/// Debug builds allow them so that the app, its unit tests and its integration tests can talk to a
/// local issuer or a `wiremock` server. Release builds do not: a `logo_uri` is attacker-controlled
/// and would otherwise let a connection make the wallet probe the device's own network (CWE-918).
fn private_destinations_allowed() -> bool {
    cfg!(debug_assertions)
}

/// Rejects URLs that a remote party should not be able to make the wallet request: anything that is
/// not `http(s)`, and anything resolving to a loopback, private, link-local or otherwise
/// non-routable address.
///
/// This resolves the host itself rather than letting the connection do it, so it cannot prove the
/// address the socket ends up on (a DNS entry can change in between). It does remove the trivial
/// cases, which is what a supplied URL can realistically abuse.
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

/// Whether an address is publicly routable. The `std` equivalents (`IpAddr::is_global`) are still
/// unstable, so the ranges are spelled out here.
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
}
