use crate::error::AppError::{self, *};
use crate::reducer;
use crate::state::actions::{listen, Action};
use crate::state::credentials::verifiable_credential_record::VerifiableCredentialRecord;
use crate::state::AppState;
use crate::state::{actions::ActionTrait, Reducer};

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

/// Action to update the credential metadata.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateCredentialMetadata.ts")]
pub struct UpdateCredentialMetadata {
    #[ts(type = "string")]
    pub id: uuid::Uuid,
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub icon: Option<String>,
    #[ts(optional)]
    pub color: Option<String>,
    #[ts(optional)]
    pub is_favorite: Option<bool>,
}

#[typetag::serde(name = "[Credential Metadata] Update")]
impl ActionTrait for UpdateCredentialMetadata {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_credential_metadata)]
    }
}

pub async fn update_credential_metadata(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(UpdateCredentialMetadata {
        id: credential_id,
        name,
        color,
        icon,
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

        // Set color if given
        if color.is_some() {
            display_credential.display_color = color;
        }

        // Set icon if given
        if icon.is_some() {
            display_credential.display_icon = icon;
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
