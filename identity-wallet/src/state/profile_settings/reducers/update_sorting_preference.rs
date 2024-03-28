use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        connections::{Connection, Connections},
        credentials::DisplayCredential,
        profile_settings::{
            actions::update_sorting_preference::UpdateSortingPreference, ConnectionSortMethod, CredentialSortMethod,
            Preferences, ProfileSettings,
        },
        AppState,
    },
};

use log::debug;

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
            // Check the UX designs if the meaning of this comment not clear.
            // If not nested (and therefore not repeated but just checking once in a separate if let statement), the user would be able to toggle the reverse option on a sort method without selecting it.
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
            ..state
        });
    }
    Ok(state)
}

pub async fn sort_credentials(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let mut credentials: Vec<DisplayCredential> = state.credentials.clone();
    let preferences: Preferences<CredentialSortMethod> = state.profile_settings.sorting_preferences.credentials.clone();

    // testing

    // let credential = credentials.first().unwrap().data.to_string();
    // fn contains_search_term(string: Option<&str>, search_term: &str) -> bool {
    //     string
    //         .map(|string| string.to_lowercase().contains(&search_term.trim().to_lowercase()))
    //         .unwrap_or_default()
    // }

    // if contains_search_term(Some(&credential), "issuanceDate") {
    //     credentials.first().unwrap().metadata.date_issued = issuanceDate;
    // }

    //
    let name_az = |a: &DisplayCredential, b: &DisplayCredential| a.display_name.cmp(&b.display_name);
    let issuance_new_old =
        |a: &DisplayCredential, b: &DisplayCredential| a.metadata.date_issued.cmp(&b.metadata.date_issued);
    let added_new_old =
        |a: &DisplayCredential, b: &DisplayCredential| a.metadata.date_added.cmp(&b.metadata.date_added);

    credentials.sort_by(match preferences.sort_method {
        CredentialSortMethod::NameAZ => name_az,
        CredentialSortMethod::IssueDateNewOld => issuance_new_old,
        CredentialSortMethod::AddedDateNewOld => added_new_old,
    });

    if preferences.reverse {
        credentials.reverse();
    }

    Ok(AppState {
        credentials,
        current_user_prompt: None,
        ..state
    })
}

pub async fn sort_connections(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let mut connections: Vec<Connection> = state.connections.0.clone();
    let preferences: Preferences<ConnectionSortMethod> = state.profile_settings.sorting_preferences.connections.clone();

    let name_az = |a: &Connection, b: &Connection| a.name.cmp(&b.name);
    let first_interacted_new_old = |a: &Connection, b: &Connection| a.first_interacted.cmp(&b.first_interacted);
    let last_interacted_new_old = |a: &Connection, b: &Connection| a.last_interacted.cmp(&b.last_interacted);

    connections.sort_by(match preferences.sort_method {
        ConnectionSortMethod::NameAZ => name_az,
        ConnectionSortMethod::FirstInteractedNewOld => first_interacted_new_old,
        ConnectionSortMethod::LastInteractedNewOld => last_interacted_new_old,
    });

    if preferences.reverse {
        connections.reverse();
    }

    Ok(AppState {
        connections: Connections { 0: connections },
        current_user_prompt: None,
        ..state
    })
}
