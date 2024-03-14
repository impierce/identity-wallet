pub mod actions;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

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
        let current_datetime = chrono::Utc::now().to_rfc3339();
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
        self.last_interacted = chrono::Utc::now().to_rfc3339();
    }
}
