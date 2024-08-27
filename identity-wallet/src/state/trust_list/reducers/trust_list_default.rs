use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_default::TrustListDefault;
use crate::state::trust_list::TrustList;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_default(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListDefault>(action) {
        let mut trust_lists = state.trust_lists.clone();
        trust_lists.remove(&action.trust_list_id);
        trust_lists.insert(TrustList::default());

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
    use super::*;
    use crate::state::trust_list::{actions::trust_list_default::TrustListDefault, TrustList, TrustLists};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_trust_list_default() {
        let mut state = AppState::default();
        state.trust_lists.insert(TrustList::default());

        state
            .trust_lists
            .get_mut("impierce")
            .unwrap()
            .insert("test".to_string(), false);

        let action = Arc::new(TrustListDefault {
            trust_list_id: "impierce".to_string(),
        });

        let result = trust_list_default(state, action).await.unwrap();

        let mut test = TrustLists::default();
        test.insert(TrustList::default());

        assert_eq!(result.trust_lists, test);
    }
}
