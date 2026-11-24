use crate::error::AppError;
use crate::state::actions::{listen, Action};
use crate::state::connections::actions::load_issued_credentials::LoadIssuedCredentials;
use crate::state::AppState;

#[tracing::instrument(skip_all, err)]
pub async fn load_issued_credentials(state: AppState, action: Action) -> Result<AppState, AppError> {
    let Some(load_issued_credentials) = listen::<LoadIssuedCredentials>(action) else {
        return Ok(state);
    };

    // Loaded in `prepare` when dispatched from the frontend, or here when reduced directly.
    let page = match load_issued_credentials.loaded {
        Some(page) => page,
        None => load_issued_credentials.load(&state).await,
    };

    Ok(AppState {
        issued_credentials: Some(page),
        ..state
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::connections::{Connection, Connections};
    use std::sync::Arc;

    #[tokio::test]
    async fn stores_the_page_of_an_unknown_connection_as_unavailable() {
        let state = AppState {
            connections: Connections(vec![Connection {
                id: "known".to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let state = load_issued_credentials(
            state,
            Arc::new(LoadIssuedCredentials {
                connection_id: "unknown".to_string(),
                search_term: "badge".to_string(),
                offset: 0,
                limit: 20,
                loaded: None,
            }),
        )
        .await
        .unwrap();

        let page = state.issued_credentials.unwrap();
        assert_eq!(page.connection_id, "unknown");
        assert_eq!(page.search_term, "badge");
        assert!(page.unavailable);
    }
}
