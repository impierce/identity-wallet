use oid4vc::oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use std::error::Error;
use uuid::Uuid;

use crate::state::actions::Action;

// TODO: needs revision/refactor + needs oid4vc libs to properly implement error handling.
#[derive(thiserror::Error)]
pub enum AppError {
    // Generic error (all purpose)
    #[error("Error: {0}")]
    Error(&'static str),
    #[error("Error: {0}")]
    DevError(String),
    #[error("Invalid action found: `{action:?}`")]
    InvalidActionError { action: Action },
    #[error("Unable to parse QR code with content: `{0}`")]
    InvalidQRCodeError(String),
    #[error("No `{0}` manager found in the state")]
    MissingManagerError(&'static str),
    #[error("{extension} authorization request cannot be validated")]
    OID4VCAuthorizationRequestError {
        extension: &'static str,
        source: serde_json::Error,
    },
    #[error("Failed to download the file: {0}")]
    DownloadFailed(#[from] reqwest::Error),
    #[error("Failed to download the file: {0}")]
    DownloadAborted(&'static str),
    #[error("Failed to write to the file: {0}")]
    WriteFailed(#[from] std::io::Error),
    #[error("Error while initializing OID4VC provider manager")]
    OID4VCProviderManagerError(#[source] anyhow::Error),
    #[error("Error while fetching DID identifier from OID4VC subject")]
    OID4VCSubjectIdentifierError(#[source] anyhow::Error),
    #[error("Missing required parameter `{0}` in authorization request")]
    MissingAuthorizationRequestParameterError(&'static str),
    #[error("Invalid authorization request: {0}")]
    InvalidAuthorizationRequest(Box<AuthorizationRequest<Object>>),
    #[error("Invalid credential offer")]
    InvalidCredentialOffer(#[source] serde_json::Error),
    #[error("Could not find a matching credential for input descriptor")]
    NoMatchingCredentialError,
    #[error("Failed to generate authorization response")]
    GenerateAuthorizationResponseError(#[source] anyhow::Error),
    #[error("Failed to send authorization response")]
    SendAuthorizationResponseError,
    #[error("Failed to parse json")]
    InvalidUuidError(#[source] uuid::Error),
    #[error("Failed to create presentation submission")]
    PresentationSubmissionError(#[source] anyhow::Error),
    #[error("Failed to parse DID")]
    DidParseError,
    #[error("Invalid credential format")]
    InvalidCredentialFormatError,
    #[error("Failed to build verifiable presentation")]
    PresentationBuilderError(#[source] identity_credential::error::Error),
    #[error("Failed to retrieve credential offer from the credential issuer")]
    GetCredentialOfferError(#[source] anyhow::Error),
    #[error("Failed to retrieve the credential issuer's authorization server metadata")]
    GetAuthorizationServerMetadataError(#[source] anyhow::Error),
    #[error("Failed to retrieve the credential issuer's metadata")]
    GetCredentialIssuerMetadataError(#[source] anyhow::Error),
    #[error("Failed to retrieve an access token from the credential issuer")]
    GetAccessTokenError(#[source] anyhow::Error),
    #[error("Failed to retrieve credential from the credential issuer")]
    GetCredentialError(#[source] anyhow::Error),
    #[error("Failed to retrieve batch credentials from the credential issuer")]
    GetBatchCredentialError(#[source] anyhow::Error),
    #[error("Failed to find credential offer `{0}` in the credential issuer's metadata")]
    MissingCredentialOfferError(String),
    #[error("Invalid offer indices")]
    InvalidOfferIndicesError(#[source] serde_json::Error),
    #[error("No `{0}` found in the state")]
    MissingStateParameterError(&'static str),
    #[error("Failed to create stronghold")]
    StrongholdCreationError(#[source] anyhow::Error),
    #[error("Failed to load stronghold")]
    StrongholdLoadingError(#[source] anyhow::Error),
    #[error("Failed to delete credential from stronghold")]
    StrongholdDeletionError(#[source] anyhow::Error),
    #[error("Failed to insert credential into stronghold")]
    StrongholdInsertionError(#[source] anyhow::Error),
    #[error("Error while loading credentials from stronghold")]
    StrongholdValuesError(#[source] anyhow::Error),
    #[error("No credential record found for id `{0}`")]
    StrongholdMissingCredentialError(Uuid),
    #[error("Failed to retrieve public key from stronghold")]
    StrongholdPublicKeyError(#[source] anyhow::Error),
    #[error("Failed to delete state file")]
    StateFileDeletionError(#[source] anyhow::Error),
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}
