use std::{fs, sync::Arc};

use log::{debug, info};
use oid4vc::{oid4vc_manager::ProviderManager, oid4vci::Wallet};

use crate::{
    error::AppError::{
        self, OID4VCProviderManagerError, OID4VCWalletError, StrongholdLoadingError, StrongholdValuesError,
    },
    persistence::{load_state, ASSETS_DIR, STATE_FILE, STRONGHOLD},
    state::{
        actions::{listen, Action},
        backup::{actions::restore::RestoreBackup, archive, backup_store, list_backups, store::BackupStore},
        core_utils::{IdentityManager, SessionPassword},
        user_prompt::CurrentUserPrompt,
        AppState, SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS,
    },
    stronghold::StrongholdManager,
    subject::subject,
};

/// Replaces the current profile with the contents of a stored backup.
///
/// The restored state is deliberately reloaded through [`load_state`] rather than
/// deserialized here, so an archive written by an older build is carried forward
/// by the same migration chain that a state file on disk would go through.
#[tracing::instrument(skip_all, err)]
pub async fn restore_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(RestoreBackup { id, password }) = listen::<RestoreBackup>(action) {
        let bytes = backup_store().read(&id)?;

        // Fails with `ArchiveError::Authentication` for both a wrong password and
        // a modified file, so nothing is written unless the archive is intact.
        let payload = archive::open(&password, &bytes)?;

        info!(
            "restoring backup `{}` written by app version {}",
            id, payload.app_version
        );

        fs::write(STATE_FILE.lock().unwrap().as_path(), &payload.state)?;
        fs::write(STRONGHOLD.lock().unwrap().as_path(), &payload.stronghold)?;
        restore_assets(&payload.assets)?;

        let restored = load_state().await?;

        // The archived state records `is_unlocked`, but `core_utils` is skipped
        // during serialization, so the restored state would claim to be unlocked
        // with no Stronghold or identity manager behind it. Open the restored
        // vault with the password we already have, exactly as `unlock_storage`
        // does, so the profile comes back usable.
        let credentials = unlock_restored_profile(&restored, &password).await?;
        info!(
            "restored profile from backup `{id}` ({} credentials)",
            credentials.len()
        );

        return Ok(AppState {
            credentials,
            backups: list_backups()?,
            backup_preview: None,
            is_unlocked: true,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..restored
        });
    }
    Ok(state)
}

/// Opens the freshly restored Stronghold snapshot and rebuilds the managers the
/// rest of the app depends on, returning the credentials it holds.
///
/// Mirrors `unlock_storage`; the Stronghold vault, not the archived state, is the
/// source of truth for credentials.
async fn unlock_restored_profile(
    state: &AppState,
    password: &str,
) -> Result<Vec<crate::state::credentials::DisplayCredential>, AppError> {
    let stronghold_manager = Arc::new(StrongholdManager::load(password).map_err(StrongholdLoadingError)?);
    let subject = subject(stronghold_manager.clone(), password.to_string()).await;

    let provider_manager = ProviderManager::new(
        subject.clone(),
        Vec::from(SUPPORTED_DID_METHODS),
        Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
    )
    .map_err(OID4VCProviderManagerError)?;
    let wallet: Wallet = Wallet::new(
        subject.clone(),
        Vec::from(SUPPORTED_DID_METHODS),
        Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
    )
    .map_err(OID4VCWalletError)?;

    debug!("Loading credentials from the restored Stronghold vault");
    let credentials: Vec<_> = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap_or_default()
        .into_iter()
        .map(|record| record.display_credential)
        .collect();

    let mut managers = state.core_utils.managers.lock().await;
    managers.identity_manager.replace(IdentityManager {
        subject,
        provider_manager,
        wallet,
    });
    managers.stronghold_manager.replace(stronghold_manager);
    // Mirroring `unlock_storage` includes the session password: without it a
    // restored profile would silently stop taking automatic backups until the
    // next time the user unlocked by hand.
    managers.backup_password.replace(SessionPassword::new(password));

    Ok(credentials)
}

