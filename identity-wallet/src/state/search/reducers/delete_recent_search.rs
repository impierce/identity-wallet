use crate::error::AppError;
use crate::state::search::actions::delete_recent_search::DeleteRecentSearch;
use crate::state::search::SearchResults;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn delete_recent_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(deletion) = listen::<DeleteRecentSearch>(action) {
        let mut recents = state.search_results.recents.clone();
        recents.retain(|recent| recent != &deletion.search_term);
        return Ok(AppState {
            search_results: SearchResults {
                recents,
                ..state.search_results
            },
            ..state
        });
    }
    Ok(state)
}
