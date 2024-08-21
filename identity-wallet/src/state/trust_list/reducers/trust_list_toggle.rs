use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_toggle::TrustListToggle;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_toggle(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListToggle>(action) {
        let mut trust_list = state.trust_list.clone();

        if let Some(value) = trust_list.get_mut(&action.domain) {
            *value = !*value;
        }

        return Ok(AppState {
            trust_list,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_toggle() {
        let state = AppState {
            trust_list: HashMap::from_iter(vec![("test".to_string(), true)]),
            ..Default::default()
        };

        let action = Arc::new(TrustListToggle {
            domain: "test".to_string(),
        });

        let result = trust_list_toggle(state, action).await.unwrap();

        assert_eq!(result.trust_list, HashMap::from_iter(vec![("test".to_string(), false)]));
    }
}
