use crate::error::AppError::{self};
use crate::persistence::clear_assets_tmp_folder;
use crate::state::actions::{listen, Action};
use crate::state::common::actions::cancel_user_flow::CancelUserFlow;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;

#[tracing::instrument(skip_all, err)]
pub async fn cancel_user_flow(state: AppState, action: Action) -> Result<AppState, AppError> {
    log::debug!("Cancelling active user flow");
    if let Some(cancel_user_flow) = listen::<CancelUserFlow>(action) {
        clear_assets_tmp_folder().ok();
        if let Some(active_flow) = &state.core_utils.active_flow {
            let counterparty = state
                .core_utils
                .pending_connection_data
                .as_ref()
                .map(|data| data.client_metadata.client_name.as_str())
                .unwrap_or("unknown counterparty");
            state.core_utils.debug_log.info(format!(
                "{} with {counterparty} cancelled by the user",
                active_flow.name()
            ));
        }
        return Ok(AppState {
            current_user_prompt: cancel_user_flow
                .redirect
                .map(|target| CurrentUserPrompt::Redirect { target }),
            ..state
        });
    }

    Ok(state)
}
