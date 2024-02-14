use crate::{reducer, state::{actions::{ActionTrait, Reducer}, extension::reducers::test_feat_state}};
use ts_rs::TS;



/// Action to test the extension field.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/Test.ts")]
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
