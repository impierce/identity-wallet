use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod actions;
pub mod reducers;

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


#[cfg(test)]
mod tests {
    use crate::state::{actions::Action, credentials::{CredentialDisplay, CredentialMetadata, DisplayCredential}, AppState};
    use super::*;
    use oid4vc::oid4vci::credential_format_profiles::{
        w3c_verifiable_credentials::jwt_vc_json::JwtVcJson, CredentialFormats, Profile,
    };
    use tests::{actions::UserDataQuery, reducers::{contains_search_term, credential_query}};
    use std::{sync::Arc, vec};

    #[test]
    fn test_contains_search_term() {
        assert!(contains_search_term(Some(&"John Doe".to_string()), "john"));
        assert!(contains_search_term(Some(&"John Doe".to_string()), "doe"));
        assert!(contains_search_term(Some(&"John Doe".to_string()), "john doe"));
        assert!(contains_search_term(Some(&"John Doe".to_string()), "JOHN DOE"));
        assert!(contains_search_term(Some(&"John Doe".to_string()), "JOHN"));
        assert!(contains_search_term(Some(&"John Doe".to_string()), "DOE"));
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
                sort_reverse: false,
            }) as Action,
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
                sort_reverse: false,
            }) as Action,
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
                sort_reverse: false,
            }) as Action,
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
                        issuer_name: Some("Example Organization".to_string()),
                        format: CredentialFormats::JwtVcJson(Profile { format: JwtVcJson }),
                        data: serde_json::json!({"last_name": "Ferris"}),
                        metadata: CredentialMetadata {
                            date_issued: "2021-01-01".to_string(),
                            date_added: "2021-01-01".to_string(),
                            display: CredentialDisplay {
                                name: Some("John".to_string()),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    },
                    DisplayCredential {
                        id: "2".to_string(),
                        issuer_name: Some("Example Organization".to_string()),
                        format: CredentialFormats::JwtVcJson(Profile { format: JwtVcJson }),
                        data: serde_json::json!({"last_name": "John"}),
                        metadata: CredentialMetadata {
                            date_issued: "2021-01-02".to_string(),
                            date_added: "2021-02-01".to_string(),
                            display: CredentialDisplay {
                                name: Some("Jane".to_string()),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    },
                    DisplayCredential {
                        id: "3".to_string(),
                        issuer_name: Some("John Organization".to_string()),
                        format: CredentialFormats::JwtVcJson(Profile { format: JwtVcJson }),
                        data: serde_json::json!({"last_name": "Ferris"}),
                        metadata: CredentialMetadata {
                            date_issued: "2021-01-03".to_string(),
                            date_added: "2021-03-01".to_string(),
                            display: CredentialDisplay {
                                name: Some("Jeff".to_string()),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    },
                ],
            ..Default::default()
        }
    }
}
