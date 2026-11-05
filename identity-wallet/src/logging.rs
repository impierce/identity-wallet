//! Rules for what UniMe may log, see `docs/adr/0003-no-personal-data-in-logs.md`.
//!
//! Log lines contain only IDs, credential types, counts, issuer/verifier names and hosts, statuses, and durations.
//! Credentials, claims, tokens, codes, nonces, protocol payloads, actions, and app state are logged only through
//! [`log_payload!`], which is a no-op unless [`full_payloads_enabled`] returns `true`.

/// Whether full payloads may be logged: only in debug builds, and only when `LOG_FULL_PAYLOADS=true` is set.
pub fn full_payloads_enabled() -> bool {
    cfg!(debug_assertions) && env_flag("LOG_FULL_PAYLOADS")
}

/// Reads a boolean environment variable, defaulting to `false`.
pub(crate) fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(false)
}

/// Logs a full payload at DEBUG, only when [`full_payloads_enabled`] returns `true`.
#[macro_export]
macro_rules! log_payload {
    ($($arg:tt)+) => {
        if $crate::logging::full_payloads_enabled() {
            log::debug!($($arg)+);
        }
    };
}

/// Returns the host of a URL for logging, or a placeholder when it has none or cannot be parsed. Paths and query
/// strings are left out because they can carry codes, tokens, and request URIs.
pub fn url_host(url: impl AsRef<str>) -> String {
    url::Url::parse(url.as_ref())
        .ok()
        .and_then(|url| url.host_str().map(ToString::to_string))
        .unwrap_or_else(|| "<no host>".to_string())
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
}
