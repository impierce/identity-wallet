use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_add::TrustListAdd;
use crate::state::trust_list::TrustList;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_add(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListAdd>(action) {
        let mut trust_lists = state.trust_lists.clone();
        trust_lists.insert(TrustList {
            name: action.trust_list_name,
            trust_list: Default::default(),
        });

        return Ok(AppState {
            trust_lists,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use crate::state::trust_list::TrustLists;

    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_trust_list_add() {
        let state = AppState::default();
        let action = Arc::new(TrustListAdd {
            trust_list_name: "example".to_string(),
        });

        let result = trust_list_add(state, action).await.unwrap();

        let mut test = TrustLists::default();
        test.insert(TrustList {
            name: "example".to_string(),
            trust_list: Default::default(),
        });

        assert_eq!(result.trust_lists, test);
    }
}
