use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_edit::TrustListEdit;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_edit(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListEdit>(action) {
        let mut trust_lists = state.trust_lists.clone();

        let new_bool = trust_lists
            .get_mut(&action.trust_list_name)
            .expect("error: invalid trust list name sent by frontend.")
            .get(&action.old_domain)
            .expect("error: invalid domain value sent by frontend.")
            .clone();

        // Unwraps no problem here as the first check already has the expect() implemented.
        trust_lists.get_mut(&action.trust_list_name).unwrap().remove(&action.old_domain);
        trust_lists.get_mut(&action.trust_list_name).unwrap().insert(action.new_domain, new_bool);

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
    async fn test_trust_list_edit() {
        let state = AppState {
            trust_lists: TrustLists::default(),
            ..Default::default()
        };

        let action = Arc::new( TrustListEdit {
            trust_list_name: "impierce".to_string(),
            old_domain: "https://www.impierce.com".to_string(),
            new_domain: "new".to_string(),
        });

        let result = trust_list_edit(state, action).await.unwrap();

        let mut test = TrustLists::new();
        test.insert(
            TrustList { 
                name: "impierce".to_string(),
                trust_list: HashMap::from([("new".to_string(), true)])
            }
        );

        assert_eq!(result.trust_lists, test);
    }
}
