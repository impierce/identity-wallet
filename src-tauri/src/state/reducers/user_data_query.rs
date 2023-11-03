use crate::crypto::stronghold::StrongholdManager;
use crate::state::actions::Action;
use crate::state::{AppState, Connection, Profile, QueryTarget, TransferState, UserDataQuery};
use crate::verifiable_credential_record::VerifiableCredentialRecord;

fn credential_query(state: &AppState, query: UserDataQuery) -> anyhow::Result<()> {
    if let Some(search_term) = &query.search_term {
        let filtered_creds = state.credentials.lock().unwrap()
            .iter().filter(|cred| {
                if let Some(issuer_name) = &cred.issuer_name {
                    issuer_name.contains(search_term)
                } else {
                    false
                }
            }).map(|cred| cred.id.clone()).collect();

            let mut user_data_query = state.user_data_query.lock().unwrap();
            *user_data_query = filtered_creds;
    }
    Ok()
}

fn connection_query(state: &AppState, query: UserDataQuery) -> anyhow::Result<()> {
    let mut connections = state.connections.lock().unwrap();
    let mut connections = if let Some(search_term) = query.search_term {
        connections.filter|s| { client_name.contains(&payload.clone().unwrap().to_string())

    } else {
        connections
    };

    let mut connections = if let Some(sort_method) = query.sort_method {
        connections
    } else {
        connections
    };

    Ok(())
}
}

pub async fn user_data_query(state: &AppState, action: Action) -> anyhow::Result<()> {
    let query: UserDataQuery = serde_json::from_value(action.payload.unwrap()).unwrap();

    match query.target {
        QueryTarget::Credentials => credential_query(state, query),
        QueryTarget::Connections => connection_query(state, query),
    }
}
