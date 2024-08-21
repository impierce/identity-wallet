use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::actions::delete_credential::DeleteCredential,
        AppState,
    },
};

pub async fn delete_credential(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(delete_credential) = listen::<DeleteCredential>(action) {

        let mut credentials = state.credentials.clone();
        credentials.retain(|credential| credential.id != delete_credential.id.to_string());
        
        return Ok(AppState {
            credentials,
            current_user_prompt: None,
            ..state
        });
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::state::credentials::DisplayCredential;
    use crate::state::AppState;

    #[tokio::test]
    async fn test_delete_credential() {
        let credential = DisplayCredential {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
            ..Default::default()
        };

        let state = AppState {
            credentials: vec![credential],
            current_user_prompt: None,
            ..Default::default()
        };

        let action = Arc::new(DeleteCredential {
            id: uuid::Uuid::parse_str(&state.credentials[0].id).unwrap(),
        });

        let result = delete_credential(state, action).await.unwrap();

        assert_eq!(result.credentials, vec![]);
    }
}
