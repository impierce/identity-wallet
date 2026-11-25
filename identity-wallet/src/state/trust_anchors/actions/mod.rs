use crate::reducer;
use crate::state::trust_anchors::reducers::{
    accept_trust_anchor, deny_trust_anchor, fetch_trust_anchor, remove_trust_anchor, toggle_trust_anchor,
};
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Fetches the certificate at `url` for the user to accept or deny as a trust anchor.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/FetchTrustAnchor.ts")]
pub struct FetchTrustAnchor {
    pub url: String,
}

#[typetag::serde(name = "[Trust Anchors] Fetch")]
impl ActionTrait for FetchTrustAnchor {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(fetch_trust_anchor)]
    }
}

/// Adds the fetched certificate to the trust anchors, enabled.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AcceptTrustAnchor;

#[typetag::serde(name = "[Trust Anchors] Accept")]
impl ActionTrait for AcceptTrustAnchor {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(accept_trust_anchor)]
    }
}

/// Discards the fetched certificate.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DenyTrustAnchor;

#[typetag::serde(name = "[Trust Anchors] Deny")]
impl ActionTrait for DenyTrustAnchor {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(deny_trust_anchor)]
    }
}

/// Enables or disables the trust anchor `id`.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/ToggleTrustAnchor.ts")]
pub struct ToggleTrustAnchor {
    pub id: String,
}

#[typetag::serde(name = "[Trust Anchors] Toggle")]
impl ActionTrait for ToggleTrustAnchor {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(toggle_trust_anchor)]
    }
}

/// Removes the trust anchor `id`, one the user added.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/RemoveTrustAnchor.ts")]
pub struct RemoveTrustAnchor {
    pub id: String,
}

#[typetag::serde(name = "[Trust Anchors] Remove")]
impl ActionTrait for RemoveTrustAnchor {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(remove_trust_anchor)]
    }
}
