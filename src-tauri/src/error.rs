use crate::state::actions::ActionType;
use std::error::Error;

// TODO: needs revision/refactor + needs oid4vc libs to properly implement error handling.
#[derive(thiserror::Error)]
pub enum AppError {
    #[error("Required payload is missing")]
    MissingPayloadError,
    #[error("Required payload value is missing: {0}")]
    MissingPayloadValueError(&'static str),
    #[error("Unable to parse QR code with content: `{0}`")]
    InvalidQRCodeError(&'static str),
    #[error("Received unknown action type `{r#type:?}` with payload: `{payload:?}`")]
    UnknownActionTypeError {
        r#type: ActionType,
        payload: Option<serde_json::Value>,
    },
    #[error("No `{0}` manager found in the state")]
    MissingManagerError(&'static str),
    #[error("{extension} authorization request cannot be validated")]
    OID4VCProviderManagerError {
        extension: &'static str,
        source: serde_json::Error,
    },
    #[error("Missing required parameter `{0}` in authorization request")]
    MissingAuthorizationRequestParameterError(&'static str),
    #[error("Invalid authorization request: {0}")]
    InvalidAuthorizationRequest(serde_json::Value),
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
    #[error("Failed to load stronghold")]
    StrongholdLoadingError(#[source] anyhow::Error),
    #[error("Failed to delete credential from stronghold")]
    StrongholdDeletionError(#[source] anyhow::Error),
    #[error("Failed to insert credential into stronghold")]
    StrongholdInsertionError(#[source] anyhow::Error),
    #[error("Error while loading credentials from stronghold")]
    StrongholdValuesError(#[source] anyhow::Error),
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
