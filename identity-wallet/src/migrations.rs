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
            1 => migrate_v1_to_v2(&mut app_state_object)?,
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

/// Identities (ADR 0004): a connection's `did` becomes its `identity`, as does the `client_id` of the client metadata of
/// a pending `accept_connection` prompt. A DID is a valid identity, so the values stay as they are.
fn migrate_v1_to_v2(app_state_object: &mut Map<String, Value>) -> anyhow::Result<(), AppError> {
    if let Some(connections) = app_state_object.get_mut("connections").and_then(Value::as_array_mut) {
        for connection in connections.iter_mut().filter_map(Value::as_object_mut) {
            rename(connection, "did", "identity");
        }
    }

    if let Some(client_metadata) = app_state_object
        .get_mut("current_user_prompt")
        .and_then(|prompt| prompt.get_mut("client_metadata"))
        .and_then(Value::as_object_mut)
    {
        rename(client_metadata, "client_id", "identity");
    }

    app_state_object.insert("version".to_string(), Value::from(2));
    Ok(())
}

/// Renames the field `from` of `object` to `to`, if it has one.
fn rename(object: &mut Map<String, Value>, from: &str, to: &str) {
    if let Some(value) = object.remove(from) {
        object.insert(to.to_string(), value);
    }
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
            "version": 2,
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

    #[test]
    fn test_migrate_v1_to_v2() {
        let app_state_object: Map<String, Value> = serde_json::from_value(serde_json::json!({
            "version": 1,
            "connections": [{
                "id": "connection",
                "name": "Issuer",
                "url": "https://issuer.example",
                "did": "did:web:issuer.example",
                "verified": false,
                "first_interacted": "2026-01-01T00:00:00Z",
                "last_interacted": "2026-01-01T00:00:00Z"
            }],
            "current_user_prompt": {
                "type": "accept-connection",
                "client_metadata": {
                    "client_name": "Verifier",
                    "logo_uri": null,
                    "connection_url": "https://verifier.example",
                    "redirect_uri": null,
                    "client_id": "did:web:verifier.example"
                },
                "connection_data": null,
                "domain_validation": {
                    "status": "Unknown",
                    "url": "https://verifier.example/",
                    "name": null,
                    "issuance_date": null,
                    "message": null
                },
                "linked_verifiable_presentations": null,
                "ecosystems": null
            }
        }))
        .unwrap();

        let app_state = apply_state_migrations(app_state_object, 1).unwrap();
        let app_state_value = serde_json::to_value(&app_state).unwrap();

        assert_eq!(app_state_value["version"], 2);
        assert_eq!(app_state_value["connections"][0]["identity"], "did:web:issuer.example");
        assert!(app_state_value["connections"][0].get("did").is_none());
        assert_eq!(
            app_state_value["current_user_prompt"]["client_metadata"]["identity"],
            "did:web:verifier.example"
        );
        assert!(app_state_value["current_user_prompt"]["client_metadata"]
            .get("client_id")
            .is_none());
    }
}
