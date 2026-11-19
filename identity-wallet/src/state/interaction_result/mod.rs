pub mod actions;
mod reducers;

use oid4vc_core::{
    authorization_response::AuthorizationResponse, openid4vc_extension::Extension,
    utils::form_urlencoded::to_form_urlencoded_string,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

use crate::http_client::{plain_http_allowed, post_encoded_form_to_public_destination};

/// The outcome of the last interaction with an issuer or verifier, shown to the user in the result pop-up.
#[derive(Debug, TS, Clone, Serialize, Deserialize, PartialEq)]
#[ts(export, export_to = "bindings/interaction_result/InteractionResult.ts")]
pub struct InteractionResult {
    pub kind: InteractionKind,
    /// Display name of the issuer or verifier.
    pub counterparty: String,
    pub outcome: InteractionOutcome,
    /// The host a verifier asks the user to continue at after an accepted presentation. The result pop-up names it
    /// on the button that opens the verifier's `redirect_uri`.
    pub continue_to: Option<String>,
    /// The verifier's `redirect_uri` behind [`Self::continue_to`]. It can carry a response code, so it stays in the
    /// backend: it is neither sent to the frontend nor written to disk.
    #[serde(skip)]
    #[ts(skip)]
    pub redirect_uri: Option<Url>,
}

impl InteractionResult {
    /// The result of an interaction that ends in the wallet.
    pub fn new(kind: InteractionKind, counterparty: String, outcome: InteractionOutcome) -> Self {
        InteractionResult {
            kind,
            counterparty,
            outcome,
            continue_to: None,
            redirect_uri: None,
        }
    }

    /// Adds the `redirect_uri` the verifier asks the user to continue at.
    pub fn continuing_at(self, redirect_uri: Option<Url>) -> Self {
        InteractionResult {
            continue_to: redirect_uri
                .as_ref()
                .and_then(|redirect_uri| redirect_uri.host_str().map(ToString::to_string)),
            redirect_uri,
            ..self
        }
    }
}

#[derive(Debug, TS, Clone, Serialize, Deserialize, PartialEq)]
#[ts(export, export_to = "bindings/interaction_result/InteractionKind.ts")]
#[serde(rename_all = "snake_case")]
pub enum InteractionKind {
    /// Receiving credentials (OpenID4VCI).
    Issuance,
    /// Sharing credentials (OpenID4VP).
    Presentation,
}

#[derive(Debug, TS, Clone, Serialize, Deserialize, PartialEq)]
#[ts(export, export_to = "bindings/interaction_result/InteractionOutcome.ts")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InteractionOutcome {
    /// The request reached the counterparty and it accepted it.
    Approved,
    /// The request reached the counterparty, but it answered with an error.
    Rejected {
        error_code: Option<String>,
        error_description: Option<String>,
        http_status: Option<u16>,
    },
    /// The request did not get an answer (network error, timeout).
    NotSent { reason: String },
}

impl InteractionOutcome {
    /// Builds a [`InteractionOutcome::Rejected`] from an error response, reading OAuth-style (`error`,
    /// `error_description`) and Problem Details (`type`, `title`, `detail`) bodies.
    pub fn rejected(http_status: u16, body: &str) -> Self {
        let json: serde_json::Value = serde_json::from_str(body).unwrap_or_default();
        let field = |key: &str| json.get(key).and_then(|value| value.as_str()).map(ToString::to_string);

        let error_code = field("error").or_else(|| {
            field("type")
                .and_then(|type_url| type_url.rsplit(['#', '/']).next().map(ToString::to_string))
                .filter(|code| !code.is_empty() && code.parse::<u16>().is_err())
        });
        // A Problem Details `title` only repeats the HTTP status, so it is not used as a description.
        let error_description = field("error_description").or_else(|| field("detail"));

        InteractionOutcome::Rejected {
            error_code,
            error_description,
            http_status: Some(http_status),
        }
    }

