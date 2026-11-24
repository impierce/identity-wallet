use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use crate::{
    error::AppError,
    http_client::get_http_client_builder,
    logging::url_host,
    state::{
        actions::{listen, Action},
        core_utils::DateUtils,
        credentials::{
            actions::refresh_credential_status::RefreshCredentialStatus, CredentialStatus, CredentialStatusType,
            DisplayCredential, VerifiableCredentialRecord,
        },
        AppState,
    },
    subject::Subject,
};
use futures::StreamExt;
use identity_kit::{status_list::StatusListToken, vc::Jwt};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use log::{info, warn};
use oid4vc_core::{utils::did::extract_normalized_did_kid_from_jwt, Verify};
use reqwest::{header, redirect::Policy};
use url::Url;

/// How many status lists are requested at the same time when several credentials are refreshed.
const MAX_CONCURRENT_STATUS_LIST_REQUESTS: usize = 8;

/// How long a refresh waits for status lists. It stays below the action's time limit (`DEFAULT_ACTION_TIME_LIMIT`),
/// so that what has been fetched is still applied and saved: a credential whose status list has not answered by then
/// keeps its status, as it does when the request fails.
const STATUS_REFRESH_BUDGET: Duration = Duration::from_secs(7);

#[tracing::instrument(skip_all, err)]
pub async fn refresh_credential_status(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(refresh_credential_status) = listen::<RefreshCredentialStatus>(action) {
        let credential_id = refresh_credential_status.credential_id;

        let credential = state
            .credentials
            .iter()
            .find(|c| c.id == credential_id)
            .ok_or_else(|| {
                // This should never happen, as the credential ID send by the frontend is supposed to be valid.
                warn!("No credential found with id: `{credential_id}`");
                AppError::NoCredentialWithIdError(credential_id.clone())
            })?;

        if credential.credential_status.is_none() {
            // The frontend should already be displaying the fact that there is no credentialStatus for this credential, so only a log message is enough here.
            warn!("No credential status found for credential with id: `{credential_id}`");

            // Abort the function without any changes to the state.
            // No error needed here since the credential_status is optional.
            return Ok(AppState {
                current_user_prompt: None,
                ..state
            });
        }

        let display_name = credential.display_name.clone();
        let (state, summary) = refresh_statuses(state, std::slice::from_ref(&credential_id)).await?;

        if summary.refreshed == 1 {
            crate::debug_log_ok!(
                state.core_utils.debug_log,
                "Refreshing credential status for `{display_name}` took {}ms (success)",
                summary.duration.as_millis()
            );
        } else {
            crate::debug_log_error!(
                state.core_utils.debug_log,
                "Refreshing credential status for `{display_name}` took {}ms (failed)",
                summary.duration.as_millis()
            );
        }

        return Ok(AppState {
            current_user_prompt: None,
            ..state
        });
    }

    Ok(state)
}

/// The outcome of [`refresh_statuses`].
#[derive(Debug, PartialEq)]
pub struct RefreshSummary {
    /// Credentials whose status was refreshed.
    pub refreshed: usize,
    /// Credentials whose status list could not be fetched, did not verify or did not answer in time. They keep their
    /// status and `last_checked`.
    pub failed: usize,
    /// The distinct status lists requested.
    pub status_lists: usize,
    pub duration: Duration,
}

