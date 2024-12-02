use log::info;

use crate::error::AppError;
use crate::state::trust_list::actions::edit_trust_list::EditTrustList;
use crate::state::trust_list::TrustList;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_edit(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<EditTrustList>(action) {
        let mut trust_lists = state.trust_lists;
        let trust_list = trust_lists
            .get_mut(&action.trust_list_id)
            .ok_or_else(|| AppError::TrustListNotFoundError(action.trust_list_id.clone()))?
            .clone();

        trust_lists.remove(&action.trust_list_id);

        trust_lists.insert(TrustList {
            id: trust_list.id,
            display_name: action.new_display_name.clone(),
            custom: trust_list.custom,
            entries: trust_list.entries,
        });

        info!(
            "Edited trust list {} in trust lists: {:#?}",
            action.new_display_name, trust_lists
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
    use crate::state::trust_list::TrustLists;

    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_edit() {
        let mut state = AppState::default();
        let default_trust_list = TrustList {
            id: Uuid::new_v4().to_string(),
            display_name: "impierce".to_string(),
            custom: true,
            entries: HashMap::from([(Url::parse("https://example.com").unwrap(), true)]),
        };
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(EditTrustList {
            trust_list_id: default_trust_list.id.clone(),
            new_display_name: "example".to_string(),
        });

        let result = trust_list_edit(state, action).await.unwrap();

        let mut expected = TrustLists::new();
        expected.insert(TrustList {
            id: default_trust_list.id.clone(),
            display_name: "example".to_string(),
            custom: true,
            entries: HashMap::from([(Url::parse("https://example.com").unwrap(), true)]),
        });

        assert_eq!(result.trust_lists, expected);
    }
}