    /// Classifies an error from the OpenID4VCI wallet client. The client reports HTTP errors as
    /// "... (HTTP 400 Bad Request): <body>"; anything else means the request got no answer.
    pub fn from_wallet_error(error: &anyhow::Error) -> Self {
        let message = format!("{error:#}");

        let http_error = message.split_once("(HTTP ").and_then(|(_, rest)| {
            let (status, rest) = rest.split_once([' ', ')'])?;
            let status = status.parse::<u16>().ok()?;
            let body = rest.split_once("): ").map(|(_, body)| body).unwrap_or_default();
            Some((status, body))
        });

        match http_error {
            Some((status, body)) => Self::rejected(status, body),
            None => InteractionOutcome::NotSent { reason: message },
        }
    }
}

/// How a verifier's Response Endpoint answered an authorization response.
#[derive(Debug, PartialEq)]
pub struct ResponseAnswer {
    pub outcome: InteractionOutcome,
    /// Where the verifier asks the user to continue after an accepted response: "When this parameter is present the
    /// Wallet MUST redirect the user agent to this URI" (OpenID4VP 1.0, Section 8.2). UniMe offers it to the user and
    /// opens it when they choose to, see [`redirect_uri_of`].
    pub redirect_uri: Option<Url>,
}

impl From<InteractionOutcome> for ResponseAnswer {
    fn from(outcome: InteractionOutcome) -> Self {
        ResponseAnswer {
            outcome,
            redirect_uri: None,
        }
    }
}

/// Sends an authorization response with the `direct_post` response mode and reports how the verifier answered.
///
/// Unlike `ProviderManager::send_response`, this reads the response body, so a verifier's error code and its
/// `redirect_uri` reach the caller.
pub async fn send_authorization_response<E: Extension>(
    authorization_response: &AuthorizationResponse<E>,
) -> ResponseAnswer {
    match to_form_urlencoded_string(authorization_response) {
        Ok(encoded) => post_authorization_response(&authorization_response.redirect_uri, encoded).await,
        Err(error) => InteractionOutcome::NotSent {
            reason: error.to_string(),
        }
        .into(),
    }
}

/// The `redirect_uri` in the JSON a Response Endpoint answered an accepted response with, when it is one UniMe offers
/// the user to continue at. The verifier chooses it freely, and it is opened outside the wallet, so:
/// - the Response Endpoint was reached over `https`: over plain `http` anyone on the way could have written the answer;
/// - it is an `https` URL: no other scheme is handed to the system to open;
/// - it has no user name or password, which a browser shows in front of the host to mislead;
/// - its host is a domain name, not an IP address. An internationalized name is in its ASCII form here, so that is
///   also what the user is shown.
///
/// `local` lifts the `https` and domain name rules for a verifier on the developer's machine, where requests over plain
/// `http` are allowed ([`plain_http_allowed`]: debug builds with `UNIME_ALLOW_PLAIN_HTTP=true`).
fn redirect_uri_of(body: &str, response_uri: &Url, local: bool) -> Option<Url> {
    if response_uri.scheme() != "https" && !local {
        return None;
    }
    let json: serde_json::Value = serde_json::from_str(body).ok()?;
    let redirect_uri = Url::parse(json.get("redirect_uri")?.as_str()?).ok()?;

    let is_web_address = match redirect_uri.scheme() {
        "https" => true,
        "http" => local,
        _ => false,
    };
    let has_credentials = !redirect_uri.username().is_empty() || redirect_uri.password().is_some();
    let has_named_host = match redirect_uri.host() {
        Some(url::Host::Domain(_)) => true,
        Some(_) => local,
        None => false,
    };
    (is_web_address && !has_credentials && has_named_host).then_some(redirect_uri)
}

