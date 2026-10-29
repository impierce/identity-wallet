use crate::error::AppError::{self, *};

use serde::Serialize;
use std::error::Error;
use ts_rs::TS;

/// An error as it is shown to the user: what they were doing, why it failed and the technical details for dev mode.
#[derive(Debug, Clone, Serialize, TS, PartialEq)]
#[ts(export, export_to = "bindings/error/UserError.ts")]
pub struct UserError {
    pub context: ErrorContext,
    pub cause: ErrorCause,
    /// Display name of the issuer or verifier involved, if any.
    pub counterparty: Option<String>,
    /// The technical error message. Shown in dev mode only.
    pub detail: String,
}

/// What the user was doing when the error occurred.
#[derive(Debug, Clone, Copy, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "bindings/error/ErrorContext.ts")]
#[serde(rename_all = "snake_case")]
pub enum ErrorContext {
    Unlock,
    CreateProfile,
    ScanQrCode,
    Connect,
    ReceiveCredential,
    ShareCredential,
    DeleteCredential,
    UpdateCredential,
    RefreshCredentialStatus,
    SelfIssueCredential,
    SharePublicLink,
    DeleteConnection,
    VerifyEmail,
    TrustList,
    Settings,
    Biometrics,
    ResetApp,
    General,
}

impl ErrorContext {
    /// Maps the serialized action type (e.g. `[Storage] Unlock`) to what the user was doing.
    pub fn from_action_type(action_type: &str) -> Self {
        use ErrorContext::*;

        match action_type {
            "[Storage] Unlock" | "[Storage] Check password" => Unlock,
            "[DID] Create new" | "[DEV] Load DEV profile" => CreateProfile,
            "[QR Code] Scanned" => ScanQrCode,
            "[Authenticate] Connection accepted" => Connect,
            "[Credential Offer] Selected" | "[Credential Offer] Code received" => ReceiveCredential,
            "[Authenticate] Credentials selected" => ShareCredential,
            "[Credential] Delete" => DeleteCredential,
            "[Credential Metadata] Update" => UpdateCredential,
            "[Credential] Refresh status" | "[Credential] Refresh all statuses" => RefreshCredentialStatus,
            "[Credential] Self Issue" => SelfIssueCredential,
            "[Credential] Share to LinkedIn" => SharePublicLink,
            "[Connection] Delete" => DeleteConnection,
            "[Verified Data] Check service health"
            | "[Verified Data] Send verification email"
            | "[Verified Data] Redeem code"
            | "[Verified Data] Reset email verification" => VerifyEmail,
            action if action.starts_with("[Trust List") => TrustList,
            "[Biometrics] Enable" => Biometrics,
            "[App] Reset" => ResetApp,
            action
                if action.starts_with("[Settings]") || action.starts_with("[DID]") || action.starts_with("[Keys]") =>
            {
                Settings
            }
            _ => General,
        }
    }
}

/// Why the error occurred, in categories the frontend has wording for.
#[derive(Debug, Clone, Copy, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "bindings/error/ErrorCause.ts")]
#[serde(rename_all = "snake_case")]
pub enum ErrorCause {
    /// The other party could not be reached (offline, DNS, connection refused).
    Unreachable,
    /// The operation took too long.
    Timeout,
    /// The other party answered with an error caused by the request (HTTP 4xx).
    Rejected,
    /// The other party has a problem on its side (HTTP 5xx).
    ServerError,
    /// The credential offer has expired, was already used or no longer exists.
    OfferExpired,
    /// The other party sent something UniMe cannot read (malformed metadata, offer or request).
    InvalidResponse,
    /// The scanned QR code or pasted text is not something UniMe understands.
    InvalidQrCode,
    /// The request or credential uses something UniMe does not support.
    Unsupported,
    /// The wallet does not hold the credentials that were asked for.
    MissingCredentials,
    /// The other party could not be verified, so UniMe stopped for the user's safety.
    Untrusted,
    /// The request is no longer active, for example because it was cancelled or the app was restarted.
    FlowExpired,
    /// Information the user entered is not valid.
    InvalidInput,
    /// The password is wrong.
    WrongPassword,
    /// The wallet's data could not be read or saved.
    Storage,
    /// Opening a link in the browser failed.
    BrowserUnavailable,
    Unknown,
}