/// Refreshes the status of the credentials `credential_ids` that have a `credentialStatus`, and saves those refreshed
/// to Stronghold.
///
/// - Each status list is requested once, however many credentials refer to it, and its signature is verified once.
/// - At most [`MAX_CONCURRENT_STATUS_LIST_REQUESTS`] status lists are requested at the same time.
/// - Status lists that have not answered after [`STATUS_REFRESH_BUDGET`] count as failed.
/// - The refreshed credentials are saved with one Stronghold commit, which encrypts and writes the whole snapshot.
pub async fn refresh_statuses(
    state: AppState,
    credential_ids: &[String],
) -> Result<(AppState, RefreshSummary), AppError> {
    let start = Instant::now();

    let subject = state
        .core_utils
        .managers
        .lock()
        .await
        .identity_manager
        .as_ref()
        .ok_or(AppError::MissingManagerError("identity"))?
        .subject
        .clone();

    let status_lists: HashSet<Url> = state
        .credentials
        .iter()
        .filter(|credential| credential_ids.contains(&credential.id))
        .filter_map(|credential| credential.credential_status.as_ref())
        .map(|credential_status| credential_status.uri.clone())
        .collect();

    let tokens = fetch_status_list_tokens(status_lists.iter().cloned(), &subject).await;

    let mut credentials = state.credentials.clone();
    let mut refreshed_ids = Vec::new();
    let mut failed = 0;
    for credential in credentials
        .iter_mut()
        .filter(|credential| credential_ids.contains(&credential.id))
    {
        let Some(credential_status) = credential.credential_status.as_mut() else {
            continue;
        };
        let status = match tokens.get(&credential_status.uri) {
            Some(Ok(token)) => status_at(token, credential_status.idx),
            Some(Err(e)) => Err(e.clone()),
            None => Err(StatusRefreshError::NoAnswer),
        };
        match status {
            Ok(status) => {
                info!(
                    "Refreshed the status of credential `{}`: `{status:?}` (previous status: `{:?}`)",
                    credential.id, credential_status.status
                );
                credential_status.status = status;
                credential_status.last_checked = DateUtils::new_date_string();
                refreshed_ids.push(credential.id.clone());
            }
            Err(e) => {
                // We log the error, keep the status and the last_checked field the same.
                // We don't make any assumptions on the credential status since the error has to do with the reachability of the status list, not the status itself.
                // However, it is up to the verifier how to handle a status that is unreachable.
                // This means a seemingly valid credential could be rejected when a user shares it.
                warn!(
                    "Failed to refresh the status of credential `{}`, it stays `{:?}`: {e}",
                    credential.id, credential_status.status
                );
                failed += 1;
            }
        }
    }

    save_credentials(&state, &credentials, &refreshed_ids).await?;

    let summary = RefreshSummary {
        refreshed: refreshed_ids.len(),
        failed,
        status_lists: status_lists.len(),
        duration: start.elapsed(),
    };
    info!(
        "Refreshed the status of {} credential(s) from {} status list(s) in {}ms, {} failed",
        summary.refreshed,
        summary.status_lists,
        summary.duration.as_millis(),
        summary.failed
    );

    Ok((AppState { credentials, ..state }, summary))
}

/// Why a credential's status could not be refreshed.
#[derive(Debug, Clone, thiserror::Error)]
enum StatusRefreshError {
    #[error("{0}")]
    Fetch(String),
    #[error("its status list did not answer within {STATUS_REFRESH_BUDGET:?}")]
    NoAnswer,
}

/// Fetches and verifies the status lists `uris`, at most [`MAX_CONCURRENT_STATUS_LIST_REQUESTS`] at the same time. A
/// status list that has not answered after [`STATUS_REFRESH_BUDGET`] is left out.
async fn fetch_status_list_tokens(
    uris: impl IntoIterator<Item = Url>,
    subject: &Subject,
) -> HashMap<Url, Result<StatusListToken, StatusRefreshError>> {
    let mut fetches = futures::stream::iter(uris)
        .map(|uri| async move {
            let token = fetch_status_list_token(&uri, subject)
                .await
                .map_err(|e| StatusRefreshError::Fetch(e.to_string()));
            (uri, token)
        })
        .buffer_unordered(MAX_CONCURRENT_STATUS_LIST_REQUESTS);

    let deadline = tokio::time::Instant::now() + STATUS_REFRESH_BUDGET;
    let mut tokens = HashMap::new();
    while let Ok(Some((uri, token))) = tokio::time::timeout_at(deadline, fetches.next()).await {
        tokens.insert(uri, token);
    }
    tokens
}

