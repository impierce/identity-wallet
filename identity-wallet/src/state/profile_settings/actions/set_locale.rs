use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        profile_settings::{
            reducers::{
                set_locale::set_locale,
                update_sorting_preference::{sort_connections, sort_credentials},
            },
            Locale,
        },
        Reducer,
    },
};

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
        vec![
            reducer!(set_locale),
            reducer!(sort_connections),
            reducer!(sort_credentials),
        ]
    }
}
