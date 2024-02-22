use super::actions::CustomExtensionTest;
use super::CustomExtension;
use crate::error::AppError;
use crate::state::actions::{listen, Action};
use crate::state::AppState;

pub async fn test_feat_state(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(test_feat_state) = listen::<CustomExtensionTest>(action) {
        
        let mut new_state = state.clone();

        new_state.extensions.insert("test".to_string(), Box::new(CustomExtension{
            name: "new".to_string(),
            value: if test_feat_state.test_bool { "new".to_string() } else { "old".to_string() }
        }));

        return Ok(new_state);
    }
    Ok(state)
}
