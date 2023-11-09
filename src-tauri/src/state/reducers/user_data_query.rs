use crate::state::actions::Action;
use crate::state::{AppState, QueryTarget, SortMethod, UserDataQuery};

fn credential_query(state: &AppState, query: UserDataQuery) -> anyhow::Result<()> {
    let mut user_data_query: Vec<String> = vec![] ;

    if let Some(search_term) = &query.search_term {
        let filtered_creds = state
            .credentials
            .lock()
            .unwrap()
            .iter()
            .filter(|cred| {
                if let Some(issuer_name) = &cred.issuer_name {
                    issuer_name.contains(search_term)
                } else {
                    false
                }
            })
            .map(|cred| cred.id.clone())
            .collect();

        user_data_query = filtered_creds;
    }
    if let Some(sort_method) = &query.sort_method {
        let mut creds = state.credentials.lock().unwrap();

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
        }
        else {
            sorted_creds.retain(|s| user_data_query.contains(s));
            user_data_query = sorted_creds;
        }
    }
    
    *state.user_data_query.lock().unwrap() = user_data_query;
    Ok(())
}

fn connection_query(state: &AppState, query: UserDataQuery) -> anyhow::Result<()> {
    let mut user_data_query: Vec<String> = vec![];

    if let Some(search_term) = &query.search_term {
        let filtered_connects: Vec<String> = state
            .connections
            .lock()
            .unwrap()
            .iter()
            .filter(|connects| connects.client_name.contains(search_term))
            .map(|connects| connects.client_name.clone())
            .collect();

        user_data_query = filtered_connects;
    }
    if let Some(sort_method) = &query.sort_method {
        let mut connects = state.connections.lock().unwrap();

        let mut sorted_connects: Vec<String> = match sort_method {
            SortMethod::NameAZ => {
                connects.sort_by(|a, b| a.client_name.cmp(&b.client_name));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            SortMethod::FirstConnectedNewOld => {
                connects.sort_by(|a, b| a.first_connected.cmp(&b.first_connected));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            SortMethod::LastConnectedNewOld => {
                connects.sort_by(|a, b| a.last_connected.cmp(&b.last_connected));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            _ => Vec::new(),
        };

        if query.sort_reverse.unwrap() {
            sorted_connects.reverse();
        }

        if user_data_query.is_empty() && query.search_term.is_none() {
            user_data_query = sorted_connects;
        }
        else {
            sorted_connects.retain(|s| user_data_query.contains(s));
            user_data_query = sorted_connects;
        }
    }

    *state.user_data_query.lock().unwrap() = user_data_query;
    Ok(())
}

pub async fn user_data_query(state: &AppState, action: Action) -> anyhow::Result<()> {
    let query: UserDataQuery = serde_json::from_value(action.payload.unwrap()).unwrap();

    match query.target {
        QueryTarget::Credentials => credential_query(state, query),
        QueryTarget::Connections => connection_query(state, query),
    }
}
