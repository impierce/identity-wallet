//! Where backup archives are kept.
//!
//! The wallet talks to backups through [`BackupStore`], which deliberately mirrors
//! the `tauri-plugin-cloud-storage` contract: opaque identifiers rather than paths,
//! display names that are neither unique nor keys, and unordered listings. Code
//! written against this trait keeps working when the cloud provider is plugged in
//! behind it.
//!
//! [`LocalBackupStore`] is the filesystem implementation. It is what desktop
//! development uses — UniMe ships on mobile, so nothing here is a shipping
//! backend, but it behaves like one closely enough to exercise the flows.
//!
//! Each backup occupies its own directory named by a generated UUID, holding a
//! single file under its display name:
//!
//! ```text
//! <root>/<uuid>/<display name>
//! ```
//!
//! That layout is what buys the cloud-like semantics: the UUID is the opaque id,
//! and two backups can share a display name without colliding.

use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::AppError;

/// Metadata for one stored backup. Mirrors the plugin's `CloudFile`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupFile {
    /// Opaque identifier. Not a path, and not derived from the name.
    pub id: String,
    /// Display name. Duplicates are allowed and it is never used to look a file up.
    pub name: String,
    pub size: u64,
    /// RFC 3339, UTC.
    pub modified_at: String,
}

/// A place backups can be written to and read back from.
pub trait BackupStore {
    /// Unordered. Callers that care about order sort the result themselves.
    fn list(&self) -> Result<Vec<BackupFile>, AppError>;
    /// Always produces a new backup, even if `name` is already taken.
    fn create(&self, name: &str, data: &[u8]) -> Result<BackupFile, AppError>;
    fn read(&self, id: &str) -> Result<Vec<u8>, AppError>;
    fn delete(&self, id: &str) -> Result<(), AppError>;
}

/// A [`BackupStore`] backed by a directory on this device.
pub struct LocalBackupStore {
    root: PathBuf,
}

impl LocalBackupStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Resolves an id to its directory, rejecting anything that is not a UUID we
    /// generated. Ids arrive from the frontend, so `../..` must never resolve.
    fn directory_for(&self, id: &str) -> Result<PathBuf, AppError> {
        let uuid = Uuid::parse_str(id).map_err(|_| AppError::Error(format!("No backup found with id `{id}`")))?;
        Ok(self.root.join(uuid.to_string()))
    }

    /// The single file inside a backup directory, whatever it is named.
    fn file_in(directory: &Path) -> Result<PathBuf, AppError> {
        fs::read_dir(directory)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| path.is_file())
            .ok_or_else(|| AppError::Error(format!("Backup directory `{}` is empty", directory.display())))
    }
}

