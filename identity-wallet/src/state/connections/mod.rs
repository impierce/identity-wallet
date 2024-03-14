pub mod actions;
pub mod reducers;

use super::{core_utils::DateUtils, FeatTrait};

use log::info;
use serde::{Deserialize, Serialize};
use std::ops::Not;
use ts_rs::TS;

#[derive(Default, Serialize, Deserialize, Clone, Debug, TS)]
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
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
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
