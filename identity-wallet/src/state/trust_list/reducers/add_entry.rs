use log::info;
use url::Url;

use crate::error::AppError;
use crate::state::trust_list::actions::add_trust_list_entry::AddTrustListEntry;
use crate::state::{
    actions::{listen, Action},
    AppState,
};

pub async fn add_trust_list_entry(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(action) = listen::<AddTrustListEntry>(action) {
        let mut trust_lists = state.trust_lists;

        // Parse the domain value into a Url
        let domain = Url::parse(&action.domain)
            // If the domain value does not contain a scheme, then apply the `https` scheme as the default scheme.
            .or_else(|_| Url::parse(&format!("https://{}", action.domain)))
            .map_err(|_| AppError::Error(format!("Invalid domain value: `{}`", action.domain)))?;

        let trust_list = trust_lists
            .get_mut(&action.trust_list_id)
            .ok_or_else(|| AppError::TrustListNotFoundError(action.trust_list_id.clone()))?;

        trust_list.insert(domain, true);

        info!(
            "Added trusted domain `{}` to list `{}`",
            action.domain, trust_list.display_name
        );

        return Ok(AppState {
            trust_lists,
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use url::Url;
    use uuid::Uuid;

    use super::*;
    use crate::state::trust_list::TrustList;

    use std::{collections::HashMap, sync::Arc};

    #[tokio::test]
    #[serial_test::serial]
    async fn test_add_trust_list_entry() {
        let mut state = AppState::default();
        let default_trust_list = TrustList {
            id: Uuid::new_v4().to_string(),
            display_name: "impierce".to_string(),
            custom: true,
            entries: HashMap::from([(Url::parse("https://impierce.com").unwrap(), true)]),
        };
        state.trust_lists.insert(default_trust_list.clone());

        let action = Arc::new(AddTrustListEntry {
            trust_list_id: default_trust_list.id.clone(),
            domain: "example.com".to_string(),
        });

        let result = add_trust_list_entry(state, action).await.unwrap();

        let mut expected = default_trust_list.clone();
        expected.insert(Url::parse("https://example.com").unwrap(), true);

        assert_eq!(result.trust_lists.0.first().unwrap().clone(), expected);
    }
}
