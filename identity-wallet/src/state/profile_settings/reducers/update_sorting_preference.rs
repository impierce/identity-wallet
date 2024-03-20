use crate::{
    error::AppError,
    state::{
        actions::{listen, Action}, profile_settings::actions::update_sorting_preference::UpdateSortingPreference, AppState
    },
};

use log::debug;

pub async fn update_sorting_preference(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(update_sorting) = listen::<UpdateSortingPreference>(action) {
        if let Some(credential_sorting) = update_sorting.credential_sorting {
            debug!("Update credential sorting preference set to: `{:?}`", credential_sorting);
        }
        if let Some(connection_sorting) = update_sorting.connection_sorting {
            debug!("Update connection sorting preference set to: `{:?}`", connection_sorting);
        }
        return Ok(AppState {
            ..state
        });
    }
    Ok(state)
}
