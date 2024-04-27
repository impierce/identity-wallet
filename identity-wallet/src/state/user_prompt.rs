use std::collections::HashMap;

use oid4vc::oid4vci::credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// "User prompts" are a way for the backend to communicate a desired/required user interaction to the frontend.
/// This application design leaves it up to the frontend how it wants to handle such "user prompts".
/// Having too much frontend logic in the backend would pollute the loose coupling and make it a lot harder to maintain.
/// For example, the backend can "ask" the frontend to redirect the user to a specific page (e.g. the welcome page).
/// Another example would be that the backend requires some user input to continue, but does not want to be specific about what
/// is displayed to the user exactly.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub enum CurrentUserPrompt {
    #[serde(rename = "redirect")]
    Redirect { target: String },
    #[serde(rename = "warning")]
    Warning { message: String },
    #[serde(rename = "password-required")]
    PasswordRequired,
    #[serde(rename = "accept-connection")]
    AcceptConnection {
        client_name: String,
        #[ts(optional)]
        logo_uri: Option<String>,
        redirect_uri: String,
        previously_connected: bool,
    },
    #[serde(rename = "credential-offer")]
    CredentialOffer {
        issuer_name: String,
        #[ts(optional)]
        logo_uri: Option<String>,
        #[ts(type = "Record<string, any>")]
        credential_configurations: HashMap<String, CredentialConfigurationsSupportedObject>,
    },
    #[serde(rename = "share-credentials")]
    ShareCredentials {
        client_name: String,
        #[ts(optional)]
        logo_uri: Option<String>,
        options: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_current_user_prompt() {
        let prompt = CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        };

        let serialized = serde_json::to_string(&prompt).unwrap();
        assert_eq!(serialized, r#"{"type":"redirect","target":"welcome"}"#);

        let prompt = CurrentUserPrompt::Warning {
            message: "This is a warning!".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&prompt).unwrap(),
            r#"{"type":"warning","message":"This is a warning!"}"#
        );

        let prompt = CurrentUserPrompt::PasswordRequired;
        assert_eq!(
            serde_json::to_string(&prompt).unwrap(),
            r#"{"type":"password-required"}"#
        );

        let prompt = CurrentUserPrompt::AcceptConnection {
            client_name: "Test Client".to_string(),
            logo_uri: None,
            redirect_uri: "https://example.com".to_string(),
            previously_connected: false,
        };
        assert_eq!(
            serde_json::to_string(&prompt).unwrap(),
            r#"{"type":"accept-connection","client_name":"Test Client","logo_uri":null,"redirect_uri":"https://example.com","previously_connected":false}"#
        );
    }
}
