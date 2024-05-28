use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        did::actions::set_preferred_keytype::SetPreferredKeyType,
        profile_settings::ProfileSettings,
        AppState,
    },
};
use log::info;
use oid4vc::oid4vc_core::SubjectSyntaxType;
use std::str::FromStr;

pub async fn set_preferred_key_type(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(key_type) = listen::<SetPreferredKeyType>(action).map(|payload| payload.key_type) {
        // TODO: re-generate and update the "dids" field
        let mut managers = state.core_utils.managers.lock().await;

        let identity_manager = managers
            .identity_manager
            .as_mut()
            .ok_or(AppError::MissingManagerError("identity"))?;

        let algorithm = match key_type.as_str() {
            "Ed25519" => jsonwebtoken::Algorithm::EdDSA,
            "ES256" => jsonwebtoken::Algorithm::ES256,
            _ => return Err(AppError::Error("Unsupported key type".to_string())),
        };

        // Update the deterministic DIDs
        let did_jwk = identity_manager.subject.identifier("did:jwk", algorithm).await.unwrap();
        let did_key = identity_manager.subject.identifier("did:key", algorithm).await.unwrap();

        let mut dids = state.dids.clone();
        dids.insert("did:jwk".to_string(), did_jwk);
        dids.insert("did:key".to_string(), did_key);

        drop(managers);

        return Ok(AppState {
            dids,
            profile_settings: ProfileSettings {
                preferred_key_type: key_type,
                ..state.profile_settings
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
