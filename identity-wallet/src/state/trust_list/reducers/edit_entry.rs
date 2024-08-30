use crate::error::AppError;
use crate::state::trust_list::actions::edit_trust_list_entry::EditTrustListEntry;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn edit_trust_list_entry(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<EditTrustListEntry>(action) {
        let mut trust_lists = state.trust_lists.clone();

        let new_bool = trust_lists
            .get_mut(&action.trust_list_id)
            .expect("error: invalid trust list name sent by frontend.")
            .get(&action.old_domain)
            .expect("error: invalid domain value sent by frontend.")
            .clone();

        // Unwraps no problem here as the first check already has the expect() implemented.
        trust_lists
            .get_mut(&action.trust_list_id)
            .unwrap()
            .remove(&action.old_domain);
        trust_lists
            .get_mut(&action.trust_list_id)
            .unwrap()
            .insert(action.new_domain, new_bool);

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
    async fn test_edit_trust_list_entry() {
        let mut state = AppState::default();
        let default_trust_list = TrustList::default();
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(EditTrustListEntry {
            trust_list_id: default_trust_list.id.clone(),
            old_domain: "impierce.com".to_string(),
            new_domain: "new.com".to_string(),
        });

        let result = edit_trust_list_entry(state, action).await.unwrap();

        let mut expected = TrustLists::new();
        expected.insert(TrustList {
            id: default_trust_list.id.clone(),
            display_name: default_trust_list.display_name.clone(),
            owned: true,
            entries: HashMap::from([("new.com".to_string(), true)]),
        });

        assert_eq!(result.trust_lists, expected);
    }
}
