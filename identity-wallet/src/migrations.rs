use crate::state::APP_STATE_VERSION;
use crate::{error::AppError, state::AppState};
use log::debug;
use serde_json::{Map, Value};

/// This function is used to migrate the app state from one version to the next.
pub fn apply_state_migrations(
    mut app_state_object: serde_json::Map<String, serde_json::Value>,
    mut current_version: u32,
) -> Result<AppState, AppError> {
    while current_version < APP_STATE_VERSION {
        match current_version {
            0 => migrate_v0_to_v1(&mut app_state_object)?,
            _ => {
                return Err(AppError::AppStateMigrationError(
                    current_version,
                    APP_STATE_VERSION,
                    "Unsupported AppState version".to_string(),
                ))
            }
        }

        let previous_version = current_version;

        current_version = app_state_object
            .get("version")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32)
            .ok_or_else(|| AppError::Error("Failed to get version while migrating AppState".to_string()))?;

        debug!("Successfully migrated AppState version from {previous_version} to {current_version}");
    }

    let app_state_value = serde_json::Value::Object(app_state_object);
    // If somehow the AppState updates weren't successful after all, the function will throw an error trying to deserialize here
    let app_state: AppState = serde_json::from_value(app_state_value)?;

    Ok(app_state)
}

/// A new field `version` is added which marks the beginning of versioning the AppState.
fn migrate_v0_to_v1(app_state_object: &mut Map<String, Value>) -> anyhow::Result<(), AppError> {
    app_state_object.insert("version".to_string(), Value::from(1));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{migrations::apply_state_migrations, state::AppState};
    use serde_json::{Map, Value};

    #[tokio::test]
    async fn test_migrate_v0_to_v1() {
        let rdr =
            std::fs::File::open("../unime/src-tauri/tests/fixtures/states/no_profile_redirect_welcome.json").unwrap();

        let app_state_object: Map<String, Value> = serde_json::from_reader(&rdr).unwrap();
        let app_state = apply_state_migrations(app_state_object, 0).unwrap();
        let app_state_value = serde_json::to_value(&app_state).unwrap();

        let expected_app_state: AppState = serde_json::from_str(
            r#"
        {
            "version": 1,
            "current_user_prompt": {
                "type": "redirect",
                "target": "welcome"
            }
        }
        "#,
        )
        .unwrap();
        let expected_app_state_value = serde_json::to_value(&expected_app_state).unwrap();

        assert_eq!(app_state_value, expected_app_state_value);
    }
}
