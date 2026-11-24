pub mod actions;
mod reducers;

use oid4vc_core::{
    authorization_response::AuthorizationResponse, openid4vc_extension::Extension,
    utils::form_urlencoded::to_form_urlencoded_string,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

use crate::error::AppError;
use crate::http_client::post_encoded_form_to_public_destination;

/// The outcome of the last interaction with an issuer or verifier, shown to the user in the result pop-up.
#[derive(Debug, TS, Clone, Serialize, Deserialize, PartialEq)]
#[ts(export, export_to = "bindings/interaction_result/InteractionResult.ts")]
pub struct InteractionResult {
    pub kind: InteractionKind,
    /// Display name of the issuer or verifier.
    pub counterparty: String,
    pub outcome: InteractionOutcome,
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

/// A one-line description for the logs and the Debug screen, e.g. `rejected (HTTP 400, invalid_request: Missing nonce)`.
impl std::fmt::Display for InteractionOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionOutcome::Approved => write!(f, "approved"),
            InteractionOutcome::Rejected {
                error_code,
                error_description,
                http_status,
            } => {
                let status = http_status.map(|status| format!("HTTP {status}"));
                let error = match (error_code, error_description) {
                    (Some(code), Some(description)) => Some(format!("{code}: {description}")),
                    (code, description) => code.clone().or(description.clone()),
                };
                let details: Vec<String> = status.into_iter().chain(error).collect();
                if details.is_empty() {
                    write!(f, "rejected")
                } else {
                    write!(f, "rejected ({})", details.join(", "))
                }
            }
            InteractionOutcome::NotSent { reason } => write!(f, "not sent ({reason})"),
        }
    }
}

/// Sends an authorization response with the `direct_post` response mode and reports how the verifier answered.
///
/// Unlike `ProviderManager::send_response`, this reads the response body, so a verifier's error code reaches the user.
pub async fn send_authorization_response<E: Extension>(
    authorization_response: &AuthorizationResponse<E>,
) -> InteractionOutcome {
    match to_form_urlencoded_string(authorization_response) {
        Ok(encoded) => post_authorization_response(&authorization_response.redirect_uri, encoded).await,
        Err(error) => InteractionOutcome::NotSent {
            reason: error.to_string(),
        },
    }
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
async fn post_authorization_response(response_uri: &str, encoded: String) -> InteractionOutcome {
    let not_sent = |error: anyhow::Error| InteractionOutcome::NotSent {
        reason: format!("{:#}", error.context("Failed to send the authorization response")),
    };

    let response_uri = match Url::parse(response_uri) {
        Ok(response_uri) => response_uri,
        Err(error) => return not_sent(error.into()),
    };
    let response = match post_encoded_form_to_public_destination(&response_uri, encoded).await {
        Ok(response) => response,
        Err(error) => return not_sent(without_url(error).into()),
    };

    let status = response.status();
    if status.is_success() {
        return InteractionOutcome::Approved;
    }

    // An answer that is too large or cannot be read is treated as an empty one: the status still decides the outcome.
    let body = bounded_text(response).await.unwrap_or_default();
    InteractionOutcome::rejected(status.as_u16(), &body)
}

/// Drops the request URI from a failed send. `reqwest` adds the full URI, query included, to its `Display` output, and
/// the reason reaches the logs, the Debug screen and the user, while the verifier's `response_uri` may carry request
/// parameters.
fn without_url(error: AppError) -> AppError {
    match error {
        AppError::DownloadFailed(error) => AppError::DownloadFailed(error.without_url()),
        error => error,
    }
}

/// The largest answer of a Response Endpoint that is read: far above any error body.
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

    async fn post_to(response_endpoint: &MockServer) -> InteractionOutcome {
        post_authorization_response(&format!("{}/response", response_endpoint.uri()), ENCODED.to_string()).await
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
                post_authorization_response(response_uri, ENCODED.to_string()).await,
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

    #[tokio::test]
    async fn a_failed_send_does_not_reveal_the_response_uri() {
        // Nothing listens on port 1, so the request fails before reaching a server.
        let outcome =
            post_authorization_response("http://127.0.0.1:1/callback?session=secret", ENCODED.to_string()).await;

        let InteractionOutcome::NotSent { reason } = outcome else {
            panic!("expected NotSent, got {outcome:?}");
        };
        assert!(
            reason.starts_with("Failed to send the authorization response: "),
            "{reason}"
        );
        assert!(!reason.contains("127.0.0.1") && !reason.contains("secret"), "{reason}");
    }

    #[test]
    fn displays_the_outcome_on_one_line() {
        assert_eq!(InteractionOutcome::Approved.to_string(), "approved");
        assert_eq!(
            InteractionOutcome::rejected(
                400,
                r#"{"error":"invalid_request","error_description":"Missing nonce"}"#
            )
            .to_string(),
            "rejected (HTTP 400, invalid_request: Missing nonce)"
        );
        assert_eq!(InteractionOutcome::rejected(500, "").to_string(), "rejected (HTTP 500)");
        assert_eq!(
            InteractionOutcome::NotSent {
                reason: "connection refused".to_string()
            }
            .to_string(),
            "not sent (connection refused)"
        );
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
