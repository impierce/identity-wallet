use crate::{
    crypto::stronghold::StrongholdManager,
    state::{
        actions::Action,
        user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect},
        AppState,
    },
    verifiable_credential_record::DisplayCredential,
};
use log::info;
use std::sync::Arc;

pub async fn unlock_storage(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let password = payload["password"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read password from json payload"))?;

    let stronghold_manager = StrongholdManager::load(password).unwrap();

    // TEMP fix by Daniel (can be removed)
    let mut credentials: Vec<DisplayCredential> = vec![];

    stronghold_manager
        .values()?
        .unwrap()
        .into_iter()
        .for_each(|verifiable_credential_record| {
            credentials.push(DisplayCredential::from(verifiable_credential_record.display_credential));
        });

    *state.credentials.lock().unwrap() = credentials;

    state
        .managers
        .lock()
        .await
        .stronghold_manager
        .replace(Arc::new(stronghold_manager));

    info!("storage unlocked");

    state
        .current_user_prompt
        .lock()
        .unwrap()
        .replace(CurrentUserPrompt::Redirect(Redirect {
            r#type: CurrentUserPromptType::Redirect,
            target: "me".to_string(),
        }));

    Ok(())
}
