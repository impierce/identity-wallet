use crate::error::AppError;
use crate::state::actions::{listen, Action};
use crate::state::{AppState, Connection};
use crate::verifiable_credential_record::DisplayCredential;
use super::actions::UserDataQuery;
use super::{QueryTarget, SortMethod};
use itertools::concat;

pub async fn credential_query(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(query) = listen::<UserDataQuery>(action).filter(|payload| payload.target == QueryTarget::Credentials) {
        let user_data_query: Vec<String> = query
            .search_term
            .as_ref()
            .filter(|search_term| !search_term.is_empty())
            .map(|search_term| {
                let (filtered_creds_name, credentials): (Vec<_>, Vec<_>) =
                    state.credentials.iter().partition(|credential| {
                        contains_search_term(credential.metadata.display.name.as_ref(), search_term)
                    });

                let (filtered_creds_issuer, credentials): (Vec<_>, Vec<_>) = credentials
                    .into_iter()
                    .partition(|credential| contains_search_term(credential.issuer_name.as_ref(), search_term));

                let (filtered_creds_content, _): (Vec<_>, Vec<_>) = credentials.into_iter().partition(|credential| {
                    contains_search_term(Some(credential.data.to_string()).as_ref(), search_term)
                });

                concat(vec![filtered_creds_name, filtered_creds_issuer, filtered_creds_content])
                    .iter()
                    .map(|credential| credential.id.clone())
                    .collect()
            })
            .unwrap_or_default();

        let user_data_query = if let Some(sort_method) = &query.sort_method {
            let mut creds: Vec<&DisplayCredential> = state.credentials.iter().collect();

            let name_az =
                |a: &&DisplayCredential, b: &&DisplayCredential| a.metadata.display.name.cmp(&b.metadata.display.name);
            let issuance_new_old =
                |a: &&DisplayCredential, b: &&DisplayCredential| a.metadata.date_issued.cmp(&b.metadata.date_issued);
            let added_new_old =
                |a: &&DisplayCredential, b: &&DisplayCredential| a.metadata.date_added.cmp(&b.metadata.date_added);

            creds.sort_by(match sort_method {
                SortMethod::NameAZ => name_az,
                SortMethod::IssuanceNewOld => issuance_new_old,
                SortMethod::AddedNewOld => added_new_old,
                _ => name_az,
            });

            let mut sorted_credentials: Vec<String> = creds.iter().map(|s| s.id.clone()).collect();

            query.sort_reverse.then(|| sorted_credentials.reverse());

            if user_data_query.is_empty() && query.search_term.is_none() {
                sorted_credentials
            } else {
                sorted_credentials.retain(|s| user_data_query.contains(s));
                sorted_credentials
            }
        } else {
            user_data_query
        };

        return Ok(AppState {
            user_data_query,
            current_user_prompt: None,
            ..state
        });
    }

    Ok(state)
}

pub async fn connection_query(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(query) = listen::<UserDataQuery>(action).filter(|payload| payload.target == QueryTarget::Connections) {
        let user_data_query: Vec<String> = query
            .search_term
            .as_ref()
            .filter(|search_term| !search_term.is_empty())
            .map(|search_term| {
                let (filtered_connects_name, connections): (Vec<_>, Vec<_>) = state
                    .connections
                    .iter()
                    .partition(|connection| contains_search_term(Some(&connection.client_name), search_term));

                let (filtered_connects_url, _): (Vec<_>, Vec<_>) = connections
                    .into_iter()
                    .partition(|connection| contains_search_term(Some(&connection.url), search_term));

                concat(vec![filtered_connects_name, filtered_connects_url])
                    .iter()
                    .map(|connection| connection.client_name.clone())
                    .collect()
            })
            .unwrap_or_default();

        let user_data_query = if let Some(sort_method) = &query.sort_method {
            let mut connections: Vec<&Connection> = state.connections.iter().collect();

            let name_az = |a: &&Connection, b: &&Connection| a.client_name.cmp(&b.client_name);
            let first_interacted_new_old =
                |a: &&Connection, b: &&Connection| a.first_interacted.cmp(&b.first_interacted);
            let last_interacted_new_old = |a: &&Connection, b: &&Connection| a.last_interacted.cmp(&b.last_interacted);

            connections.sort_by(match sort_method {
                SortMethod::NameAZ => name_az,
                SortMethod::FirstInteractedNewOld => first_interacted_new_old,
                SortMethod::LastInteractedNewOld => last_interacted_new_old,
                _ => name_az,
            });

            let mut sorted_connect: Vec<String> = connections.iter().map(|s| s.client_name.clone()).collect();

            query.sort_reverse.then(|| sorted_connect.reverse());

            if user_data_query.is_empty() && query.search_term.is_none() {
                sorted_connect
            } else {
                sorted_connect.retain(|s| user_data_query.contains(s));
                sorted_connect
            }
        } else {
            user_data_query
        };

        return Ok(AppState {
            user_data_query,
            current_user_prompt: None,
            ..state
        });
    }

    Ok(state)
}

pub fn contains_search_term(string: Option<&String>, search_term: &str) -> bool {
    string
        .map(|string| string.to_lowercase().contains(&search_term.to_lowercase()))
        .unwrap_or_default()
}
