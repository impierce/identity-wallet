use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        did::actions::produce::ProduceDid,
        AppError::MissingManagerError,
        AppState,
    },
};

pub async fn produce(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(method) = listen::<ProduceDid>(action).map(|payload| payload.method) {
        let state_guard = state.core_utils.managers.lock().await;
        let method_str = method.to_string();
        let identity_manager = state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?;
        let did = identity_manager.subject.identifier(&method_str).await.unwrap();
        state.dids.insert(method_str, did);
        state.current_user_prompt = None;
    }
    Ok(state)
}
