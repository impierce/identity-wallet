use crate::error::AppError;
use crate::state::search::actions::add_recent_search::AddRecentSearch;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

const MAX_RECENT_SEARCHES: usize = 3;

/// Add the search term to the recents list, with a max of 20.
/// Currently we only search credentials, therefore the recents list is called recents_credentials.
pub async fn add_recent_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(recent_search) = listen::<AddRecentSearch>(action) {
        let mut search_results = state.search_results.clone();

        search_results
            .recents_credentials
            .retain(|recent| recent != &recent_search.search_hit);
        search_results.recents_credentials.insert(0, recent_search.search_hit);

        if search_results.recents_credentials.len() > MAX_RECENT_SEARCHES {
            search_results.recents_credentials.remove(MAX_RECENT_SEARCHES);
        }

        return Ok(AppState {
            search_results,
            current_user_prompt: None,
            ..state
        });
    }

    Ok(state)
}
