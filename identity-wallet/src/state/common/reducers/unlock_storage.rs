use crate::error::AppError::{self, *};
use crate::state::actions::{listen, Action};
use crate::state::common::actions::unlock_storage::UnlockStorage;
use crate::state::core_utils::IdentityManager;
use crate::state::credentials::self_issued_migration::migrate_self_issued_credentials;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
use crate::stronghold::StrongholdManager;
use crate::subject::subject;

use log::{debug, info};
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

#[tracing::instrument(skip_all, err)]
pub async fn unlock_storage(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<UnlockStorage>(action).map(|payload| payload.password) {
        let mut state_guard = state.core_utils.managers.lock().await;

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

        state_guard.identity_manager.replace(IdentityManager {
            subject: subject.clone(),
            provider_manager,
            wallet,
        });

        migrate_self_issued_credentials(&stronghold_manager, subject, &state.dids).await;

        debug!("Loading credentials from Stronghold vault");

        let credentials: Vec<_> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        let count = credentials.len();
        state_guard.stronghold_manager.replace(stronghold_manager);

        info!("Stronghold vault unlocked successfully ({count} credentials loaded)");

        drop(state_guard);
        return Ok(AppState {
            credentials,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            is_unlocked: true,
            ..state
        });
    }

    Ok(state)
}
