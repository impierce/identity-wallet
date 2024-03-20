use crate::{
    error::AppError,
    state::{
        actions::{listen, Action}, profile_settings::{actions::update_sorting_preference::UpdateSortingPreference, ProfileSettings}, AppState
    },
};

use log::debug;

pub async fn update_sorting_preference(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(update_sorting) = listen::<UpdateSortingPreference>(action) {
        let mut sorting_preferences = state.profile_settings.sorting_preferences.clone().unwrap_or_default();

        if let Some(credential_sorting) = update_sorting.credential_sorting {
            debug!("Update credential sorting preference set to: `{:?}`", credential_sorting);
            sorting_preferences.credential_sorting = Some(credential_sorting);
        }
        else if let Some(connection_sorting) = update_sorting.connection_sorting {
            debug!("Update connection sorting preference set to: `{:?}`", connection_sorting);
            sorting_preferences.connection_sorting = Some(connection_sorting);
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
