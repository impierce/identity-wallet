use std::sync::Arc;

use crate::{
    crypto::stronghold::StrongholdManager,
    get_jwt_claims,
    state::{
        actions::Action,
        user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect},
        AppState,
    },
};
use log::info;
use oid4vci::credential_format_profiles::CredentialFormats;

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
        .for_each(|(uuid, credential)| {
            let credential_display = match credential {
                CredentialFormats::JwtVcJson(credential) => {
                    serde_json::from_value::<identity_credential::credential::Credential>(
                        get_jwt_claims(&credential.credential)["vc"].clone(),
                    )
                    .unwrap()
                }
                _ => unimplemented!(),
            };

            state
                .credentials
                .lock()
                .unwrap()
                .push((uuid.to_string(), credential_display));
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
