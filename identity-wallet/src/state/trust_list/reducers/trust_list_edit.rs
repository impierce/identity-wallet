use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_edit::TrustListEdit;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_edit(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListEdit>(action) {
        let mut trust_list = state.trust_list.clone();

        let new_bool = trust_list
            .get(&action.old_domain)
            .expect("error: invalid domain value send by frontend.")
            .clone();

        trust_list.remove(&action.old_domain);
        trust_list.insert(action.new_domain, new_bool);

        return Ok(AppState {
            trust_list,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_edit() {
        let state = AppState {
            trust_list: HashMap::from_iter(vec![("old".to_string(), true)]),
            ..Default::default()
        };

        let action = Arc::new(TrustListEdit {
            old_domain: "old".to_string(),
            new_domain: "new".to_string(),
        });

        let result = trust_list_edit(state, action).await.unwrap();

        assert_eq!(result.trust_list, HashMap::from_iter(vec![("new".to_string(), true)]));
    }
}
