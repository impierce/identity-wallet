use crate::error::AppError;
use crate::state::search::actions::delete_recent_search::DeleteRecentSearch;
use crate::state::search::SearchResults;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

#[tracing::instrument(skip_all, err)]
pub async fn delete_recent_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(recent_search) = listen::<DeleteRecentSearch>(action) {
        log::debug!("Deleting recent search credential id: `{}`", recent_search.id);
        let search_results: SearchResults = {
            let mut recent_credentials = state.search_results.recent_credentials;
            recent_credentials.retain(|recent| recent != &recent_search.id);
            SearchResults {
                recent_credentials,
                ..state.search_results
            }
        };
        return Ok(AppState {
            search_results,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
