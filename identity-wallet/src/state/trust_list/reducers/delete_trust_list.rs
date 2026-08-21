use log::{info, warn};

use crate::error::AppError;
use crate::state::trust_list::actions::delete_trust_list::DeleteTrustList;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

#[tracing::instrument(skip_all, err)]
pub async fn trust_list_delete(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<DeleteTrustList>(action) {
        let mut trust_lists = state.trust_lists;
        if let Some(trust_list) = trust_lists.remove(&action.trust_list_id) {
            info!("Deleted trust list `{}`", trust_list.display_name);
        } else {
            warn!("Attempted to delete non-existent trust list `{}`", action.trust_list_id);
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
    use url::Url;
    use uuid::Uuid;

    use crate::state::trust_list::{TrustList, TrustLists};

    use super::*;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_delete() {
        let mut state = AppState::default();
        let default_trust_list = TrustList {
            id: Uuid::new_v4().to_string(),
            display_name: "impierce".to_string(),
            custom: true,
            entries: HashMap::from([(Url::parse("https://example.com").unwrap(), true)]),
        };
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(DeleteTrustList {
            trust_list_id: default_trust_list.id,
        });

        let result = trust_list_delete(state, action).await.unwrap();

        let expected = TrustLists::new();

        assert_eq!(result.trust_lists, expected);
    }
}
