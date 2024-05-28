pub mod actions;
pub mod reducers;

use super::{core_utils::DateUtils, FeatTrait};

use identity_iota::did::CoreDID;
use log::info;
use serde::{Deserialize, Serialize};
use std::ops::Not;
use ts_rs::TS;

#[derive(Default, Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/connections/Connections.ts")]
pub struct Connections(pub Vec<Connection>);

impl Connections {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn contains(&self, url: &str, name: &str) -> bool {
        self.0
            .iter()
            .any(|connection| connection.url == url && connection.name == name)
    }

    /// Inserts a new connection into the list of connections.
    /// Modelled after the `std::collections::HashMap::insert` method.
    fn insert(&mut self, connection: Connection) -> Option<&Connection> {
        self.contains(&connection.url, &connection.name)
            .not()
            .then(|| {
                self.0.push(connection);
                self.0.last()
            })
            .flatten()
    }

    /// Returns a mutable reference to the connection with the given `url` and `name`.
    /// Modelled after the `std::collections::HashMap::get_mut` method.
    fn get_mut(&mut self, url: &str, name: &str) -> Option<&mut Connection> {
        self.0
            .iter_mut()
            .find(|connection| connection.url == url && connection.name == name)
    }

    /// Inserts a new connection into the list of connections if it does not already exist. If it does exist, updates
    /// the last interaction time and returns a reference to the connection.
    pub fn update_or_insert(&mut self, url: &str, name: &str, did: Option<CoreDID>) -> &Connection {
        if self.contains(url, name) {
            info!("Updating existing connection: {} {}", name, url);
            self.get_mut(url, name).map(|connection| {
                if let Some(core_did) = did {
                    connection.did = Some(core_did.to_string());
                }
                connection.update_last_interaction_time();
                &*connection
            })
        } else {
            info!("Inserting new connection: {} {}", name, url);
            self.insert(Connection::new(
                name.to_string(),
                url.to_string(),
                did.map(|d| d.to_string()),
            ))
        }
        .expect("Failed to update or insert connection")
    }
}

#[typetag::serde(name = "connections")]
impl FeatTrait for Connections {}

/// Connection represents a connection to either a `Client` or an `Issuer`. In the OpenID 4 Verifiable Credentials
/// (OID4VC) context, a `Client` is often referred to as a `Relying Party` and an `Issuer` is often referred to as a
/// `Credential Issuer`.
/// More information can be found here:
/// - [Relying Party](https://github.com/impierce/openid4vc/tree/dev/siopv2)
/// - [Credential Issuer](https://github.com/impierce/openid4vc/tree/dev/oid4vci)
#[derive(Clone, Serialize, Debug, Deserialize, TS, Default)]
#[ts(export, export_to = "bindings/connections/Connection.ts")]
#[serde(default)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub url: String,
    #[ts(optional)]
    pub did: Option<String>,
    pub verified: bool,
    pub first_interacted: String,
    pub last_interacted: String,
}

impl Connection {
    pub fn new(name: String, url: String, did: Option<String>) -> Self {
        // TODO(ngdil): Temporary solution to support NGDIL demo, replace with different unique identifier to distinguish connection
        let id = sha256::digest([name.as_bytes(), url.as_bytes()].concat()).to_string();
        let current_datetime = DateUtils::new_date_string();
        Self {
            id,
            name,
            url,
            did,
            verified: false,
            first_interacted: current_datetime.clone(),
            last_interacted: current_datetime,
        }
    }

    pub fn update_last_interaction_time(&mut self) {
        self.last_interacted = DateUtils::new_date_string();
    }
}

/// Implement PartialEq for Connection to allow for comparison of Connection instances for testing purposes.
/// TODO(test): This implementation should be behind a "test" feature flag.
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.url == other.url && self.verified == other.verified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_or_insert() {
        let mut connections = Connections::new();
        let url = "https://example.com";
        let name = "Example";
        let connection = connections.update_or_insert(url, name, None);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(url, name));

        let connection = connections.update_or_insert(url, name, None);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        // The last interaction time should have been updated.
        assert_ne!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
    }

    #[test]
    fn test_update_or_insert_with_duplicate_names() {
        let mut connections = Connections::new();
        let url = "https://example.com";
        let name = "Example";
        let connection = connections.update_or_insert(url, name, None);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(url, name));

        // A different server with the same name is treated as a different connection.
        let url = "https://example2.com";
        let connection = connections.update_or_insert(url, name, None);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 2);
        assert!(connections.contains(url, name));
    }

    #[test]
    fn test_update_or_insert_with_duplicate_urls() {
        let mut connections = Connections::new();
        let url = "https://example.com";
        let name = "Example";
        let connection = connections.update_or_insert(url, name, None);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(url, name));

        // The same server is used with a different name.
        let name = "Example2";
        let connection = connections.update_or_insert(url, name, None);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 2);
        assert!(connections.contains(url, name));
    }
}
