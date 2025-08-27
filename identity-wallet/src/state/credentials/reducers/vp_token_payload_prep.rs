use crate::error::AppError;
use crate::state::credentials::reducers::self_issue_credential::SubjectWrapper;
use crate::state::credentials::DisplayClaim;
use crate::state::AppState;
use crate::stronghold::StrongholdManager;
use chrono::{Duration, Utc};
use did_manager::Resolver;
use identity_core::common::Object as IotaObject;

use identity_credential::sd_jwt_v2::{JsonObject, KeyBindingJwtBuilder, SdJwt, Sha256Hasher};
use identity_credential::sd_jwt_vc::SdJwtVc;
use identity_credential::{credential::Jwt, presentation::Presentation};
use identity_iota::core::{Timestamp, Url};
use jsonwebtoken::Algorithm;
use jsonwebtoken::Header;
use oid4vc::oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use oid4vc::oid4vc_core::claim_path_pointer::{ClaimPathElement, ClaimPathPointer};
use oid4vc::oid4vc_core::{jwt, Subject};
use oid4vc::oid4vp::dcql::dcql_query::{CredentialQuery, Format};
use oid4vc::oid4vp::oid4vp::OID4VP;
use oid4vc::oid4vp::token::{
    verifiable_presentation_jwt::VerifiablePresentationJwt,
    vp_token::{PresentationFormat, VpToken},
    vp_token_builder::VpTokenBuilder,
};
use serde_json::Value;
use std::ops::Deref;
use std::sync::Arc;

pub async fn prepare_vp_token_object(
    selected_verifiable_credentials: Vec<(CredentialQuery, Value, Vec<DisplayClaim>)>,
    did_method: &str,
    subject_manager: &Arc<dyn Subject>,
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
    signing_algorithm: Algorithm,
) -> Result<VpToken, AppError> {
    let verifier_audience = oid4vp_authorization_request.body.client_id.to_string();
    let required_nonce = oid4vp_authorization_request.body.extension.nonce.clone();

    let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
    let mut builder = VpTokenBuilder::builder_dcql_query(dcql_query.clone());

    let key_id = subject_manager
        .key_id(did_method, signing_algorithm)
        .await
        .ok_or_else(|| AppError::Error(format!("Failed to get signing method ID for DID method {did_method}")))?;

    for (credential_query_from_dcql, vc_value, display_claims) in selected_verifiable_credentials {
        let credential_query_id = credential_query_from_dcql.id.clone();
        let format_from_query = credential_query_from_dcql.format;

        let presentation_format_item = match format_from_query {
            Format::JwtVcJson => {
                let raw_vc_jwt_string = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .to_string();

                let vc_jwt: Jwt = raw_vc_jwt_string.into();

                let full_did = subject_manager
                    .identifier(did_method, signing_algorithm)
                    .await
                    .map_err(|e| AppError::Error(format!("Failed to get DID identifier: {e}")))?;

                let full_did_string = full_did.to_string();

                let holder_url: Url = Url::parse(&full_did_string)
                    .map_err(|e| AppError::Error(format!("Failed to parse DID as URL: {e}")))?;

                let presentation = Presentation::builder(holder_url, IotaObject::default())
                    .credential(vc_jwt)
                    .build()
                    .map_err(AppError::PresentationBuilderError)?;

                let verifiable_presentation_jwt = VerifiablePresentationJwt::builder()
                    .iss(full_did_string.clone())
                    .sub(full_did_string.clone())
                    .aud(verifier_audience.to_string())
                    .nonce(required_nonce.to_string())
                    .iat(Utc::now().timestamp())
                    .exp((Utc::now() + Duration::minutes(10)).timestamp())
                    .verifiable_presentation(presentation)
                    .build()
                    .map_err(|e| AppError::Error(format!("Failed to build VerifiablePresentationJwt: {e}")))?;

                let jwt_header = Header {
                    alg: signing_algorithm,
                    kid: Some(key_id.to_string()),
                    typ: Some("JWT".to_string()),
                    ..Default::default()
                };

                let signed_vc_presentation_jwt_string = jwt::encode(
                    subject_manager.clone(),
                    jwt_header,
                    &verifiable_presentation_jwt,
                    did_method,
                )
                .await
                .map_err(|e| AppError::Error(format!("Failed to sign VP JWT: {e}")))?;

                PresentationFormat::JwtVcJson(signed_vc_presentation_jwt_string)
            }
            Format::DcSdJwt => {
                let sd_jwt_vc = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .to_string()
                    .parse::<SdJwtVc>()
                    .unwrap();

                let subject_wrapper = SubjectWrapper {
                    subject: subject_manager.clone(),
                    preferred_did_method: did_method.to_string(),
                };

                let key_binding_jwt = KeyBindingJwtBuilder::new()
                    .iat(Utc::now().timestamp())
                    .aud(verifier_audience.to_string())
                    .nonce(required_nonce.to_string())
                    .finish(&sd_jwt_vc, &Sha256Hasher::new(), "RS256", &subject_wrapper)
                    .await
                    .map_err(|e| AppError::Error(format!("Failed to build KeyBindingJwt: {e}")))?;

                let all_claims = sd_jwt_vc.claims().deref().deref();

                let (sd_jwt_vc, _) = sd_jwt_vc
                    .into_presentation(&Sha256Hasher::new())
                    .unwrap()
                    // TODO: Conceal claims
                    .attach_key_binding_jwt(key_binding_jwt)
                    .finish()
                    .unwrap();

                // TODO: Implement proper SD-JWT presentation logic here

                PresentationFormat::DcSdJwt(sd_jwt_vc.to_string())
            }
            _ => {
                return Err(AppError::InvalidCredentialFormatError);
            }
        };

        builder = builder.add_presentation(credential_query_id, presentation_format_item);
    }

    // Build and validate the VP token
    builder
        .build()
        .map_err(|e| AppError::Error(format!("Failed to build VpToken: {e:?}",)))
}

