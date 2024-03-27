use crate::{
    error::AppError,
    state::{
        actions::{listen, Action}, connections::Connection, credentials::DisplayCredential, profile_settings::{actions::update_sorting_preference::{ConnectionSorting, CredentialSorting, UpdateSortingPreference}, ConnectionSortMethod, CredentialSortMethod, ProfileSettings}, AppState
    },
};

use log::debug;

pub async fn update_sorting_preference(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(update_sorting) = listen::<UpdateSortingPreference>(action) {
        let mut sorting_preferences = state.profile_settings.sorting_preferences.clone().unwrap_or_default();

        if let Some(credential_sorting) = update_sorting.credential_sorting {
            debug!("Update credential sorting preference set to: `{:?}`", credential_sorting);
            if let Some(method) = credential_sorting.method {
                sorting_preferences.credentials.sort_method = method;
            }
            if let Some(reverse) = credential_sorting.reverse {
                sorting_preferences.credentials.reverse = reverse;
            }
        }
        else if let Some(connection_sorting) = update_sorting.connection_sorting {
            debug!("Update connection sorting preference set to: `{:?}`", connection_sorting);
            if let Some(method) = connection_sorting.method {
                sorting_preferences.connections.sort_method = method;
            }
            if let Some(reverse) = connection_sorting.reverse {
                sorting_preferences.connections.reverse = reverse;
            }
        }

        return Ok( AppState {
            profile_settings: ProfileSettings {
                sorting_preferences: Some(sorting_preferences),
                ..state.profile_settings
            },
            ..state
        });
    }
    Ok(state)
}

pub async fn sort_credentials(state: AppState, sorting: CredentialSorting) -> Result<AppState, AppError> {
    let mut credentials: Vec<DisplayCredential> = state.credentials.clone();

    let name_az = |a: &DisplayCredential, b: &DisplayCredential| a.display_name.cmp(&b.display_name);
    let issuance_new_old =
        |a: &DisplayCredential, b: &DisplayCredential| a.metadata.date_issued.cmp(&b.metadata.date_issued);
    let added_new_old =
        |a: &DisplayCredential, b: &DisplayCredential| a.metadata.date_added.cmp(&b.metadata.date_added);

    credentials.sort_by(match sorting.method.unwrap_or_default() {
        CredentialSortMethod::NameAZ => name_az,
        CredentialSortMethod::IssueDateNewOld => issuance_new_old,
        CredentialSortMethod::AddedDateNewOld => added_new_old,
    });


    if sorting.reverse.unwrap_or_default() {
        credentials.reverse();
    }

    Ok(AppState {
        credentials,
        ..state
    })
}

pub async fn sort_connections(state: AppState, sorting: ConnectionSorting) -> Result<AppState, AppError> {
    let mut connections: Vec<Connection> = state.connections.clone();

    let name_az = |a: &Connection, b: &Connection| a.client_name.cmp(&b.client_name);
    let first_interacted_new_old =
        |a: &Connection, b: &Connection| a.first_interacted.cmp(&b.first_interacted);
    let last_interacted_new_old = |a: &Connection, b: &Connection| a.last_interacted.cmp(&b.last_interacted);

    connections.sort_by(match sorting.method.unwrap_or_default() {
        ConnectionSortMethod::NameAZ => name_az,
        ConnectionSortMethod::FirstInteractedNewOld => first_interacted_new_old,
        ConnectionSortMethod::LastInteractedNewOld => last_interacted_new_old,
    });

    if sorting.reverse.unwrap_or_default() {
        connections.reverse();
    }

    Ok(AppState {
        connections,
        ..state
    })
}
