use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        credentials::actions::share_to_linkedin::ShareToLinkedIn,
        AppState,
    },
};

pub async fn share_to_linkedin(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(share_to_linkedin) = listen::<ShareToLinkedIn>(action) {

        // return Ok(AppState {
        //     credentials,
        //     current_user_prompt: redirect_prompt,
        //     ..state
        // });
    }

    Ok(state)
}
