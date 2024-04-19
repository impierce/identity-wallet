use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        connections::{Connection, Connections},
        credentials::DisplayCredential,
        profile_settings::{
            actions::update_sorting_preference::UpdateSortingPreference, ConnectionSortMethod, CredentialSortMethod,
            Locale, Preferences, ProfileSettings,
        },
        AppState,
    },
};

use icu::collator::*;
use log::debug;
use unicode_normalization::UnicodeNormalization;

// Frontend shouldn't persist reverse setting on any of the sorting options which aren't selected.
// Because the backend doesn't persist this either.

pub async fn update_sorting_preference(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(update_sorting) = listen::<UpdateSortingPreference>(action) {
        let mut sorting_preferences = state.profile_settings.sorting_preferences.clone();

        if let Some(credential_sorting) = update_sorting.credential_sorting {
            debug!(
                "Update credential sorting preference set to: `{:?}`",
                credential_sorting
            );
            sorting_preferences.credentials.sort_method = credential_sorting;
            // With this nested if let statement the user (should) automatically select the sort method when toggling the reverse icon on that sort method.
            // If not nested (and therefore not repeated but just checking once in a separate if let statement), the user would be able to toggle the reverse option on a sort method without selecting it.
            // Check the UX designs if the meaning of this comment not clear.
            if let Some(reverse) = update_sorting.reverse {
                debug!("Update credential sorting preference set to: `{:?}`", reverse);
                sorting_preferences.credentials.reverse = reverse;
            }
        } else if let Some(connection_sorting) = update_sorting.connection_sorting {
            debug!(
                "Update connection sorting preference set to: `{:?}`",
                connection_sorting
            );
            sorting_preferences.connections.sort_method = connection_sorting;
            if let Some(reverse) = update_sorting.reverse {
                debug!("Update connection sorting preference set to: `{:?}`", reverse);
                sorting_preferences.connections.reverse = reverse;
            }
        }

        return Ok(AppState {
            profile_settings: ProfileSettings {
                sorting_preferences,
                ..state.profile_settings
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

pub async fn sort_credentials(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let mut credentials: Vec<DisplayCredential> = state.credentials.clone();
    let preferences: Preferences<CredentialSortMethod> = state.profile_settings.sorting_preferences.credentials.clone();

    let list: Vec<String> = credentials.iter().map(|x| x.display_name.clone()).collect();
    let sorted_list = sort(list, state.profile_settings.locale.clone());

    let name_az = |a: &DisplayCredential, b: &DisplayCredential| {
        let pos_a = sorted_list
            .iter()
            .position(|display_name| display_name == &a.display_name)
            .unwrap();
        let pos_b = sorted_list
            .iter()
            .position(|display_name| display_name == &b.display_name)
            .unwrap();
        pos_a.cmp(&pos_b)
    };

    match preferences.sort_method {
        CredentialSortMethod::NameAZ => credentials.sort_by(name_az),
        CredentialSortMethod::IssueDateNewOld => credentials.sort_by(|a: &DisplayCredential, b: &DisplayCredential| {
            a.metadata.date_issued.cmp(&b.metadata.date_issued).reverse()
        }),
        CredentialSortMethod::AddedDateNewOld => credentials.sort_by(|a: &DisplayCredential, b: &DisplayCredential| {
            a.metadata.date_added.cmp(&b.metadata.date_added).reverse()
        }),
    };

    if preferences.reverse {
        credentials.reverse();
    }

    // current_user_prompt is not set to None,
    // as this reducer is often used in combination with reducers that need to send a user_prompt to the frontend.
    Ok(AppState { credentials, ..state })
}

pub async fn sort_connections(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let mut connections: Vec<Connection> = state.connections.0.clone();
    let preferences: Preferences<ConnectionSortMethod> = state.profile_settings.sorting_preferences.connections.clone();

    let list: Vec<String> = connections.iter().map(|x| x.name.clone()).collect();
    let sorted_list = sort(list, state.profile_settings.locale.clone());

    let name_az = |a: &Connection, b: &Connection| {
        let pos_a = sorted_list.iter().position(|name| name == &a.name).unwrap();
        let pos_b = sorted_list.iter().position(|name| name == &b.name).unwrap();
        pos_a.cmp(&pos_b)
    };

    match preferences.sort_method {
        ConnectionSortMethod::NameAZ => connections.sort_by(name_az),
        ConnectionSortMethod::FirstInteractedNewOld => {
            connections.sort_by(|a: &Connection, b: &Connection| a.first_interacted.cmp(&b.first_interacted))
        }
        ConnectionSortMethod::LastInteractedNewOld => {
            connections.sort_by(|a: &Connection, b: &Connection| a.last_interacted.cmp(&b.last_interacted))
        }
    };

    if preferences.reverse {
        connections.reverse();
    }

    // current_user_prompt is not set to None,
    // as this reducer is often used in combination with reducers that need to send a user_prompt to the frontend.
    Ok(AppState {
        connections: Connections(connections),
        ..state
    })
}

// Helpers //

/// Sort a Vector of words alphabetically, taking into account the locale of the words:
/// standard `.sorted()` -> ["Zambia", "enlever", "zoo", "énigme"]
/// `sort(list, locale)` -> ["énigme", "enlever", "Zambia", "zoo"]
/// Also making it Unicode compatible, using NFC-normalized form.
pub fn sort(list: Vec<String>, locale: Locale) -> Vec<String> {
    // https://github.com/unicode-org/icu4x/tree/main/components/collator#examples

    // Convert locale from AppState to icu_locid::Locale
    let locale: &str = locale.into();
    let locale: icu::locid::Locale = locale.replace('_', "-").parse().unwrap();

    // Normalize all strings to Unicode, using NFC-normalized form.
    let normalized_list: Vec<String> = list.iter().map(|name| name.nfc().collect::<String>()).collect();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Secondary);
    let collator: Collator = Collator::try_new(&locale.into(), options).unwrap();

    let mut sorted_list = normalized_list;
    sorted_list.sort_by(|a, b| collator.compare(a, b));
    sorted_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::credentials::CredentialMetadata;

    use std::sync::Arc;

    // update_sorting_preference testing //

    // Currently we #[PartialEq(ignore)] the metadata.date_added field since it will fail many other tests.
    // The json files are static but some tests create new credentials and the date_added field will be added at this time.
    #[tokio::test]
    async fn test_credentials_update_sorting_setting_date_added() {

        let state = AppState::default();
        let action = Arc::new(UpdateSortingPreference {
            credential_sorting: Some(CredentialSortMethod::AddedDateNewOld),
            reverse: Some(true),
            ..Default::default()
        });

        let result = update_sorting_preference(state, action.clone()).await.unwrap();
        let result = sort_credentials(result, action).await.unwrap();

        assert_eq!(
            result.profile_settings.sorting_preferences.credentials,
            Preferences {
                sort_method: CredentialSortMethod::AddedDateNewOld,
                reverse: true,
            }
        );
    }

    // sort_credentials tests //

    #[tokio::test]
    async fn test_credentials_sorting_name_az() {
        let state = init_credential_names("C".to_string(), "A".to_string(), "B".to_string());
        let action = Arc::new(UpdateSortingPreference { ..Default::default() });

        let result = update_sorting_preference(state, action.clone()).await.unwrap();
        let result = sort_credentials(result, action).await.unwrap();

        assert_eq!(
            result
                .credentials
                .iter()
                .map(|x| x.display_name.clone())
                .collect::<Vec<String>>(),
            vec!["A".to_string(), "B".to_string(), "C".to_string()]
        );
    }

    #[tokio::test]
    async fn test_credentials_sorting_name_az_reverse() {
        let state = init_credential_names("C".to_string(), "A".to_string(), "B".to_string());
        let action = Arc::new(UpdateSortingPreference {
            credential_sorting: Some(CredentialSortMethod::NameAZ),
            reverse: Some(true),
            ..Default::default()
        });

        let result = update_sorting_preference(state, action.clone()).await.unwrap();
        let result = sort_credentials(result, action).await.unwrap();

        assert_eq!(
            result
                .credentials
                .iter()
                .map(|x| x.display_name.clone())
                .collect::<Vec<String>>(),
            vec!["C".to_string(), "B".to_string(), "A".to_string()]
        );
    }

    #[tokio::test]
    async fn test_credentials_sorting_issue_reverse() {
        let state = init_credential_issuance_dates(
            "2022-00-00T00:00:00Z".to_string(),
            "2019-00-00T00:00:00Z".to_string(),
            "2020-00-00T00:00:00Z".to_string(),
        );
        let action = Arc::new(UpdateSortingPreference {
            credential_sorting: Some(CredentialSortMethod::IssueDateNewOld),
            reverse: Some(true),
            ..Default::default()
        });

        let result = update_sorting_preference(state, action.clone()).await.unwrap();
        let result = sort_credentials(result, action).await.unwrap();

        assert_eq!(
            result
                .credentials
                .iter()
                .map(|x| x.metadata.date_issued.clone())
                .collect::<Vec<String>>(),
            vec![
                "2019-00-00T00:00:00Z".to_string(),
                "2020-00-00T00:00:00Z".to_string(),
                "2022-00-00T00:00:00Z".to_string()
            ]
        );
    }

    // sort_connections tests //

    #[tokio::test]
    async fn test_connections_sorting_name_az_reverse() {
        let state = init_connection_names(
            "Gym".to_string(),
            "Work".to_string(),
            "School".to_string(),
        );
        let action = Arc::new(UpdateSortingPreference {
            connection_sorting: Some(ConnectionSortMethod::NameAZ),
            reverse: Some(true),
            ..Default::default()
        });

        let result = update_sorting_preference(state, action.clone()).await.unwrap();
        let result = sort_connections(result, action).await.unwrap();

        assert_eq!(
            result
                .connections.0
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>(),
            vec![
                "Work".to_string(),
                "School".to_string(),
                "Gym".to_string(),
            ]
        );
    }

    // In fact, the {last_interacted, reverse: true} is the same as {first_interacted, reverse:false}.
    // For this reason the reverse button will disabled for these sorting options.
    #[tokio::test]
    async fn test_connections_sorting_first_interact() {
        let state = init_connection_interactions(
            "2019-00-00T00:00:00Z".to_string(),
            "2020-00-00T00:00:00Z".to_string(),
            "2022-00-00T00:00:00Z".to_string(),
        );
        let action = Arc::new(UpdateSortingPreference {
            connection_sorting: Some(ConnectionSortMethod::FirstInteractedNewOld),
            reverse: Some(false),
            ..Default::default()
        });

        let result = update_sorting_preference(state, action.clone()).await.unwrap();
        let result = sort_connections(result, action).await.unwrap();

        assert_eq!(
            result
                .connections.0
                .iter()
                .map(|x| x.first_interacted.clone())
                .collect::<Vec<String>>(),
            vec![
                "2019-00-00T00:00:00Z".to_string(),
                "2020-00-00T00:00:00Z".to_string(),
                "2022-00-00T00:00:00Z".to_string(),
            ]
        );
    }

    #[tokio::test]
    async fn test_connections_sorting_last_interact() {
        let state = init_connection_interactions(
            "2020-00-00T00:00:00Z".to_string(),
            "2019-00-00T00:00:00Z".to_string(),
            "2022-00-00T00:00:00Z".to_string(),
        );
        let action = Arc::new(UpdateSortingPreference {
            connection_sorting: Some(ConnectionSortMethod::LastInteractedNewOld),
            reverse: Some(false),
            ..Default::default()
        });

        let result = update_sorting_preference(state, action.clone()).await.unwrap();
        let result = sort_connections(result, action).await.unwrap();

        assert_eq!(
            result
                .connections.0
                .iter()
                .map(|x| x.last_interacted.clone())
                .collect::<Vec<String>>(),
            vec![
                "2019-00-00T00:00:00Z".to_string(),
                "2020-00-00T00:00:00Z".to_string(),
                "2022-00-00T00:00:00Z".to_string(),
            ]
        );
    }
    // Helpers - credentials//

    fn init_credential_names(test_1: String, test_2: String, test_3: String) -> AppState {
        AppState {
            credentials: vec![
                DisplayCredential {
                    display_name: test_1,
                    ..Default::default()
                },
                DisplayCredential {
                    display_name: test_2,
                    ..Default::default()
                },
                DisplayCredential {
                    display_name: test_3,
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    fn init_credential_issuance_dates(test_1: String, test_2: String, test_3: String) -> AppState {
        AppState {
            credentials: vec![
                DisplayCredential {
                    metadata: CredentialMetadata {
                        date_issued: test_1,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                DisplayCredential {
                    metadata: CredentialMetadata {
                        date_issued: test_2,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                DisplayCredential {
                    metadata: CredentialMetadata {
                        date_issued: test_3,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    // Helpers - connections //

    fn init_connection_names(test_1: String, test_2: String, test_3: String) -> AppState {
        AppState {
            connections: Connections(vec![
                Connection {
                    name: test_1,
                    ..Default::default()
                },
                Connection {
                    name: test_2,
                    ..Default::default()
                },
                Connection {
                    name: test_3,
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }
    }

    fn init_connection_interactions(test_1: String, test_2: String, test_3: String) -> AppState {
        AppState {
            connections: Connections(vec![
                Connection {
                    first_interacted: test_1.clone(),
                    last_interacted: test_1,
                    ..Default::default()
                },
                Connection {
                    first_interacted: test_2.clone(),
                    last_interacted: test_2,
                    ..Default::default()
                },
                Connection {
                    first_interacted: test_3.clone(),
                    last_interacted: test_3,
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }
    }
}
