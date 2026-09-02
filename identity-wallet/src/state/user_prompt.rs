use identity_iota::did::CoreDID;
use oid4vc::oid4vci::credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject;
use oid4vc::oid4vci::credential_offer::TxCodeConstraints;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::state::{core_utils::history_event::HistoryEvent, did::validate_domain_linkage::ValidationResult};

use super::did::validate_linked_verifiable_presentations::LinkedVerifiableCredentialData;

/// "User prompts" are a way for the backend to communicate a desired/required user interaction to the frontend.
/// This application design leaves it up to the frontend how it wants to handle such "user prompts".
/// Having too much frontend logic in the backend would pollute the loose coupling and make it a lot harder to maintain.
/// For example, the backend can "ask" the frontend to redirect the user to a specific page (e.g. the welcome page).
/// Another example would be that the backend requires some user input to continue, but does not want to be specific about what
/// is displayed to the user exactly.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
#[ts(export, export_to = "bindings/user_prompt/CurrentUserPrompt.ts")]
pub enum CurrentUserPrompt {
    #[serde(rename = "redirect")]
    Redirect { target: String },
    #[serde(rename = "password-required")]
    PasswordRequired,
    #[serde(rename = "accept-connection")]
    AcceptConnection {
        client_metadata: ClientMetadata,
        // The connection_data field is optional, None means that the user has never interacted with this connection before.
        #[ts(optional)]
        #[serde(skip_serializing_if = "Option::is_none")]
        connection_data: Option<ConnectionData>,
        domain_validation: Box<ValidationResult>,
        #[ts(optional)]
        #[serde(skip_serializing_if = "Option::is_none")]
        linked_verifiable_presentations: Option<Vec<LinkedVerifiableCredentialData>>,
        #[ts(optional)]
        #[serde(skip_serializing_if = "Option::is_none")]
        ecosystems: Option<Vec<EcosystemProfile>>,
    },
    #[serde(rename = "credential-offer")]
    CredentialOffer {
        issuer_name: String,
        #[ts(optional)]
        logo_uri: Option<String>,
        #[ts(type = "Record<string, any>")]
        credential_configurations: HashMap<String, CredentialConfigurationsSupportedObject>,
        #[ts(optional, type = "{ input_mode?: 'numeric' | 'text'; length?: number }")]
        tx_code: Option<TxCodeConstraints>,
    },
    #[serde(rename = "share-credentials")]
    ShareCredentials {
        client_name: String,
        #[ts(optional)]
        logo_uri: Option<String>,
        options: Vec<String>,
        // If this is set to `true`, then this means that the credentials being requested are required during an
        // OID4VCI Interactive Authorization flow.
        #[serde(default)]
        is_interactive: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/ClientMetadata.ts")]

pub struct ClientMetadata {
    pub client_name: String,
    pub logo_uri: Option<String>,
    pub connection_url: String,
    pub redirect_uri: Option<String>,
    #[ts(type = "string")]
    pub client_id: CoreDID,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/ConnectionData.ts")]
pub struct ConnectionData {
    pub first_interacted_at: String,
    pub last_interacted_at: String,
    pub interactions: Vec<HistoryEvent>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/EcosystemProfile.ts")]
pub struct EcosystemProfile {
    pub logo_uri: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub ecosystem_leader: Member,
    pub member_count: usize,
    pub members: Vec<Member>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/Member.ts")]
pub struct Member {
    pub logo_uri: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub domain: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::did::validate_domain_linkage::ValidationStatus;

    #[test]
    fn test_serialize_current_user_prompt() {
        let prompt = CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        };

        let serialized = serde_json::to_string(&prompt).unwrap();
        assert_eq!(serialized, r#"{"type":"redirect","target":"welcome"}"#);

        let prompt = CurrentUserPrompt::PasswordRequired;
        assert_eq!(
            serde_json::to_string(&prompt).unwrap(),
            r#"{"type":"password-required"}"#
        );

        let prompt = CurrentUserPrompt::AcceptConnection {
            client_metadata: ClientMetadata {
                client_name: "Test Client".to_string(),
                logo_uri: None,
                connection_url: "https://example.com".to_string(),
                redirect_uri: Some("https://example.com".to_string()),
                client_id: "did:example:123".parse().unwrap(),
            },
            connection_data: None,
            domain_validation: Box::new(ValidationResult {
                status: ValidationStatus::default(),
                url: "https://example.com".parse().unwrap(),
                name: None,
                logo_uri: None,
                issuance_date: None,
                message: None,
            }),
            linked_verifiable_presentations: Default::default(),
            ecosystems: None,
        };
        assert_eq!(
            serde_json::to_string(&prompt).unwrap(),
            r#"{"type":"accept-connection","client_name":"Test Client","redirect_uri":"https://example.com","domain_validation":{"status":"Unknown","url":"https://example.com/"}}"#
        );
    }
}
