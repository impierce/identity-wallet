//! The credentials an issuer connection says it issues, read from the `credential_configurations_supported` of its
//! OpenID4VCI Credential Issuer Metadata.
//!
//! The metadata is one document without paging or search, so it is fetched once per issuer, kept in memory for
//! [`CACHE_TTL`] and paged and searched here. Only the page the frontend asks for travels in the app state.

use crate::persistence::{hash, temp_asset_exists};
use crate::state::connections::Connection;
use crate::state::core_utils::helpers::download_logo;
use crate::state::credentials::DisplayCredential;
use crate::state::profile_settings::Locale;
use crate::well_known::get_credential_issuer_metadata;

use log::{info, warn};
use oid4vci::credential_issuer::{
    credential_configurations_supported::CredentialConfigurationsSupportedDisplay,
    credential_issuer_metadata::CredentialIssuerMetadata,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
    time::{Duration, Instant},
};
use ts_rs::TS;

pub const CACHE_TTL: Duration = Duration::from_secs(5 * 60);
pub const METADATA_TIMEOUT: Duration = Duration::from_secs(15);
pub const LOGOS_TIMEOUT: Duration = Duration::from_secs(10);
pub const MAX_PAGE_SIZE: u32 = 50;

/// One credential configuration of an issuer, as shown in the list.
#[derive(Serialize, Deserialize, TS, Clone, Debug, PartialEq)]
#[ts(export, export_to = "bindings/connections/IssuedCredential.ts")]
pub struct IssuedCredential {
    /// The `credential_configuration_id`.
    pub id: String,
    pub name: String,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    /// The wallet holds a credential of this configuration from this connection.
    #[serde(default)]
    pub owned: bool,
}

/// A page of the credentials a connection issues that match `search_term`.
#[derive(Serialize, Deserialize, TS, Clone, Debug, PartialEq, Default)]
#[ts(export, export_to = "bindings/connections/IssuedCredentialsPage.ts")]
pub struct IssuedCredentialsPage {
    pub connection_id: String,
    pub search_term: String,
    pub offset: u32,
    pub items: Vec<IssuedCredential>,
    /// How many credentials match `search_term` in total.
    pub total: u32,
    /// The issuer metadata could not be loaded.
    pub unavailable: bool,
}

type Cache = Mutex<HashMap<(String, String), (Instant, Arc<Vec<IssuedCredential>>)>>;

static CACHE: LazyLock<Cache> = LazyLock::new(Default::default);

/// The Credential Issuer Identifier of `connection`. Connections from before it was recorded fall back to their URL,
/// which is the issuer's origin, and is the identifier of issuers that are not served from a path.
pub fn credential_issuer_url(connection: &Connection) -> Option<url::Url> {
    let candidate = connection.credential_issuer.as_deref().unwrap_or(&connection.url);
    url::Url::parse(candidate)
        .or_else(|_| url::Url::parse(&format!("https://{candidate}")))
        .inspect_err(|e| warn!("Connection `{}` has no valid issuer URL: {e}", connection.id))
        .ok()
}

/// Loads the requested page of the credentials `issuer_url` issues, fetching its metadata when it is not cached, and
/// downloads the logos of that page.
pub async fn load_page(
    connection_id: String,
    issuer_url: Option<url::Url>,
    locale: Locale,
    search_term: String,
    offset: u32,
    limit: u32,
) -> IssuedCredentialsPage {
    let unavailable = IssuedCredentialsPage {
        connection_id: connection_id.clone(),
        search_term: search_term.clone(),
        offset,
        unavailable: true,
        ..Default::default()
    };

    let Some(issuer_url) = issuer_url else {
        return unavailable;
    };
    let Some(all) = issued_credentials(&issuer_url, &locale).await else {
        return unavailable;
    };

    let (items, total) = page(&all, &search_term, offset, limit.clamp(1, MAX_PAGE_SIZE));
    download_logos(&items).await;

    IssuedCredentialsPage {
        connection_id,
        search_term,
        offset,
        items,
        total,
        unavailable: false,
    }
}

