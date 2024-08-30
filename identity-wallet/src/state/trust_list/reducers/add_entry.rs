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
            .get_mut(&action.trust_list_id)
            .expect("error: incorrect trust_list_id dispatched by frontend")
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
    use super::*;
    use crate::state::trust_list::TrustList;

    use std::sync::Arc;

    #[tokio::test]
    async fn test_add_trust_list_entry() {
        let mut state = AppState::default();
        let default_trust_list = TrustList::default();
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(AddTrustListEntry {
            trust_list_id: default_trust_list.id.clone(),
            domain: "example.com".to_string(),
        });

        let result = add_trust_list_entry(state, action).await.unwrap();

        let mut expected = default_trust_list.clone();
        expected.insert("example.com".to_string(), true);

        assert_eq!(result.trust_lists.0.first().unwrap().clone(), expected);
    }
}