impl ErrorCause {
    /// Classifies a message in the format the OpenID4VC wallet client and UniMe use for HTTP errors:
    /// `... (HTTP 404 Not Found): <body>`, or `Failed to parse ... (HTTP 200 OK)` for an unreadable answer.
    fn from_message(message: &str) -> Option<Self> {
        let lowercase = message.to_lowercase();
        if let Some((before, rest)) = message.split_once("(HTTP ") {
            if before.to_lowercase().contains("failed to parse") {
                return Some(ErrorCause::InvalidResponse);
            }
            let status = rest.split([' ', ')']).next()?.parse::<u16>().ok()?;
            return Some(Self::from_http_status(status, rest));
        }
        if lowercase.contains("timed out") || lowercase.contains("timeout") {
            return Some(ErrorCause::Timeout);
        }
        if [
            "error sending request",
            "dns error",
            "connection refused",
            "failed to lookup",
            "tcp connect error",
        ]
        .iter()
        .any(|pattern| lowercase.contains(pattern))
        {
            return Some(ErrorCause::Unreachable);
        }
        None
    }

    fn from_http_status(status: u16, body: &str) -> Self {
        match status {
            // An unknown or already redeemed offer.
            404 | 410 => ErrorCause::OfferExpired,
            400 if body.contains("invalid_grant") || body.contains("credential-offer-not-pending") => {
                ErrorCause::OfferExpired
            }
            500.. => ErrorCause::ServerError,
            _ => ErrorCause::Rejected,
        }
    }

    fn from_reqwest(error: &reqwest::Error) -> Self {
        if error.is_timeout() {
            ErrorCause::Timeout
        } else if let Some(status) = error.status() {
            Self::from_http_status(status.as_u16(), "")
        } else if error.is_decode() {
            ErrorCause::InvalidResponse
        } else {
            ErrorCause::Unreachable
        }
    }

    /// Looks for a network error anywhere in the chain of causes.
    fn from_source(source: Option<&(dyn Error + 'static)>) -> Option<Self> {
        let mut current = source;
        while let Some(error) = current {
            if let Some(reqwest_error) = error.downcast_ref::<reqwest::Error>() {
                return Some(Self::from_reqwest(reqwest_error));
            }
            if let Some(cause) = Self::from_message(&error.to_string()) {
                return Some(cause);
            }
            current = error.source();
        }
        None
    }

    /// Network errors (offline, refused by the server) can occur in many operations, so they are recognised first.
    /// Otherwise the error variant decides.
    pub fn from_app_error(error: &AppError) -> Self {
        let network_cause = match error {
            DownloadFailed(reqwest_error) | FetchCredentialListError(reqwest_error) => {
                Some(Self::from_reqwest(reqwest_error))
            }
            Error(message) => Self::from_message(message),
            _ => Self::from_source(error.source()),
        };
        if let Some(cause) = network_cause {
            return cause;
        }

        use ErrorCause::*;
        match error {
            InvalidQRCodeError(_) | InvalidAuthorizationRequest(_) | OID4VCAuthorizationRequestError { .. } => {
                InvalidQrCode
            }
            InvalidCredentialOffer(_) | MissingCredentialOfferError(_) | UnknownCredentialConfigurationIdError(_) => {
                InvalidResponse
            }
            GetCredentialOfferError(_) => OfferExpired,
            GetCredentialIssuerMetadataError(_)
            | GetAuthorizationServerMetadataError(_)
            | GetAccessTokenError(_)
            | GetCredentialError(_)
            | GetBatchCredentialError(_)
            | SendAuthorizationResponseError
            | EmailVerificationServiceError(_) => Unreachable,
            EmailVerificationServiceStatus(status) => Self::from_http_status(*status, ""),
            InvalidCredentialFormatError | InvalidCredentialStatusFormatError | UnsupportedError(_) => Unsupported,
            NoMatchingCredentialError | MissingRequiredCredentialError(_) => MissingCredentials,
            UntrustedError(_) | DidParseError => Untrusted,
            FlowExpiredError(_) | MissingStateParameterError(_) => FlowExpired,
            InvalidInputError(_) => InvalidInput,
            StrongholdLoadingError(_) => WrongPassword,
            StrongholdCreationError(_)
            | StrongholdDeletionError(_)
            | StrongholdInsertionError(_)
            | StrongholdValuesError(_)
            | StrongholdMissingCredentialError(_)
            | StrongholdPublicKeyError(_)
            | StateFileDeletionError(_)
            | DataDirCreationError(_)
            | WriteFailed(_)
            | AppStateMigrationError(..) => Storage,
            BrowserUnavailableError(_) => BrowserUnavailable,
            GetCredentialStatusError => InvalidResponse,
            _ => Unknown,
        }
    }
}

impl UserError {
    pub fn new(error: &AppError, action_type: &str, counterparty: Option<String>) -> Self {
        let context = ErrorContext::from_action_type(action_type);
        let mut cause = ErrorCause::from_app_error(error);
        // Only unlocking compares a password; elsewhere a Stronghold that does not load is a storage problem.
        if cause == ErrorCause::WrongPassword && context != ErrorContext::Unlock {
            cause = ErrorCause::Storage;
        }
        UserError {
            context,
            cause,
            counterparty,
            detail: format!("{error:?}").trim_end().to_string(),
        }
    }

