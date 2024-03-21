use crate::error::AppError::{self};
use crate::state::actions::{listen, Action};
use crate::state::common::actions::cancel_user_flow::CancelUserFlow;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;

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
