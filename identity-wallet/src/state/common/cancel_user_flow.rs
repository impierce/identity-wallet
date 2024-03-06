use crate::error::AppError::{self};
use crate::state::actions::{listen, Action};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;
use crate::{
    reducer,
    state::{actions::ActionTrait, Reducer},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to cancel the user flow and redirect to a specific route.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CancelUserFlow.ts")]
pub struct CancelUserFlow {
    #[ts(optional)]
    pub redirect: Option<String>,
}

#[typetag::serde(name = "[User Flow] Cancel")]
impl ActionTrait for CancelUserFlow {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_flow)]
    }
}

pub async fn cancel_user_flow(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(cancel_user_flow) = listen::<CancelUserFlow>(action) {
        return Ok(AppState {
            current_user_prompt: cancel_user_flow
                .redirect
                .map(|target| CurrentUserPrompt::Redirect { target }),
            ..state
        });
    }

    Ok(state)
}
