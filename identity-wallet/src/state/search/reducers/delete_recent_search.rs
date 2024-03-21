use crate::error::AppError;
use crate::state::search::actions::delete_recent_search::DeleteRecentSearch;
use crate::state::search::SearchResults;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn delete_recent_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(deletion) = listen::<DeleteRecentSearch>(action) {
        let search_results: SearchResults = {
            let mut recents_credentials = state.search_results.recent_credentials.clone();
            recents_credentials.retain(|recent| recent != &deletion.search_hit);
            SearchResults {
                recent_credentials: recents_credentials,
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
