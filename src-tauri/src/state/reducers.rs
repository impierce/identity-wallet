// TODO: find abstraction for each reducer (Reducer trait?, problem: reducers are functions, can we have trait-like behavior for functions?)
use tracing::warn;
use crate::state::actions::Action;
use crate::state::state::{AppState, Profile, StateStatus};

pub fn set_locale(state: &AppState, action: Action) -> anyhow::Result<()> {
    let supported_locales = vec!["en", "nl", "de"];
    if !supported_locales.contains(&action.payload.as_ref().unwrap().as_str()) {
        warn!("unsupported locale: `{}`", action.payload.unwrap());
    } else {
        *state.locale.lock().unwrap() = action.payload.unwrap();
    }
    Ok(())
}

pub fn create_did_key(state: &AppState, action: Action) -> anyhow::Result<()> {
    let mock_profile = Profile {
        display_name: action.payload.unwrap(),
        primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
    };

    *state.status.lock().unwrap() = StateStatus::Stable;
    *state.active_profile.lock().unwrap() = Some(mock_profile);
    Ok(())
}

pub fn reset_state(state: &AppState, _action: Action) -> anyhow::Result<()> {
    *state.status.lock().unwrap() = Default::default();
    *state.active_profile.lock().unwrap() = None;
    *state.locale.lock().unwrap() = "en".to_string();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::actions::ActionType;

    #[test]
    fn test_set_locale() {
        let state = AppState::default();

        set_locale(
            &state,
            Action {
                r#type: ActionType::SetLocale,
                payload: Some("nl".to_string()),
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
                payload: Some("Test profile".to_string()),
            },
        )
        .unwrap();

        assert_eq!(*state.status.lock().unwrap(), StateStatus::Stable);
        assert_eq!(
            *state.active_profile.lock().unwrap(),
            Some(Profile {
                display_name: "Test profile".to_string(),
                primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
            })
        );
    }
}
