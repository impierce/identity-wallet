pub mod actions;
mod reducers;

use anyhow::Context;
use oid4vc::oid4vc_core::{
    authorization_response::AuthorizationResponse, openid4vc_extension::Extension,
    utils::form_urlencoded::to_form_urlencoded_string,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

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

/// Sends an authorization response with the `direct_post` response mode and reports how the verifier answered.
///
/// Unlike `ProviderManager::send_response`, this reads the response body, so a verifier's error code reaches the user.
pub async fn send_authorization_response<E: Extension>(
    authorization_response: &AuthorizationResponse<E>,
) -> InteractionOutcome {
    let encoded = match to_form_urlencoded_string(authorization_response) {
        Ok(encoded) => encoded,
        Err(error) => {
            return InteractionOutcome::NotSent {
                reason: error.to_string(),
            }
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
        }
    };

    let status = response.status();
    if status.is_success() {
        return InteractionOutcome::Approved;
    }

    let body = response.text().await.unwrap_or_default();
    InteractionOutcome::rejected(status.as_u16(), &body)
}

#[cfg(test)]
mod tests {
    use super::*;

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
