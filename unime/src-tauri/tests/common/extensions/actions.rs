use crate::common::extensions::reducers::test_feat_state;
use identity_wallet::{
    reducer,
    state::{actions::ActionTrait, Reducer},
};

/// Action to test the extension field.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CustomExtensionTest {
    pub test_term: Option<String>,
    #[serde(default)]
    pub test_bool: bool,
}

#[typetag::serde(name = "[Test] Test")]
impl ActionTrait for CustomExtensionTest {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(test_feat_state)]
    }
}
