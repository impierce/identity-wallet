pub mod actions;
pub mod reducers;

use super::FeatTrait;

use oid4vc::{
    oid4vc_core::authorization_request::{AuthorizationRequest, Object},
    oid4vp::oid4vp::OID4VP,
    siopv2::siopv2::SIOPv2,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Connection contains the ID and information of a connection.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Connection {
    pub id: String,
    pub client_name: String,
    pub url: String,
    pub verified: bool,
    pub first_interacted: String,
    pub last_interacted: String,
}

#[typetag::serde(name = "connection")]
impl FeatTrait for Connection {}

#[derive(Serialize, Deserialize)]
pub enum ConnectionRequest {
    SIOPv2(Box<AuthorizationRequest<Object<SIOPv2>>>),
    OID4VP(Box<AuthorizationRequest<Object<OID4VP>>>),
}