/// Posts an encoded authorization response to the verifier's Response Endpoint. It carries the user's credentials,
/// so it is sent once and to that endpoint only:
/// - **No retry once sent.** A request that got an error or no answer may still have been read; sending it again is
///   the user's decision. Only a request whose connection could not be opened is sent once more, see
///   [`post_encoded_form_to_public_destination`].
/// - **Time for the verifier.** The verifier may check the presentation before it answers, so the request may take
///   longer than other requests of the app.
/// - **No redirect is followed.** A redirect would send the credentials to an address the request never named, and
///   over plain `http` if the redirect says so. It is reported as [`InteractionOutcome::Rejected`], like any other
///   answer that is not a success.
/// - **Public destinations only**, as for every URL a remote party supplies.
///
/// [`InteractionOutcome::NotSent`] means no answer came back. Any other outcome means the endpoint answered, and so
/// has the credentials.
async fn post_authorization_response(response_uri: &str, encoded: String) -> ResponseAnswer {
    let not_sent = |error: anyhow::Error| {
        ResponseAnswer::from(InteractionOutcome::NotSent {
            reason: format!("{:#}", error.context("Failed to send the authorization response")),
        })
    };

    let response_uri = match Url::parse(response_uri) {
        Ok(response_uri) => response_uri,
        Err(error) => return not_sent(error.into()),
    };
    let response = match post_encoded_form_to_public_destination(&response_uri, encoded).await {
        Ok(response) => response,
        Err(error) => return not_sent(error.into()),
    };

    let status = response.status();
    // An answer that is too large or cannot be read is treated as an empty one: the status still decides the outcome.
    let body = bounded_text(response).await.unwrap_or_default();
    if status.is_success() {
        return ResponseAnswer {
            outcome: InteractionOutcome::Approved,
            redirect_uri: redirect_uri_of(&body, &response_uri, plain_http_allowed()),
        };
    }

    InteractionOutcome::rejected(status.as_u16(), &body).into()
}

/// The largest answer of a Response Endpoint that is read: far above an error body or a `redirect_uri`.
const MAX_ANSWER_SIZE: usize = 1 << 20;

