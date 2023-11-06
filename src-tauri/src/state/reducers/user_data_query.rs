use crate::state::actions::Action;
use crate::state::{AppState, QueryTarget, SortMethod, UserDataQuery};

fn credential_query(state: &AppState, query: UserDataQuery) -> anyhow::Result<()> {
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

        let mut user_data_query = state.user_data_query.lock().unwrap();
        *user_data_query = filtered_creds;
    }
    if let Some(sort_method) = &query.sort_method {
        let mut creds = state.credentials.lock().unwrap();

        let mut sorted_creds: Vec<String> = match sort_method {
            SortMethod::NameAZ => {
                creds.sort_by(|a, b| a.metadata.display.name.cmp(&b.metadata.display.name));

                creds.iter().map(|s| s.metadata.display.name.clone().unwrap()).collect()
            }
            SortMethod::IssuanceNewOld => {
                creds.sort_by(|a, b| a.metadata.display.name.cmp(&b.metadata.display.name));

                creds.iter().map(|s| s.metadata.display.name.clone().unwrap()).collect()
            }
            SortMethod::AddedNewOld => {
                creds.sort_by(|a, b| a.metadata.display.name.cmp(&b.metadata.display.name));

                creds.iter().map(|s| s.metadata.display.name.clone().unwrap()).collect()
            }
            _ => Vec::new(),
        };

        if query.sort_reverse.unwrap() {
            sorted_creds.reverse();
        }

        let mut user_data_query = state.user_data_query.lock().unwrap();
        *user_data_query = sorted_creds;
    }
    Ok(())
}

fn connection_query(state: &AppState, query: UserDataQuery) -> anyhow::Result<()> {
    if let Some(search_term) = &query.search_term {
        let filtered_creds = state
            .connections
            .lock()
            .unwrap()
            .iter()
            .filter(|cred| cred.client_name.contains(search_term))
            .map(|cred| cred.client_name.clone())
            .collect();

        let mut user_data_query = state.user_data_query.lock().unwrap();
        *user_data_query = filtered_creds;
    }
    if let Some(sort_method) = &query.sort_method {
        let mut connects = state.connections.lock().unwrap();

        let mut sorted_creds: Vec<String> = match sort_method {
            SortMethod::NameAZ => {
                connects.sort_by(|a, b| a.client_name.cmp(&b.client_name));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            SortMethod::IssuanceNewOld => {
                connects.sort_by(|a, b| a.client_name.cmp(&b.client_name));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            SortMethod::AddedNewOld => {
                connects.sort_by(|a, b| a.client_name.cmp(&b.client_name));

                connects.iter().map(|s| s.client_name.clone()).collect()
            }
            _ => Vec::new(),
        };

        if query.sort_reverse.unwrap() {
            sorted_creds.reverse();
        }

        let mut user_data_query = state.user_data_query.lock().unwrap();
        *user_data_query = sorted_creds;
    }
    Ok(())
}

pub async fn user_data_query(state: &AppState, action: Action) -> anyhow::Result<()> {
    let query: UserDataQuery = serde_json::from_value(action.payload.unwrap()).unwrap();

    match query.target {
        QueryTarget::Credentials => credential_query(state, query),
        QueryTarget::Connections => connection_query(state, query),
    }
}
