//! OpenID4VP requests that UniMe reads and authenticates, but must not answer with credentials. It answers them with
//! an Authorization Error Response at the request's `response_uri` (OpenID4VP 1.0, Sections 5.6 and 8.5), so the
//! Verifier learns why, and then ends the flow with an error for the user.
//!
//! A request whose Request Object cannot be authenticated (a bad signature, a wrong `typ`) never gets here:
//! `validate_request` refuses it, and nothing is sent, since its `response_uri` cannot be trusted.

use log::{info, warn};
use oid4vc_core::authorization_request::{AuthorizationRequest, Object, RedirectOrResponseUri};
use oid4vp::oid4vp::OID4VP;
use serde_json::Value;
use std::time::Duration;
use url::Url;

use crate::http_client::post_form_to_public_destination;
use crate::logging::url_host;

/// How long sending an Authorization Error Response may take. It is sent while the action holds the state, so it
/// stays well inside the action's time limit.
const ERROR_RESPONSE_TIME_LIMIT: Duration = Duration::from_secs(5);

/// Why UniMe refuses a request, as the Authorization Error Response says it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// The OAuth 2.0 / OpenID4VP `error` code.
    pub error: &'static str,
    /// The `error_description`, also shown to the user.
    pub description: String,
}

impl Refusal {
    fn new(error: &'static str, description: &str) -> Self {
        Refusal {
            error,
            description: description.to_string(),
        }
    }
}

/// What a request carries that UniMe must refuse before showing it, if anything.
pub fn refusal_of(request: &AuthorizationRequest<Object>) -> Option<Refusal> {
    let body = &request.body;
    if body.client_id.starts_with("origin:") {
        // Section 5.9.3: "The Wallet MUST NOT accept this Client Identifier Prefix in requests."
        return Some(Refusal::new(
            "invalid_request",
            "The origin Client Identifier Prefix is not accepted in requests.",
        ));
    }
    if response_mode(request) == Some("direct_post") && has_both_uris(request) {
        // Section 8.2: "If the redirect_uri Authorization Request parameter is present when the Response Mode is
        // direct_post, the Wallet MUST return an invalid_request Authorization Response error."
        return Some(Refusal::new(
            "invalid_request",
            "The request has a redirect_uri with the Response Mode direct_post.",
        ));
    }
    if parameter(request, "transaction_data").is_some() {
        // Section 5: "Wallets that do not support this parameter MUST reject requests that contain it."
        return Some(Refusal::new(
            "invalid_transaction_data",
            "UniMe does not support transaction data.",
        ));
    }
    None
}

/// Where an Authorization Error Response to `request` goes: its `response_uri`, for the Response Mode `direct_post`.
/// `None` for another Response Mode, which UniMe cannot answer with an error.
pub fn error_response_uri(request: &AuthorizationRequest<Object>) -> Option<Url> {
    if response_mode(request) != Some("direct_post") {
        return None;
    }
    match &request.body.uri {
        RedirectOrResponseUri::ResponseUri(response_uri) => Some(response_uri.clone()),
        // With both URIs present, the parsed request may hold the `redirect_uri`; the `response_uri` is then among
        // the other parameters.
        RedirectOrResponseUri::RedirectUri(_) => parameter(request, "response_uri")
            .and_then(Value::as_str)
            .and_then(|uri| Url::parse(uri).ok()),
    }
}

/// Where an Authorization Error Response to a parsed OID4VP request goes: its `response_uri`, for `direct_post`.
pub fn oid4vp_error_response_uri(request: &AuthorizationRequest<Object<OID4VP>>) -> Option<Url> {
    match &request.body.uri {
        RedirectOrResponseUri::ResponseUri(response_uri) if request.body.extension.response_mode == "direct_post" => {
            Some(response_uri.clone())
        }
        _ => None,
    }
}

/// The refusal of a request asking for credentials, or claims, that UniMe does not hold: "If the Wallet cannot deliver
/// all non-optional Credentials requested by the Verifier according to these rules, it MUST NOT return any
/// Credential(s)" (OpenID4VP 1.0, Section 6.4.2), answered with `access_denied` (Section 8.5).
pub fn missing_credentials() -> Refusal {
    Refusal::new(
        "access_denied",
        "The Wallet does not hold every Credential, with every claim, that the request requires.",
    )
}

