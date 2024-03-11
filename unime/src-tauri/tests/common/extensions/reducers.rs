use identity_wallet::error::AppError;
use identity_wallet::state::actions::{listen, Action};
use identity_wallet::state::AppState;

use super::actions::CustomExtensionTest;
use super::CustomExtension;

pub async fn test_feat_state(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(test_feat_state) = listen::<CustomExtensionTest>(action) {
        let mut new_state = state;

        new_state.extensions.insert(
            "test".to_string(),
            Box::new(CustomExtension {
                name: "new".to_string(),
                value: if test_feat_state.test_bool {
                    "new".to_string()
                } else {
                    "old".to_string()
                },
            }),
        );

        return Ok(new_state);
    }
    Ok(state)
}
