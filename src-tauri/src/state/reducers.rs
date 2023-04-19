use crate::state::actions::Action;
use crate::state::state::{AppState, Profile, StateStatus};
use tracing::{warn, info};

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub fn set_locale(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.unwrap();
    let locale = payload["locale"].as_str().expect("unable to read locale from json payload");
    let supported_locales = vec!["en", "nl", "de"];
    if !supported_locales.contains(&locale) {
        warn!("unsupported locale: `{}`", locale);
    } else {
        *state.locale.lock().unwrap() = locale.to_string();
        info!("locale set to: `{}`", locale);
    }
    Ok(())
}

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub fn create_did_key(state: &AppState, action: Action) -> anyhow::Result<()> {
    let payload = action.payload.unwrap();
    let display_name = payload["display_name"].as_str().expect("unable to read display_name from json payload");
    let mock_profile = Profile {
        display_name: display_name.to_string(),
        primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
    };
    *state.status.lock().unwrap() = StateStatus::Stable;
    *state.active_profile.lock().unwrap() = Some(mock_profile);
    Ok(())
}

/// Completely resets the state to its default values.
pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.status.lock().unwrap() = Default::default();
    *state.active_profile.lock().unwrap() = None;
    *state.locale.lock().unwrap() = "en".to_string();
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::state::actions::ActionType;

    #[test]
    fn test_set_locale() {
        let state = AppState::default();

        set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: Some(json!({"locale": "nl"})),
            },
        )
        .unwrap();

        assert_eq!(*state.status.lock().unwrap(), StateStatus::Stable);
        assert_eq!(*state.locale.lock().unwrap(), "nl".to_string());
    }

    #[test]
    fn test_create_did_key() {
        let state = AppState::default();

        create_did_key(
            &state,
            Action {
                r#type: ActionType::CreateNew,
                payload: Some(json!({"display_name": "Ferris"})),
            },
        )
        .unwrap();

        assert_eq!(*state.status.lock().unwrap(), StateStatus::Stable);
        assert_eq!(
            *state.active_profile.lock().unwrap(),
            Some(Profile {
                display_name: "Ferris".to_string(),
                primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            })
        );
    }

    #[test]
    fn test_reset_state() {
        let state = AppState {
            status: StateStatus::Loading.into(),
            active_profile: Some(Profile {
                display_name: "Ferris".to_string(),
                primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            }).into(),
            locale: "nl".to_string().into(),
        };

        reset_state(
            &state,
            Action {
                r#type: ActionType::Reset,
                payload: None,
            },
        )
        .unwrap();

        assert_eq!(*state.status.lock().unwrap(), StateStatus::Stable);
        assert_eq!(*state.active_profile.lock().unwrap(), None);
        assert_eq!(*state.locale.lock().unwrap(), "en".to_string());
    }
}