/// Saves the credentials `ids` of `credentials` to Stronghold, with a single commit.
async fn save_credentials(state: &AppState, credentials: &[DisplayCredential], ids: &[String]) -> Result<(), AppError> {
    if ids.is_empty() {
        return Ok(());
    }

    let managers = state.core_utils.managers.lock().await;
    let stronghold_manager = managers
        .stronghold_manager
        .as_ref()
        .ok_or(AppError::MissingManagerError("stronghold"))?;

    let mut records = Vec::with_capacity(ids.len());
    for credential in credentials.iter().filter(|credential| ids.contains(&credential.id)) {
        let key: uuid::Uuid = credential.id.parse().map_err(AppError::InvalidUuidError)?;
        let mut verifiable_credential_record = stronghold_manager
            .get(key)
            .map_err(AppError::StrongholdValuesError)?
            .and_then(|data| serde_json::from_slice::<VerifiableCredentialRecord>(data.as_slice()).ok())
            .ok_or(AppError::StrongholdMissingCredentialError(key))?;
        verifiable_credential_record.display_credential = credential.clone();
        records.push((
            key,
            serde_json::json!(verifiable_credential_record).to_string().into_bytes(),
        ));
    }

    stronghold_manager
        .insert_all(records)
        .map_err(AppError::StrongholdInsertionError)
}

// Helpers

/// Fetches the Status List Token from the Status Provider URI provided in the credentialStatus, and checks the given index, returning the Status.
/// There are multiple decoding and decompressing steps involved, please refer to the OAuth Token Status List specification for more details.
#[tracing::instrument(skip_all, err)]
pub async fn fetch_credential_status(
    credential_status_data: &CredentialStatus,
    subject: &Subject,
) -> Result<CredentialStatusType, AppError> {
    let token = fetch_status_list_token(&credential_status_data.uri, subject).await?;
    status_at(&token, credential_status_data.idx).map_err(|_| AppError::GetCredentialStatusError)
}

/// Fetches the Status List Token at `uri`, verifies its signature with the key its `kid` names and checks its claims.
#[tracing::instrument(skip_all, err)]
pub async fn fetch_status_list_token(uri: &Url, subject: &Subject) -> Result<StatusListToken, AppError> {
    // TODO: we only request JWTs, since we can't handle CWT yet. However when we implement CWT we then need some way to discover what encoding the Status List Provider is using.
    let status_list_jwt = fetch_status_list(uri.as_str()).await?;

    // Checks that `typ` is `statuslist+jwt` and reads the claims; the signature is verified below.
    let invalid_token = |e: &dyn std::fmt::Display| {
        warn!("Invalid status list token: {e}");
        AppError::GetCredentialStatusError
    };
    let jwt = Jwt::new(status_list_jwt.as_str()).map_err(|e| invalid_token(&e))?;
    let (_, status_list_token) = StatusListToken::decode(&jwt).map_err(|e| invalid_token(&e))?;

    let jwt_header = decode_header(&status_list_jwt).map_err(|_| AppError::GetCredentialStatusError)?;
    let key_id =
        extract_normalized_did_kid_from_jwt(&status_list_jwt).map_err(|_| AppError::GetCredentialStatusError)?;

    let public_key = subject
        .public_key(&key_id)
        .await
        .map_err(|_| AppError::GetCredentialStatusError)?;
    let decoding_key = match jwt_header.alg {
        Algorithm::EdDSA => DecodingKey::from_ed_der(&public_key),
        Algorithm::ES256 => DecodingKey::from_ec_der(&public_key),
        _ => {
            warn!("Unsupported algorithm: {:?}", jwt_header.alg);
            return Err(AppError::GetCredentialStatusError);
        }
    };
    let mut validation = Validation::new(jwt_header.alg);
    validation.set_required_spec_claims(&["sub", "iat"]);
    validation.validate_exp = false;
    validation.validate_aud = false;
    decode::<serde_json::Value>(&status_list_jwt, &decoding_key, &validation).map_err(|e| {
        warn!("The status list token signature does not verify: {e}");
        AppError::GetCredentialStatusError
    })?;

    check_status_list_token(&status_list_token, chrono::Utc::now().timestamp())?;

    info!("Fetched the status list from {}", url_host(uri));

    Ok(status_list_token)
}

