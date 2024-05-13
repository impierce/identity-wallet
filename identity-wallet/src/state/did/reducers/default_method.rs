use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        did::actions::set_default_method::SetDefaultDidMethod,
        AppState,
    },
};

pub async fn set_default_did_method(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(method) = listen::<SetDefaultDidMethod>(action).map(|payload| payload.method) {
        state.profile_settings.default_did_method = method.to_string();
        state.current_user_prompt = None;
    }
    Ok(state)
}
