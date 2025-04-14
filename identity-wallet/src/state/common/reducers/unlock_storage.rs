use crate::error::AppError::{self, *};
use crate::state::actions::{listen, Action};
use crate::state::common::actions::unlock_storage::UnlockStorage;
use crate::state::core_utils::IdentityManager;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
use crate::stronghold::StrongholdManager;
use crate::subject::subject;

use log::info;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

pub async fn unlock_storage(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some((password, check_password_only)) =
        listen::<UnlockStorage>(action).map(|payload| (payload.password, payload.check_password_only))
    {
        let mut state_guard = state.core_utils.managers.lock().await;

        if check_password_only.unwrap_or_default() {
            drop(state_guard);
            if StrongholdManager::load(&password).is_ok() {
                return Ok(AppState {
                    debug_messages: {
                        let mut debug_messages = state.debug_messages.clone();
                        debug_messages.push_back("Stronghold password OK".to_string());
                        debug_messages
                    },
                    current_user_prompt: None,
                    ..state
                });
            } else {
                return Ok(AppState {
                    debug_messages: {
                        let mut debug_messages = state.debug_messages.clone();
                        debug_messages.push_back("Wrong Stronghold password".to_string());
                        debug_messages
                    },
                    current_user_prompt: None,
                    ..state
                });
            }
        }

        let stronghold_manager = Arc::new(StrongholdManager::load(&password).map_err(StrongholdLoadingError)?);

        let subject = subject(stronghold_manager.clone(), password).await;

        let provider_manager = ProviderManager::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .map_err(OID4VCWalletError)?;

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
