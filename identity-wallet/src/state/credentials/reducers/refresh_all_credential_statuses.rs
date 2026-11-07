use log::{info, warn};
use std::collections::{HashMap, HashSet};

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        common::actions::unlock_storage::UnlockStorage,
        core_utils::DateUtils,
        credentials::{
            reducers::refresh_credential_status::{fetch_verified_status_list, status_at},
            VerifiableCredentialRecord,
        },
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

/// Refreshes the status of every credential that has one.
///
/// Credentials often share a status list, so each list is fetched once, and all of them at the same time. A list that
/// cannot be fetched or verified leaves the statuses that depend on it as they were. Only credentials whose status
/// changed are written back to Stronghold, in a single commit: each commit encrypts and rewrites the whole snapshot.
#[tracing::instrument(skip_all, err)]
pub async fn refresh_all_credential_statuses(state: AppState, action: Action) -> Result<AppState, AppError> {
    let redirect_prompt = listen::<UnlockStorage>(action).map(|_unlock_storage| CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    let status_list_uris: HashSet<String> = state
        .credentials
        .iter()
        .filter_map(|credential| credential.credential_status.as_ref())
        .map(|credential_status| credential_status.uri.to_string())
        .collect();

    if status_list_uris.is_empty() {
        return Ok(AppState {
            current_user_prompt: redirect_prompt,
            ..state
        });
    }

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

    info!(
        "Refreshing the statuses of {} credentials from {} status lists",
        state.credentials.len(),
        status_list_uris.len()
    );

    let status_lists: HashMap<String, _> = futures::future::join_all(status_list_uris.into_iter().map(|uri| {
        let subject = &subject;
        async move {
            let status_list = fetch_verified_status_list(&uri, subject).await;
            (uri, status_list)
        }
    }))
    .await
    .into_iter()
    .filter_map(|(uri, status_list)| {
        status_list
            .inspect_err(|e| warn!("Failed to refresh status list `{uri}`, keeping the statuses it holds: {e}"))
            .ok()
            .map(|status_list| (uri, status_list))
    })
    .collect();

    let now = DateUtils::new_date_string();
    let mut credentials = state.credentials.clone();
    let mut changed = Vec::new();

    for credential in credentials.iter_mut() {
        let Some(credential_status) = credential.credential_status.as_mut() else {
            continue;
        };
        let Some(status_list) = status_lists.get(&credential_status.uri.to_string()) else {
            continue;
        };

        match status_at(status_list, credential_status.idx) {
            Ok(status) => {
                credential_status.last_checked = now.clone();
                if status != credential_status.status {
                    info!(
                        "Status of credential `{}` changed from `{:?}` to `{status:?}`",
                        credential.id, credential_status.status
                    );
                    credential_status.status = status;
                    changed.push(credential.clone());
                }
            }
            Err(e) => warn!("Failed to read the status of credential `{}`: {e}", credential.id),
        }
    }

    if !changed.is_empty() {
        let managers = state.core_utils.managers.lock().await;
        let stronghold_manager = managers
            .stronghold_manager
            .as_ref()
            .ok_or(AppError::MissingManagerError("stronghold"))?;

        let mut entries = Vec::with_capacity(changed.len());
        for updated_credential in changed {
            let key: uuid::Uuid = updated_credential.id.parse().map_err(AppError::InvalidUuidError)?;
            let mut verifiable_credential_record = stronghold_manager
                .get(key)
                .map_err(AppError::StrongholdValuesError)?
                .and_then(|data| serde_json::from_slice::<VerifiableCredentialRecord>(&data).ok())
                .ok_or(AppError::StrongholdMissingCredentialError(key))?;
            verifiable_credential_record.display_credential = updated_credential;
            entries.push((key, serde_json::to_vec(&verifiable_credential_record)?));
        }

        stronghold_manager
            .insert_all(entries)
            .map_err(AppError::StrongholdInsertionError)?;
    }

    Ok(AppState {
        credentials,
        current_user_prompt: redirect_prompt,
        ..state
    })
}