/// Writes the archived assets back, replacing whatever is there.
///
/// Names come out of the archive, so each one is checked to be a plain file name
/// before it is joined onto the assets directory.
fn restore_assets(assets: &[archive::Asset]) -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    fs::create_dir_all(&assets_dir)?;

    for asset in assets {
        if asset.name.is_empty() || asset.name.contains(['/', '\\']) || asset.name == "." || asset.name == ".." {
            return Err(AppError::Error(format!(
                "Backup contains an invalid asset name `{}`",
                asset.name
            )));
        }
        fs::write(assets_dir.join(&asset.name), &asset.data)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use serial_test::serial;

    use super::*;
    use crate::{
        persistence::BACKUPS_DIR,
        state::{
            backup::{actions::create::CreateBackup, list_backups, reducers::create::create_backup},
            APP_STATE_VERSION,
        },
        stronghold::StrongholdManager,
    };

    /// `create_backup` verifies the password against the Stronghold snapshot, so
    /// the tests need a real one rather than placeholder bytes.
    const PASSWORD: &str = "sup3rSecr3t";

    /// The storage paths are process-global, so these tests are serialised and
    /// point every path at a fresh temporary directory.
    fn point_storage_at(root: &Path) {
        *STATE_FILE.lock().unwrap() = root.join("state.json");
        *STRONGHOLD.lock().unwrap() = root.join("stronghold.bin");
        *ASSETS_DIR.lock().unwrap() = root.join("assets");
        *BACKUPS_DIR.lock().unwrap() = root.join("backups");
        fs::create_dir_all(root.join("assets")).unwrap();
        fs::create_dir_all(root.join("backups")).unwrap();
    }

    fn seed_profile(root: &Path) {
        StrongholdManager::create(PASSWORD).unwrap();
        fs::write(root.join("assets").join("issuer-logo.png"), b"\x89PNG fake").unwrap();
        // Scratch space that must not end up in the archive.
        fs::create_dir_all(root.join("assets").join("tmp")).unwrap();
        fs::write(root.join("assets").join("tmp").join("scratch.png"), b"scratch").unwrap();
    }

    fn wipe_profile(root: &Path) {
        fs::remove_file(root.join("state.json")).unwrap();
        fs::remove_file(root.join("stronghold.bin")).unwrap();
        fs::remove_dir_all(root.join("assets")).unwrap();
    }

    fn stored_backup_id(_root: &Path) -> String {
        let listed = list_backups().unwrap();
        assert_eq!(listed.len(), 1, "expected exactly one stored backup");
        listed[0].id.clone()
    }

    #[tokio::test]
    #[serial]
    async fn a_profile_round_trips_through_create_and_restore() {
        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        let mut state = AppState {
            version: APP_STATE_VERSION,
            ..Default::default()
        };
        state.dids.insert("did:key".to_string(), "did:key:z6Mk".to_string());

        create_backup(
            state.clone(),
            Arc::new(CreateBackup {
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();

        let id = stored_backup_id(tmp.path());
        let stronghold_before = fs::read(tmp.path().join("stronghold.bin")).unwrap();
        wipe_profile(tmp.path());

        let restored = restore_backup(
            AppState::default(),
            Arc::new(RestoreBackup {
                id: id.clone(),
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();

        assert_eq!(restored.dids, state.dids);
        assert_eq!(fs::read(tmp.path().join("stronghold.bin")).unwrap(), stronghold_before);

        // The restored profile must be genuinely usable, not just marked unlocked:
        // `core_utils` is skipped during serialization, so the managers have to be
        // rebuilt from the restored vault.
        assert!(restored.is_unlocked);
        let managers = restored.core_utils.managers.lock().await;
        assert!(managers.stronghold_manager.is_some(), "stronghold manager not restored");
        assert!(managers.identity_manager.is_some(), "identity manager not restored");
        assert_eq!(
            fs::read(tmp.path().join("assets").join("issuer-logo.png")).unwrap(),
            b"\x89PNG fake"
        );
        // `assets/tmp` is scratch space and is deliberately not carried over.
        assert!(!tmp.path().join("assets").join("tmp").join("scratch.png").exists());
    }

    #[tokio::test]
    #[serial]
    async fn a_backup_written_by_an_older_state_version_is_migrated_on_restore() {
        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        // Version 0 predates state versioning; restoring must replay the chain
        // rather than deserialising the archive's state as-is.
        let state = AppState {
            version: 0,
            ..Default::default()
        };

        create_backup(
            state,
            Arc::new(CreateBackup {
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();

        let id = stored_backup_id(tmp.path());
        wipe_profile(tmp.path());

        let restored = restore_backup(
            AppState::default(),
            Arc::new(RestoreBackup {
                id,
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();

        assert_eq!(restored.version, APP_STATE_VERSION);
    }

    #[tokio::test]
    #[serial]
    async fn the_wrong_password_leaves_the_profile_untouched() {
        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        let state = AppState {
            version: APP_STATE_VERSION,
            ..Default::default()
        };

        create_backup(
            state,
            Arc::new(CreateBackup {
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();

        let id = stored_backup_id(tmp.path());
        fs::write(tmp.path().join("stronghold.bin"), b"current stronghold").unwrap();

        let result = restore_backup(
            AppState::default(),
            Arc::new(RestoreBackup {
                id,
                password: "wrong".to_string(),
            }),
        )
        .await;

        assert!(result.is_err());
        // Nothing is written until the archive authenticates.
        assert_eq!(
            fs::read(tmp.path().join("stronghold.bin")).unwrap(),
            b"current stronghold"
        );
    }

    #[tokio::test]
    #[serial]
    async fn creating_and_deleting_keep_the_listing_current() {
        use crate::state::backup::{actions::delete::DeleteBackup, reducers::delete::delete_backup};

        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        let state = AppState {
            version: APP_STATE_VERSION,
            ..Default::default()
        };
        assert!(state.backups.is_empty());

        let after_create = create_backup(
            state,
            Arc::new(CreateBackup {
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();
        assert_eq!(after_create.backups.len(), 1);

        let id = after_create.backups[0].id.clone();
        let after_delete = delete_backup(after_create, Arc::new(DeleteBackup { id }))
            .await
            .unwrap();
        assert!(after_delete.backups.is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn the_listing_is_newest_first() {
        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());

        let store = crate::state::backup::backup_store();
        store.create("older.unime", b"a").unwrap();
        // Filesystem timestamps are coarse; make the ordering unambiguous.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        store.create("newer.unime", b"b").unwrap();

        let listed = list_backups().unwrap();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].name, "newer.unime");
        assert_eq!(listed[1].name, "older.unime");
    }

    #[tokio::test]
    #[serial]
    async fn a_preview_describes_the_archive_without_touching_disk() {
        use crate::state::{
            backup::{actions::preview::PreviewBackup, reducers::preview::preview_backup},
            connections::{Connection, Connections},
            credentials::DisplayCredential,
            profile_settings::{Profile, ProfileSettings},
        };

        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        let state = AppState {
            version: APP_STATE_VERSION,
            credentials: vec![DisplayCredential::default(), DisplayCredential::default()],
            connections: Connections(vec![Connection::default()]),
            profile_settings: ProfileSettings {
                profile: Some(Profile {
                    name: "Ferris".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let after_create = create_backup(
            state,
            Arc::new(CreateBackup {
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();
        let id = after_create.backups[0].id.clone();

        // Anything a restore would overwrite must be untouched by a preview.
        let stronghold_before = fs::read(tmp.path().join("stronghold.bin")).unwrap();

        let previewed = preview_backup(
            after_create,
            Arc::new(PreviewBackup {
                id: id.clone(),
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();

        let preview = previewed.backup_preview.expect("expected a preview");
        assert_eq!(preview.id, id);
        assert_eq!(preview.profile_name.as_deref(), Some("Ferris"));
        assert_eq!(preview.credentials, 2);
        assert_eq!(preview.connections, 1);
        // `issuer-logo.png` only; `assets/tmp` is excluded.
        assert_eq!(preview.assets, 1);
        assert_eq!(fs::read(tmp.path().join("stronghold.bin")).unwrap(), stronghold_before);
    }

    #[tokio::test]
    #[serial]
    async fn a_preview_with_the_wrong_password_is_refused() {
        use crate::state::backup::{actions::preview::PreviewBackup, reducers::preview::preview_backup};

        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        let after_create = create_backup(
            AppState {
                version: APP_STATE_VERSION,
                ..Default::default()
            },
            Arc::new(CreateBackup {
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();
        let id = after_create.backups[0].id.clone();

        let result = preview_backup(
            after_create,
            Arc::new(PreviewBackup {
                id,
                password: "wrong".to_string(),
            }),
        )
        .await;

        assert!(matches!(result, Err(AppError::BackupArchiveError(_))));
    }

    #[tokio::test]
    #[serial]
    async fn restoring_clears_the_preview() {
        use crate::state::backup::{actions::preview::PreviewBackup, reducers::preview::preview_backup};

        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        let after_create = create_backup(
            AppState {
                version: APP_STATE_VERSION,
                ..Default::default()
            },
            Arc::new(CreateBackup {
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();
        let id = after_create.backups[0].id.clone();

        let previewed = preview_backup(
            after_create,
            Arc::new(PreviewBackup {
                id: id.clone(),
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();
        assert!(previewed.backup_preview.is_some());

        let restored = restore_backup(
            previewed,
            Arc::new(RestoreBackup {
                id,
                password: PASSWORD.to_string(),
            }),
        )
        .await
        .unwrap();
        assert!(restored.backup_preview.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn creating_with_the_wrong_password_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());
        seed_profile(tmp.path());

        let result = create_backup(
            AppState {
                version: APP_STATE_VERSION,
                ..Default::default()
            },
            Arc::new(CreateBackup {
                password: "not the profile password".to_string(),
            }),
        )
        .await;

        // Sealing with a mistyped password would produce an archive that nobody,
        // including its owner, could ever open.
        assert!(matches!(result, Err(AppError::BackupPasswordMismatch)));
        assert!(list_backups().unwrap().is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn restoring_an_unknown_id_fails_cleanly() {
        let tmp = tempfile::tempdir().unwrap();
        point_storage_at(tmp.path());

        let result = restore_backup(
            AppState::default(),
            Arc::new(RestoreBackup {
                id: uuid::Uuid::new_v4().to_string(),
                password: "pw".to_string(),
            }),
        )
        .await;

        assert!(result.is_err());
    }
}
