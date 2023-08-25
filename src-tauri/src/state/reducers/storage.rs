use crate::{
    crypto::stronghold::StrongholdManager,
    state::{
        actions::Action,
        user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect},
        AppState,
    },
};
use log::info;
use std::sync::Arc;

pub async fn unlock_storage(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let password = payload["password"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read password from json payload"))?;

    let stronghold_manager = StrongholdManager::load(password).unwrap();

    stronghold_manager
        .get_all()?
        .unwrap()
        .into_iter()
        .for_each(|(_uuid, verifiable_credential_record)| {
            state
                .credentials
                .lock()
                .unwrap()
                .push(verifiable_credential_record.display_credential);
        });

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
            target: "profile".to_string(),
        }));

    Ok(())
}
