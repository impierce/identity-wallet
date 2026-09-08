//! A read-only summary of what a backup holds.
//!
//! Recovery is destructive, so the flow decrypts an archive and shows what is in
//! it before anything is overwritten. Producing a preview writes nothing to disk.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{error::AppError, migrations::apply_state_migrations, state::AppState, state::APP_STATE_VERSION};

/// What the user is about to restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "bindings/backup/BackupPreview.ts")]
#[serde(rename_all = "camelCase")]
pub struct BackupPreview {
    /// The backup this preview describes, so a stale preview is never applied
    /// to a different archive.
    pub id: String,
    /// App version that wrote the archive.
    pub app_version: String,
    pub profile_name: Option<String>,
    pub credentials: u32,
    pub connections: u32,
    pub assets: u32,
}

/// Deserializes the archived state, replaying migrations so that the counts
/// describe what a restore would actually produce rather than what an older
/// build happened to write.
pub fn state_from_archive(state: &[u8]) -> Result<AppState, AppError> {
    let object: serde_json::Map<String, serde_json::Value> = serde_json::from_slice(state)?;

    let version = object
        .get("version")
        .and_then(|version| version.as_u64())
        .map(|version| version as u32)
        .unwrap_or(0);

    if version == APP_STATE_VERSION {
        Ok(serde_json::from_slice(state)?)
    } else {
        apply_state_migrations(object, version)
    }
}
