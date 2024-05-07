use crate::error::AppError::{self, *};
use crate::state::actions::{listen, Action};
use crate::state::common::actions::unlock_storage::UnlockStorage;
use crate::state::core_utils::IdentityManager;
use crate::state::subject::Subject;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;
use crate::stronghold::StrongholdManager;

use did_manager::SecretManager;
use log::info;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

pub async fn unlock_storage(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<UnlockStorage>(action).map(|payload| payload.password) {
        let mut state_guard = state.core_utils.managers.lock().await;
        let default_did_method = state.profile_settings.default_did_method.as_str();

        let stronghold_manager = Arc::new(StrongholdManager::load(&password).map_err(StrongholdLoadingError)?);

        let client_path = crate::persistence::STRONGHOLD
            .lock()
            .unwrap()
            .to_str()
            .ok_or(anyhow::anyhow!("failed to get stronghold path"))
            .unwrap()
            .to_owned();
        let password = "sup3rSecr3t".to_owned();
        let subject = Arc::new(Subject {
            stronghold_manager: stronghold_manager.clone(),
            secret_manager: SecretManager::load(client_path, password, "key-0".to_owned(), None, None)
                .await
                .unwrap(),
        });

        let provider_manager =
            ProviderManager::new(subject.clone(), default_did_method).map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(subject.clone(), default_did_method).map_err(OID4VCWalletError)?;

        info!("loading credentials from stronghold");
        let credentials = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        state_guard.stronghold_manager.replace(stronghold_manager);

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        info!("storage unlocked");

        drop(state_guard);
        return Ok(AppState {
            credentials,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..state
        });
    }

    Ok(state)
}
