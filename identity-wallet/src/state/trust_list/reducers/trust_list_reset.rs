use crate::error::AppError;
use crate::persistence::ASSETS_DIR;
use crate::state::trust_list::actions::trust_list_reset::TrustListReset;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn trust_list_reset(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(_action) = listen::<TrustListReset>(action) {
        let trust_list: std::collections::HashMap<String, bool>;
        
        if let Ok(default_trust_list_file) =
            std::fs::File::open(ASSETS_DIR.lock().unwrap().join("default_trust_list.json"))
        {
            trust_list = serde_json::from_reader(default_trust_list_file)
                .expect("error: failed to deserialize default_trust_list.json");
        } else { // strictly for testing purposes because ASSETS_DIR is not available in unit tests
            println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
            let path = std::fs::File::open("resources/default_trust_list.json")
                .expect("error: default_trust_list.json not found");
            trust_list = serde_json::from_reader(path).expect("error: failed to deserialize default_trust_list.json");
        }
        // .expect("error: default_trust_list.json not found");

        return Ok(AppState {
            trust_list,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::trust_list::actions::trust_list_reset::TrustListReset;
    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    async fn test_trust_list_reset() {
        let mut state = AppState::default();
        state.trust_list.insert("https://www.example.com".to_string(), false);

        let action = Arc::new(TrustListReset {});

        let result = trust_list_reset(state, action).await.unwrap();

        assert_eq!(
            result.trust_list,
            HashMap::from([("https://www.impierce.com".to_string(), true)])
        );
    }
}
