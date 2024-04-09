use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        credentials::{actions::update_credential_metadata::UpdateCredentialMetadata, VerifiableCredentialRecord},
        AppState,
    },
};

use log::info;
use serde_json::json;

pub async fn update_credential_metadata(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(UpdateCredentialMetadata {
        id: credential_id,
        name,
        is_favorite,
    }) = listen::<UpdateCredentialMetadata>(action)
    {
        let state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let mut verifiable_credential_record: VerifiableCredentialRecord = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .find(|record| record.display_credential.id == credential_id.to_string())
            .ok_or(StrongholdMissingCredentialError(credential_id))?;

        let display_credential = &mut verifiable_credential_record.display_credential;

        info!(
            "verifiable_credential_record (before): {:?}",
            display_credential.metadata
        );

        // Set name if given
        if let Some(name) = name {
            display_credential.display_name = name;
        }

        // Set favorite if given
        if let Some(is_favorite) = is_favorite {
            verifiable_credential_record.display_credential.metadata.is_favorite = is_favorite;
        }

        info!(
            "verifiable_credential_record (after): {:?}",
            verifiable_credential_record.display_credential.metadata
        );

        stronghold_manager
            .insert(
                credential_id,
                json!(verifiable_credential_record).to_string().as_bytes().to_vec(),
            )
            .map_err(StrongholdInsertionError)?;
        info!("credential metadata updated");

        let credentials = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|record| record.display_credential)
            .collect();

        drop(state_guard);
        return Ok(AppState {
            credentials,
            current_user_prompt: None,
            ..state
        });
    };

    Ok(state)
}
