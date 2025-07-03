use crate::error::AppError;
use chrono::{Duration, Utc};
use identity_core::common::Object as IotaObject;

use identity_credential::{credential::Jwt, presentation::Presentation};
use identity_iota::core::Url;
use identity_iota::did::{CoreDID, DIDUrl, DID};
use jsonwebtoken::Header;
use oid4vc::oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use oid4vc::oid4vc_core::{jwt, Subject};
use oid4vc::oid4vp::dcql::dcql_query::{CredentialQuery, Format};
use oid4vc::oid4vp::oid4vp::OID4VP;
use oid4vc::oid4vp::token::{
    vp_token::{PresentationFormat, VpToken},
    vp_token_builder::VpTokenBuilder,
};
use serde_json::Value;
use std::sync::Arc;
pub async fn prepare_vp_token_object(
    selected_verifiable_credentials: Vec<(CredentialQuery, Value)>,
    subject_did: &CoreDID,
    subject_manager: &Arc<dyn Subject>,
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> Result<VpToken, AppError> {
    let verifier_audience = oid4vp_authorization_request.body.client_id.to_string();
    let required_nonce = oid4vp_authorization_request.body.extension.nonce.clone();

    let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
    let mut builder = VpTokenBuilder::builder_dcql_query(dcql_query.clone());
    let signing_method_id =
        DIDUrl::parse(format!("{}#{}", subject_did, subject_did.method_id())).map_err(|_| AppError::DidParseError)?;

    for (credential_query_from_dcql, vc_value) in selected_verifiable_credentials {
        let credential_id = credential_query_from_dcql.id.clone();
        let format_from_query = credential_query_from_dcql.format;
        // .map_err(|e| AppError::Error(format!("Invalid credential ID: {}", e)))?;

        let presentation_format_item = match format_from_query {
            Format::JwtVcJson => {
                let raw_vc_jwt_string = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .to_string();

                let vc_jwt: Jwt = raw_vc_jwt_string.into();

                let holder_url: Url = Url::parse(&subject_did.to_string())
                    .map_err(|e| AppError::Error(format!("Failed to parse DID as URL: {}", e)))?;

                let presentation = Presentation::builder(holder_url, IotaObject::default())
                    .credential(vc_jwt)
                    .build()
                    .map_err(|e| AppError::PresentationBuilderError(e))?;

                let vp_jwt_claims = serde_json::json!({
                    "vp": presentation,
                    "iss": subject_did.to_string(),
                    "sub": subject_did.to_string(),
                    "aud": verifier_audience.to_string(),
                    "nonce": required_nonce.to_string(),
                    "iat": Utc::now().timestamp(),
                    "exp": (Utc::now() + Duration::minutes(10)).timestamp()
                });

                let jwt_header = Header {
                    alg: jsonwebtoken::Algorithm::ES256, //algorithm - dynamic eddsa
                    kid: Some(signing_method_id.to_string()),
                    typ: Some("JWT".to_string()),
                    ..Default::default()
                };

                // let did_method = format!("did:{}", subject_did.method());
                let signed_vc_presentation_jwt_string = jwt::encode(
                    subject_manager.clone(),
                    jwt_header,
                    vp_jwt_claims,
                    &format!("did:{}", subject_did.method()),
                )
                .await
                .map_err(|e| AppError::Error(format!("Failed to sign VP JWT: {}", e)))?;

                PresentationFormat::JwtVcJson(signed_vc_presentation_jwt_string)
            }
            Format::DcSdJwt => {
                let sd_jwt_vc_string = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .to_string();

                // TODO: Implement proper SD-JWT presentation logic here

                PresentationFormat::DcSdJwt(sd_jwt_vc_string)
            }
            _ => {
                return Err(AppError::InvalidCredentialFormatError);
            }
        };

        builder = builder.add_presentation(credential_id, presentation_format_item);
    }

    // Build and validate the VP token
    builder
        .build()
        .map_err(|e| AppError::Error(format!("Failed to build VpToken: {:?}", e)))
}
