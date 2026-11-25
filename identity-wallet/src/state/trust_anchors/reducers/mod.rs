use crate::error::AppError::{self, *};
use crate::http_client::{get_http_client, plain_http_allowed};
use crate::logging::url_host;
use crate::state::trust_anchors::actions::{FetchTrustAnchor, RemoveTrustAnchor, ToggleTrustAnchor};
use crate::state::trust_anchors::certificate::{self, MAX_CERTIFICATE_SIZE};
use crate::state::trust_anchors::{
    PendingTrustAnchor, TrustAnchor, TrustAnchorRefusal, TrustAnchorSource, TrustAnchors,
};
use crate::state::{
    actions::{listen, Action},
    AppState,
};

use log::info;
use url::Url;

/// Fetches the certificate at the URL of a [`FetchTrustAnchor`] action, PEM or DER, and holds it for the user to
/// review, with the reason it cannot be a trust anchor if it cannot.
#[tracing::instrument(skip_all, err)]
pub async fn fetch_trust_anchor(state: AppState, action: Action) -> Result<AppState, AppError> {
    let Some(FetchTrustAnchor { url }) = listen::<FetchTrustAnchor>(action) else {
        return Ok(state);
    };
    let url = Url::parse(url.trim()).map_err(|_| InvalidInputError("Not a URL".to_string()))?;
    check_scheme(&url, plain_http_allowed())?;
    let host = url_host(&url);

    let bytes = download(&url).await?;
    let (pem, details) = certificate::read(&bytes)
        .map_err(|error| InvalidTrustAnchorError(format!("{host} serves no certificate: {error:#}")))?;

    let refusal = if state
        .trust_anchors
        .anchors
        .iter()
        .any(|anchor| anchor.id == details.sha256_thumbprint)
    {
        Some(TrustAnchorRefusal::AlreadyTrusted)
    } else {
        certificate::refusal(&pem, &details, chrono::Utc::now().timestamp())
            .map_err(|error| InvalidTrustAnchorError(format!("{error:#}")))?
    };
    info!("Fetched a certificate from {host} to review as a trust anchor, refusal: {refusal:?}");

    let anchor = TrustAnchor {
        id: details.sha256_thumbprint.clone(),
        name: certificate::subject_name(pem.as_bytes()).unwrap_or_else(|| details.subject.clone()),
        source: TrustAnchorSource::Added {
            url: Some(url.to_string()),
        },
        enabled: true,
        certificate: pem,
        details,
    };
    Ok(AppState {
        trust_anchors: TrustAnchors {
            pending: Some(PendingTrustAnchor { anchor, refusal }),
            ..state.trust_anchors
        },
        ..state
    })
}

/// Refuses a URL the wallet's HTTP client would refuse, as the user's input rather than as a failed request: `https`
/// only, unless plain `http` is allowed (see [`plain_http_allowed`]).
fn check_scheme(url: &Url, plain_http: bool) -> Result<(), AppError> {
    match url.scheme() {
        "https" => Ok(()),
        "http" if plain_http => Ok(()),
        scheme => Err(InvalidInputError(format!(
            "Only `https` URLs are fetched, not `{scheme}`"
        ))),
    }
}

/// The body of `url`, at most [`MAX_CERTIFICATE_SIZE`] bytes.
async fn download(url: &Url) -> Result<Vec<u8>, AppError> {
    let mut response = get_http_client()
        .await
        .get(url.clone())
        .send()
        .await
        .map_err(|error| Error(format!("Failed to fetch the certificate: {error:#}")))?;
    let status = response.status();
    if !status.is_success() {
        return Err(Error(format!(
            "Failed to fetch the certificate (HTTP {})",
            status.as_u16()
        )));
    }

    let too_large = || InvalidTrustAnchorError(format!("{} serves more than a certificate", url_host(url)));
    if response
        .content_length()
        .is_some_and(|length| length > MAX_CERTIFICATE_SIZE as u64)
    {
        return Err(too_large());
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| Error(format!("Failed to fetch the certificate: {error:#}")))?
    {
        bytes.extend_from_slice(&chunk);
        if bytes.len() > MAX_CERTIFICATE_SIZE {
            return Err(too_large());
        }
    }
    Ok(bytes)
}

