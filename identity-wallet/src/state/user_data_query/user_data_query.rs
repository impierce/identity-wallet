use crate::error::AppError;
use crate::reducer;
use crate::state::connections::Connection;
use crate::state::credentials::DisplayCredential;
use crate::state::{
    actions::{listen, Action, ActionTrait},
    AppState, Reducer,
};
use itertools::concat;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[typetag::serde(name = "[User Data] Query")]
impl ActionTrait for UserDataQuery {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(credential_query), reducer!(connection_query)]
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum QueryTarget {
    Credentials,
    Connections,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum SortMethod {
    NameAZ,
    IssuanceNewOld,
    AddedNewOld,
    FirstInteractedNewOld,
    LastInteractedNewOld,
}

/// Action to query user data.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UserDataQuery.ts")]
pub struct UserDataQuery {
    pub target: QueryTarget,
    #[ts(optional)]
    pub search_term: Option<String>,
    #[ts(optional)]
    pub sort_method: Option<SortMethod>,
    #[ts(optional)]
    pub sort_reverse: Option<bool>,
}

/// Helper for connection/credential_query to check if a string contains a search term.
fn contains_search_term(opt_str: Option<&str>, search_term: &str) -> bool {
    opt_str
        .map(|str| str.to_lowercase().contains(&search_term.to_lowercase()))
        .unwrap_or_default()
}

pub async fn credential_query(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(query) = listen::<UserDataQuery>(action).filter(|payload| payload.target == QueryTarget::Credentials) {
        let user_data_query: Vec<String> = query
            .search_term
            .as_ref()
            .filter(|search_term| !search_term.is_empty())
            .map(|search_term| {
                let (filtered_creds_name, credentials): (Vec<_>, Vec<_>) = state
                    .credentials
                    .iter()
                    .partition(|credential| contains_search_term(Some(&credential.display_name), search_term));

                let (filtered_creds_issuer, credentials): (Vec<_>, Vec<_>) = credentials
                    .into_iter()
                    .partition(|credential| contains_search_term(Some(&credential.issuer_name), search_term));

                let filtered_creds_content: Vec<_> = credentials
                    .into_iter()
                    .filter(|credential| {
                        // Use the to_string function instead of as_str, because the as_str function for Value
                        // works differently than the as_str function from String.
                        contains_search_term(Some(&credential.data.to_string()), search_term)
                    })
                    .collect();

                concat(vec![filtered_creds_name, filtered_creds_issuer, filtered_creds_content])
                    .iter()
                    .map(|credential| credential.id.clone())
                    .collect()
            })
            .unwrap_or_default();

        let user_data_query = if let Some(sort_method) = &query.sort_method {
            let mut creds: Vec<&DisplayCredential> = state.credentials.iter().collect();

            let name_az = |a: &&DisplayCredential, b: &&DisplayCredential| a.display_name.cmp(&b.display_name);
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

            if let Some(sort_reverse) = query.sort_reverse {
                if sort_reverse {
                    sorted_credentials.reverse();
                }
            }

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

                let filtered_connects_url: Vec<_> = connections
                    .into_iter()
                    .filter(|connection| contains_search_term(Some(&connection.url), search_term))
                    .collect();

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

            if let Some(sort_reverse) = query.sort_reverse {
                if sort_reverse {
                    sorted_connect.reverse();
                }
            }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::credentials::CredentialMetadata;
    use oid4vc::oid4vci::credential_format_profiles::{
        w3c_verifiable_credentials::jwt_vc_json::JwtVcJson, CredentialFormats, Profile,
    };
    use std::{sync::Arc, vec};

    #[test]
    fn test_contains_search_term() {
        assert!(contains_search_term(Some("John Doe"), "john"));
        assert!(contains_search_term(Some("John Doe"), "doe"));
        assert!(contains_search_term(Some("John Doe"), "john doe"));
        assert!(contains_search_term(Some("John Doe"), "JOHN DOE"));
        assert!(contains_search_term(Some("John Doe"), "JOHN"));
        assert!(contains_search_term(Some("John Doe"), "DOE"));
        assert!(!contains_search_term(None, "john doe"));
        assert!(!contains_search_term(None, ""));
    }

    #[tokio::test]
    async fn test_user_data_query() {
        let mut app_state = app_state();

        // Assert that the `user_data_query` is empty when the `search_term` is empty.
        app_state = credential_query(
            app_state,
            Arc::new(UserDataQuery {
                target: QueryTarget::Credentials,
                search_term: Some("".to_string()),
                sort_method: None,
                sort_reverse: None,
            }),
        )
        .await
        .unwrap();
        assert_eq!(app_state.user_data_query, Vec::<String>::new());

        // Assert that the `user_data_query` results are returned in their order of search relevance.
        app_state = credential_query(
            app_state,
            Arc::new(UserDataQuery {
                target: QueryTarget::Credentials,
                search_term: Some("John".to_string()),
                sort_method: None,
                sort_reverse: None,
            }),
        )
        .await
        .unwrap();
        assert_eq!(app_state.user_data_query, vec!["1", "3", "2"]);

        // Assert that the `user_data_query` results are sorted by name in ascending order.
        app_state = credential_query(
            app_state,
            Arc::new(UserDataQuery {
                target: QueryTarget::Credentials,
                search_term: None,
                sort_method: Some(SortMethod::NameAZ),
                sort_reverse: None,
            }),
        )
        .await
        .unwrap();
        assert_eq!(app_state.user_data_query, vec!["2", "3", "1"]);
    }

    fn app_state() -> AppState {
        AppState {
            credentials: vec![
                DisplayCredential {
                    id: "1".to_string(),
                    issuer_name: "Example Organization".to_string(),
                    format: CredentialFormats::JwtVcJson(Profile { format: JwtVcJson }),
                    data: serde_json::json!({"last_name": "Ferris"}),
                    metadata: CredentialMetadata {
                        date_issued: "2021-01-01".to_string(),
                        date_added: "2021-01-01".to_string(),
                        ..Default::default()
                    },
                    display_name: "John".to_string(),
                    display_icon: None,
                    display_color: None,
                },
                DisplayCredential {
                    id: "2".to_string(),
                    issuer_name: "Example Organization".to_string(),
                    format: CredentialFormats::JwtVcJson(Profile { format: JwtVcJson }),
                    data: serde_json::json!({"last_name": "John"}),
                    metadata: CredentialMetadata {
                        date_issued: "2021-01-02".to_string(),
                        date_added: "2021-02-01".to_string(),
                        ..Default::default()
                    },
                    display_name: "Jane".to_string(),
                    display_icon: None,
                    display_color: None,
                },
                DisplayCredential {
                    id: "3".to_string(),
                    issuer_name: "John Organization".to_string(),
                    format: CredentialFormats::JwtVcJson(Profile { format: JwtVcJson }),
                    data: serde_json::json!({"last_name": "Ferris"}),
                    metadata: CredentialMetadata {
                        date_issued: "2021-01-03".to_string(),
                        date_added: "2021-03-01".to_string(),
                        ..Default::default()
                    },
                    display_name: "Jeff".to_string(),
                    display_icon: None,
                    display_color: None,
                },
            ],
            ..Default::default()
        }
    }
}
