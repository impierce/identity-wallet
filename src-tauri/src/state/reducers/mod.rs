pub mod authorization;
pub mod credential_offer;
pub mod load_dev_profile;
pub mod storage;

use crate::crypto::stronghold::StrongholdManager;
use crate::did::did_key::generate_new_did;
use crate::state::actions::Action;
use crate::state::user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect};
use crate::state::{AppState, Profile};
use log::info;

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub fn set_locale(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let locale = payload["locale"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read locale from json payload"))?;
    *state.locale.lock().unwrap() = locale.to_string();
    info!("locale set to: `{}`", locale);
    Ok(())
}

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub async fn create_did_key(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let display_name = payload["display_name"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read display_name from json payload"))?;

    let public_key = state
        .managers
        .lock()
        .unwrap()
        .stronghold_manager
        .as_ref()
        .unwrap()
        .get_public_key()?;
    let did_document = generate_new_did(public_key).await?;
    let profile = Profile {
        display_name: display_name.to_string(),
        primary_did: did_document.id,
    };
    *state.active_profile.lock().unwrap() = Some(profile);
    Ok(())
}

pub async fn initialize_stronghold(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let password = payload["password"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read password from json payload"))?;

    state
        .managers
        .lock()
        .unwrap()
        .stronghold_manager
        .replace(StrongholdManager::create(password)?);

    info!("stronghold initialized");

    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.active_profile.lock().unwrap() = None;
    *state.locale.lock().unwrap() = "en".to_string();
    *state.credentials.lock().unwrap() = vec![];
    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Redirect(Redirect {
        r#type: CurrentUserPromptType::Redirect,
        target: "welcome".to_string(),
    }));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{crypto::stronghold::hash_password, state::actions::ActionType, STRONGHOLD};
    use iota_stronghold::{
        procedures::{GenerateKey, KeyType, StrongholdProcedure},
        Client, KeyProvider, Location, SnapshotPath, Stronghold,
    };
    use serde_json::json;
    use tempfile::NamedTempFile;

    #[test]
    fn test_set_locale() {
        let state = AppState::default();

        assert!(set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: Some(json!({"locale": "nl"})),
            },
        )
        .is_ok());
        assert_eq!(*state.locale.lock().unwrap(), "nl".to_string());
    }

    #[test]
    fn test_set_locale_with_invalid_payload() {
        let state = AppState::default();

        // Assert that a `SetLocale` action without a payload returns an error.
        assert!(set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: None,
            },
        )
        .is_err());

        // Assert that a `SetLocale` action with an invalid payload returns an error.
        assert!(set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: Some(json!({"foo": "bar"})),
            },
        )
        .is_err());
    }

    // #[tokio::test]
    // #[serial_test::serial]
    // async fn test_create_new_with_method_did_key() {
    //     let path = NamedTempFile::new().unwrap().into_temp_path();
    //     *STRONGHOLD.lock().unwrap() = path.as_os_str().into();

    //     // create new temp stronghold for testing
    //     let stronghold = Stronghold::default();
    //     let path = STRONGHOLD.lock().unwrap().clone().to_str().unwrap().to_owned();
    //     let client: Client = stronghold.create_client(path.clone()).expect("cannot create client");
    //     let output_location = Location::counter(path.clone(), 0u8);
    //     client
    //         .execute_procedure(StrongholdProcedure::GenerateKey(GenerateKey {
    //             ty: KeyType::Ed25519,
    //             output: output_location.clone(),
    //         }))
    //         .ok();
    //     stronghold
    //         .write_client(path.clone())
    //         .expect("store client state into snapshot state failed");
    //     stronghold
    //         .commit_with_keyprovider(
    //             &SnapshotPath::from_path(format!("{path}.snapshot")),
    //             &KeyProvider::try_from(hash_password("s3cr3t").unwrap()).unwrap(),
    //         )
    //         .ok();

    //     let state = AppState::default();

    //     assert!(create_did_key(
    //         &state,
    //         Action {
    //             r#type: ActionType::CreateNew,
    //             payload: Some(json!({"display_name": "Ferris", "password": "s3cr3t"})),
    //         },
    //     )
    //     .await
    //     .is_ok());

    //     let profile = state.active_profile.lock().unwrap();
    //     assert_eq!(profile.as_ref().unwrap().display_name, "Ferris");
    //     assert!(profile.as_ref().unwrap().primary_did.starts_with("did:key:"));
    // }

    #[test]
    fn test_reset_state() {
        let state = AppState {
            active_profile: Some(Profile {
                display_name: "Ferris".to_string(),
                primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            })
            .into(),
            ..AppState::default()
        };

        assert!(reset_state(
            &state,
            Action {
                r#type: ActionType::Reset,
                payload: None,
            },
        )
        .is_ok());
        assert_eq!(*state.active_profile.lock().unwrap(), None);
        assert_eq!(*state.locale.lock().unwrap(), "en".to_string());
    }
}