/// Sends an Authorization Error Response (`error`, `error_description`, and `state` when the request had one) as a
/// form POST to `response_uri`. It can be sent before the user has accepted anything, so it goes to a public
/// destination only, once, without following a redirect, and within [`ERROR_RESPONSE_TIME_LIMIT`]. What the Verifier
/// answers is logged, not returned: the flow ends either way.
pub async fn send_error_response(response_uri: &Url, state: Option<&str>, refusal: &Refusal) {
    let mut form = vec![
        ("error", refusal.error.to_string()),
        ("error_description", refusal.description.clone()),
    ];
    if let Some(state) = state {
        form.push(("state", state.to_string()));
    }
    let host = url_host(response_uri);
    let sent = tokio::time::timeout(
        ERROR_RESPONSE_TIME_LIMIT,
        post_form_to_public_destination(response_uri, &form),
    )
    .await;
    match sent {
        Ok(Ok(response)) => info!(
            "Sent the error response {} to {host}: HTTP {}",
            refusal.error,
            response.status()
        ),
        Ok(Err(error)) => warn!("Failed to send the error response {} to {host}: {error}", refusal.error),
        Err(_) => warn!(
            "Gave up sending the error response {} to {host} after {ERROR_RESPONSE_TIME_LIMIT:?}",
            refusal.error
        ),
    }
}

fn parameter<'a>(request: &'a AuthorizationRequest<Object>, name: &str) -> Option<&'a Value> {
    request.body.extension.get(name)
}

fn response_mode(request: &AuthorizationRequest<Object>) -> Option<&str> {
    parameter(request, "response_mode").and_then(Value::as_str)
}

/// Whether the request has both a `redirect_uri` and a `response_uri`: one of them is parsed into `uri`, the other
/// stays among the parameters.
fn has_both_uris(request: &AuthorizationRequest<Object>) -> bool {
    let other = match request.body.uri {
        RedirectOrResponseUri::RedirectUri(_) => "response_uri",
        RedirectOrResponseUri::ResponseUri(_) => "redirect_uri",
    };
    parameter(request, other).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn request(parameters: Value) -> AuthorizationRequest<Object> {
        let mut object = json!({
            "client_id": "decentralized_identifier:did:jwk:abc",
            "response_type": "vp_token",
            "response_mode": "direct_post",
            "response_uri": "https://verifier.example/response",
            "nonce": "n-1",
            "dcql_query": { "credentials": [] }
        });
        for (name, value) in parameters.as_object().unwrap() {
            if value.is_null() {
                object.as_object_mut().unwrap().remove(name);
            } else {
                object[name] = value.clone();
            }
        }
        serde_json::from_value(object).unwrap()
    }

    #[test]
    fn a_conformant_request_is_not_refused() {
        assert_eq!(refusal_of(&request(json!({}))), None);
        assert_eq!(
            error_response_uri(&request(json!({}))).unwrap().as_str(),
            "https://verifier.example/response"
        );
    }

    #[test]
    fn the_origin_prefix_is_refused_with_invalid_request() {
        let refusal = refusal_of(&request(json!({ "client_id": "origin:https://verifier.example" }))).unwrap();
        assert_eq!(refusal.error, "invalid_request");
    }

    #[test]
    fn a_redirect_uri_with_direct_post_is_refused_and_answered_at_the_response_uri() {
        let both = request(json!({ "redirect_uri": "https://verifier.example/back" }));
        assert_eq!(refusal_of(&both).unwrap().error, "invalid_request");
        assert_eq!(
            error_response_uri(&both).unwrap().as_str(),
            "https://verifier.example/response"
        );
        // A redirect_uri alone, with another Response Mode, is no reason to refuse.
        let fragment = request(json!({
            "response_mode": "fragment",
            "response_uri": null,
            "redirect_uri": "https://verifier.example/back"
        }));
        assert_eq!(refusal_of(&fragment), None);
        assert_eq!(error_response_uri(&fragment), None);
    }

    #[test]
    fn transaction_data_is_refused_with_invalid_transaction_data() {
        let refusal = refusal_of(&request(json!({ "transaction_data": ["eyJ0eXBlIjoieCJ9"] }))).unwrap();
        assert_eq!(refusal.error, "invalid_transaction_data");
    }

    #[tokio::test]
    async fn the_error_response_is_posted_once_and_a_redirect_is_not_followed() {
        use wiremock::matchers::{body_string_contains, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/response"))
            .and(body_string_contains("error=access_denied"))
            .and(body_string_contains("state=s-1"))
            .respond_with(ResponseTemplate::new(307).insert_header("location", "/elsewhere"))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(path("/elsewhere"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&server)
            .await;

        let response_uri = Url::parse(&format!("{}/response", server.uri())).unwrap();
        send_error_response(&response_uri, Some("s-1"), &missing_credentials()).await;
    }

    #[test]
    fn unknown_parameters_are_no_reason_to_refuse() {
        assert_eq!(
            refusal_of(&request(
                json!({ "x_test_unknown": "1", "iss": "https://unrelated.example" })
            )),
            None
        );
    }
}