/// The status at `idx` of a verified Status List Token.
fn status_at(token: &StatusListToken, idx: u64) -> Result<CredentialStatusType, StatusRefreshError> {
    // TODO: use `StatusListToken::status_of`, which also checks that `sub` is the referenced URI (Token Status List
    // §8.3). The status list tokens in the integration tests carry a different `sub`.
    token
        .status_list
        .get(idx)
        .ok_or(AppError::GetCredentialStatusError)
        .and_then(CredentialStatusType::try_from)
        .map_err(|_| StatusRefreshError::Fetch(format!("the status list has no valid status at index {idx}")))
}

/// Requests the status list token from the provided URI and returns the response body as a String.
/// A gzip encoded response is decompressed by `reqwest`.
#[tracing::instrument(skip_all, err)]
pub async fn fetch_status_list(uri: &str) -> Result<String, AppError> {
    // 3xx redirects should be followed, but infinite loops are caught after 5 redirects.
    // The timeout of 10 seconds is an estimated guess of how long a status list request should take at maximum.
    let client = get_http_client_builder()
        .redirect(Policy::limited(5))
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(fetch_status_list_error)?;

    client
        .get(uri)
        .header(header::ACCEPT, StatusListToken::MEDIA_TYPE)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(fetch_status_list_error)?
        .text()
        .await
        .map(|body| body.trim().to_owned())
        .map_err(fetch_status_list_error)
}

/// Drops the request URI from a status list request error, so it never reaches the logs or the Debug screen
/// (`reqwest` includes the full URI, query included, in its `Display` output).
fn fetch_status_list_error(error: reqwest::Error) -> AppError {
    AppError::FetchCredentialListError(error.without_url())
}

/// Rejects a status list token with an empty `sub` or list, one issued after `now`, or one that has expired by `now`
/// (`exp` is optional; RFC 7519 §4.1.4 accepts a token only before its `exp`).
fn check_status_list_token(token: &StatusListToken, now: i64) -> Result<(), AppError> {
    let reason = if token.sub.is_empty() || token.status_list.is_empty() {
        "it has an empty `sub` or status list"
    } else if token.iat > now {
        "it was issued in the future"
    } else if token.exp.is_some_and(|exp| exp <= now) {
        "it has expired"
    } else {
        return Ok(());
    };
    warn!("Rejected the status list token: {reason}");
    Err(AppError::GetCredentialStatusError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use identity_kit::status_list::StatusList;

    const NOW: i64 = 1_800_000_000;

    fn token(iat: i64) -> StatusListToken {
        StatusListToken::new(
            "https://example.com/statuslists/1",
            DateTime::<Utc>::from_timestamp(iat, 0).unwrap(),
            StatusList::new(1, 16).unwrap(),
        )
    }

    fn at(timestamp: i64) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap()
    }

    #[test]
    fn accepts_a_token_without_exp() {
        assert!(check_status_list_token(&token(NOW - 60), NOW).is_ok());
    }

    #[test]
    fn accepts_a_token_that_has_not_expired() {
        assert!(check_status_list_token(&token(NOW - 60).with_expires_at(at(NOW + 60)), NOW).is_ok());
    }

    #[test]
    fn rejects_an_expired_token() {
        assert!(check_status_list_token(&token(NOW - 120).with_expires_at(at(NOW - 60)), NOW).is_err());
    }

    #[test]
    fn rejects_a_token_at_its_exp() {
        assert!(check_status_list_token(&token(NOW - 60).with_expires_at(at(NOW)), NOW).is_err());
    }

    #[test]
    fn rejects_a_token_issued_in_the_future() {
        assert!(check_status_list_token(&token(NOW + 3600), NOW).is_err());
    }

    #[test]
    fn rejects_a_token_with_an_empty_sub() {
        let mut token = token(NOW - 60);
        token.sub = String::new();
        assert!(check_status_list_token(&token, NOW).is_err());
    }
}
