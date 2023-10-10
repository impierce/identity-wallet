use oid4vci::credential_offer::CredentialOffer;
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
        logo_uri: Option<String>,
        redirect_uri: String,
        previously_connected: bool,
    },
    #[serde(rename = "credential-offer")]
    CredentialOffer {
        issuer_name: String,
        logo_uri: Option<String>,
        #[ts(type = "object")]
        credential_offer: CredentialOffer,
    },
    #[serde(rename = "share-credentials")]
    ShareCredentials {
        client_name: String,
        logo_uri: Option<String>,
        options: Vec<String>,
    },
}
