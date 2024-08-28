use crate::error::AppError;
use crate::state::trust_list::actions::toggle_trust_list_entry::ToggleTrustListEntry;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn toggle_trust_list_entry(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<ToggleTrustListEntry>(action) {
        let mut trust_lists = state.trust_lists.clone();

        if let Some(trust_list) = trust_lists.get_mut(&action.trust_list_id) {
            if let Some(value) = trust_list.get_mut(&action.domain) {
                *value = !*value;
            }
        }

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
    use crate::state::trust_list::{TrustList, TrustLists};

    use super::*;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_toggletrust_list_entry() {
        let mut state = AppState::default();
        state.trust_lists.insert(TrustList::default());

        let action = Arc::new(ToggleTrustListEntry {
            trust_list_id: "impierce".to_string(),
            domain: "impierce.com".to_string(),
        });

        let result = toggle_trust_list_entry(state, action).await.unwrap();

        let mut test = TrustLists::new();
        test.insert(TrustList {
            name: "impierce".to_string(),
            owned: true,
            trust_list: HashMap::from([("impierce.com".to_string(), false)]),
        });

        assert_eq!(result.trust_lists, test);
    }
}