/// Adds the certificate under review to the trust anchors, enabled.
#[tracing::instrument(skip_all, err)]
pub async fn accept_trust_anchor(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let mut trust_anchors = state.trust_anchors;
    let Some(PendingTrustAnchor { anchor, refusal }) = trust_anchors.pending.take() else {
        return Err(InvalidTrustAnchorError("No certificate to accept".to_string()));
    };
    if let Some(refusal) = refusal {
        return Err(InvalidTrustAnchorError(format!(
            "The certificate cannot be accepted: {refusal:?}"
        )));
    }
    if trust_anchors.anchors.iter().any(|trusted| trusted.id == anchor.id) {
        return Err(InvalidTrustAnchorError("Already a trust anchor".to_string()));
    }
    info!("Added trust anchor {}", anchor.id);
    trust_anchors.anchors.push(anchor);
    Ok(AppState { trust_anchors, ..state })
}

/// Discards the certificate under review.
#[tracing::instrument(skip_all, err)]
pub async fn deny_trust_anchor(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        trust_anchors: TrustAnchors {
            pending: None,
            ..state.trust_anchors
        },
        ..state
    })
}

#[tracing::instrument(skip_all, err)]
pub async fn toggle_trust_anchor(state: AppState, action: Action) -> Result<AppState, AppError> {
    let Some(ToggleTrustAnchor { id }) = listen::<ToggleTrustAnchor>(action) else {
        return Ok(state);
    };
    let mut trust_anchors = state.trust_anchors;
    let anchor = trust_anchors
        .find_mut(&id)
        .ok_or_else(|| TrustAnchorNotFoundError(id.clone()))?;
    anchor.enabled = !anchor.enabled;
    info!("Trust anchor {id} enabled: {}", anchor.enabled);
    Ok(AppState { trust_anchors, ..state })
}

