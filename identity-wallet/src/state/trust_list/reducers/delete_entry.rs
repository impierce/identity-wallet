use log::info;

use crate::error::AppError;
use crate::state::trust_list::actions::delete_trust_list_entry::DeleteTrustListEntry;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn delete_trust_list_entry(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<DeleteTrustListEntry>(action) {
        let mut trust_lists = state.trust_lists;

        let trust_list = trust_lists
            .get_mut(&action.trust_list_id)
            .ok_or_else(|| AppError::TrustListNotFoundError(action.trust_list_id.clone()))?;

        trust_list.remove(&action.domain);

        info!(
            "Deleted trusted domain `{}` from list `{}`",
            action.domain, trust_list.display_name
        );

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
    use url::Url;
    use uuid::Uuid;

    use super::*;
    use crate::state::trust_list::{TrustList, TrustLists};

    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    #[serial_test::serial]
    async fn test_delete_trust_list_entry() {
        let mut state = AppState::default();
        let default_trust_list = TrustList {
            id: Uuid::new_v4().to_string(),
            display_name: "impierce".to_string(),
            custom: true,
            entries: HashMap::from([(Url::parse("https://example.com").unwrap(), true)]),
        };
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(DeleteTrustListEntry {
            trust_list_id: default_trust_list.id.clone(),
            domain: Url::parse("https://example.com").unwrap(),
        });

        let result = delete_trust_list_entry(state, action).await.unwrap();

        let mut expected = TrustLists::new();
        expected.insert(TrustList {
            id: default_trust_list.id.clone(),
            display_name: default_trust_list.display_name.clone(),
            custom: true,
            entries: HashMap::new(),
        });

        assert_eq!(result.trust_lists, expected);
    }
}
