use std::str::FromStr;

use log::debug;
use oid4vc::oid4vc_core::SubjectSyntaxType;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        did::actions::set_preferred_method::SetPreferredDidMethod,
        profile_settings::ProfileSettings,
        AppState,
    },
};

pub async fn set_preferred_did_method(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(method) = listen::<SetPreferredDidMethod>(action).map(|payload| payload.method) {
        let mut preferred_did_methods = state.profile_settings.preferred_did_methods;

        debug!("Order of preferred DID methods (current): {:?}", preferred_did_methods);

        let current_position = preferred_did_methods
            .iter()
            .position(|m| m == &method.to_string())
            .unwrap();

        let element = preferred_did_methods.remove(current_position);

        preferred_did_methods.insert(0, element);

        debug!("Order of preferred DID methods (updated): {:?}", preferred_did_methods);

        let mut state_guard = state.core_utils.managers.lock().await;

        let identity_manager = state_guard.identity_manager.as_mut().unwrap();

        identity_manager
            .provider_manager
            .provider
            .supported_subject_syntax_types = preferred_did_methods
            .iter()
            .map(|m| SubjectSyntaxType::from_str(m).unwrap())
            .collect();

        identity_manager.wallet.supported_subject_syntax_types = preferred_did_methods
            .iter()
            .map(|m| SubjectSyntaxType::from_str(m).unwrap())
            .collect();

        drop(state_guard);
        return Ok(AppState {
            profile_settings: ProfileSettings {
                preferred_did_methods,
                ..state.profile_settings
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
