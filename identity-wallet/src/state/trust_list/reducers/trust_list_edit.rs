use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_edit::TrustListEdit;
use crate::state::trust_list::TrustList;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_edit(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListEdit>(action) {
        let mut trust_lists = state.trust_lists.clone();
        let trust_list = trust_lists
            .get_mut(&action.trust_list_name)
            .expect("error: incorrect trust_list_name dispatched by frontend")
            .trust_list
            .clone();

        trust_lists.insert(TrustList {
            name: action.new_trust_list_name,
            trust_list,
        });

        trust_lists.remove(&action.trust_list_name);

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
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_edit() {
        let state = AppState::default();
        let action = Arc::new(TrustListEdit {
            trust_list_name: "impierce".to_string(),
            new_trust_list_name: "example".to_string(),
        });

        let result = trust_list_edit(state, action).await.unwrap();

        let mut test = TrustLists::new();
        test.insert(TrustList {
            name: "example".to_string(),
            trust_list: HashMap::from([("https://www.impierce.com".to_string(), true)]),
        });

        assert_eq!(result.trust_lists, test);
    }
}
