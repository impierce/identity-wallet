use std::time::Duration;

use crate::{
    error::AppError,
    persistence::{hash, keep_asset},
    state::{
        actions::{listen, Action},
        connections::EcosystemsCheck,
        credentials::actions::refresh_credential_ecosystems::{RefreshCredentialEcosystems, ResolvedEcosystems},
        qr_code::reducers::accept_connection::{discover_ecosystems, EcosystemsUnavailable},
        user_prompt::EcosystemProfile,
        AppState,
    },
};
use log::{info, warn};

#[cfg(test)]
use crate::state::actions::ActionTrait as _;

/// Records the ecosystems of a credential's issuer on the issuer's connection, which every credential from that issuer
/// shares. A failed attempt keeps the ecosystems found before and is marked on the connection's `ecosystems_check`, so
/// the credential still shows, with a warning. Credentials without an issuer connection, such as self-issued ones, are
/// left as they are.
#[tracing::instrument(skip_all, err)]
pub async fn refresh_credential_ecosystems(state: AppState, action: Action) -> Result<AppState, AppError> {
    let Some(refresh_credential_ecosystems) = listen::<RefreshCredentialEcosystems>(action) else {
        return Ok(state);
    };

    // Resolved in `prepare` when dispatched from the frontend, or here when reduced directly.
    let ResolvedEcosystems { connection_id, result } = match refresh_credential_ecosystems.resolved {
        Some(resolved) => resolved,
        None => {
            let Some((connection_id, url)) = issuer_connection(&state, &refresh_credential_ecosystems.credential_id)
            else {
                return Ok(state);
            };
            ResolvedEcosystems {
                connection_id,
                result: resolve_issuer_ecosystems(&url).await,
            }
        }
    };

    let mut connections = state.connections.clone();
    let Some(connection) = connections
        .0
        .iter_mut()
        .find(|connection| connection.id == connection_id)
    else {
        info!("Connection `{connection_id}` was deleted while its ecosystems were resolved");
        return Ok(state);
    };

    let (ecosystems, ecosystems_check) = EcosystemsCheck::outcome(result);
    connection.update_ecosystems(ecosystems, Some(ecosystems_check));

    Ok(AppState { connections, ..state })
}

/// The id and URL of the connection to the issuer of the credential with `credential_id`, if it has one.
pub(crate) fn issuer_connection(state: &AppState, credential_id: &str) -> Option<(String, url::Url)> {
    let Some(credential) = state
        .credentials
        .iter()
        .find(|credential| credential.id == credential_id)
    else {
        warn!("No credential found with id: `{credential_id}`");
        return None;
    };

    let connection_id = credential.connection_id.as_ref()?;
    let connection = state
        .connections
        .0
        .iter()
        .find(|connection| &connection.id == connection_id)?;

    url::Url::parse(&connection.url)
        .inspect_err(|e| {
            warn!(
                "Connection `{connection_id}` has no valid URL `{}`: {e}",
                connection.url
            )
        })
        .ok()
        .map(|url| (connection.id.clone(), url))
}

/// Discovery stops here, short of the 10 seconds after which `handle_action` gives up on an action and reports an error.
/// That limit also covers waiting for the state lock, e.g. behind the status refresh dispatched alongside.
const ECOSYSTEMS_TIMEOUT: Duration = Duration::from_secs(6);

