use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        did::actions::set_preferred_method::SetPreferredDidMethod,
        AppState,
    },
};

pub async fn set_preferred_did_method(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(method) = listen::<SetPreferredDidMethod>(action).map(|payload| payload.method) {
        state.profile_settings.preferred_did_method = method.to_string();
        state.current_user_prompt = None;
    }
    Ok(state)
}