    pub fn timeout(action_type: &str, counterparty: Option<String>) -> Self {
        UserError {
            context: ErrorContext::from_action_type(action_type),
            cause: ErrorCause::Timeout,
            counterparty,
            detail: "Operation timed out".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cause(error: AppError) -> ErrorCause {
        ErrorCause::from_app_error(&error)
    }

    #[test]
    fn recognises_http_errors_from_the_wallet_client() {
        assert_eq!(
            cause(GetAccessTokenError(anyhow::anyhow!(
                r#"Token request failed (HTTP 400 Bad Request): {{"error":"invalid_grant"}}"#
            ))),
            ErrorCause::OfferExpired
        );
        assert_eq!(
            cause(GetCredentialOfferError(anyhow::anyhow!(
                "Failed to get credential offer (HTTP 404 Not Found): "
            ))),
            ErrorCause::OfferExpired
        );
        assert_eq!(
            cause(GetCredentialError(anyhow::anyhow!(
                "Credential issuance failed (HTTP 500 Internal Server Error): oops"
            ))),
            ErrorCause::ServerError
        );
        assert_eq!(
            cause(GetCredentialError(anyhow::anyhow!(
                "Credential issuance failed (HTTP 400 Bad Request): {{\"error\":\"invalid_proof\"}}"
            ))),
            ErrorCause::Rejected
        );
        assert_eq!(
            cause(GetCredentialIssuerMetadataError(anyhow::anyhow!(
                "Failed to parse credential issuer metadata (HTTP 200 OK): expected value"
            ))),
            ErrorCause::InvalidResponse
        );
    }

    #[test]
    fn recognises_network_errors() {
        assert_eq!(
            cause(GetCredentialIssuerMetadataError(
                anyhow::anyhow!("error sending request for url (https://issuer.example/.well-known)")
                    .context("Failed to get metadata")
            )),
            ErrorCause::Unreachable
        );
        assert_eq!(
            cause(Error("Failed to get nonce: error sending request for url".to_string())),
            ErrorCause::Unreachable
        );
    }

    #[test]
    fn falls_back_to_the_error_variant() {
        assert_eq!(
            cause(InvalidQRCodeError("not a request".to_string())),
            ErrorCause::InvalidQrCode
        );
        assert_eq!(
            cause(StrongholdLoadingError(anyhow::anyhow!("invalid snapshot"))),
            ErrorCause::WrongPassword
        );
        assert_eq!(cause(Error("anything else".to_string())), ErrorCause::Unknown);
    }

    #[test]
    fn a_failing_stronghold_is_only_a_wrong_password_when_unlocking() {
        let error = StrongholdLoadingError(anyhow::anyhow!("invalid snapshot"));
        assert_eq!(
            UserError::new(&error, "[Storage] Unlock", None).cause,
            ErrorCause::WrongPassword
        );
        assert_eq!(
            UserError::new(&error, "[Credential] Delete", None).cause,
            ErrorCause::Storage
        );
    }

    #[test]
    fn maps_actions_to_contexts() {
        assert_eq!(
            ErrorContext::from_action_type("[Credential Offer] Selected"),
            ErrorContext::ReceiveCredential
        );
        assert_eq!(
            ErrorContext::from_action_type("[Trust Lists] Add"),
            ErrorContext::TrustList
        );
        assert_eq!(
            ErrorContext::from_action_type("[Settings] Set locale"),
            ErrorContext::Settings
        );
        assert_eq!(
            ErrorContext::from_action_type("[Unknown] Action"),
            ErrorContext::General
        );
    }
}
