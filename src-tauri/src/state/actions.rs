use std::str::FromStr;

use serde::Deserialize;
use ts_rs::TS;

/// A redux-like Action with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
#[derive(Deserialize, TS)]
#[ts(export)]
pub struct Action {
    pub(crate) r#type: String,
    pub(crate) payload: Option<String>,
}

/// Actions that the backend knows how to handle (reduce).
pub enum KnownAction {
    GetState,
    Reset,
    CreateNew,
    SetLocale,
}

/// Maps a given string to a [KnownAction].
impl FromStr for KnownAction {
    type Err = ();
    fn from_str(s: &str) -> Result<KnownAction, Self::Err> {
        match s {
            "[INIT] Get state" => Ok(KnownAction::GetState),
            "[INIT] Reset" => Ok(KnownAction::Reset),
            "[DID] Create new" => Ok(KnownAction::CreateNew),
            "[SETTINGS] Set locale" => Ok(KnownAction::SetLocale),
            _ => Err(()),
        }
    }
}
