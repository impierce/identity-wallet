use crate::error::AppError;
use crate::state::search::actions::search_query::{QueryTarget, SearchQuery};
use crate::state::search::SearchResults;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

use itertools::concat;

pub async fn credential_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(query) = listen::<SearchQuery>(action).filter(|query| !query.search_term.is_empty()) {
        let search_results_current: Vec<String> = {
            let (filtered_creds_name, credentials): (Vec<_>, Vec<_>) = state
                .credentials
                .iter()
                .partition(|credential| contains_search_term(Some(&credential.display_name), &query.search_term));

            let (filtered_creds_issuer, credentials): (Vec<_>, Vec<_>) = credentials
                .into_iter()
                .partition(|credential| contains_search_term(Some(&credential.issuer_name), &query.search_term));

            let filtered_creds_content: Vec<_> = credentials
                .into_iter()
                .filter(|credential| {
                    // Use the to_string function instead of as_str, because the as_str function for Value
                    // works differently than the as_str function from String.
                    contains_search_term(Some(&credential.data.to_string()), &query.search_term)
                })
                .collect();

            concat(vec![filtered_creds_name, filtered_creds_issuer, filtered_creds_content])
                .iter()
                .map(|credential| credential.id.clone())
                .collect()
        };

        let search_results = SearchResults {
            current: search_results_current,
            ..state.search_results
        };

        return Ok(AppState {
            search_results,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

pub async fn connection_search(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(query) = listen::<SearchQuery>(action).filter(|payload| payload.target == QueryTarget::Connections) {
        let search_results_current: Vec<String> = {
            let (filtered_connects_name, connections): (Vec<_>, Vec<_>) = state
                .connections
                .iter()
                .partition(|connection| contains_search_term(Some(&connection.client_name), &query.search_term));

            let filtered_connects_url: Vec<_> = connections
                .into_iter()
                .filter(|connection| contains_search_term(Some(&connection.url), &query.search_term))
                .collect();

            concat(vec![filtered_connects_name, filtered_connects_url])
                .iter()
                .map(|connection| connection.client_name.clone())
                .collect()
        };

        let search_results = SearchResults {
            current: search_results_current,
            ..state.search_results
        };

        return Ok(AppState {
            search_results,
            current_user_prompt: None,
            ..state
        });
    }

    Ok(state)
}

/// Helper for search_query to check if a string contains a search term.
fn contains_search_term(string: Option<&str>, search_term: &str) -> bool {
    string
        .map(|string| string.to_lowercase().contains(&search_term.to_lowercase()))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::credentials::CredentialMetadata;
    use crate::state::credentials::DisplayCredential;

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
        app_state = credential_search(
            app_state,
            Arc::new(SearchQuery {
                target: QueryTarget::Credentials,
                search_term: "".to_string(),
            }),
        )
        .await
        .unwrap();
        assert_eq!(app_state.search_results.current, Vec::<String>::new());

        // Assert that the `user_data_query` results are returned in their order of search relevance.
        app_state = credential_search(
            app_state,
            Arc::new(SearchQuery {
                target: QueryTarget::Credentials,
                search_term: "John".to_string(),
            }),
        )
        .await
        .unwrap();
        assert_eq!(app_state.search_results.current, vec!["1", "3", "2"]);
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
