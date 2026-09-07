use std::fs;

use chrono::Utc;
use log::info;

use crate::{
    error::AppError,
    persistence::{save_state, ASSETS_DIR, BACKUPS_DIR, STATE_FILE, STRONGHOLD},
    state::{
        actions::{listen, Action},
        backup::{
            actions::create::CreateBackup,
            archive::{self, Asset, Payload},
            store::{BackupStore, LocalBackupStore},
        },
        AppState,
    },
};

/// Seals the current profile into an archive and hands it to the backup store.
#[tracing::instrument(skip_all, err)]
pub async fn create_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateBackup { password }) = listen::<CreateBackup>(action) {
        // Flush the running state first, otherwise the archive captures whatever
        // was last written rather than what the user is looking at.
        save_state(&state).await?;

        let payload = collect_payload()?;
        let bytes = archive::seal(&password, &payload)?;

        let store = LocalBackupStore::new(BACKUPS_DIR.lock().unwrap().clone());
        let file = store.create(&backup_name(), &bytes)?;

        info!("created backup `{}` ({} bytes, id `{}`)", file.name, file.size, file.id);

        return Ok(AppState {
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

/// Gathers everything a restore needs to rebuild the profile.
fn collect_payload() -> Result<Payload, AppError> {
    let state = fs::read(STATE_FILE.lock().unwrap().as_path())?;
    let stronghold = fs::read(STRONGHOLD.lock().unwrap().as_path())?;

    Ok(Payload {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        state,
        stronghold,
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
