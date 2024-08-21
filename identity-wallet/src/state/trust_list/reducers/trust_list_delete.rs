use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_delete::TrustListDelete;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_delete(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListDelete>(action) {
        let mut trust_list = state.trust_list.clone();
        trust_list.remove(&action.domain);

        return Ok(AppState {
            trust_list,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
