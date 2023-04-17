use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A redux-like Action with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
/// For the type string, we're using ngrx style: "\[Feature\] Action name" (see https://ngrx.io/guide/store/actions)
#[derive(Deserialize, TS)]
#[ts(export)]
pub struct Action {
    pub(crate) r#type: ActionType,
    #[ts(optional)]
    pub(crate) payload: Option<String>, // TODO: payload should be optional json value (or something else easily serde-able)
}

/// Actions that the backend knows how to handle (reduce).
#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub enum ActionType {
    #[serde(rename = "[App] Get state")]
    GetState,
    #[serde(rename = "[App] Reset")]
    Reset,
    #[serde(rename = "[DID] Create new")]
    CreateNew,
    #[serde(rename = "[Settings] Set locale")]
    SetLocale,
}
