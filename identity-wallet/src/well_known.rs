//! Locating `.well-known` resources.
//!
//! Specs disagree on where `.well-known` goes when the base URL has a path of its own. OpenID Connect Discovery
//! appends it (`/tenant/.well-known/x`), RFC 8414 and OpenID4VCI insert it after the host (`/.well-known/x/tenant`),
//! and some deployments only serve it at the host (`/.well-known/x`). UniMe tries all three, in the order of
//! [`well_known_candidates`].

use log::debug;
use oid4vc::oid4vci::credential_issuer::{
    authorization_server_metadata::AuthorizationServerMetadata, credential_issuer_metadata::CredentialIssuerMetadata,
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::http_client::get_http_client;

pub const OPENID_CREDENTIAL_ISSUER: &str = "openid-credential-issuer";
const OAUTH_AUTHORIZATION_SERVER: &str = "oauth-authorization-server";
const OPENID_CONFIGURATION: &str = "openid-configuration";

/// The URLs at which the `.well-known/{suffix}` resource of `base` may live, in the order UniMe tries them: appended
/// to the full URL, at the host only, and inserted between the host and the path.
///
/// The query and fragment of `base` are dropped, and duplicates are removed, so a `base` without a path gives one URL.
pub fn well_known_candidates(base: &Url, suffix: &str) -> Vec<Url> {
    let path = base.path().trim_end_matches('/');
    let paths = [
        format!("{path}/.well-known/{suffix}"),
        format!("/.well-known/{suffix}"),
        format!("/.well-known/{suffix}{path}"),
    ];

    let mut candidates: Vec<Url> = Vec::with_capacity(paths.len());
    for path in paths {
        let mut candidate = base.clone();
        candidate.set_path(&path);
        candidate.set_query(None);
        candidate.set_fragment(None);
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    candidates
}

/// Fetches the credential issuer metadata of `credential_issuer_url`.
pub async fn get_credential_issuer_metadata(credential_issuer_url: &Url) -> anyhow::Result<CredentialIssuerMetadata> {
    fetch_well_known(
        credential_issuer_url,
        &[OPENID_CREDENTIAL_ISSUER],
        "credential issuer metadata",
    )
    .await
}

/// Fetches the metadata of `authorization_server_url`. RFC 8414 OAuth metadata is preferred, with an OpenID Provider
/// configuration as the fallback (RFC 8414 section 5).
pub async fn get_authorization_server_metadata(
    authorization_server_url: &Url,
) -> anyhow::Result<AuthorizationServerMetadata> {
    fetch_well_known(
        authorization_server_url,
        &[OAUTH_AUTHORIZATION_SERVER, OPENID_CONFIGURATION],
        "authorization server metadata",
    )
    .await
}

/// Fetches the `.well-known` resource of `base` and parses it as `T`, trying every candidate URL of each suffix in
/// turn (see [`well_known_candidates`]) until one parses.
///
/// An HTTP error status or an unparseable body moves on to the next candidate: a host may answer any path, such as
/// with a single-page app's `index.html`. A request that fails outright stops the search, because every candidate is
/// on the same host and would fail the same way, each time spending the client's timeout.
///
/// The error messages keep the `(HTTP <status>)` and `Failed to parse` wording that
/// [`ErrorCause`](crate::user_error::ErrorCause) recognises.
async fn fetch_well_known<T: DeserializeOwned>(base: &Url, suffixes: &[&str], resource: &str) -> anyhow::Result<T> {
    let client = get_http_client().await;
    let mut last_error = None;

    for url in suffixes.iter().flat_map(|suffix| well_known_candidates(base, suffix)) {
        debug!("Trying {resource} at {url}");

        let response = client
            .get(url.clone())
            .send()
            .await
            .map_err(|err| anyhow::Error::new(err).context(format!("Failed to get {resource} from {url}")))?;
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|err| anyhow::Error::new(err).context(format!("Failed to read {resource} from {url}")))?;

        if !status.is_success() {
            debug!("No {resource} at {url} (HTTP {status})");
            last_error = Some(anyhow::anyhow!(
                "Failed to get {resource} from {url} (HTTP {status}): {text}"
            ));
            continue;
        }

        match serde_json::from_str::<T>(&text) {
            Ok(resource_value) => {
                debug!("Found {resource} at {url}");
                return Ok(resource_value);
            }
            Err(err) => {
                debug!("Unparseable {resource} at {url}: {err}");
                last_error = Some(anyhow::anyhow!(
                    "Failed to parse {resource} from {url} (HTTP {status}): {err}"
                ));
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("No {resource} URL could be built from {base}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn url(url: &str) -> Url {
        Url::parse(url).unwrap()
    }

    fn strings(urls: Vec<Url>) -> Vec<String> {
        urls.into_iter().map(String::from).collect()
    }

    #[test]
    fn candidates_for_a_url_with_a_path_are_append_then_host_then_insert() {
        assert_eq!(
            strings(well_known_candidates(
                &url("https://issuer.example/tenant/a"),
                OPENID_CREDENTIAL_ISSUER
            )),
            vec![
                "https://issuer.example/tenant/a/.well-known/openid-credential-issuer",
                "https://issuer.example/.well-known/openid-credential-issuer",
                "https://issuer.example/.well-known/openid-credential-issuer/tenant/a",
            ]
        );
    }

    #[test]
    fn a_url_without_a_path_gives_one_candidate() {
        for base in ["https://issuer.example", "https://issuer.example/"] {
            assert_eq!(
                strings(well_known_candidates(&url(base), OPENID_CREDENTIAL_ISSUER)),
                vec!["https://issuer.example/.well-known/openid-credential-issuer"],
                "{base}"
            );
        }
    }

    #[test]
    fn a_trailing_slash_does_not_change_the_candidates() {
        assert_eq!(
            well_known_candidates(&url("https://issuer.example/tenant/"), OPENID_CREDENTIAL_ISSUER),
            well_known_candidates(&url("https://issuer.example/tenant"), OPENID_CREDENTIAL_ISSUER),
        );
    }

    #[test]
    fn port_is_kept_and_query_and_fragment_are_dropped() {
        assert_eq!(
            strings(well_known_candidates(
                &url("http://localhost:3033/oid4vci?x=1#frag"),
                OPENID_CREDENTIAL_ISSUER
            )),
            vec![
                "http://localhost:3033/oid4vci/.well-known/openid-credential-issuer",
                "http://localhost:3033/.well-known/openid-credential-issuer",
                "http://localhost:3033/.well-known/openid-credential-issuer/oid4vci",
            ]
        );
    }

    fn credential_issuer_metadata(credential_issuer: &str) -> serde_json::Value {
        json!({
            "credential_issuer": credential_issuer,
            "credential_endpoint": format!("{credential_issuer}/credential"),
            "credential_configurations_supported": {}
        })
    }

    async fn mount_json(mock_server: &MockServer, at: &str, body: serde_json::Value) {
        Mock::given(method("GET"))
            .and(path(at))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(mock_server)
            .await;
    }

    #[tokio::test]
    async fn finds_the_issuer_metadata_at_each_position_in_order() {
        for (position, at) in [
            ("append", "/tenant/.well-known/openid-credential-issuer"),
            ("host", "/.well-known/openid-credential-issuer"),
            ("insert", "/.well-known/openid-credential-issuer/tenant"),
        ] {
            let mock_server = MockServer::start().await;
            let issuer = format!("{}/tenant", mock_server.uri());
            mount_json(&mock_server, at, credential_issuer_metadata(&issuer)).await;

            let metadata = get_credential_issuer_metadata(&url(&issuer)).await;

            assert!(metadata.is_ok(), "{position}: {metadata:?}");
            let requested: Vec<String> = mock_server
                .received_requests()
                .await
                .unwrap()
                .into_iter()
                .map(|request| request.url.path().to_string())
                .collect();
            let tried = well_known_candidates(&url(&issuer), OPENID_CREDENTIAL_ISSUER)
                .into_iter()
                .map(|candidate| candidate.path().to_string())
                .take_while(|candidate| candidate != at)
                .chain([at.to_string()])
                .collect::<Vec<_>>();
            assert_eq!(requested, tried, "{position}");
        }
    }

    #[tokio::test]
    async fn the_first_candidate_that_parses_wins() {
        let mock_server = MockServer::start().await;
        let issuer = format!("{}/tenant", mock_server.uri());
        Mock::given(method("GET"))
            .and(path("/tenant/.well-known/openid-credential-issuer"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<!doctype html><html></html>"))
            .mount(&mock_server)
            .await;
        mount_json(
            &mock_server,
            "/.well-known/openid-credential-issuer",
            credential_issuer_metadata(&issuer),
        )
        .await;
        Mock::given(method("GET"))
            .and(path("/.well-known/openid-credential-issuer/tenant"))
            .respond_with(ResponseTemplate::new(200).set_body_json(credential_issuer_metadata("https://other.example")))
            .expect(0)
            .mount(&mock_server)
            .await;

        let metadata = get_credential_issuer_metadata(&url(&issuer)).await.unwrap();

        assert_eq!(metadata.credential_issuer.as_str(), url(&issuer).as_str());
    }

    #[tokio::test]
    async fn reports_the_status_of_the_last_candidate_when_none_is_found() {
        let mock_server = MockServer::start().await;
        let issuer = format!("{}/tenant", mock_server.uri());

        let error = get_credential_issuer_metadata(&url(&issuer)).await.unwrap_err();

        assert!(error.to_string().contains("(HTTP 404 Not Found)"), "{error}");
        assert!(
            error
                .to_string()
                .contains("/.well-known/openid-credential-issuer/tenant"),
            "{error}"
        );
        assert_eq!(mock_server.received_requests().await.unwrap().len(), 3);
    }

    #[tokio::test]
    async fn an_unreachable_host_stops_at_the_first_candidate() {
        // A port that was free a moment ago. Dropping a `MockServer` does not do: it returns to a pool and keeps
        // listening.
        let closed_port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let issuer = format!("http://127.0.0.1:{closed_port}/tenant");

        let error = get_credential_issuer_metadata(&url(&issuer)).await.unwrap_err();

        assert_eq!(
            error.to_string(),
            format!("Failed to get credential issuer metadata from {issuer}/.well-known/openid-credential-issuer"),
        );
        assert!(error.chain().any(|cause| cause.is::<reqwest::Error>()), "{error:?}");
    }

    #[tokio::test]
    async fn authorization_server_metadata_falls_back_to_openid_configuration_in_every_position() {
        let mock_server = MockServer::start().await;
        let authorization_server = format!("{}/tenant", mock_server.uri());
        mount_json(
            &mock_server,
            "/.well-known/openid-configuration/tenant",
            json!({ "issuer": authorization_server, "token_endpoint": format!("{authorization_server}/token") }),
        )
        .await;

        let metadata = get_authorization_server_metadata(&url(&authorization_server)).await;

        assert!(metadata.is_ok(), "{metadata:?}");
        let requested: Vec<String> = mock_server
            .received_requests()
            .await
            .unwrap()
            .into_iter()
            .map(|request| request.url.path().to_string())
            .collect();
        assert_eq!(
            requested,
            vec![
                "/tenant/.well-known/oauth-authorization-server",
                "/.well-known/oauth-authorization-server",
                "/.well-known/oauth-authorization-server/tenant",
                "/tenant/.well-known/openid-configuration",
                "/.well-known/openid-configuration",
                "/.well-known/openid-configuration/tenant",
            ]
        );
    }
}
