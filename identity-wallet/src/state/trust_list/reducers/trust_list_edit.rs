use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_edit::TrustListsEdit;
use crate::state::trust_list::TrustList;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

// todo: remove expect
pub async fn trust_list_edit(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListsEdit>(action) {
        let mut trust_lists = state.trust_lists.clone();
        let trust_list = trust_lists
            .get_mut(&action.trust_list_id)
            .expect("error: incorrect trust_list_id dispatched by frontend")
            .clone();

        trust_lists.remove(&action.trust_list_id);

        trust_lists.insert(TrustList {
            id: trust_list.id,
            display_name: action.new_display_name,
            owned: trust_list.owned,
            entries: trust_list.entries,
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
    use super::*;
    use crate::state::trust_list::TrustLists;

    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_edit() {
        let mut state = AppState::default();
        let default_trust_list = TrustList::default();
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(TrustListsEdit {
            trust_list_id: default_trust_list.id.clone(),
            new_display_name: "example".to_string(),
        });

        let result = trust_list_edit(state, action).await.unwrap();

        let mut expected = TrustLists::new();
        expected.insert(TrustList {
            id: default_trust_list.id.clone(),
            display_name: "example".to_string(),
            owned: true,
            entries: HashMap::from([("impierce.com".to_string(), true)]),
        });

        assert_eq!(result.trust_lists, expected);
    }
}
