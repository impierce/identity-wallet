use crate::error::AppError;
use crate::state::actions::Action;
use crate::state::{AppState, QueryTarget, SortMethod, UserDataQuery};
use itertools::concat;

fn credential_query(state: &mut AppState, query: UserDataQuery) -> Result<(), AppError> {
    let mut user_data_query: Vec<String> = vec![];

    if let Some(search_term) = &query.search_term {
        let filtered_creds_name: Vec<String> = state
            .credentials
            .iter()
            .filter(|cred| {
                if let Some(name) = &cred.metadata.display.name {
                    name.to_lowercase().contains(&search_term.to_lowercase())
                } else {
                    false
                }
            })
            .map(|cred| cred.id.clone())
            .collect();

        let filtered_creds_issuer: Vec<String> = state
            .credentials
            .iter()
            .filter(|cred| {
                if let Some(issuer_name) = &cred.issuer_name {
                    issuer_name.to_lowercase().contains(&search_term.to_lowercase())
                } else {
                    false
                }
            })
            .map(|cred| cred.id.clone())
            .filter(|id| !filtered_creds_name.contains(id))
            .collect();

        // prevent content search from going through hash/did keys and the likes.
        let filtered_creds_content: Vec<String> = state
            .credentials
            .iter()
            .filter(|cred| {
                cred.data
                    .to_string()
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            })
            .map(|cred| cred.id.clone())
            .filter(|id| !filtered_creds_name.contains(id) && !filtered_creds_issuer.contains(id))
            .collect();

        user_data_query = concat(vec![filtered_creds_name, filtered_creds_issuer, filtered_creds_content]);
    }

    if let Some(sort_method) = &query.sort_method {
        let mut creds = state.credentials.clone();

        let mut sorted_creds: Vec<String> = match sort_method {
            SortMethod::NameAZ => {
                creds.sort_by(|a, b| a.issuer_name.cmp(&b.issuer_name));

                creds.iter().map(|s| s.id.clone()).collect()
            }
            SortMethod::IssuanceNewOld => {
                creds.sort_by(|a, b| a.metadata.date_issued.cmp(&b.metadata.date_issued));

                creds.iter().map(|s| s.id.clone()).collect()
            }
            SortMethod::AddedNewOld => {
                creds.sort_by(|a, b| a.metadata.date_added.cmp(&b.metadata.date_added));

                creds.iter().map(|s| s.id.clone()).collect()
            }
            _ => Vec::new(),
        };

        if query.sort_reverse.unwrap() {
            sorted_creds.reverse();
        }

        if user_data_query.is_empty() && query.search_term.is_none() {
            user_data_query = sorted_creds;
        } else {
            sorted_creds.retain(|s| user_data_query.contains(s));
            user_data_query = sorted_creds;
        }
    }

    state.user_data_query = user_data_query;
    state.current_user_prompt = None;
    Ok(())
}

fn connection_query(state: &mut AppState, query: UserDataQuery) -> Result<(), AppError> {
    let mut user_data_query: Vec<String> = vec![];

    if let Some(search_term) = &query.search_term {
        let filtered_connects_name: Vec<String> = state
            .connections
            .iter()
            .filter(|connects| {
                connects
                    .client_name
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            })
            .map(|connects| connects.client_name.clone())
            .collect();

        let filtered_connects_url: Vec<String> = state
            .connections
            .iter()
            .filter(|connects| connects.url.to_lowercase().contains(&search_term.to_lowercase()))
            .map(|connects| connects.client_name.clone())
            .filter(|client_name| !filtered_connects_name.contains(client_name))
            .collect();

        user_data_query = concat(vec![filtered_connects_name, filtered_connects_url]);
    }

    if let Some(sort_method) = &query.sort_method {
        let mut connects = state.connections.clone();

        let mut sorted_connects: Vec<String> = match sort_method {
            SortMethod::NameAZ => {
                connects.sort_by(|a, b| a.client_name.cmp(&b.client_name));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            SortMethod::FirstInteractedNewOld => {
                connects.sort_by(|a, b| a.first_interacted.cmp(&b.first_interacted));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            SortMethod::LastInteractedNewOld => {
                connects.sort_by(|a, b| a.last_interacted.cmp(&b.last_interacted));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            _ => Vec::new(),
        };

        if query.sort_reverse.unwrap() {
            sorted_connects.reverse();
        }

        if user_data_query.is_empty() && query.search_term.is_none() {
            user_data_query = sorted_connects;
        } else {
            sorted_connects.retain(|s| user_data_query.contains(s));
            user_data_query = sorted_connects;
        }
    }

    state.user_data_query = user_data_query;
    Ok(())
}

pub async fn user_data_query(state: &mut AppState, action: Action) -> Result<(), AppError> {
    let query: UserDataQuery = serde_json::from_value(action.payload.unwrap()).unwrap();

    match query.target {
        QueryTarget::Credentials => credential_query(state, query),
        QueryTarget::Connections => connection_query(state, query),
    }
}
