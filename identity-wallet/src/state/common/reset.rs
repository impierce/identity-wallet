use crate::{
    reducer,
    state::{actions::ActionTrait, Reducer},
};
use crate::error::AppError::{self};
use crate::persistence::{clear_all_assets, delete_state_file, delete_stronghold};
use crate::state::actions::Action;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;

use serde::{Deserialize, Serialize};

/// Action to completely purge and reset the app state.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reset;

#[typetag::serde(name = "[App] Reset")]
impl ActionTrait for Reset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_state)]
    }
}

/// Completely resets the state to its default values.
pub async fn reset_state(state: AppState, _action: Action) -> Result<AppState, AppError> {
    delete_state_file().await.ok();
    delete_stronghold().await.ok();
    clear_all_assets().ok();

    Ok(AppState {
        current_user_prompt: Some(CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        }),
        // Keep maintaing dev_mode state
        dev_mode: state.dev_mode,
        ..Default::default()
    })
}
