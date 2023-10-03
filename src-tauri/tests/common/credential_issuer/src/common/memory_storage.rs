use std::fs::File;

use jsonwebtoken::{Algorithm, Header};
use lazy_static::lazy_static;
use oid4vc_core::{authentication::subject::SigningSubject, generate_authorization_code, jwt, JsonValue};
use oid4vc_manager::storage::Storage;
use oid4vci::{
    authorization_response::AuthorizationResponse,
    credential_format_profiles::{
        w3c_verifiable_credentials::jwt_vc_json::JwtVcJson, Credential, CredentialFormatCollection, CredentialFormats,
    },
    credential_issuer::credentials_supported::CredentialsSupportedObject,
    credential_offer::{AuthorizationCode, PreAuthorizedCode},
    credential_response::{CredentialResponse, CredentialResponseType},
    token_request::TokenRequest,
    token_response::TokenResponse,
    VerifiableCredentialJwt,
};
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

lazy_static! {
    pub static ref CODE: String = generate_authorization_code(16);
    pub static ref PRE_AUTHORIZED_CODE: PreAuthorizedCode = PreAuthorizedCode {
        pre_authorized_code: generate_authorization_code(16),
        ..Default::default()
    };
    pub static ref USER_PIN: String = "493536".to_string();
    pub static ref ACCESS_TOKEN: String = "czZCaGRSa3F0MzpnWDFmQmF0M2JW".to_string();
    pub static ref C_NONCE: String = "tZignsnFbp".to_string();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryStorage;

impl<CFC: CredentialFormatCollection + DeserializeOwned> Storage<CFC> for MemoryStorage {
    fn get_credentials_supported(&self) -> Vec<CredentialsSupportedObject<CFC>> {
        vec![
            serde_json::from_reader(
                File::open("./src/common/credentials_supported_objects/university_degree.json").unwrap(),
            )
            .unwrap(),
            serde_json::from_reader(
                File::open("./src/common/credentials_supported_objects/driver_license.json").unwrap(),
            )
            .unwrap(),
        ]
    }

    fn get_authorization_code(&self) -> Option<AuthorizationCode> {
        None
    }

    fn get_authorization_response(&self) -> Option<AuthorizationResponse> {
        Some(AuthorizationResponse {
            code: CODE.clone(),
            state: None,
        })
    }

    fn get_pre_authorized_code(&self) -> Option<PreAuthorizedCode> {
        Some(PRE_AUTHORIZED_CODE.clone())
    }

    fn get_token_response(&self, token_request: TokenRequest) -> Option<TokenResponse> {
        match token_request {
            TokenRequest::AuthorizationCode { code, .. } => code == CODE.clone(),
            TokenRequest::PreAuthorizedCode {
                pre_authorized_code, ..
            } => pre_authorized_code == PRE_AUTHORIZED_CODE.pre_authorized_code,
        }
        .then_some(TokenResponse {
            // TODO: dynamically create this.
            access_token: ACCESS_TOKEN.clone(),
            token_type: "bearer".to_string(),
            expires_in: Some(86400),
            refresh_token: None,
            scope: None,
            c_nonce: Some(C_NONCE.clone()),
            c_nonce_expires_in: Some(86400),
        })
    }

    fn get_credential_response(
        &self,
        access_token: String,
        subject_did: Url,
        issuer_did: Url,
        credential_format: CFC,
        signer: SigningSubject,
    ) -> Option<CredentialResponse> {
        let type_ = match serde_json::from_value::<CredentialFormats>(serde_json::to_value(credential_format).unwrap())
            .unwrap()
        {
            CredentialFormats::JwtVcJson(credential) => credential.parameters.credential_definition.type_,
            _ => unreachable!("Credential format not supported"),
        };

        let credential_json = match &type_[..] {
            [_, b] if b == "UniversityDegreeCredential" => {
                File::open("./tests/common/credentials/university_degree.json").unwrap()
            }
            [_, b] if b == "DriverLicenseCredential" => {
                File::open("./tests/common/credentials/driver_license.json").unwrap()
            }
            _ => unreachable!(),
        };

        let mut verifiable_credential: JsonValue = serde_json::from_reader(credential_json).unwrap();
        verifiable_credential["issuer"] = json!(issuer_did);
        verifiable_credential["credentialSubject"]["id"] = json!(subject_did);

        (access_token == ACCESS_TOKEN.clone()).then_some(CredentialResponse {
            credential: CredentialResponseType::Immediate(CredentialFormats::JwtVcJson(Credential {
                format: JwtVcJson,
                credential: serde_json::to_value(
                    jwt::encode(
                        signer.clone(),
                        Header::new(Algorithm::EdDSA),
                        VerifiableCredentialJwt::builder()
                            .sub(subject_did.clone())
                            .iss(issuer_did.clone())
                            .iat(0)
                            .exp(9999999999i64)
                            .verifiable_credential(verifiable_credential)
                            .build()
                            .ok(),
                    )
                    .ok(),
                )
                .unwrap(),
            })),
            c_nonce: Some(C_NONCE.clone()),
            c_nonce_expires_in: Some(86400),
        })
    }

    fn get_state(&self) -> Option<String> {
        None
    }

    fn set_state(&mut self, _state: String) {}
}
