use crate::error::AppError;
use crate::state::trust_list::actions::add_trust_list_entry::AddTrustListEntry;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn add_trust_list_entry(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<AddTrustListEntry>(action) {
        let mut trust_lists = state.trust_lists.clone();
        trust_lists
            .get_mut(&action.trust_list_name)
            .expect("error: incorrect trust_list_name dispatched by frontend")
            .insert(action.domain, true);

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
    async fn test_add_trust_list_entry() {
        let state = AppState::default();
        let action = Arc::new(AddTrustListEntry {
            trust_list_name: "impierce".to_string(),
            domain: "test".to_string(),
        });

        let result = add_trust_list_entry(state, action).await.unwrap();

        let mut test = TrustLists::default();
        test.get_mut("impierce").unwrap().insert("test".to_string(), true);

        assert_eq!(result.trust_lists, test);
    }
}
