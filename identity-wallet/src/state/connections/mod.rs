pub mod actions;
pub mod reducers;

use super::{core_utils::DateUtils, FeatTrait};

use identity_iota::did::{CoreDID, DID};
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

    pub fn contains(&self, did: &str) -> bool {
        self.0.iter().any(|connection| connection.did == did)
    }

    /// Inserts a new connection into the list of connections.
    /// Modelled after the `std::collections::HashMap::insert` method.
    fn insert(&mut self, connection: Connection) -> Option<&Connection> {
        self.contains(&connection.did)
            .not()
            .then(|| {
                self.0.push(connection);
                self.0.last()
            })
            .flatten()
    }

    /// Returns a mutable reference to the connection with the given `url` and `name`.
    /// Modelled after the `std::collections::HashMap::get_mut` method.
    fn get_mut(&mut self, did: &str) -> Option<&mut Connection> {
        self.0.iter_mut().find(|connection| connection.did == did)
    }

    /// Inserts a new connection into the list of connections if it does not already exist. If it does exist, updates
    /// the last interaction time and returns a reference to the connection.
    pub fn update_or_insert(&mut self, url: &str, name: &str, did: CoreDID) -> &Connection {
        if self.contains(did.as_str()) {
            info!("Updating existing connection: {name}, {url}, {did}");
            self.get_mut(did.as_str()).map(|connection| {
                // TODO: what to do here when any information besides the DID has changed?
                connection.update_last_interaction_time();
                &*connection
            })
        } else {
            info!("Inserting new connection: {name}, {url}, {did}");
            self.insert(Connection::new(name.to_string(), url.to_string(), did.to_string()))
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
    pub did: String,
    pub verified: bool,
    pub first_interacted: String,
    pub last_interacted: String,
}

impl Connection {
    pub fn new(name: String, url: String, did: String) -> Self {
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
    use std::str::FromStr;

    use identity_iota::did::DID;

    use super::*;

    #[test]
    fn test_update_or_insert() {
        let mut connections = Connections::new();
        let url = "https://example.com";
        let name = "Example";
        let did = CoreDID::from_str("did:example:123").unwrap();
        let connection = connections.update_or_insert(url, name, did.clone());
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(did.as_str()));

        let connection = connections.update_or_insert(url, name, did.clone());
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        // The last interaction time should have been updated.
        assert_ne!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
    }

    #[test]
    fn test_update_or_insert_distinguishes_connections_by_did() {
        let mut connections = Connections::new();
        let did = CoreDID::from_str("did:example:123").unwrap();
        let url = "https://example.com";
        let name = "Example";
        let connection = connections.update_or_insert(url, name, did.clone());
        assert_eq!(connection.url, url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 1);
        assert!(connections.contains(did.as_str()));

        // A different DID is a different connection, even when the display name is identical.
        let other_did = CoreDID::from_str("did:example:456").unwrap();
        let other_url = "https://example2.com";
        let connection = connections.update_or_insert(other_url, name, other_did.clone());
        assert_eq!(connection.url, other_url);
        assert_eq!(connection.name, name);
        assert_eq!(connection.first_interacted, connection.last_interacted);
        assert_eq!(connections.0.len(), 2);
        assert!(connections.contains(did.as_str()));
        assert!(connections.contains(other_did.as_str()));
    }
}
