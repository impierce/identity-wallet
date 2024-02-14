pub mod actions;
pub mod reducers;

use serde::{Deserialize, Serialize};
use strum::EnumString;
use ts_rs::TS;
use crate::state::FeatTrait;

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: Option<String>,
    pub primary_did: String,
}

#[typetag::serde(name = "profile")]
impl FeatTrait for Profile {}


#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default, EnumString)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Locale {
    #[default]
    En,
    De,
    Nl,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::actions::SetLocale;
    use super::reducers::set_locale;
    use crate::state::{actions::Action, AppState};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_set_locale() {
        let mut app_state = AppState::default();

        app_state = set_locale(app_state, Arc::new(SetLocale { locale: Locale::Nl }) as Action)
            .await
            .unwrap();

        assert_eq!(app_state.locale, Locale::Nl);
    }
}
