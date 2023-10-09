/// "User prompts" are a way for the backend to communicate a desired/required user interaction to the frontend.
/// This application design leaves it up to the frontend how it wants to handle such "user prompts".
/// Having too much frontend logic in the backend would pollute the loose coupling and make it a lot harder to maintain.
/// For example, the backend can "ask" the frontend to redirect the user to a specific page (e.g. the welcome page).
/// Another example would be that the backend requires some user input to continue, but does not want to be specific about what
/// is displayed to the user exactly.
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/Redirect.ts")]
#[serde(deny_unknown_fields)]
pub struct Redirect {
    pub r#type: CurrentUserPromptType,
    pub target: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/Warning.ts")]
#[serde(deny_unknown_fields)]
pub struct Warning {
    pub r#type: CurrentUserPromptType,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/Selection.ts")]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub r#type: CurrentUserPromptType,
    /// An option is in the form: (<option_name>, <option_value>)
    // pub options: Vec<(String, String)>,
    pub options: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/CredentialOffer.ts")]
#[serde(deny_unknown_fields)]
pub struct CredentialOffer {
    pub r#type: CurrentUserPromptType,
    pub issuer_name: String,
    pub logo_uri: Option<String>,
    #[ts(type = "object")]
    pub credential_offer: serde_json::Value,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/PasswordRequired.ts")]
#[serde(deny_unknown_fields)]
pub struct PasswordRequired {
    pub r#type: CurrentUserPromptType,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/AcceptConnection.ts")]
#[serde(deny_unknown_fields)]
pub struct AcceptConnection {
    pub r#type: CurrentUserPromptType,
    pub client_name: String,
    pub logo_uri: Option<String>,
    pub redirect_uri: String,
    pub previously_connected: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/ShareCredentials.ts")]
#[serde(deny_unknown_fields)]
pub struct ShareCredentials {
    pub r#type: CurrentUserPromptType,
    pub client_name: String,
    pub logo_uri: Option<String>,
    pub options: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[serde(untagged)]
#[ts(export, export_to = "bindings/user-prompt/CurrentUserPrompt.ts")]
pub enum CurrentUserPrompt {
    Redirect(Redirect),
    Warning(Warning),
    Selection(Selection),
    CredentialOffer(CredentialOffer),
    AcceptConnection(AcceptConnection),
    ShareCredentials(ShareCredentials),
    PasswordRequired(PasswordRequired),
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/CurrentUserPromptType.ts")]
pub enum CurrentUserPromptType {
    #[serde(rename = "redirect")]
    Redirect,
    // TODO: remove or rename generic warning
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "accept-connection")] // SIOPv2
    AcceptConnection,
    #[serde(rename = "share-credentials")] // VP
    ShareCredentials,
    #[serde(rename = "credential-offer")]
    CredentialOffer,
    #[serde(rename = "password-required")]
    PasswordRequired,
}
