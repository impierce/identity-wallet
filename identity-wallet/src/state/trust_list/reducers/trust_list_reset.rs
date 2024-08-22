use crate::error::AppError;
use crate::persistence::ASSETS_DIR;
use crate::state::trust_list::actions::trust_list_reset::TrustListReset;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_reset(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(_action) = listen::<TrustListReset>(action) {
        let default_trust_list_file = std::fs::File::open(ASSETS_DIR.lock().unwrap().join("default_trust_list.json"))
            .expect("error: default_trust_list.json not found");
        let trust_list = serde_json::from_reader(default_trust_list_file)
            .expect("error: failed to deserialize default_trust_list.json");

        return Ok(AppState {
            trust_list,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
