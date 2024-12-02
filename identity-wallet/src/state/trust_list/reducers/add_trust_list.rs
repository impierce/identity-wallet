use log::info;

use crate::error::AppError;
use crate::state::trust_list::actions::add_trust_list::AddTrustList;
use crate::state::trust_list::TrustList;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_add(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<AddTrustList>(action) {
        let mut trust_lists = state.trust_lists.clone();
        trust_lists.insert(TrustList {
            id: uuid::Uuid::new_v4().to_string(),
            display_name: action.display_name.clone(),
            custom: true,
            entries: Default::default(),
        });

        info!(
            "Added trust list {} to trust lists: {:#?}",
            action.display_name, trust_lists
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
    use super::*;
    use crate::state::trust_list::TrustLists;

    use std::sync::Arc;

    #[tokio::test]
    async fn test_trust_list_add() {
        let state = AppState::default();

        let action = Arc::new(AddTrustList {
            display_name: "example".to_string(),
        });

        let result = trust_list_add(state, action).await.unwrap();

        let mut expected = TrustLists::default();
        expected.insert(TrustList {
            id: "_".to_string(),
            display_name: "example".to_string(),
            custom: true,
            entries: Default::default(),
        });

        let actual = result.trust_lists.0.first().unwrap();
        let expected = expected.0.first().unwrap();

        // ID is not asserted as it is randomly generated upon creation.
        assert_eq!(actual.display_name, expected.display_name);
        assert_eq!(actual.custom, expected.custom);
        assert_eq!(actual.entries, expected.entries);
    }
}
