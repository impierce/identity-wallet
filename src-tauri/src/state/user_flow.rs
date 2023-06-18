/// "User flows" are a way for the backend to communicate a desired/required user interaction to the frontend.
/// This application design leaves it up to the frontend how it wants to handle such "user flows".
/// Having too much frontend logic in the backend would pollute the loose coupling and make it a lot harder to maintain.
/// For example, the backend can "ask" the frontend to redirect the user to a specific page (e.g. the welcome page).
/// Another example would be that the backend requires some user input to continue, but does not want to be specific about what
/// is displayed to the user exactly.
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to="bindings/user-flow/Redirect.ts")]
pub struct Redirect {
    pub r#type: CurrentUserFlowType,
    pub target: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to="bindings/user-flow/Warning.ts")]
pub struct Warning {
    pub r#type: CurrentUserFlowType,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to="bindings/user-flow/Selection.ts")]
pub struct Selection {
    pub r#type: CurrentUserFlowType,
    pub options: Vec<String>,
}

// pub trait UserActionFlow {
//     fn get_type(&self) -> String;
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// struct UserActionFlow {
//     r#type: UserActionFlowType,
// }

// impl UserActionFlow for Redirect {
//     fn get_type(&self) -> String {
//         self.r#type.to_string()
//     }
// }

// impl<U: UserActionFlow> From<U> for Redirect where U: ?Sized {
//     fn from(user_action_flow: U) -> Redirect {
//         Redirect {
//             r#type: UserActionFlowType::Redirect,
//             target: user_action_flow.get_type()
//         }
//     }
// }

// impl WarningDialog for UserActionFlow {
//     // user_action_flow: UserActionFlow,
//     // message: 'be_careful',
// }

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to="bindings/user-flow/CurrentUserFlow.ts")]
pub enum CurrentUserFlow {
    Redirect(Redirect),
    Warning(Warning),
    Selection(Selection),
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to="bindings/user-flow/CurrentUserFlowType.ts")]
pub enum CurrentUserFlowType {
    #[serde(rename = "redirect")]
    Redirect,
    // TODO: remove or rename generic warning
    #[serde(rename = "warning")]
    Warning,
    // TODO: remove or rename generic selection
    #[serde(rename = "selection")]
    Selection,
    #[serde(rename = "select-credentials")]
    SelectCredentials,
}