async fn issued_credentials(issuer_url: &url::Url, locale: &Locale) -> Option<Arc<Vec<IssuedCredential>>> {
    let key = (issuer_url.to_string(), locale_tag(locale));
    if let Some((fetched_at, credentials)) = CACHE.lock().unwrap().get(&key) {
        if fetched_at.elapsed() < CACHE_TTL {
            return Some(credentials.clone());
        }
    }

    let metadata = match tokio::time::timeout(METADATA_TIMEOUT, get_credential_issuer_metadata(issuer_url)).await {
        Ok(Ok(metadata)) => metadata,
        Ok(Err(e)) => {
            warn!(
                "Failed to load the credentials {} issues: {e}",
                issuer_url.host_str().unwrap_or_default()
            );
            return None;
        }
        Err(_) => {
            warn!(
                "Loading the credentials {} issues timed out",
                issuer_url.host_str().unwrap_or_default()
            );
            return None;
        }
    };

    let credentials = Arc::new(from_metadata(&metadata, locale));
    info!(
        "{} issues {} credential(s)",
        issuer_url.host_str().unwrap_or_default(),
        credentials.len()
    );
    CACHE.lock().unwrap().insert(key, (Instant::now(), credentials.clone()));
    Some(credentials)
}

/// All credential configurations of `metadata`, sorted by name so that pages stay stable.
pub fn from_metadata(metadata: &CredentialIssuerMetadata, locale: &Locale) -> Vec<IssuedCredential> {
    let mut credentials: Vec<IssuedCredential> = metadata
        .credential_configurations_supported
        .iter()
        .map(|(id, configuration)| {
            let display = configuration
                .credential_metadata
                .as_ref()
                .and_then(|credential_metadata| credential_metadata.display.as_deref())
                .and_then(|display| localized(display, locale));
            IssuedCredential {
                id: id.clone(),
                name: display
                    .map(|display| display.name.clone())
                    .unwrap_or_else(|| id.clone()),
                description: display.and_then(|display| display.description.clone()),
                logo_uri: display
                    .and_then(|display| display.logo.as_ref())
                    .map(|logo| logo.uri.to_string()),
                owned: false,
            }
        })
        .collect();

    credentials.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.id.cmp(&right.id))
    });
    credentials
}

/// The display in the user's language, else the one without a locale, else the first.
fn localized<'a>(
    display: &'a [CredentialConfigurationsSupportedDisplay],
    locale: &Locale,
) -> Option<&'a CredentialConfigurationsSupportedDisplay> {
    let tag = locale_tag(locale).to_lowercase();
    let language = tag.split('-').next().unwrap_or_default().to_string();

    let locale_of =
        |display: &CredentialConfigurationsSupportedDisplay| display.locale.as_deref().map(str::to_lowercase);

    display
        .iter()
        .find(|display| locale_of(display).as_deref() == Some(tag.as_str()))
        .or_else(|| {
            display.iter().find(|display| {
                locale_of(display).is_some_and(|locale| locale.split('-').next() == Some(language.as_str()))
            })
        })
        .or_else(|| display.iter().find(|display| display.locale.is_none()))
        .or_else(|| display.first())
}

/// Marks the items of `page` that the wallet already holds a credential of. Credentials received before their
/// `credential_configuration_id` was recorded are matched by their display name, which was taken from the same
/// configuration.
pub fn mark_owned(page: &mut IssuedCredentialsPage, credentials: &[DisplayCredential]) {
    let from_connection: Vec<_> = credentials
        .iter()
        .filter(|credential| credential.connection_id.as_deref() == Some(page.connection_id.as_str()))
        .collect();
    for item in &mut page.items {
        item.owned =
            from_connection.iter().any(
                |credential| match credential.metadata.credential_configuration_id.as_deref() {
                    Some(id) => id == item.id,
                    None => credential.display_name == item.name,
                },
            );
    }
}

/// The BCP 47 tag of `locale`, such as `en-US`.
fn locale_tag(locale: &Locale) -> String {
    serde_json::to_value(locale)
        .ok()
        .and_then(|tag| tag.as_str().map(ToString::to_string))
        .unwrap_or_default()
}

