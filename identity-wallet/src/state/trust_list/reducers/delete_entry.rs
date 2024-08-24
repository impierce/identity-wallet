use crate::error::AppError;
use crate::state::trust_list::actions::delete_trust_list_entry::DeleteTrustListEntry;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn delete_trust_list_entry(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<DeleteTrustListEntry>(action) {
        let mut trust_lists = state.trust_lists.clone();
        trust_lists
            .get_mut(&action.trust_list_name)
            .expect("error: incorrect trust_list_name dispatched by frontend")
            .remove(&action.domain);

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
    async fn test_delete_trust_list_entry() {
        let state = AppState::default();

        let action = Arc::new(DeleteTrustListEntry {
            trust_list_name: "impierce".to_string(),
            domain: "https://www.impierce.com".to_string(),
        });

        let result = delete_trust_list_entry(state, action).await.unwrap();

        let mut test = TrustLists::new();
        test.insert(TrustList {
            name: "impierce".to_string(),
            trust_list: HashMap::new(),
        });

        assert_eq!(result.trust_lists, test);
    }
}
