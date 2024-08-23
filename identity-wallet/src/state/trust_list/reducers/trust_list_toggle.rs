use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_toggle::TrustListToggle;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_toggle(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListToggle>(action) {
        let mut trust_lists = state.trust_lists.clone();

        if let Some(trust_list) = trust_lists.get_mut(&action.trust_list_name)
        {
            if let Some(value) = trust_list.get_mut(&action.domain) {
                *value = !*value;
            }
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
    use crate::state::trust_list::{TrustList, TrustLists};

    use super::*;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_toggle() {
        let state = AppState {
            trust_lists: TrustLists::default(),
            ..Default::default()
        };

        let action = Arc::new(TrustListToggle {
            trust_list_name: "impierce".to_string(),
            domain: "https://www.impierce.com".to_string(),
        });

        let result = trust_list_toggle(state, action).await.unwrap();

        let mut test = TrustLists::new();
        test.insert(
            TrustList { 
                name: "impierce".to_string(),
                trust_list: HashMap::from([("https://www.impierce.com".to_string(), false)])
            }
        );

        assert_eq!(result.trust_lists, test);
    }
}
