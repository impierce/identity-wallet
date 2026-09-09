use log::debug;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        backup::actions::enable::EnableBackup,
        profile_settings::ProfileSettings,
        AppState,
    },
};

/// Records whether unlocking should take a backup.
///
/// Nothing else: the switch does not create a backup when it is turned on, and
/// does not delete anything when it is turned off. Creating one on demand is what
/// the "Back up now" button is for, and the two are deliberately independent.
#[tracing::instrument(skip_all, err)]
pub async fn enable_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    let Some(enable) = listen::<EnableBackup>(action).map(|payload| payload.enable) else {
        return Ok(state);
    };

    debug!("automatic backups enabled: `{enable}`");

    Ok(AppState {
        profile_settings: ProfileSettings {
            backup_enabled: enable,
            ..state.profile_settings
        },
        current_user_prompt: None,
        ..state
    })
}
