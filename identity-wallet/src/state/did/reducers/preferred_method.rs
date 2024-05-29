use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        did::actions::set_preferred_method::SetPreferredDidMethod,
        profile_settings::ProfileSettings,
        AppState,
    },
};
use oid4vc::oid4vc_core::SubjectSyntaxType;
use std::str::FromStr;

pub async fn set_preferred_did_method(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(method) = listen::<SetPreferredDidMethod>(action).map(|payload| payload.method) {
        let mut managers = state.core_utils.managers.lock().await;

        let identity_manager = managers
            .identity_manager
            .as_mut()
            .ok_or(AppError::MissingManagerError("identity"))?;

        // TODO: add error handling here
        let subject_syntax_type = SubjectSyntaxType::from_str(&method.to_string()).unwrap();

        // identity_manager.provider_manager.provider.default_subject_syntax_type = subject_syntax_type.clone();
        // identity_manager.wallet.default_subject_syntax_type = subject_syntax_type;

        drop(managers);

        return Ok(AppState {
            profile_settings: ProfileSettings {
                preferred_did_method: method.to_string(),
                ..state.profile_settings
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
