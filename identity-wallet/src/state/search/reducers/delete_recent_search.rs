use crate::error::AppError;
use crate::state::search::actions::delete_recent_search::DeleteRecentSearch;
use crate::state::search::SearchResults;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn delete_recent_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(recent_search) = listen::<DeleteRecentSearch>(action) {
        let search_results: SearchResults = {
            let mut recent_credentials = state.search_results.recent_credentials.clone();
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
