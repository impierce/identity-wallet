use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

pub async fn dismiss_interaction_result(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        interaction_result: None,
        ..state
    })
}

/// Opens the verifier's `redirect_uri` of the last presentation in the browser and closes the result pop-up. Without
/// a `redirect_uri` it only closes the pop-up. When the browser cannot be opened, the result stays, so the user can
/// try again or close it.
pub async fn continue_to_verifier(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let redirect_uri = state
        .interaction_result
        .as_ref()
        .and_then(|interaction_result| interaction_result.redirect_uri.as_ref());

    // When testing, Tauri is often not initialized and the link doesn't actually need to be opened anyway.
    #[cfg(not(feature = "test_utils"))]
    if let Some(redirect_uri) = redirect_uri {
        use tauri_plugin_opener::OpenerExt;

        // Only the host is logged: the URI can carry a response code.
        let host = crate::logging::url_host(redirect_uri);
        let app_handle = state
            .core_utils
            .app_handle
            .as_ref()
            .ok_or(AppError::Error("Tauri app handle is not available".to_string()))?;
        log::info!("Opening the verifier's redirect_uri at {host} in the browser");
        app_handle
            .opener()
            .open_url(redirect_uri.as_str(), None::<&str>)
            .map_err(|err| AppError::BrowserUnavailableError(format!("Failed to open URL in browser: {err}")))?;
    }
    #[cfg(feature = "test_utils")]
    let _ = redirect_uri;

    Ok(AppState {
        interaction_result: None,
        ..state
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::interaction_result::{
        actions::ContinueToVerifier, InteractionKind, InteractionOutcome, InteractionResult,
    };
    use std::sync::Arc;

    fn state_with(redirect_uri: Option<&str>) -> AppState {
        AppState {
            interaction_result: Some(
                InteractionResult::new(
                    InteractionKind::Presentation,
                    "Verifier".to_string(),
                    InteractionOutcome::Approved,
                )
                .continuing_at(redirect_uri.map(|redirect_uri| redirect_uri.parse().unwrap())),
            ),
            ..AppState::default()
        }
    }

    #[tokio::test]
    async fn continuing_without_a_redirect_uri_closes_the_result() {
        let state = continue_to_verifier(state_with(None), Arc::new(ContinueToVerifier))
            .await
            .unwrap();

        assert_eq!(state.interaction_result, None);
    }

    // Without Tauri there is no browser to open: the action fails, and `main_exec` then keeps the state, so the
    // result with its `redirect_uri` stays for another try.
    #[cfg(not(feature = "test_utils"))]
    #[tokio::test]
    async fn continuing_fails_when_the_browser_cannot_be_opened() {
        let result = continue_to_verifier(
            state_with(Some("https://verifier.example/done")),
            Arc::new(ContinueToVerifier),
        )
        .await;

        assert!(result.is_err());
    }
}
