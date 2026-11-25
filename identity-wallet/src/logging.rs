//! Rules for what UniMe may log, see `docs/adr/0003-no-personal-data-in-logs.md`.
//!
//! Log lines contain only IDs, credential types, counts, issuer/verifier names and hosts, statuses, and durations.
//! Credentials, claims, tokens, codes, nonces, protocol payloads, actions, and app state are logged only through
//! [`log_payload!`], which is a no-op unless `LOG_FULL_PAYLOADS=true` is set in a debug build.

/// Whether a debug switch such as `LOG_FULL_PAYLOADS` is on: only in debug builds, and only when the environment
/// variable is set to `true`. Always `false` in release builds, whatever the environment says.
pub fn debug_flag(name: &str) -> bool {
    cfg!(debug_assertions)
        && std::env::var(name)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(false)
}

/// Logs a full payload at DEBUG, only when `LOG_FULL_PAYLOADS=true` is set in a debug build.
#[macro_export]
macro_rules! log_payload {
    ($($arg:tt)+) => {
        if $crate::logging::debug_flag("LOG_FULL_PAYLOADS") {
            log::debug!($($arg)+);
        }
    };
}

/// Returns only the host of a URL, for logging URLs whose path or query grants access to something. Returns
/// `<no host>` when the URL has no host (such as `openid-credential-offer://?...`) or cannot be parsed.
///
/// Use it for URLs that act as a bearer secret, where anyone who has the full URL can obtain what it points to:
///
/// - a `credential_offer_uri` (OID4VCI 1.0, Section 4.1.3): a plain GET returns the Credential Offer, including any
///   `pre-authorized_code`, and with it the credential unless a Transaction Code is required (Sections 3.5 and
///   13.6.1). The spec only recommends a unique URI per offer, not a single-use one.
/// - a public link to a credential (`{verifier}/verify/{jti}`): anyone with the link can view the credential at the
///   verifier. This is UniMe's own mechanism, not part of a spec.
///
/// URLs that carry a secret only in their query, such as the authorization endpoint after a Pushed Authorization
/// Request, can be logged with [`url_without_query`] instead, which keeps the path. Never log a credential offer by
/// value (`credential_offer=...`) or an authorization response redirect (`?code=...&state=...`), not even its host:
/// they come from QR codes and deep links, whose contents are not logged at all.
pub fn url_host(url: impl AsRef<str>) -> String {
    url::Url::parse(url.as_ref())
        .ok()
        .and_then(|url| url.host_str().map(ToString::to_string))
        .unwrap_or_else(|| "<no host>".to_string())
}

/// Returns a URL without its query and fragment, for logging an endpoint whose query carries request parameters.
///
/// After a Pushed Authorization Request, the authorization URL carries only `client_id` and `request_uri` in its
/// query (RFC 9126, Section 4). The `request_uri` is short-lived, single-use and bound to the client (Section 2.2),
/// and the resulting authorization code still needs the PKCE `code_verifier`, so a logged `request_uri` is of little
/// use to an attacker. It is dropped anyway because ADR 0003 keeps request URIs out of logs, and the endpoint path
/// is what helps when debugging. The fragment is dropped because response modes such as OID4VP's `fragment` return
/// the response there (OpenID4VP 1.0, Section 3.1).
pub fn url_without_query(url: &url::Url) -> String {
    let mut url = url.clone();
    url.set_query(None);
    url.set_fragment(None);
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_host_drops_path_and_query() {
        assert_eq!(
            url_host("https://issuer.example.com/authorize?request_uri=urn:secret&code=abc"),
            "issuer.example.com"
        );
        assert_eq!(
            url_host("openid-credential-offer://?credential_offer=%7B%22grants%22%7D"),
            "<no host>"
        );
        assert_eq!(url_host("not a url"), "<no host>");
    }

    #[test]
    fn url_without_query_keeps_the_endpoint() {
        let url = url::Url::parse("https://as.example.com/oauth/authorize?client_id=unime&request_uri=urn:secret#frag")
            .unwrap();
        assert_eq!(url_without_query(&url), "https://as.example.com/oauth/authorize");
    }
}
