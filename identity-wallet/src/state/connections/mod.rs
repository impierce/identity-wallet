pub mod actions;
pub mod reducers;

use super::{core_utils::DateUtils, FeatTrait};

use log::info;
use serde::{Deserialize, Serialize};
use std::ops::Not;
use ts_rs::TS;

#[derive(Default, Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export)]
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
    /// Modeled after the `std::collections::HashMap::insert` method.
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
    /// Modeled after the `std::collections::HashMap::get_mut` method.
    fn get_mut(&mut self, url: &str, name: &str) -> Option<&mut Connection> {
        self.0
            .iter_mut()
            .find(|connection| connection.url == url && connection.name == name)
    }

    /// Inserts a new connection into the list of connections if it does not already exist. If it does exist, updates
    /// the last interaction time and returns a reference to the connection.
    pub fn insert_or_update(&mut self, url: &str, name: &str) -> &Connection {
        if self.contains(url, name) {
            info!("Updating existing connection: {} {}", name, url);
            self.get_mut(url, name).map(|connection| {
                connection.update_last_interaction_time();
                &*connection
            })
        } else {
            info!("Inserting new connection: {} {}", name, url);
            self.insert(Connection::new(name.to_string(), url.to_string()))
        }
        .expect("Failed to insert or update connection")
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
#[ts(export)]
#[serde(default)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub url: String,
    pub verified: bool,
    pub first_interacted: String,
    pub last_interacted: String,
}

impl Connection {
    pub fn new(name: String, url: String) -> Self {
        // TODO(ngdil): Temporary solution to support NGDIL demo, replace with different unique identifier to distinguish connection
        let id = base64::encode_config([name.as_bytes(), url.as_bytes()].concat(), base64::URL_SAFE);
        let current_datetime = DateUtils::new_date_string();
        Self {
            id,
            name,
            url,
            verified: false,
            first_interacted: current_datetime.clone(),
            last_interacted: current_datetime.clone(),
        }
    }

    pub fn update_last_interaction_time(&mut self) {
        self.last_interacted = DateUtils::new_date_string();
    }
}

/// Implement PartialEq for Connection to allow for comparison of Connection instances for testing purposes.
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.url == other.url && self.verified == other.verified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_or_update() {
        let mut connections = Connections::new();
        let url = "https://example.com";
        let name = "Example";
        let connection = connections.insert_or_update(url, name);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(url, name));

        let connection = connections.insert_or_update(url, name);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        // The last interaction time should have been updated.
        assert_ne!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
    }

    #[test]
    fn test_insert_or_update_with_duplicate_names() {
        let mut connections = Connections::new();
        let url = "https://example.com";
        let name = "Example";
        let connection = connections.insert_or_update(url, name);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(url, name));

        // A different server with the same name is treated as a different connection.
        let url = "https://example2.com";
        // The same name is used.
        let name = name;
        let connection = connections.insert_or_update(url, name);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 2);
        assert!(connections.contains(url, name));
    }

    #[test]
    fn test_insert_or_update_with_duplicate_urls() {
        let mut connections = Connections::new();
        let url = "https://example.com";
        let name = "Example";
        let connection = connections.insert_or_update(url, name);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(url, name));

        // The same server is used with a different name.
        let url = url;
        let name = "Example2";
        let connection = connections.insert_or_update(url, name);
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 2);
        assert!(connections.contains(url, name));
    }
}
