use chrono::{DateTime, TimeDelta, Utc};
use log::{debug, info, warn};

use crate::{
    error::AppError,
    persistence::save_state,
    state::{
        actions::{listen, Action},
        backup::{
            actions::create_automatic::CreateAutomaticBackup, list_backups, reducers::create::seal_and_store, retention,
        },
        AppState,
    },
};

/// How stale the newest backup has to be before unlocking takes another one.
///
/// The point of a throttle rather than change detection is predictability: the
/// state file is rewritten on almost every action, and a Stronghold snapshot is
/// re-encrypted with fresh nonces whenever it is written, so "did anything
/// change?" cannot be answered by comparing bytes without a lot of guesswork.
/// A daily ceiling is something we can explain to the user and reason about.
const BACKUP_INTERVAL: TimeDelta = TimeDelta::hours(24);

/// Takes a backup when the profile is unlocked, if one is due.
///
/// This is the whole of "automatic": the password is only in memory because the
/// user just typed it to unlock Stronghold, so a backup can be sealed then and
/// at no other time. Nothing here runs in the background or on a schedule.
///
/// Every reason to skip is a normal outcome, not an error — the frontend fires
/// this after every unlock without knowing whether one is wanted.
#[tracing::instrument(skip_all, err)]
pub async fn create_automatic_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    if listen::<CreateAutomaticBackup>(action).is_none() {
        return Ok(state);
    }

    if !state.profile_settings.backup_enabled {
        return Ok(state);
    }

    if !is_due(&state) {
        debug!("automatic backup skipped: the newest backup is still recent");
        return Ok(state);
    }

    // Held only while the profile is unlocked. Absent for profiles that were not
    // opened through `unlock_storage`, such as the dev-mode profiles, which is a
    // reason to skip rather than to fail.
    let password = {
        let managers = state.core_utils.managers.lock().await;
        managers
            .backup_password
            .as_ref()
            .map(|password| password.expose().to_string())
    };
    let Some(password) = password else {
        debug!("automatic backup skipped: no password held for this session");
        return Ok(state);
    };

    // Same reasoning as the manual backup: archive the in-memory state, but flush
    // it first so the on-disk copy does not lag behind what was just sealed.
    save_state(&state).await?;

    // A failed automatic backup must not block the user getting into the app, so
    // report it and carry on rather than propagating the error into unlock.
    match seal_and_store(&state, &password) {
        Ok(file) => {
            info!(
                "created automatic backup `{}` ({} bytes, id `{}`)",
                file.name, file.size, file.id
            );
            retention::prune()?;
            Ok(AppState {
                backups: list_backups()?,
                ..state
            })
        }
        Err(error) => {
            warn!("automatic backup failed: {error}");
            Ok(state)
        }
    }
}

/// True when there is no backup yet, or the newest one is older than
/// [`BACKUP_INTERVAL`].
///
/// A timestamp that cannot be parsed is treated as due: the alternative is
/// silently never backing up again because of one unreadable entry.
fn is_due(state: &AppState) -> bool {
    let newest = state
        .backups
        .iter()
        .filter_map(|backup| DateTime::parse_from_rfc3339(&backup.modified_at).ok())
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .max();

    match newest {
        Some(timestamp) => Utc::now().signed_duration_since(timestamp) >= BACKUP_INTERVAL,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::backup::store::BackupFile;

    /// `is_due` only reads `modified_at`, so the rest is filler.
    fn backup_taken_at(timestamp: DateTime<Utc>) -> BackupFile {
        BackupFile {
            id: "b0a7f6a4-0000-4000-8000-000000000000".to_string(),
            name: "unime.unime".to_string(),
            size: 1,
            modified_at: timestamp.to_rfc3339(),
        }
    }

    fn state_with(backups: Vec<BackupFile>) -> AppState {
        AppState {
            backups,
            ..Default::default()
        }
    }

    #[test]
    fn a_profile_with_no_backup_yet_is_due() {
        assert!(is_due(&state_with(Vec::new())));
    }

    #[test]
    fn a_backup_from_just_now_is_not_due_again() {
        let state = state_with(vec![backup_taken_at(Utc::now())]);
        assert!(!is_due(&state));
    }

    #[test]
    fn a_backup_from_before_the_interval_is_due() {
        let state = state_with(vec![backup_taken_at(
            Utc::now() - BACKUP_INTERVAL - TimeDelta::minutes(1),
        )]);
        assert!(is_due(&state));
    }

    #[test]
    fn the_newest_backup_decides_even_when_the_listing_is_unordered() {
        // The store's contract is that listings are unordered, so a stale entry
        // must not make a freshly backed-up profile look due.
        let state = state_with(vec![
            backup_taken_at(Utc::now() - TimeDelta::days(30)),
            backup_taken_at(Utc::now()),
            backup_taken_at(Utc::now() - TimeDelta::days(7)),
        ]);
        assert!(!is_due(&state));
    }

    #[test]
    fn an_unreadable_timestamp_does_not_stop_backups_forever() {
        let mut backup = backup_taken_at(Utc::now());
        backup.modified_at = "not a timestamp".to_string();
        assert!(is_due(&state_with(vec![backup])));
    }

    #[test]
    fn an_unreadable_timestamp_is_ignored_when_a_readable_one_is_recent() {
        let mut unreadable = backup_taken_at(Utc::now());
        unreadable.modified_at = "not a timestamp".to_string();
        let state = state_with(vec![unreadable, backup_taken_at(Utc::now())]);
        assert!(!is_due(&state));
    }
}
