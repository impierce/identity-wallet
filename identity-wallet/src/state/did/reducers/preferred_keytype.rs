use std::str::FromStr;

use jsonwebtoken::Algorithm;
use log::debug;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        did::actions::set_preferred_keytype::SetPreferredKeyType,
        profile_settings::ProfileSettings,
        AppState,
    },
};

pub async fn set_preferred_key_type(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(key_type) = listen::<SetPreferredKeyType>(action).map(|payload| payload.key_type) {
        let mut managers = state.core_utils.managers.lock().await;

        let mut preferred_key_types = state.profile_settings.preferred_key_types;

        debug!("Order of preferred key types (current): {:?}", preferred_key_types);

        let current_position = preferred_key_types
            .iter()
            .position(|k| k == &key_type.to_string())
            .unwrap();

        let element = preferred_key_types.remove(current_position);

        preferred_key_types.insert(0, element);

        debug!("Order of preferred key types (updated): {:?}", preferred_key_types);

        let identity_manager = managers
            .identity_manager
            .as_mut()
            .ok_or(AppError::MissingManagerError("identity"))?;

        let algorithm = match key_type.as_str() {
            "EdDSA" => jsonwebtoken::Algorithm::EdDSA,
            "ES256" => jsonwebtoken::Algorithm::ES256,
            _ => return Err(AppError::Error("Unsupported key type".to_string())),
        };

        // Update the (deterministic) DIDs
        let did_jwk = identity_manager.subject.identifier("did:jwk", algorithm).await.unwrap();
        let did_key = identity_manager.subject.identifier("did:key", algorithm).await.unwrap();

        let mut dids = state.dids.clone();
        dids.insert("did:jwk".to_string(), did_jwk);
        dids.insert("did:key".to_string(), did_key);

        // Update the Identity Manager
        let supported_subject_syntax_types = preferred_key_types
            .iter()
            .map(String::as_str)
            .map(Algorithm::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Error(e.to_string()))?;
        identity_manager
            .provider_manager
            .provider
            .supported_signing_algorithms
            .clone_from(&supported_subject_syntax_types);
        identity_manager
            .wallet
            .proof_signing_alg_values_supported
            .clone_from(&supported_subject_syntax_types);

        drop(managers);
        return Ok(AppState {
            dids,
            profile_settings: ProfileSettings {
                preferred_key_types,
                ..state.profile_settings
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