/// The body of `response` as text, read in chunks so that a verifier cannot make the wallet hold more than
/// [`MAX_ANSWER_SIZE`] bytes. `None` when the body is larger than that or cannot be read.
async fn bounded_text(mut response: reqwest::Response) -> Option<String> {
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.ok()? {
        if body.len() + chunk.len() > MAX_ANSWER_SIZE {
            return None;
        }
        body.extend_from_slice(&chunk);
    }
    String::from_utf8(body).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_string, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const ENCODED: &str = "vp_token=%7B%7D&state=abc";

    async fn answer_of(response_endpoint: &MockServer) -> ResponseAnswer {
        post_authorization_response(&format!("{}/response", response_endpoint.uri()), ENCODED.to_string()).await
    }

    async fn post_to(response_endpoint: &MockServer) -> InteractionOutcome {
        answer_of(response_endpoint).await.outcome
    }

    fn response_uri() -> Url {
        "https://verifier.example/response".parse().unwrap()
    }

    fn redirect_uri_in(redirect_uri: &str, local: bool) -> Option<Url> {
        let body = serde_json::json!({ "redirect_uri": redirect_uri }).to_string();
        redirect_uri_of(&body, &response_uri(), local)
    }

    #[test]
    fn reads_the_redirect_uri_of_a_response_endpoint() {
        assert_eq!(
            redirect_uri_in("https://verifier.example/done#response_code=091535f6", false)
                .unwrap()
                .as_str(),
            "https://verifier.example/done#response_code=091535f6"
        );

        // No body, another JSON, or a value that is no URL: nowhere to continue.
        for body in ["", "{}", r#"{"redirect_uri":"done"}"#, r#"{"redirect_uri":42}"#] {
            assert_eq!(redirect_uri_of(body, &response_uri(), false), None, "{body}");
        }
    }

    #[test]
    fn only_https_addresses_are_offered() {
        for other in [
            "javascript:alert(1)",
            "file:///etc/passwd",
            "unime://x",
            "openid4vp://?client_id=x",
            "data:text/html,x",
            "http://verifier.example/done",
        ] {
            assert_eq!(redirect_uri_in(other, false), None, "{other}");
        }
        // Not even for a local verifier is another scheme than `http` and `https` opened.
        for other in ["javascript:alert(1)", "file:///etc/passwd", "unime://x"] {
            assert_eq!(redirect_uri_in(other, true), None, "{other}");
        }
    }

    #[test]
    fn an_address_that_hides_its_host_is_not_offered() {
        for hidden in [
            // A user name or password in front of the host reads as the host.
            "https://bank.example@attacker.example/",
            "https://bank.example:secret@attacker.example/",
            // An address without a name.
            "https://203.0.113.7/",
            "https://[2001:db8::1]/",
            "https://2130706433/",
        ] {
            assert_eq!(redirect_uri_in(hidden, false), None, "{hidden}");
        }
        // Credentials are refused for a local verifier as well.
        assert_eq!(redirect_uri_in("http://bank.example@localhost:8080/", true), None);
    }

    #[test]
    fn the_host_is_shown_as_the_browser_resolves_it() {
        // A name that looks like `apple.example` in another script is offered in its ASCII form.
        let result = InteractionResult::new(
            InteractionKind::Presentation,
            "Verifier".to_string(),
            InteractionOutcome::Approved,
        )
        .continuing_at(redirect_uri_in("https://\u{430}pple.example/done", false));

        assert_eq!(result.continue_to.as_deref(), Some("xn--pple-43d.example"));
    }

    #[test]
    fn a_response_endpoint_reached_over_http_cannot_send_the_user_on() {
        let body = r#"{"redirect_uri":"https://verifier.example/done"}"#;
        let over_http: Url = "http://verifier.example/response".parse().unwrap();

        assert_eq!(redirect_uri_of(body, &over_http, false), None);
        assert!(redirect_uri_of(body, &response_uri(), false).is_some());
    }

    #[test]
    fn a_local_verifier_is_reached_over_http_in_debug_builds() {
        let body = r#"{"redirect_uri":"http://127.0.0.1:8080/done"}"#;
        let local: Url = "http://127.0.0.1:8080/response".parse().unwrap();

        assert!(redirect_uri_of(body, &local, true).is_some());
        assert_eq!(redirect_uri_of(body, &local, false), None);
    }

    #[test]
    fn the_redirect_uri_is_not_sent_to_the_frontend() {
        let result = InteractionResult::new(
            InteractionKind::Presentation,
            "Verifier".to_string(),
            InteractionOutcome::Approved,
        )
        .continuing_at(redirect_uri_in(
            "https://verifier.example/done#response_code=091535f6",
            false,
        ));

        let sent = serde_json::to_value(&result).unwrap();
        assert_eq!(sent["continue_to"], "verifier.example");
        assert!(!sent.to_string().contains("response_code"), "{sent}");
    }

    #[tokio::test]
    async fn an_accepted_presentation_offers_the_redirect_uri_of_the_response_endpoint() {
        let response_endpoint = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/response"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(r#"{"redirect_uri":"https://verifier.example/done"}"#),
            )
            .mount(&response_endpoint)
            .await;

        // The mock is reached over `http`, which tests allow as a debug build with `UNIME_ALLOW_PLAIN_HTTP=true` does.
        let answer = answer_of(&response_endpoint).await;
        assert_eq!(answer.outcome, InteractionOutcome::Approved);
        assert_eq!(
            answer.redirect_uri.as_ref().map(Url::as_str),
            Some("https://verifier.example/done")
        );
    }

    #[tokio::test]
    async fn a_rejected_presentation_offers_no_redirect_uri() {
        let response_endpoint = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_string(r#"{"error":"invalid_request","redirect_uri":"https://verifier.example/done"}"#),
            )
            .mount(&response_endpoint)
            .await;

        assert_eq!(answer_of(&response_endpoint).await.redirect_uri, None);
    }

    #[tokio::test]
    async fn an_accepted_presentation_is_posted_as_a_form() {
        let response_endpoint = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/response"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .and(body_string(ENCODED))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&response_endpoint)
            .await;

        assert_eq!(post_to(&response_endpoint).await, InteractionOutcome::Approved);
    }

    #[tokio::test]
    async fn a_verifier_that_answers_after_the_limit_of_other_requests_is_waited_for() {
        // Longer than the five seconds a request of the shared clients may take.
        let response_endpoint = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(6)))
            .expect(1)
            .mount(&response_endpoint)
            .await;

        assert_eq!(post_to(&response_endpoint).await, InteractionOutcome::Approved);
    }

    #[tokio::test]
    async fn a_presentation_is_posted_once_when_the_verifier_answers_with_an_error() {
        // A transient error, which the shared client would send the request again for.
        let response_endpoint = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(503).set_body_string(r#"{"error":"temporarily_unavailable"}"#))
            .expect(1)
            .mount(&response_endpoint)
            .await;

        assert_eq!(
            post_to(&response_endpoint).await,
            InteractionOutcome::Rejected {
                error_code: Some("temporarily_unavailable".to_string()),
                error_description: None,
                http_status: Some(503),
            }
        );
        // `expect(1)` is verified when the server is dropped.
    }

    #[tokio::test]
    async fn a_redirect_of_the_response_endpoint_is_not_followed() {
        let elsewhere = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&elsewhere)
            .await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&elsewhere)
            .await;

        // 307 keeps the method and the body, 302 turns the request into a GET: neither is followed.
        for status in [307, 302] {
            let response_endpoint = MockServer::start().await;
            Mock::given(method("POST"))
                .respond_with(
                    ResponseTemplate::new(status)
                        .insert_header("location", format!("{}/response", elsewhere.uri()).as_str()),
                )
                .expect(1)
                .mount(&response_endpoint)
                .await;

            assert_eq!(
                post_to(&response_endpoint).await,
                InteractionOutcome::Rejected {
                    error_code: None,
                    error_description: None,
                    http_status: Some(status),
                }
            );
        }
    }

    #[tokio::test]
    async fn a_presentation_is_not_sent_to_an_address_that_is_not_a_web_address() {
        for response_uri in ["file:///etc/hosts", "not a URL"] {
            assert!(matches!(
                post_authorization_response(response_uri, ENCODED.to_string())
                    .await
                    .outcome,
                InteractionOutcome::NotSent { .. }
            ));
        }
    }

    #[tokio::test]
    async fn an_answer_is_read_up_to_a_size_limit() {
        let response_endpoint = MockServer::start().await;
        let answer_of = |size: usize| {
            let response_endpoint = &response_endpoint;
            async move {
                response_endpoint.reset().await;
                Mock::given(method("GET"))
                    .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![b'a'; size]))
                    .mount(response_endpoint)
                    .await;
                let response = reqwest::get(response_endpoint.uri()).await.unwrap();
                bounded_text(response).await
            }
        };

        assert_eq!(
            answer_of(MAX_ANSWER_SIZE).await.map(|body| body.len()),
            Some(MAX_ANSWER_SIZE)
        );
        assert_eq!(answer_of(MAX_ANSWER_SIZE + 1).await, None);
    }

    #[test]
    fn reads_oauth_error_bodies() {
        assert_eq!(
            InteractionOutcome::rejected(
                400,
                r#"{"error":"invalid_grant","error_description":"Offer already used"}"#
            ),
            InteractionOutcome::Rejected {
                error_code: Some("invalid_grant".to_string()),
                error_description: Some("Offer already used".to_string()),
                http_status: Some(400),
            }
        );
    }

    #[test]
    fn reads_problem_details_bodies() {
        assert_eq!(
            InteractionOutcome::rejected(
                400,
                r#"{"type":"https://docs.example.com/verification#credential-revoked","title":"Credential Revoked","detail":"The credential has been revoked."}"#
            ),
            InteractionOutcome::Rejected {
                error_code: Some("credential-revoked".to_string()),
                error_description: Some("The credential has been revoked.".to_string()),
                http_status: Some(400),
            }
        );
        assert_eq!(
            InteractionOutcome::rejected(
                400,
                r#"{"type":"https://httpstatuses.com/400","status":400,"title":"Bad Request"}"#
            ),
            InteractionOutcome::Rejected {
                error_code: None,
                error_description: None,
                http_status: Some(400),
            }
        );
    }

    #[test]
    fn classifies_wallet_errors() {
        let rejected = anyhow::Error::msg(r#"Token request failed (HTTP 400 Bad Request): {"error":"invalid_grant"}"#)
            .context("Failed to get access token");
        assert_eq!(
            InteractionOutcome::from_wallet_error(&rejected),
            InteractionOutcome::Rejected {
                error_code: Some("invalid_grant".to_string()),
                error_description: None,
                http_status: Some(400),
            }
        );

        let not_sent = anyhow::anyhow!("error sending request for url (https://example.com/auth/token)");
        assert!(matches!(
            InteractionOutcome::from_wallet_error(&not_sent),
            InteractionOutcome::NotSent { .. }
        ));
    }
}
