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
pub struct Redirect {
    pub r#type: CurrentUserPromptType,
    pub target: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/Warning.ts")]
pub struct Warning {
    pub r#type: CurrentUserPromptType,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/Selection.ts")]
pub struct Selection {
    pub r#type: CurrentUserPromptType,
    /// An option is in the form: (<option_name>, <option_value>)
    // pub options: Vec<(String, String)>,
    pub options: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/CredentialOffer.ts")]
pub struct CredentialOffer {
    pub r#type: CurrentUserPromptType,
    /// An option is in the form: (<option_name>, <option_value>)
    // pub options: Vec<(String, String)>,
    #[ts(type = "object")]
    pub credential_offer: serde_json::Value,
}

// pub trait UserActionprompt {
//     fn get_type(&self) -> String;
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// struct UserActionprompt {
//     r#type: UserActionpromptType,
// }

// impl UserActionprompt for Redirect {
//     fn get_type(&self) -> String {
//         self.r#type.to_string()
//     }
// }

// impl<U: UserActionprompt> From<U> for Redirect where U: ?Sized {
//     fn from(user_action_prompt: U) -> Redirect {
//         Redirect {
//             r#type: UserActionpromptType::Redirect,
//             target: user_action_prompt.get_type()
//         }
//     }
// }

// impl WarningDialog for UserActionprompt {
//     // user_action_prompt: UserActionprompt,
//     // message: 'be_careful',
// }

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[serde(untagged)]
#[ts(export, export_to = "bindings/user-prompt/CurrentUserPrompt.ts")]
pub enum CurrentUserPrompt {
    Redirect(Redirect),
    Warning(Warning),
    Selection(Selection),
    CredentialOffer(CredentialOffer),
}

#[derive(Clone, Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/user-prompt/CurrentUserPromptType.ts")]
pub enum CurrentUserPromptType {
    #[serde(rename = "redirect")]
    Redirect,
    // TODO: remove or rename generic warning
    #[serde(rename = "warning")]
    Warning,
    // TODO: remove or rename generic selection
    // #[serde(rename = "selection")]
    // Selection,
    #[serde(rename = "select-credentials")]
    SelectCredentials,
    #[serde(rename = "credential-offer")]
    CredentialOffer,
}
