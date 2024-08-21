use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_delete::TrustListDelete;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_delete(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListDelete>(action) {
        let mut trust_list = state.trust_list.clone();
        trust_list.remove(&action.domain);

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
    async fn test_trust_list_delete() {
        let state = AppState {
            trust_list: HashMap::from_iter(vec![("test".to_string(), true)]),
            ..Default::default()
        };

        let action = Arc::new(TrustListDelete {
            domain: "test".to_string(),
        });

        let result = trust_list_delete(state, action).await.unwrap();

        assert_eq!(result.trust_list, HashMap::from_iter(vec![]));
    }
}
