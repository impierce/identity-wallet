use crate::{
    error::AppError,
    state::{dev_mode::actions::dev_profile::OID4VCSteps, AppState},
};

pub async fn selv_flow(state: AppState, _selv_step: OID4VCSteps, _auto_confirm: bool) -> Result<AppState, AppError> {
    Ok(state)
}
