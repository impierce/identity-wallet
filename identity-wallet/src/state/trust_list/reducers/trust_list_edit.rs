use crate::error::AppError;
use crate::state::trust_list::actions::trust_list_edit::TrustListEdit;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_edit(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<TrustListEdit>(action) {
        let mut trust_list = state.trust_list.clone();

        trust_list.remove(&action.old_domain);
        trust_list.insert(
            action.new_domain,
            trust_list
                .get(&action.old_domain)
                .expect("error: invalid domain value send by frontend.")
                .clone(),
        );

        return Ok(AppState {
            trust_list,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