impl BackupStore for LocalBackupStore {
    fn list(&self) -> Result<Vec<BackupFile>, AppError> {
        if !self.root.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();
        for entry in fs::read_dir(&self.root)?.filter_map(Result::ok) {
            let directory = entry.path();
            if !directory.is_dir() {
                continue;
            }
            // Anything not named like one of our ids is not ours; skip it rather
            // than failing the whole listing.
            let Some(id) = directory
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| Uuid::parse_str(name).is_ok())
            else {
                continue;
            };
            let Ok(file) = Self::file_in(&directory) else {
                continue;
            };
            let metadata = file.metadata()?;
            let modified: DateTime<Utc> = metadata.modified()?.into();

            backups.push(BackupFile {
                id: id.to_string(),
                name: file
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_string(),
                size: metadata.len(),
                modified_at: modified.to_rfc3339(),
            });
        }
        Ok(backups)
    }

    fn create(&self, name: &str, data: &[u8]) -> Result<BackupFile, AppError> {
        if name.is_empty() || name.contains(['/', '\\']) || name == "." || name == ".." {
            return Err(AppError::Error(format!("Invalid backup name `{name}`")));
        }

        let id = Uuid::new_v4().to_string();
        let directory = self.root.join(&id);
        fs::create_dir_all(&directory)?;

        let file = directory.join(name);
        fs::write(&file, data)?;

        let metadata = file.metadata()?;
        let modified: DateTime<Utc> = metadata.modified()?.into();

        Ok(BackupFile {
            id,
            name: name.to_string(),
            size: metadata.len(),
            modified_at: modified.to_rfc3339(),
        })
    }

    fn read(&self, id: &str) -> Result<Vec<u8>, AppError> {
        let directory = self.directory_for(id)?;
        if !directory.is_dir() {
            return Err(AppError::Error(format!("No backup found with id `{id}`")));
        }
        Ok(fs::read(Self::file_in(&directory)?)?)
    }

    fn delete(&self, id: &str) -> Result<(), AppError> {
        let directory = self.directory_for(id)?;
        if !directory.is_dir() {
            return Err(AppError::Error(format!("No backup found with id `{id}`")));
        }
        fs::remove_dir_all(directory)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (tempfile::TempDir, LocalBackupStore) {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalBackupStore::new(dir.path());
        (dir, store)
    }

    #[test]
    fn a_created_backup_can_be_listed_and_read_back() {
        let (_dir, store) = store();
        let created = store.create("backup.unime", b"payload").unwrap();

        let listed = store.list().unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0], created);
        assert_eq!(store.read(&created.id).unwrap(), b"payload");
    }

    #[test]
    fn listing_an_absent_directory_is_empty_rather_than_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalBackupStore::new(dir.path().join("not-created-yet"));
        assert!(store.list().unwrap().is_empty());
    }

    #[test]
    fn duplicate_names_produce_distinct_backups() {
        let (_dir, store) = store();
        let first = store.create("backup.unime", b"one").unwrap();
        let second = store.create("backup.unime", b"two").unwrap();

        assert_ne!(first.id, second.id);
        assert_eq!(first.name, second.name);
        assert_eq!(store.read(&first.id).unwrap(), b"one");
        assert_eq!(store.read(&second.id).unwrap(), b"two");
        assert_eq!(store.list().unwrap().len(), 2);
    }

    #[test]
    fn deleting_removes_only_the_named_backup() {
        let (_dir, store) = store();
        let keep = store.create("keep.unime", b"keep").unwrap();
        let drop = store.create("drop.unime", b"drop").unwrap();

        store.delete(&drop.id).unwrap();

        let listed = store.list().unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, keep.id);
        assert!(store.read(&drop.id).is_err());
    }

    #[test]
    fn an_unknown_id_is_an_error_rather_than_a_panic() {
        let (_dir, store) = store();
        let absent = Uuid::new_v4().to_string();
        assert!(store.read(&absent).is_err());
        assert!(store.delete(&absent).is_err());
    }

    #[test]
    fn ids_that_are_not_uuids_are_refused() {
        let (_dir, store) = store();
        // A traversal attempt must not resolve to anything outside the root.
        for id in ["../../etc/passwd", "..", "", "not-a-uuid"] {
            assert!(store.read(id).is_err(), "`{id}` should not resolve");
            assert!(store.delete(id).is_err(), "`{id}` should not resolve");
        }
    }

    #[test]
    fn names_that_are_paths_are_refused() {
        let (_dir, store) = store();
        for name in ["../escape", "nested/name", "", ".."] {
            assert!(store.create(name, b"x").is_err(), "`{name}` should be refused");
        }
    }

    #[test]
    fn foreign_directories_are_ignored_by_listings() {
        let (dir, store) = store();
        store.create("mine.unime", b"mine").unwrap();
        fs::create_dir_all(dir.path().join("something-else")).unwrap();
        fs::write(dir.path().join("loose-file"), b"x").unwrap();

        let listed = store.list().unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].name, "mine.unime");
    }

    #[test]
    fn empty_backups_round_trip() {
        let (_dir, store) = store();
        let created = store.create("empty.unime", b"").unwrap();
        assert_eq!(created.size, 0);
        assert_eq!(store.read(&created.id).unwrap(), Vec::<u8>::new());
    }
}