/// Resolves the ecosystems of the issuer at `url`, and keeps the logos they show beyond this app session: credentials
/// show them in the credential list, before any of them is opened again.
pub(crate) async fn resolve_issuer_ecosystems(url: &url::Url) -> Result<Vec<EcosystemProfile>, EcosystemsUnavailable> {
    let result = tokio::time::timeout(ECOSYSTEMS_TIMEOUT, discover_ecosystems(url))
        .await
        .unwrap_or_else(|_| {
            Err(EcosystemsUnavailable(format!(
                "Resolving the ecosystems of {url} took longer than {ECOSYSTEMS_TIMEOUT:?}"
            )))
        });

    if let Ok(ecosystems) = &result {
        let logo_uris = ecosystems.iter().flat_map(|ecosystem| {
            ecosystem.logo_uri.iter().chain(
                ecosystem
                    .trust_chain
                    .iter()
                    .filter_map(|member| member.logo_uri.as_ref()),
            )
        });
        for logo_uri in logo_uris {
            keep_asset(&hash(logo_uri)).ok();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        connections::{Connection, Connections},
        credentials::DisplayCredential,
        user_prompt::Member,
    };
    use std::sync::Arc;
    use wiremock::{matchers::path, Mock, MockServer, ResponseTemplate};

    const CREDENTIAL_ID: &str = "credential";
    const CONNECTION_ID: &str = "issuer";

    fn ecosystem(name: &str) -> EcosystemProfile {
        EcosystemProfile {
            logo_uri: None,
            name: name.to_string(),
            description: None,
            ecosystem_leader: Member {
                logo_uri: None,
                name: name.to_string(),
                description: None,
                identifier: None,
            },
            member_count: 0,
            members: vec![],
            trust_chain: vec![],
        }
    }

    fn state(issuer_url: &str, connection_id: Option<&str>) -> AppState {
        AppState {
            credentials: vec![DisplayCredential {
                id: CREDENTIAL_ID.to_string(),
                connection_id: connection_id.map(str::to_string),
                ..Default::default()
            }],
            connections: Connections(vec![Connection {
                id: CONNECTION_ID.to_string(),
                url: issuer_url.to_string(),
                ecosystems: Some(vec![ecosystem("Ecosystem One")]),
                ..Default::default()
            }]),
            ..Default::default()
        }
    }

    fn refresh() -> Action {
        Arc::new(RefreshCredentialEcosystems {
            credential_id: CREDENTIAL_ID.to_string(),
            resolved: None,
        })
    }

    #[tokio::test]
    async fn an_issuer_outside_any_federation_loses_its_ecosystems() {
        // Serves no entity configuration.
        let issuer = MockServer::start().await;

        let state = refresh_credential_ecosystems(state(&issuer.uri(), Some(CONNECTION_ID)), refresh())
            .await
            .unwrap();

        let connection = &state.connections.0[0];
        assert_eq!(connection.ecosystems, None);
        assert!(connection.ecosystems_check.as_ref().unwrap().resolved);
    }

    #[tokio::test]
    async fn an_unreachable_issuer_keeps_its_ecosystems_and_is_marked_unresolved() {
        let issuer = MockServer::start().await;
        Mock::given(path("/.well-known/openid-federation"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&issuer)
            .await;

        let state = refresh_credential_ecosystems(state(&issuer.uri(), Some(CONNECTION_ID)), refresh())
            .await
            .unwrap();

        let connection = &state.connections.0[0];
        assert_eq!(connection.ecosystems, Some(vec![ecosystem("Ecosystem One")]));
        assert!(!connection.ecosystems_check.as_ref().unwrap().resolved);
    }

    #[tokio::test]
    async fn a_credential_without_an_issuer_connection_is_left_alone() {
        let state = state("https://issuer.example", None);
        let refreshed = refresh_credential_ecosystems(state.clone(), refresh()).await.unwrap();

        assert_eq!(refreshed.connections.0[0].ecosystems, state.connections.0[0].ecosystems);
        assert_eq!(refreshed.connections.0[0].ecosystems_check, None);
    }

    #[tokio::test]
    async fn prepare_resolves_the_ecosystems_of_the_issuer_connection() {
        let issuer = MockServer::start().await;
        let state = state(&issuer.uri(), Some(CONNECTION_ID));

        let prepared = RefreshCredentialEcosystems {
            credential_id: CREDENTIAL_ID.to_string(),
            resolved: None,
        }
        .prepare(&state)
        .unwrap()
        .await;

        let prepared = crate::state::actions::listen::<RefreshCredentialEcosystems>(prepared).unwrap();
        let resolved = prepared.resolved.unwrap();
        assert_eq!(resolved.connection_id, CONNECTION_ID);
        assert_eq!(resolved.result, Ok(vec![]));
    }

    #[test]
    fn prepare_is_skipped_for_a_credential_without_an_issuer_connection() {
        let action = RefreshCredentialEcosystems {
            credential_id: CREDENTIAL_ID.to_string(),
            resolved: None,
        };

        assert!(action.prepare(&state("https://issuer.example", None)).is_none());
    }
}
