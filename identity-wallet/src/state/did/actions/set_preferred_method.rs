use crate::{
    reducer,
    state::{actions::ActionTrait, did::reducers::preferred_method::set_preferred_did_method, Reducer},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A DID method the wallet produces DIDs for, see [`SUPPORTED_DID_METHODS`](crate::state::SUPPORTED_DID_METHODS).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DidMethod {
    #[serde(rename = "did:jwk")]
    Jwk,
    #[serde(rename = "did:key")]
    Key,
}

impl std::fmt::Display for DidMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Jwk => "did:jwk",
            Self::Key => "did:key",
        })
    }
}

/// Set the preferred DID method to be used.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "bindings/actions/SetPreferredDidMethod.ts")]
pub struct SetPreferredDidMethod {
    #[ts(type = "string")]
    pub method: DidMethod,
}

#[typetag::serde(name = "[DID] Set preferred method")]
impl ActionTrait for SetPreferredDidMethod {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_preferred_did_method)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SUPPORTED_DID_METHODS;

    #[test]
    fn did_methods_serialize_as_the_supported_methods() {
        for method in [DidMethod::Jwk, DidMethod::Key] {
            assert_eq!(serde_json::to_value(method).unwrap(), method.to_string());
            assert!(SUPPORTED_DID_METHODS.contains(&method.to_string().as_str()));
        }
        assert!(serde_json::from_value::<DidMethod>("did:web".into()).is_err());
    }
}