// #[tokio::test]
// async fn test() {
//     pub(crate) const ISSUER_SECRET: &[u8] = b"0123456789ABCDEF0123456789ABCDEF";

//     /// A JWS signer that uses HS256 with a static secret string.
//     pub(crate) struct TestSigner;

//     #[async_trait::async_trait]
//     impl identity_credential::sd_jwt_v2::JwsSigner for TestSigner {
//         type Error = josekit::JoseError;
//         async fn sign(&self, header: &JsonObject, payload: &JsonObject) -> std::result::Result<Vec<u8>, Self::Error> {
//             let signer = josekit::jws::HS256.signer_from_bytes(ISSUER_SECRET)?;
//             let header = josekit::jws::JwsHeader::from_map(header.clone())?;
//             let payload = josekit::jwt::JwtPayload::from_map(payload.clone())?;
//             let jws = josekit::jwt::encode_with_signer(&payload, &header, &signer)?;

//             Ok(jws.into_bytes())
//         }
//     }

//     let display_claims: Vec<DisplayClaim> = vec![DisplayClaim {
//         path: ClaimPathPointer::try_new(vec![ClaimPathElement::String("name".to_string())]).unwrap(),
//         key: "Name".to_string(),
//         value: serde_json::json!("Ferris"),
//     }];

//     let credential_data = serde_json::json!({
//         "foo": "bar",
//         "name": "Ferris",
//         "age": 3,
//         "nested": {
//             "a": 1,
//             "b": 2
//         },
//         "another_clai"
//     });

//     let signer = TestSigner;

//     let sd_jwt_credential: SdJwt = identity_credential::sd_jwt_vc::SdJwtVcBuilder::new(credential_data)
//         .unwrap()
//         .make_concealable("/foo")
//         .unwrap()
//         .make_concealable("/name")
//         .unwrap()
//         .make_concealable("/age")
//         .unwrap()
//         .make_concealable("/nested/a")
//         .unwrap()
//         .header(std::iter::once(("kid".to_string(), serde_json::Value::String("test".to_string()))).collect())
//         .vct(
//             "https://www.ietf.org/archive/id/draft-terbu-oauth-sd-jwt-vc-00.html"
//                 .parse::<Url>()
//                 .unwrap(),
//         )
//         .iat(Timestamp::now_utc())
//         .iss("did:example:123".parse::<identity_core::common::Url>().unwrap())
//         .finish::<TestSigner>(&signer, "HS256")
//         .await
//         .unwrap()
//         .into();

//     let (sd_jwt_credential, _) = sd_jwt_credential
//         .clone()
//         .into_presentation(&Sha256Hasher::new())
//         .unwrap()
//         .conceal_all()
//         .disclose("/foo")
//         .unwrap()
//         .disclose("/name")
//         .unwrap()
//         .disclose("/nested/a")
//         .unwrap()
//         .finish()
//         .unwrap();

//     // We receive user input on which claims to disclose.
//     let user_input = vec!["foo", "name", "nested/a"];

//     let claims_to_conceal = sd_jwt_credential
//         .clone()
//         .into_disclosed_object(&Sha256Hasher::new())
//         .unwrap();

//     // let claims_to_conceal = sd_jwt_credential
//     //     // Get all the disclosures.
//     //     .disclosures()
//     //     .iter()
//     //     .filter_map(|disclosure| {
//     //         disclosure
//     //             // Get all the claim names.
//     //             .claim_name
//     //             .as_ref()
//     //             .and_then(|claim_name|
//     //                 // filter out the ones the user did NOT select.
//     //                 (!user_input.contains(&claim_name.as_str())).then(|| claim_name))
//     //     })
//     //     .collect::<Vec<_>>();

//     println!("Disclosures: {claims_to_conceal:?}");
//     // Disclosures: ["age", "a"]

//     // For each claim to conceal, call the conceal method on the presentation builder.
//     for claim_path in claims_to_conceal {
//         let path = format!("/{}", claim_path);
//         sd_jwt_presentation_builder = sd_jwt_presentation_builder.conceal(&path).unwrap();
//     }

//     let (sd_jwt_credential, _) = sd_jwt_presentation_builder.finish().unwrap();

//     println!("SD-JWT Credential: {}", sd_jwt_credential);
// }
