use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_add::TrustListAdd;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_add(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListAdd>(action) {
        let mut trust_list = state.trust_list.clone();
        trust_list.insert(action.domain, true);

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
    async fn test_trust_list_add() {
        let state = AppState::default();
        let action = Arc::new(TrustListAdd {
            domain: "test".to_string(),
        });

        let result = trust_list_add(state, action).await.unwrap();

        assert_eq!(result.trust_list, HashMap::from_iter(vec![("test".to_string(), true)]));
    }
}
