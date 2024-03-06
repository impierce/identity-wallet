use super::Locale;
use crate::{
    error::AppError::{self},
    reducer,
    state::{actions::ActionTrait, Reducer},
    state::{
        actions::{listen, Action},
        profile_settings::ProfileSettings,
        AppState,
    },
};
use log::debug;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to set the language of the app.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SetLocale.ts")]
pub struct SetLocale {
    #[ts(type = "string")]
    pub locale: Locale,
}

#[typetag::serde(name = "[Settings] Set locale")]
impl ActionTrait for SetLocale {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_locale)]
    }
}

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub async fn set_locale(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(locale) = listen::<SetLocale>(action).map(|payload| payload.locale) {
        debug!("locale set to: `{:?}`", locale);

        state.profile_settings = ProfileSettings {
            locale,
            ..state.profile_settings
        };

        state.current_user_prompt = None;
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::state::AppTheme;

    #[tokio::test]
    async fn test_set_locale() {
        let mut app_state = AppState::default();

        app_state = set_locale(app_state, Arc::new(SetLocale { locale: Locale::nl_NL }))
            .await
            .unwrap();

        assert_eq!(app_state.profile_settings.locale, Locale::nl_NL);
    }
}
