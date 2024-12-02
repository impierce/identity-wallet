use log::info;

use crate::error::AppError;
use crate::state::trust_list::actions::edit_trust_list_entry::EditTrustListEntry;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn edit_trust_list_entry(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<EditTrustListEntry>(action) {
        let mut trust_lists = state.trust_lists;

        let new_bool = *trust_lists
            .get_mut(&action.trust_list_id)
            .ok_or_else(|| AppError::TrustListNotFoundError(action.trust_list_id.clone()))?
            .get(&action.old_domain)
            .ok_or_else(|| {
                AppError::Error(format!(
                    "invalid domain value sent by frontend: {}",
                    action.old_domain.clone()
                ))
            })?;

        trust_lists
            .get_mut(&action.trust_list_id)
            .ok_or_else(|| AppError::TrustListNotFoundError(action.trust_list_id.clone()))?
            .remove(&action.old_domain);
        trust_lists
            .get_mut(&action.trust_list_id)
            .ok_or_else(|| AppError::TrustListNotFoundError(action.trust_list_id.clone()))?
            .insert(action.new_domain.clone(), new_bool);

        info!(
            "edited old domain {} to new domain {} in trust list: {:#?}",
            action.old_domain,
            action.new_domain,
            trust_lists.get_mut(&action.trust_list_id)
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

    use crate::state::trust_list::{TrustList, TrustLists};

    use super::*;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_edit_trust_list_entry() {
        let mut state = AppState::default();
        let default_trust_list = TrustList {
            id: Uuid::new_v4().to_string(),
            display_name: "impierce".to_string(),
            custom: true,
            entries: HashMap::from([(Url::parse("https://example.com").unwrap(), true)]),
        };
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(EditTrustListEntry {
            trust_list_id: default_trust_list.id.clone(),
            old_domain: Url::parse("https://example.com").unwrap(),
            new_domain: Url::parse("https://new.com").unwrap(),
        });

        let result = edit_trust_list_entry(state, action).await.unwrap();

        let mut expected = TrustLists::new();
        expected.insert(TrustList {
            id: default_trust_list.id.clone(),
            display_name: default_trust_list.display_name.clone(),
            custom: true,
            entries: HashMap::from([(Url::parse("https://new.com").unwrap(), true)]),
        });

        assert_eq!(result.trust_lists, expected);
    }
}
