use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A redux-like Action with an optional payload
/// See https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers
/// For the type string, we're using ngrx style: "\[Feature\] Action name" (see https://ngrx.io/guide/store/actions)
#[derive(Deserialize, TS, Clone)]
#[ts(export)]
pub struct Action {
    pub(crate) r#type: ActionType,
    #[ts(optional, type = "object")]
    pub(crate) payload: Option<serde_json::Value>,
}

/// Actions that the backend knows how to handle (reduce).
#[derive(Serialize, Deserialize, Debug, TS, PartialEq, Clone)]
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
    #[serde(rename = "[DEV] Load profile")]
    LoadDevProfile,
    #[serde(other)]
    #[ts(skip)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_type_deserialization() {
        assert_eq!(
            ActionType::GetState,
            serde_json::from_str(r#""[App] Get state""#).unwrap(),
        );
        assert_eq!(
            ActionType::Unknown,
            serde_json::from_str(r#""Unknown action""#).unwrap(),
        );
    }
}
