use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_delete::TrustListsDelete;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_delete(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListsDelete>(action) {
        let mut trust_lists = state.trust_lists.clone();
        trust_lists.remove(&action.trust_list_id);

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
    async fn test_trust_list_delete() {
        let state = AppState::default();
        let action = Arc::new(TrustListsDelete {
            trust_list_id: "impierce".to_string(),
        });

        let result = trust_list_delete(state, action).await.unwrap();

        let test = TrustLists::new();

        assert_eq!(result.trust_lists, test);
    }
}
