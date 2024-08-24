use crate::error::AppError;
use crate::state::trust_list::actions::trust_lists_reset::TrustListsReset;
use crate::state::trust_list::TrustLists;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_lists_reset(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(_action) = listen::<TrustListsReset>(action) {
        return Ok(AppState {
            trust_lists: TrustLists::default(),
            current_user_prompt: None,
            ..state
        });
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::trust_list::actions::trust_lists_reset::TrustListsReset;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_reset_trust_lists() {
        let mut state = AppState::default();
        state
            .trust_lists
            .get_mut("impierce")
            .unwrap()
            .insert("test".to_string(), false);

        let action = Arc::new(TrustListsReset {});

        let result = trust_lists_reset(state, action).await.unwrap();

        assert_eq!(result.trust_lists, TrustLists::default());
    }
}
