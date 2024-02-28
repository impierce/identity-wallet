use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::set_locale;
use crate::state::Locale;

use ts_rs::TS;

/// Action to set the language of the app.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
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