/// Removes a trust anchor the user added. A curated one can only be disabled.
#[tracing::instrument(skip_all, err)]
pub async fn remove_trust_anchor(state: AppState, action: Action) -> Result<AppState, AppError> {
    let Some(RemoveTrustAnchor { id }) = listen::<RemoveTrustAnchor>(action) else {
        return Ok(state);
    };
    let mut trust_anchors = state.trust_anchors;
    let index = trust_anchors
        .anchors
        .iter()
        .position(|anchor| anchor.id == id)
        .ok_or_else(|| TrustAnchorNotFoundError(id.clone()))?;
    if matches!(trust_anchors.anchors[index].source, TrustAnchorSource::Curated { .. }) {
        return Err(InvalidTrustAnchorError(
            "A curated trust anchor can only be disabled".to_string(),
        ));
    }
    trust_anchors.anchors.remove(index);
    info!("Removed trust anchor {id}");
    Ok(AppState { trust_anchors, ..state })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::trust_anchors::actions::{AcceptTrustAnchor, DenyTrustAnchor};
    use crate::state::trust_anchors::tests::{anchor, root_der};
    use std::sync::Arc;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn serve(body: Vec<u8>) -> (MockServer, String) {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/root.crt"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(body))
            .mount(&mock_server)
            .await;
        let url = format!("{}/root.crt", mock_server.uri());
        (mock_server, url)
    }

    async fn fetch(state: AppState, url: &str) -> Result<AppState, AppError> {
        fetch_trust_anchor(state, Arc::new(FetchTrustAnchor { url: url.to_string() })).await
    }

    #[tokio::test]
    async fn fetches_a_certificate_for_review_and_adds_it_when_accepted() {
        let (_server, url) = serve(root_der()).await;

        let state = fetch(AppState::default(), &url).await.unwrap();

        let pending = state.trust_anchors.pending.clone().unwrap();
        assert_eq!(pending.refusal, None);
        assert_eq!(pending.anchor.name, "x509-limbo-root");
        assert_eq!(
            pending.anchor.source,
            TrustAnchorSource::Added { url: Some(url.clone()) }
        );
        assert!(state.trust_anchors.anchors.is_empty());

        let state = accept_trust_anchor(state, Arc::new(AcceptTrustAnchor)).await.unwrap();
        assert_eq!(state.trust_anchors.pending, None);
        assert_eq!(state.trust_anchors.anchors, vec![pending.anchor]);
        assert_eq!(state.trust_anchors.enabled().len(), 1);
    }

    #[tokio::test]
    async fn discards_a_denied_certificate() {
        let (_server, url) = serve(root_der()).await;
        let state = fetch(AppState::default(), &url).await.unwrap();

        let state = deny_trust_anchor(state, Arc::new(DenyTrustAnchor)).await.unwrap();

        assert_eq!(state.trust_anchors, TrustAnchors::default());
    }

    #[tokio::test]
    async fn refuses_to_accept_a_certificate_already_trusted() {
        let (_server, url) = serve(root_der()).await;
        let mut state = AppState::default();
        state.trust_anchors.anchors.push(anchor(
            TrustAnchorSource::Curated {
                enabled_by_default: false,
            },
            false,
        ));

        let state = fetch(state, &url).await.unwrap();

        assert_eq!(
            state.trust_anchors.pending.as_ref().unwrap().refusal,
            Some(TrustAnchorRefusal::AlreadyTrusted)
        );
        let error = accept_trust_anchor(state, Arc::new(AcceptTrustAnchor))
            .await
            .unwrap_err();
        assert!(matches!(error, InvalidTrustAnchorError(_)));
    }

    #[tokio::test]
    async fn reports_a_url_that_serves_no_certificate() {
        let (_server, url) = serve(b"<html>Hello</html>".to_vec()).await;

        let error = fetch(AppState::default(), &url).await.unwrap_err();

        assert!(matches!(error, InvalidTrustAnchorError(_)));
    }

    #[tokio::test]
    async fn reports_a_url_that_serves_too_much() {
        let (_server, url) = serve(vec![0; MAX_CERTIFICATE_SIZE + 1]).await;

        let error = fetch(AppState::default(), &url).await.unwrap_err();

        assert!(matches!(error, InvalidTrustAnchorError(_)));
    }

    #[tokio::test]
    async fn reports_the_http_status_of_a_failed_download() {
        let mock_server = MockServer::start().await;

        let error = fetch(AppState::default(), &format!("{}/missing.crt", mock_server.uri()))
            .await
            .unwrap_err();

        assert!(error.to_string().contains("(HTTP 404)"), "{error}");
    }

    #[tokio::test]
    async fn refuses_what_is_no_url() {
        let error = fetch(AppState::default(), "root.crt").await.unwrap_err();

        assert!(matches!(error, InvalidInputError(_)));
    }

    #[test]
    fn refuses_a_url_other_than_https_unless_plain_http_is_allowed() {
        let http = Url::parse("http://ca.example/root.crt").unwrap();

        assert!(matches!(check_scheme(&http, false), Err(InvalidInputError(_))));
        assert!(check_scheme(&http, true).is_ok());
        assert!(check_scheme(&Url::parse("https://ca.example/root.crt").unwrap(), false).is_ok());
        assert!(matches!(
            check_scheme(&Url::parse("file:///etc/ssl/root.crt").unwrap(), true),
            Err(InvalidInputError(_))
        ));
    }

    #[tokio::test]
    async fn toggles_any_trust_anchor() {
        let mut state = AppState::default();
        let curated = anchor(
            TrustAnchorSource::Curated {
                enabled_by_default: true,
            },
            true,
        );
        state.trust_anchors.anchors.push(curated.clone());

        let state = toggle_trust_anchor(state, Arc::new(ToggleTrustAnchor { id: curated.id.clone() }))
            .await
            .unwrap();

        assert!(!state.trust_anchors.anchors[0].enabled);
        assert!(state.trust_anchors.enabled().is_empty());
    }

    #[tokio::test]
    async fn removes_only_an_added_trust_anchor() {
        let added = anchor(
            TrustAnchorSource::Added {
                url: Some("https://ca.example/root.crt".to_string()),
            },
            true,
        );
        let mut state = AppState::default();
        state.trust_anchors.anchors.push(added.clone());

        let state = remove_trust_anchor(state, Arc::new(RemoveTrustAnchor { id: added.id.clone() }))
            .await
            .unwrap();
        assert!(state.trust_anchors.anchors.is_empty());

        let mut state = AppState::default();
        let curated = anchor(
            TrustAnchorSource::Curated {
                enabled_by_default: true,
            },
            true,
        );
        state.trust_anchors.anchors.push(curated.clone());
        let error = remove_trust_anchor(state, Arc::new(RemoveTrustAnchor { id: curated.id }))
            .await
            .unwrap_err();
        assert!(matches!(error, InvalidTrustAnchorError(_)));
    }
}