/// The credentials matching `search_term` (name or description, ignoring case) from `offset`, and how many match.
pub fn page(all: &[IssuedCredential], search_term: &str, offset: u32, limit: u32) -> (Vec<IssuedCredential>, u32) {
    let term = search_term.trim().to_lowercase();
    let matches = |credential: &&IssuedCredential| {
        term.is_empty()
            || credential.name.to_lowercase().contains(&term)
            || credential
                .description
                .as_ref()
                .is_some_and(|description| description.to_lowercase().contains(&term))
    };

    let total = all.iter().filter(matches).count() as u32;
    let items = all
        .iter()
        .filter(matches)
        .skip(offset as usize)
        .take(limit as usize)
        .cloned()
        .collect();
    (items, total)
}

async fn download_logos(items: &[IssuedCredential]) {
    let logo_uris: std::collections::BTreeSet<&str> = items
        .iter()
        .filter_map(|item| item.logo_uri.as_deref())
        .filter(|logo_uri| !temp_asset_exists(&hash(logo_uri)))
        .collect();
    let downloads = logo_uris.into_iter().map(download_logo);
    if tokio::time::timeout(LOGOS_TIMEOUT, futures::future::join_all(downloads))
        .await
        .is_err()
    {
        warn!("Downloading the logos of issued credentials timed out");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn metadata() -> CredentialIssuerMetadata {
        serde_json::from_value(json!({
            "credential_issuer": "https://issuer.example/",
            "credential_endpoint": "https://issuer.example/openid4vci/credential",
            "credential_configurations_supported": {
                "b": {
                    "format": "vc+sd-jwt",
                    "credential_definition": { "type": ["VerifiableCredential"] },
                    "credential_metadata": {
                        "display": [
                            { "name": "Zwemdiploma", "locale": "nl-NL" },
                            { "name": "Swimming certificate", "locale": "en-US", "description": "Can swim",
                              "logo": { "uri": "https://issuer.example/swim.png" } }
                        ]
                    }
                },
                "a": {
                    "format": "vc+sd-jwt",
                    "credential_definition": { "type": ["VerifiableCredential", "OpenBadgeCredential"] },
                    "credential_metadata": { "display": [{ "name": "cooking 101" }] }
                },
                "no-display": {
                    "format": "vc+sd-jwt",
                    "credential_definition": { "type": ["VerifiableCredential"] }
                }
            }
        }))
        .unwrap()
    }

    #[test]
    fn sorts_by_name_and_falls_back_to_the_id() {
        let names: Vec<_> = from_metadata(&metadata(), &Locale::en_US)
            .into_iter()
            .map(|credential| credential.name)
            .collect();
        assert_eq!(names, vec!["cooking 101", "no-display", "Swimming certificate"]);
    }

    #[test]
    fn picks_the_display_of_the_user_language() {
        let credentials = from_metadata(&metadata(), &Locale::nl_NL);
        assert!(credentials.iter().any(|credential| credential.name == "Zwemdiploma"));

        let credentials = from_metadata(&metadata(), &Locale::en_GB);
        let swimming = credentials.iter().find(|credential| credential.id == "b").unwrap();
        assert_eq!(swimming.name, "Swimming certificate");
        assert_eq!(swimming.description.as_deref(), Some("Can swim"));
        assert_eq!(swimming.logo_uri.as_deref(), Some("https://issuer.example/swim.png"));
    }

    #[test]
    fn pages_and_searches() {
        let all: Vec<_> = (0..120)
            .map(|index| IssuedCredential {
                id: index.to_string(),
                name: format!("Badge {index:03}"),
                description: (index % 10 == 0).then(|| "Tenth".to_string()),
                logo_uri: None,
                owned: false,
            })
            .collect();

        let (items, total) = page(&all, "", 100, 50);
        assert_eq!(total, 120);
        assert_eq!(items.len(), 20);
        assert_eq!(items[0].name, "Badge 100");

        let (items, total) = page(&all, "  TENTH ", 0, 5);
        assert_eq!(total, 12);
        assert_eq!(items.len(), 5);

        let (items, total) = page(&all, "badge 11", 0, 50);
        assert_eq!(total, 10);
        assert_eq!(items[0].name, "Badge 110");

        let (items, total) = page(&all, "nothing", 0, 50);
        assert_eq!((items.len(), total), (0, 0));
    }

    #[test]
    fn uses_the_recorded_issuer_and_falls_back_to_the_url() {
        let mut connection = Connection {
            url: "issuer.example".to_string(),
            ..Default::default()
        };
        assert_eq!(
            credential_issuer_url(&connection).unwrap().as_str(),
            "https://issuer.example/"
        );

        connection.credential_issuer = Some("https://issuer.example/tenant".to_string());
        assert_eq!(
            credential_issuer_url(&connection).unwrap().as_str(),
            "https://issuer.example/tenant"
        );
    }

    #[test]
    fn marks_credentials_the_wallet_holds() {
        let item = |id: &str, name: &str| IssuedCredential {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            logo_uri: None,
            owned: false,
        };
        let credential = |connection_id: &str, configuration_id: Option<&str>, name: &str| DisplayCredential {
            connection_id: Some(connection_id.to_string()),
            display_name: name.to_string(),
            metadata: crate::state::credentials::CredentialMetadata {
                credential_configuration_id: configuration_id.map(ToString::to_string),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut page = IssuedCredentialsPage {
            connection_id: "issuer".to_string(),
            items: vec![
                item("a", "Badge A"),
                item("b", "Badge B"),
                item("c", "Badge C"),
                item("d", "Badge D"),
            ],
            ..Default::default()
        };

        mark_owned(
            &mut page,
            &[
                credential("issuer", Some("a"), "Renamed since"),
                credential("issuer", None, "Badge B"),
                credential("other", Some("c"), "Badge C"),
                credential("issuer", Some("x"), "Badge D"),
            ],
        );

        let owned: Vec<_> = page.items.iter().map(|item| item.owned).collect();
        assert_eq!(owned, vec![true, true, false, false]);
    }

    #[tokio::test]
    async fn reports_an_issuer_without_metadata_as_unavailable() {
        // Pooled mock servers share their URIs between tests, so each test uses a path of its own to stay out of the
        // other's cache entry.
        let server = wiremock::MockServer::start().await;
        let page = load_page(
            "connection".to_string(),
            Some(format!("{}/unavailable", server.uri()).parse().unwrap()),
            Locale::en_US,
            String::new(),
            0,
            20,
        )
        .await;
        assert!(page.unavailable);
        assert!(page.items.is_empty());
    }

    #[tokio::test]
    async fn loads_a_page_from_the_metadata() {
        let server = wiremock::MockServer::start().await;
        let issuer = format!("{}/available", server.uri());
        let mut metadata = serde_json::to_value(metadata()).unwrap();
        metadata["credential_issuer"] = json!(issuer);
        metadata["credential_endpoint"] = json!(format!("{}/credential", server.uri()));
        // Without a logo, so that no download is attempted.
        metadata["credential_configurations_supported"]["b"]["credential_metadata"]["display"][1]
            .as_object_mut()
            .unwrap()
            .remove("logo");
        wiremock::Mock::given(wiremock::matchers::path(
            "/available/.well-known/openid-credential-issuer",
        ))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(metadata))
        .expect(1)
        .mount(&server)
        .await;

        let issuer_url: url::Url = issuer.parse().unwrap();
        let first = load_page(
            "connection".to_string(),
            Some(issuer_url.clone()),
            Locale::en_US,
            String::new(),
            0,
            2,
        )
        .await;
        assert!(!first.unavailable);
        assert_eq!(first.total, 3);
        assert_eq!(first.items.len(), 2);

        // The second page comes from the cache: the mock expects one request only.
        let second = load_page(
            "connection".to_string(),
            Some(issuer_url),
            Locale::en_US,
            "swim".to_string(),
            0,
            2,
        )
        .await;
        assert_eq!(second.total, 1);
        assert_eq!(second.items[0].id, "b");
    }
}
