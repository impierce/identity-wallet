pub mod actions;
mod reducers;

use anyhow::Context;
use oid4vc_core::{
    authorization_response::AuthorizationResponse, openid4vc_extension::Extension,
    utils::form_urlencoded::to_form_urlencoded_string,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

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

/// How a verifier's Response Endpoint answered an authorization response.
#[derive(Debug, PartialEq)]
pub struct ResponseAnswer {
    pub outcome: InteractionOutcome,
    /// Where the verifier wants the user's browser sent after an accepted response: "When this parameter is present
    /// the Wallet MUST redirect the user agent to this URI" (OpenID4VP 1.0, Section 8.2).
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

/// The `redirect_uri` in the JSON a Response Endpoint answers with, when it is one a browser is sent to: an `https`
/// URL, or an `http` one when `allow_http` (a local verifier, in debug builds). Any other scheme is not opened.
fn redirect_uri_of(body: &str, allow_http: bool) -> Option<Url> {
    let json: serde_json::Value = serde_json::from_str(body).ok()?;
    let redirect_uri = Url::parse(json.get("redirect_uri")?.as_str()?).ok()?;
    match redirect_uri.scheme() {
        "https" => Some(redirect_uri),
        "http" if allow_http => Some(redirect_uri),
        _ => None,
    }
}

/// Sends an authorization response with the `direct_post` response mode and reports how the verifier answered.
///
/// Unlike `ProviderManager::send_response`, this reads the response body, so a verifier's error code and its
/// `redirect_uri` reach the caller.
pub async fn send_authorization_response<E: Extension>(
    authorization_response: &AuthorizationResponse<E>,
) -> ResponseAnswer {
    let encoded = match to_form_urlencoded_string(authorization_response) {
        Ok(encoded) => encoded,
        Err(error) => {
            return InteractionOutcome::NotSent {
                reason: error.to_string(),
            }
            .into()
        }
    };

    // The shared client pins `webpki-roots`; `Client::new()` uses the platform verifier, which on Android
    // rejects certificates without an OCSP responder (such as Let's Encrypt's) as revoked.
    let response = crate::http_client::get_http_client()
        .await
        .post(&authorization_response.redirect_uri)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded)
        .send()
        .await
        .context("Failed to send the authorization response");

    let response = match response {
        Ok(response) => response,
        Err(error) => {
            return InteractionOutcome::NotSent {
                reason: format!("{error:#}"),
            }
            .into()
        }
    };

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if status.is_success() {
        return ResponseAnswer {
            outcome: InteractionOutcome::Approved,
            redirect_uri: redirect_uri_of(&body, cfg!(debug_assertions)),
        };
    }

    InteractionOutcome::rejected(status.as_u16(), &body).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_redirect_uri_of_a_response_endpoint() {
        let body = r#"{"redirect_uri":"https://verifier.example/done#response_code=091535f6"}"#;
        assert_eq!(
            redirect_uri_of(body, false).unwrap().as_str(),
            "https://verifier.example/done#response_code=091535f6"
        );

        // No body, another JSON, or a value that is no URL: nothing to open.
        assert_eq!(redirect_uri_of("", false), None);
        assert_eq!(redirect_uri_of("{}", false), None);
        assert_eq!(redirect_uri_of(r#"{"redirect_uri":"done"}"#, false), None);
        assert_eq!(redirect_uri_of(r#"{"redirect_uri":42}"#, false), None);
    }

    #[test]
    fn only_web_addresses_are_opened() {
        for other in [
            "javascript:alert(1)",
            "file:///etc/passwd",
            "unime://x",
            "data:text/html,x",
        ] {
            let body = serde_json::json!({ "redirect_uri": other }).to_string();
            assert_eq!(redirect_uri_of(&body, true), None, "{other}");
        }

        let local = r#"{"redirect_uri":"http://localhost:8080/done"}"#;
        assert_eq!(redirect_uri_of(local, false), None);
        assert!(redirect_uri_of(local, true).is_some());
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
