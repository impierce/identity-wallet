use crate::error::AppError;
use crate::state::search::actions::add_recent::AddRecent;
use crate::state::search::actions::search_query::QueryTarget;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

const MAX_RECENT_SEARCHES: usize = 3;

/// Add the search term to the recents list, with a max of 20.
pub async fn add_recent(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(recent_search) = listen::<AddRecent>(action) {
        let mut search_results = state.search_results.clone();

        match recent_search.target {
            QueryTarget::Credentials => {
                search_results
                    .recents_credentials
                    .retain(|recent| recent != &recent_search.search_hit);
                search_results.recents_credentials.insert(0, recent_search.search_hit);

                if search_results.recents_credentials.len() > MAX_RECENT_SEARCHES {
                    search_results.recents_credentials.remove(MAX_RECENT_SEARCHES);
                }
            }
            QueryTarget::Connections => {
                search_results
                    .recents_connections
                    .retain(|recent| recent != &recent_search.search_hit);
                search_results.recents_connections.insert(0, recent_search.search_hit);

                if search_results.recents_connections.len() > MAX_RECENT_SEARCHES {
                    search_results.recents_connections.remove(MAX_RECENT_SEARCHES);
                }
            }
        }

        return Ok(AppState {
            search_results,
            ..state
        });
    }

    Ok(state)
}
