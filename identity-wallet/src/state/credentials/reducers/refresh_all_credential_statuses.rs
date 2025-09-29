use log::{info, warn};

use crate::{
    error::AppError,
    state::{
        actions::Action,
        core_utils::DateUtils,
        credentials::{reducers::refresh_credential_status::fetch_credential_status, VerifiableCredentialRecord},
        AppState,
    },
};

// TODO: test for possible poor latency/performance due to many stronghold interactions when many credentials need to be refreshed.
/// Refreshes the credential status for all credentials in the state.
pub async fn refresh_all_credential_statuses(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let state_guard = state.core_utils.managers.lock().await;
    let mut credentials = state.credentials.clone();

    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(AppError::MissingManagerError("stronghold"))?;

    for credential in credentials.iter_mut() {
        let credential_id = &credential.id;
        if let Some(credential_status_data) = credential.status.as_mut() {
            match fetch_credential_status(credential_status_data, state_guard.identity_manager.as_ref().unwrap()).await
            {
                Ok(status) => {
                    info!("Successfully fetched new credential status {status:?} for credential with id: `{credential_id}`. The old_status was: {:?}", credential_status_data.status);
                    credential_status_data.last_checked = DateUtils::new_date_string();
                    credential_status_data.status = status;

                    // Update the credential in StrongHold
                    {
                        let key: uuid::Uuid = credential.id.parse().map_err(AppError::InvalidUuidError)?;

                        let mut verifiable_credential_record = stronghold_manager
                            .remove(key)
                            .map_err(AppError::StrongholdDeletionError)?
                            .and_then(|data| serde_json::from_slice::<VerifiableCredentialRecord>(data.as_slice()).ok())
                            .ok_or(AppError::StrongholdMissingCredentialError(key))?;

                        verifiable_credential_record.display_credential = credential.clone();

                        stronghold_manager
                            .insert(
                                key,
                                serde_json::json!(verifiable_credential_record)
                                    .to_string()
                                    .as_bytes()
                                    .to_vec(),
                            )
                            .map_err(AppError::StrongholdInsertionError)?;
                    }
                }
                Err(e) => {
                    // This error handling means we don't panic when the refresh_credential_status function fails.
                    // Instead we don't bother the user with any of the errors and keep the old status and simply don't update it.
                    // However, this is also not ideal. TODO: how to handle a status that consistently fails to refresh?
                    warn!("Failed to refresh credential status for credential with id: `{credential_id}`.\nThe old status remains unchanged: {:?}.\nError: {e}", credential_status_data.status);
                    // Continue to the next credential instead of returning an error for the whole operation.
                    continue;
                }
            };
        } else {
            // The frontend should already be displaying the fact that there is no credentialStatus for this credential, so only a log message is enough here.
            info!("No credentialStatus found for credential with id: `{credential_id}`");
            continue;
        }
    }

    drop(state_guard);

    Ok(AppState {
        credentials,
        ..state.to_owned()
    })
}
