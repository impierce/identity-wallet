pub mod connection_accepted;

use super::FeatTrait;

use oid4vc::{
    oid4vc_core::authorization_request::{AuthorizationRequest, Object},
    oid4vp::oid4vp::OID4VP,
    siopv2::siopv2::SIOPv2,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize)]
pub enum ConnectionRequest {
    SIOPv2(Box<AuthorizationRequest<Object<SIOPv2>>>),
    OID4VP(Box<AuthorizationRequest<Object<OID4VP>>>),
}

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

// Helpers

// TODO: move this functionality to the oid4vc-manager crate.
pub fn get_siopv2_client_name_and_logo_uri(
    siopv2_authorization_request: &AuthorizationRequest<Object<SIOPv2>>,
) -> anyhow::Result<(String, Option<String>, String)> {
    let connection_url = siopv2_authorization_request
        .body
        .redirect_uri
        .domain()
        .ok_or(anyhow::anyhow!("unable to get domain from redirect_uri"))?
        .to_string();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(siopv2_authorization_request
        .body
        .extension
        .client_metadata
        .as_ref()
        .map(|client_metadata| {
            let client_name = client_metadata
                .client_name
                .as_ref()
                .cloned()
                .unwrap_or(connection_url.to_string());
            let logo_uri = client_metadata.logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            (client_name, logo_uri, connection_url.clone())
        })
        // Otherwise use the connection_url as the client_name.
        .unwrap_or((connection_url.to_string(), None, connection_url)))
}

// TODO: move this functionality to the oid4vc-manager crate.
pub fn get_oid4vp_client_name_and_logo_uri(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> anyhow::Result<(String, Option<String>, String)> {
    let connection_url = oid4vp_authorization_request
        .body
        .redirect_uri
        .domain()
        .ok_or(anyhow::anyhow!("unable to get domain from redirect_uri"))?
        .to_string();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(oid4vp_authorization_request
        .body
        .extension
        .client_metadata
        .as_ref()
        .map(|client_metadata| {
            let client_name = client_metadata
                .client_name
                .as_ref()
                .cloned()
                .unwrap_or(connection_url.to_string());
            let logo_uri = client_metadata.logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            (client_name, logo_uri, connection_url.clone())
        })
        // Otherwise use the connection_url as the client_name.
        .unwrap_or((connection_url.to_string(), None, connection_url)))
}
