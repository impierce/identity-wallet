use crate::error::AppError;
use crate::state::search::actions::delete_recent_search::DeleteRecentSearch;
use crate::state::search::actions::search_query::QueryTarget;
use crate::state::search::SearchResults;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn delete_recent_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(deletion) = listen::<DeleteRecentSearch>(action) {
        let search_results: SearchResults = match deletion.delete_target {
            QueryTarget::Connections => {
                let mut recents_connections = state.search_results.recents_connections.clone();
                recents_connections.retain(|recent| recent != &deletion.search_term);
                SearchResults {
                    recents_connections,
                    ..state.search_results
                }
            }
            QueryTarget::Credentials => {
                let mut recents_credentials = state.search_results.recents_credentials.clone();
                recents_credentials.retain(|recent| recent != &deletion.search_term);
                SearchResults {
                    recents_credentials,
                    ..state.search_results
                }
            }
        };
        return Ok(AppState {
            search_results,
            ..state
        });
    }
    Ok(state)
}
