use crate::reducer;
use crate::state::actions::{Action, DEFAULT_ACTION_TIME_LIMIT};
use crate::state::connections::issued_credentials::{
    credential_issuer_url, load_page, IssuedCredentialsPage, LOGOS_TIMEOUT, METADATA_TIMEOUT,
};
use crate::state::connections::reducers::load_issued_credentials::load_issued_credentials;
use crate::state::{actions::ActionTrait, AppState, Reducer};

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use ts_rs::TS;

/// Loads a page of the credentials a connection issues into `AppState::issued_credentials`.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/LoadIssuedCredentials.ts")]
pub struct LoadIssuedCredentials {
    /// Chosen by the frontend and returned in the page, so that it can tell the answer to its latest request from a
    /// page that is still in the state.
    pub request_id: u32,
    pub connection_id: String,
    pub search_term: String,
    pub offset: u32,
    pub limit: u32,
    /// Loaded by `prepare`, outside of the state lock. Never sent by the frontend.
    #[serde(skip)]
    #[ts(skip)]
    pub loaded: Option<IssuedCredentialsPage>,
}

impl LoadIssuedCredentials {
    pub(crate) fn load(&self, state: &AppState) -> BoxFuture<'static, IssuedCredentialsPage> {
        let issuer_url = state
            .connections
            .0
            .iter()
            .find(|connection| connection.id == self.connection_id)
            .and_then(credential_issuer_url);
        Box::pin(load_page(
            self.connection_id.clone(),
            issuer_url,
            state.profile_settings.locale.clone(),
            self.search_term.clone(),
            self.offset,
            self.limit,
        ))
    }
}

#[typetag::serde(name = "[Connection] Load issued credentials")]
impl ActionTrait for LoadIssuedCredentials {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_issued_credentials)]
    }

    fn prepare(&self, state: &AppState) -> Option<BoxFuture<'static, Action>> {
        let load = self.load(state);
        let action = self.clone();
        Some(Box::pin(async move {
            Arc::new(LoadIssuedCredentials {
                loaded: Some(load.await),
                ..action
            }) as Action
        }))
    }

    fn time_limit(&self) -> Duration {
        METADATA_TIMEOUT + LOGOS_TIMEOUT + DEFAULT_ACTION_TIME_LIMIT.saturating_mul(2)
    }
}
