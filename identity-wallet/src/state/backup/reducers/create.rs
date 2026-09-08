use std::fs;

use chrono::Utc;
use log::info;

use crate::{
    error::AppError,
    persistence::{save_state, ASSETS_DIR, STRONGHOLD},
    state::{
        actions::{listen, Action},
        backup::{
            actions::create::CreateBackup,
            archive::{self, Asset, Payload},
            backup_store, list_backups,
            store::BackupStore,
        },
        AppState,
    },
    stronghold::StrongholdManager,
};

/// Seals the current profile into an archive and hands it to the backup store.
#[tracing::instrument(skip_all, err)]
pub async fn create_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateBackup { password }) = listen::<CreateBackup>(action) {
        // Backups are sealed with the profile password, and restoring one needs
        // that same password to open the Stronghold snapshot inside it. Verify it
        // here rather than trusting the caller: a typo would otherwise produce an
        // archive nobody can ever open.
        if StrongholdManager::load(&password).is_err() {
            return Err(AppError::BackupPasswordMismatch);
        }

        // Keep the on-disk state current, but archive the in-memory state rather
        // than reading it back: `save_state` writes through a buffered tokio
        // `File` with no explicit flush, so a synchronous read straight after can
        // observe an empty file.
        save_state(&state).await?;

        let payload = collect_payload(&state)?;
        let bytes = archive::seal(&password, &payload)?;

        let file = backup_store().create(&backup_name(), &bytes)?;

        info!("created backup `{}` ({} bytes, id `{}`)", file.name, file.size, file.id);

        return Ok(AppState {
            backups: list_backups()?,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

/// Gathers everything a restore needs to rebuild the profile.
///
/// The state section is serialized from `state` directly, so the archive holds
/// exactly what the user is looking at and never depends on write timing.
fn collect_payload(state: &AppState) -> Result<Payload, AppError> {
    Ok(Payload {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        state: serde_json::to_vec(state)?,
        stronghold: fs::read(STRONGHOLD.lock().unwrap().as_path())?,
        assets: collect_assets()?,
    })
}

/// Reads the assets directory. Without these a restored profile comes back with
/// no issuer logos or credential images until every one is fetched again.
///
/// `assets/tmp` is skipped: it is scratch space cleared on every launch.
fn collect_assets() -> Result<Vec<Asset>, AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    if !assets_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut assets = Vec::new();
    for entry in fs::read_dir(&assets_dir)?.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        assets.push(Asset {
            name: name.to_string(),
            data: fs::read(&path)?,
        });
    }
    Ok(assets)
}

/// A human-readable display name. The store keys backups by id, so this only has
/// to be recognisable, not unique — colons are avoided because they are awkward
/// on some filesystems.
fn backup_name() -> String {
    format!("unime-{}.unime", Utc::now().format("%Y-%m-%dT%H-%M-%SZ"))
}
