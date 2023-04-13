// TODO: find abstraction for each reducer (Reducer trait?, but reducers are functions)

use crate::state::actions::Action;
use crate::state::state::AppState;

pub fn set_locale(state: &AppState, action: Action) -> anyhow::Result<()> {
    *state.locale.lock().unwrap() = action.payload.unwrap();
    Ok(())
}

fn create_iota_did() -> anyhow::Result<()> {
    Ok(())
}
