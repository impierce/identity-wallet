use crate::reducer;
use crate::state::actions::Action;
use crate::state::credentials::reducers::refresh_credential_ecosystems::{
    issuer_connection, refresh_credential_ecosystems, resolve_issuer_ecosystems,
};
use crate::state::qr_code::reducers::accept_connection::EcosystemsUnavailable;
use crate::state::user_prompt::EcosystemProfile;
use crate::state::{actions::ActionTrait, AppState, Reducer};

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

/// Resolves the ecosystems of a credential's issuer again, together with the trust chains into them.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/RefreshCredentialEcosystems.ts")]
pub struct RefreshCredentialEcosystems {
    pub credential_id: String,
    /// Resolved by `prepare`, outside of the state lock. Never sent by the frontend.
    #[serde(skip)]
    #[ts(skip)]
    pub resolved: Option<ResolvedEcosystems>,
}

#[derive(Debug, Clone)]
pub struct ResolvedEcosystems {
    pub connection_id: String,
    pub result: Result<Vec<EcosystemProfile>, EcosystemsUnavailable>,
}

#[typetag::serde(name = "[Credential] Refresh ecosystems")]
impl ActionTrait for RefreshCredentialEcosystems {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(refresh_credential_ecosystems)]
    }

    /// Discovery can take seconds, so it runs without the state lock.
    fn prepare(&self, state: &AppState) -> Option<BoxFuture<'static, Action>> {
        let (connection_id, url) = issuer_connection(state, &self.credential_id)?;
        let credential_id = self.credential_id.clone();

        Some(Box::pin(async move {
            let result = resolve_issuer_ecosystems(&url).await;
            Arc::new(RefreshCredentialEcosystems {
                credential_id,
                resolved: Some(ResolvedEcosystems { connection_id, result }),
            }) as Action
        }))
    }
}
