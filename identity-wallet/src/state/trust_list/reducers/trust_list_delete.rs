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
    use uuid::Uuid;

    use crate::state::trust_list::{TrustList, TrustLists};

    use super::*;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_delete() {
        let mut state = AppState::default();
        let default_trust_list = TrustList {
            id: Uuid::new_v4().to_string(),
            display_name: "impierce".to_string(),
            custom: true,
            entries: HashMap::from([("impierce.com".to_string(), true)]),
        };
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(TrustListsDelete {
            trust_list_id: default_trust_list.id,
        });

        let result = trust_list_delete(state, action).await.unwrap();

        let expected = TrustLists::new();

        assert_eq!(result.trust_lists, expected);
    }
}
