use crate::error::AppError::{self, *};
use crate::state::actions::{listen, Action};
use crate::state::common::actions::unlock_storage::UnlockStorage;
use crate::state::core_utils::helpers::get_credential_status;
use crate::state::core_utils::{DateUtils, IdentityManager};
use crate::state::credentials::{CredentialStatusData, DisplayCredential};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
use crate::stronghold::StrongholdManager;
use crate::subject::subject;

use log::info;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

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

        info!("loading credentials from stronghold");
        let mut credentials: Vec<DisplayCredential> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        // Check the credentialStatus for each credential
        for credential in &mut credentials {
            if let Some(credential_status) = credential.data.get_mut("credentialStatus") {
                let new_status =
                    get_credential_status(credential_status, state_guard.identity_manager.as_ref().unwrap()).await?;

                // We ok_or() with an error here because when the if let Some() statement above is true, we must have a CredentualStatusData.
                let old_status = credential
                    .credential_status
                    .as_ref()
                    .map(|s| s.status)
                    .ok_or(AppError::InvalidCredentialStatusFormatError)?;

                if old_status != new_status {
                    info!(
                        "Credential {} changed credential status from {:?} to {:?}",
                        credential.id, old_status, new_status
                    );
                }

                credential.credential_status = Some(CredentialStatusData {
                    status: new_status,
                    last_checked: DateUtils::new_date_string(),
                });
            }
        }

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
