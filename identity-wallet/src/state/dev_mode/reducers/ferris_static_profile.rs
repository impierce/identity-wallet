use crate::{
    error::AppError::{self, *},
    persistence::ASSETS_DIR,
    state::{
        connections::{Connection, Connections},
        core_utils::{
            history_event::{EventType, HistoryCredential, HistoryEvent},
            IdentityManager,
        },
        credentials::VerifiableCredentialRecord,
        dev_mode::DevMode,
        profile_settings::{AppTheme, Profile},
        trust_list::TrustList,
        user_prompt::CurrentUserPrompt,
        AppState, SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS,
    },
    stronghold::StrongholdManager,
    subject::subject,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use jsonwebtoken::Algorithm;
use lazy_static::lazy_static;
use log::info;
use oid4vc::{oid4vc_core::Subject, oid4vc_manager::ProviderManager, oid4vci::Wallet};
use serde_json::json;
use std::{fs::File, io::Write, sync::Arc};

lazy_static! {
    pub static ref PERSONAL_INFORMATION: VerifiableCredentialRecord = {
        let mut record = VerifiableCredentialRecord::try_new(
            json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.Yl841U5BwWgctX5vF5Zi8SYCEQpxFqEs8_J8KrX9D_mOwL-IRmP64BeQZvnKeAdcOoYGn6CyciV51_amdPNQBw"),
            vec![]
        ).unwrap();
        record.display_credential.metadata.is_favorite = true;
        record.display_credential.display_name = "PersonalInformation".to_string();
        record
    };
    pub static ref DRIVERS_LICENSE_CREDENTIAL_PAYLOAD: String = URL_SAFE_NO_PAD.encode(include_bytes!("../../../../resources/ferris-drivers-license-payload.json"));
    pub static ref DRIVERS_LICENSE_CREDENTIAL: VerifiableCredentialRecord = {
        let mut record = VerifiableCredentialRecord::try_new(
            json!(format!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.{}.OZCcZt5JTJcBhoLPIyrQuvZuc2dnVN65f8GvKQ3earAzJEgGMA9ZjKRNHEjI73wLwvG5MJBN7Zs_rWiNLEZ5Dg", DRIVERS_LICENSE_CREDENTIAL_PAYLOAD.clone())),
            vec![]
        ).unwrap();
        record.display_credential.display_name = "DriverLicenseCredential".to_string();
        record
    };
    pub static ref OPEN_BADGE: VerifiableCredentialRecord = {
        let mut record = VerifiableCredentialRecord::try_new(
            json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiIsImtpZCI6ImRpZDpleGFtcGxlOjEyMyNrZXktMCJ9.eyJpc3MiOiJkaWQ6ZXhhbXBsZToxMjMiLCJzdWIiOiJkaWQ6ZXhhbXBsZTo0NTYiLCJleHAiOjk5OTk5OTk5OTksImlhdCI6MCwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiLCJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy9leGFtcGxlcy92MSIsImh0dHBzOi8vcHVybC5pbXNnbG9iYWwub3JnL3NwZWMvb2IvdjNwMC9jb250ZXh0LTMuMC4yLmpzb24iXSwiaWQiOiJodHRwOi8vZXhhbXBsZS5lZHUvY3JlZGVudGlhbHMvMzczMiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJPcGVuQmFkZ2VDcmVkZW50aWFsIl0sImlzc3VlciI6eyJpZCI6Imh0dHBzOi8vZXhhbXBsZS5lZHUvaXNzdWVycy81NjUwNDkiLCJ0eXBlIjpbIklzc3VlclByb2ZpbGUiXSwibmFtZSI6IkltcGllcmNlIERlbW8gUG9ydGFsIn0sImlzc3VhbmNlRGF0ZSI6IjIwMTAtMDEtMDFUMDA6MDA6MDBaIiwibmFtZSI6IlRlYW13b3JrIEJhZGdlIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZXhhbXBsZTo0NTYiLCJ0eXBlIjpbIkFjaGlldmVtZW50U3ViamVjdCJdLCJhY2hpZXZlbWVudCI6eyJpZCI6Imh0dHBzOi8vZXhhbXBsZS5jb20vYWNoaWV2ZW1lbnRzLzIxc3QtY2VudHVyeS1za2lsbHMvdGVhbXdvcmsiLCJ0eXBlIjpbIkFjaGlldmVtZW50Il0sImRlc2NyaXB0aW9uIjoiVGhpcyBiYWRnZSByZWNvZ25pemVzIHRoZSBkZXZlbG9wbWVudCBvZiB0aGUgY2FwYWNpdHkgdG8gY29sbGFib3JhdGUgd2l0aGluIGEgZ3JvdXAgZW52aXJvbm1lbnQuIiwiY3JpdGVyaWEiOnsibmFycmF0aXZlIjoiVGVhbSBtZW1iZXJzIGFyZSBub21pbmF0ZWQgZm9yIHRoaXMgYmFkZ2UgYnkgdGhlaXIgcGVlcnMgYW5kIHJlY29nbml6ZWQgdXBvbiByZXZpZXcgYnkgRXhhbXBsZSBDb3JwIG1hbmFnZW1lbnQuIn0sImFsaWdubWVudCI6W3sidHlwZSI6IkFsaWdubWVudCIsInRhcmdldFVybCI6Imh0dHBzOi8vZXhhbXBsZS5vcmciLCJ0YXJnZXROYW1lIjoiTGVhZGVyc2hpcCIsInRhcmdldERlc2NyaXB0aW9uIjoiSSBjYW4gZGV2ZWxvcCBhIHZpc2lvbiBhbmQgY29tbXVuaWNhdGUgaXQgY2xlYXJseSB0byBpbnNwaXJlIG90aGVycy4ifV19LCJyZXN1bHQiOlt7InR5cGUiOlsiUmVzdWx0Il0sImFsaWdubWVudCI6eyJ0eXBlIjoiQWxpZ25tZW50IiwidGFyZ2V0VXJsIjoiaHR0cHM6Ly9leGFtcGxlLm9yZyIsInRhcmdldE5hbWUiOiJMZWFkZXJzaGlwIiwidGFyZ2V0RGVzY3JpcHRpb24iOiJJIGNhbiBkZXZlbG9wIGEgdmlzaW9uIGFuZCBjb21tdW5pY2F0ZSBpdCBjbGVhcmx5IHRvIGluc3BpcmUgb3RoZXJzLiJ9LCJyZXN1bHREZXNjcmlwdGlvbiI6IlRoZSBncmFkZSBpcyBhIG51bWJlciBiZXR3ZWVuIDEgYW5kIDUsIHdoZXJlIDEgaXMgdGhlIGxvd2VzdCBhbmQgNSBpcyB0aGUgaGlnaGVzdC4iLCJ2YWx1ZSI6IjEifV0sIm5hbWUiOiJUZWFtd29yayJ9fX0.uNxCghKfdSVWFDmPME-j_ebmQczxxcHsF2YuqDZ9BOfegGAAJoQpOV-HEJA35afhG_wzdTaiIJ4uVLUGV_u4ig"),
            vec![]
        ).unwrap();
        record.display_credential.connection_id = Some("424313e61e35ca4eeca44aac85dc4764c32d7cf9def83ba15f428c308bf1d181".to_string());
        record.display_credential.display_name = "Teamwork".to_string();
        record
    };
    pub static ref EDU_BADGE: VerifiableCredentialRecord = {
        let mut record = VerifiableCredentialRecord::try_new(
            json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3F5WmpEZmhzeVo1YzZOdUpoYm9zV2tTajg2Mmp5V2lDQ0tIRHpOTkttOGtoI3o2TWtxeVpqRGZoc3laNWM2TnVKaGJvc1drU2o4NjJqeVdpQ0NLSER6Tk5LbThraCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWtxeVpqRGZoc3laNWM2TnVKaGJvc1drU2o4NjJqeVdpQ0NLSER6Tk5LbThraCIsInN1YiI6ImRpZDprZXk6ejZNa3VpUktxMWZLcnpBWGVTTmlHd3JwSlBQdWdZOEF4SllBNWNwQ3ZaQ1lCRDdCIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7ImlkIjoiaHR0cDovL2V4YW1wbGUuY29tL2NyZWRlbnRpYWxzLzM1MjciLCJuYW1lIjoiVGVhbXdvcmsgQmFkZ2UiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiT3BlbkJhZGdlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWtxeVpqRGZoc3laNWM2TnVKaGJvc1drU2o4NjJqeVdpQ0NLSER6Tk5LbThraCIsIkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly9wdXJsLmltc2dsb2JhbC5vcmcvc3BlYy9vYi92M3AwL2NvbnRleHQtMy4wLjIuanNvbiJdLCJpc3N1YW5jZURhdGUiOiIyMDEwLTAxLTAxVDAwOjAwOjAwWiIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rdWlSS3ExZktyekFYZVNOaUd3cnBKUFB1Z1k4QXhKWUE1Y3BDdlpDWUJEN0IiLCJ0eXBlIjpbIkFjaGlldmVtZW50U3ViamVjdCJdLCJhY2hpZXZlbWVudCI6eyJpZCI6Imh0dHBzOi8vZGVtby5lZHViYWRnZXMubmwvcHVibGljL2Fzc2VydGlvbnMvNnBFQi0tbi1Td2laUHRXWE1DQjJqUSIsIm5hbWUiOiJFZHViYWRnZSBhY2NvdW50IGNvbXBsZXRlIiwidHlwZSI6WyJBY2hpZXZlbWVudCJdLCJpbWFnZSI6eyJpZCI6Imh0dHBzOi8vYXBpLWRlbW8uZWR1YmFkZ2VzLm5sL21lZGlhL3VwbG9hZHMvYmFkZ2VzL2lzc3Vlcl9iYWRnZWNsYXNzXzU0ODUxN2FhLWNiYWItNGE3Yi1hOTcxLTU1Y2RjY2UwZTJhNS5wbmcifSwiY3JpdGVyaWEiOnsibmFycmF0aXZlIjoiVG8gcXVhbGlmeSBmb3IgdGhpcyBlZHViYWRnZTpcclxuXHJcbiogIHlvdSBzdWNjZXNzZnVsbHkgY3JlYXRlZCBhbiBlZHVJRCxcclxuKiB5b3Ugc3VjY2Vzc2Z1bGx5IGxpbmtlZCB5b3VyIGluc3RpdHV0aW9uIHRvIHlvdXIgZWR1SUQsXHJcbiogIHlvdSBjYW4gc3RvcmUgYW5kIG1hbmFnZSB0aGVtIHNhZmVseSBpbiB5b3VyIGJhY2twYWNrLiJ9LCJkZXNjcmlwdGlvbiI6IiMjIyBXZWxjb21lIHRvIGVkdWJhZGdlcy4gTGV0IHlvdXIgbGlmZSBsb25nIGxlYXJuaW5nIGJlZ2luISAjIyNcclxuXHJcbllvdSBhcmUgbm93IHJlYWR5IHRvIGNvbGxlY3QgYWxsIHlvdXIgZWR1YmFkZ2VzIGluIHlvdXIgYmFja3BhY2suIEluIHlvdXIgYmFja3BhY2sgeW91IGNhbiBzdG9yZSBhbmQgbWFuYWdlIHRoZW0gc2FmZWx5LlxyXG5cclxuU2hhcmUgdGhlbSBhbnl0aW1lIHlvdSBsaWtlIGFuZCB3aXRoIHdob20geW91IGxpa2UuXHJcblxyXG5FZHViYWRnZXMgYXJlIHZpc3VhbCByZXByZXNlbnRhdGlvbnMgb2YgeW91ciBrbm93bGVkZ2UsIHNraWxscyBhbmQgY29tcGV0ZW5jZXMuIn19fX0.z2rEuafNmbmY9sf5t4alnkZJeuNrNZrXXGovCc0J8NWdLyFU48mZfBffy6qltvtUOODOHSJnow1lAAFQ16W9Bw"),
            vec![]
        ).unwrap();
        // Connection does not exist (simulate deletion)
        record.display_credential.connection_id = Some("c5be6c4b46535a28cfc7edcccc79f9b041d0fedbbee4c1c3aeb234af750c7980".to_string());
        record.display_credential.display_name = "Edubadge account complete".to_string();
        record
    };
}

pub async fn load_ferris_profile() -> Result<AppState, AppError> {
    let mut state = AppState::default();

    let password = "sup3rSecr3t".to_string();

    let stronghold_manager = Arc::new(StrongholdManager::create(&password).map_err(StrongholdCreationError)?);
    let resolver = state.core_utils.resolver().await;

    let subject = subject(stronghold_manager.clone(), password, resolver).await;

    let profile = Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: AppTheme::System,
    };
    state.profile_settings.profile.replace(profile);

    let provider_manager = ProviderManager::new(
        subject.clone(),
        Vec::from(SUPPORTED_DID_METHODS),
        Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
    )
    .map_err(OID4VCProviderManagerError)?;
    let wallet: Wallet = Wallet::new(
        subject.clone(),
        Vec::from(SUPPORTED_DID_METHODS),
        Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
    )
    .map_err(OID4VCWalletError)?;
    let identity_manager = IdentityManager {
        subject: subject.clone(),
        provider_manager,
        wallet,
    };
    state.is_unlocked = true;

    state
        .core_utils
        .managers
        .lock()
        .await
        .identity_manager
        .replace(identity_manager);

    // Producing DIDs (`did:jwk`, `did:key`)
    let did_jwk = subject
        // TODO: make distinction between keys using the same DID Method but different algorithms.
        .identifier("did:jwk", Algorithm::EdDSA)
        .await
        .map_err(|e| Error(e.to_string()))?;
    state.dids.insert("did:jwk".to_string(), did_jwk);

    let did_key = subject
        // TODO: make distinction between keys using the same DID Method but different algorithms.
        .identifier("did:key", Algorithm::EdDSA)
        .await
        .map_err(|e| Error(e.to_string()))?;
    state.dids.insert("did:key".to_string(), did_key);

    vec![
        PERSONAL_INFORMATION.clone(),
        DRIVERS_LICENSE_CREDENTIAL.clone(),
        OPEN_BADGE.clone(),
        EDU_BADGE.clone(),
    ]
    .into_iter()
    .for_each(|verifiable_credential_record| {
        info!("inserting credential into stronghold");
        stronghold_manager
            .insert(
                verifiable_credential_record.display_credential.id.parse().unwrap(),
                json!(verifiable_credential_record).to_string().as_bytes().to_vec(),
            )
            .unwrap();
    });

    info!("loading credentials from stronghold");
    state.credentials = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .into_iter()
        .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
        .collect();

    load_predefined_images().await?;

    state
        .core_utils
        .managers
        .lock()
        .await
        .stronghold_manager
        .replace(stronghold_manager);

    info!("loading journey from string");
    let journey_definition = r#"
        {
            "title": "NGDIL Demo",
            "description": "Set up your profile and get started with your UniMe app.",
            "description_short": "Complete your first steps",
            "creator": "UniMe",
            "goals": [
                {
                    "id": 0,
                    "label": "Set up your profile",
                    "description": "Make your UniMe app your own by choosing a profile name and profile picture.",
                    "icon": "UserCirclePlus",
                    "faqs": [
                        { "id": 0, "title": "Will this information be shared?", "content": "No. Your profile information will never leave your device." }
                    ],
                    "prerequisites": []
                },
                {
                    "id": 1,
                    "label": "Receive your first credential",
                    "type": "hold-credential",
                    "description": "Receive your first credential from a trusted source.",
                    "icon": "FileArrowDown",
                    "faqs": [
                        { "id": 0, "title": "What is a credential?", "content": "A credential is like a digital proof that verifies something about you, such as your age, education, or memberships." }
                    ],
                    "prerequisites": []
                },
                {
                    "id": 2,
                    "label": "Use a credential to sign in to a website",
                    "type": "login",
                    "icon": "Key",
                    "faqs": [],
                    "prerequisites": []
                }
            ]
        }"#;
    // let journey_definition = std::fs::read_to_string("resources/ngdil.json")?;
    let onboarding_journey: serde_json::Value = serde_json::from_str(journey_definition).unwrap();

    if std::env::var("FEATURE_USER_JOURNEYS_ENABLED")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false)
    {
        state.user_journey = Some(onboarding_journey);
    }

    state.connections = Connections(vec![
        Connection {
            id: "352eaaf022a32cc315b4ac46bfa14bcad91e901bdf3aff3925d3a5a4c13bd611".to_string(),
            name: "NGDIL Demo".to_string(),
            url: "api.ngdil-demo.tanglelabs.io".to_string(),
            did: None,
            verified: false,
            first_interacted: "2023-09-11T19:53:53.937981+00:00".to_string(),
            last_interacted: "2023-09-11T19:53:53.937981+00:00".to_string(),
        },
        Connection {
            id: "424313e61e35ca4eeca44aac85dc4764c32d7cf9def83ba15f428c308bf1d181".to_string(),
            name: "Impierce Demo Portal".to_string(),
            url: "https://demo.impierce.com".to_string(),
            did: Some("did:iota:rms:0x42ad588322e58b3c07aa39e4948d021ee17ecb5747915e9e1f35f028d7ecaf90".to_string()),
            verified: true,
            first_interacted: "2024-01-09T07:36:41.382948+00:00".to_string(),
            last_interacted: "2024-01-09T07:36:41.382948+00:00".to_string(),
        },
        Connection {
            id: "e36236d8d7117ed6c6a5d4e99167a2ee1ccb455e75d5b71cee50b08adcf11ba1".to_string(),
            name: "my-webshop.com".to_string(),
            url: "https://shop.example.com".to_string(),
            did: Some("did:key:z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL".to_string()),
            verified: false,
            first_interacted: "2022-02-03T12:33:54.191824+00:00".to_string(),
            last_interacted: "2023-11-13T19:26:40.049239+00:00".to_string(),
        },
        Connection {
            id: "a81a51b8ad26bdd333abd791a112bf0e0823d559cadc580218a240238a86c292".to_string(),
            name: "IOTA".to_string(),
            url: "https://www.iota.org".to_string(),
            did: Some("did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe".to_string()),
            verified: true,
            first_interacted: "2024-01-09T08:45:44.217Z".to_string(),
            last_interacted: "2024-01-09T08:45:44.217Z".to_string(),
        },
    ]);

    state.history = vec![
        HistoryEvent {
            connection_id: "424313e61e35ca4eeca44aac85dc4764c32d7cf9def83ba15f428c308bf1d181".to_string(),
            connection_name: "Impierce Demo Portal".to_string(),
            event_type: EventType::ConnectionAdded,
            date: (chrono::Utc::now() - chrono::Duration::try_days(2).unwrap()).to_rfc3339(),
            credentials: vec![],
        },
        HistoryEvent {
            connection_id: "424313e61e35ca4eeca44aac85dc4764c32d7cf9def83ba15f428c308bf1d181".to_string(),
            connection_name: "Impierce Demo Portal".to_string(),
            event_type: EventType::CredentialsAdded,
            date: (chrono::Utc::now() - chrono::Duration::try_hours(3).unwrap()).to_rfc3339(),
            credentials: vec![
                HistoryCredential {
                    title: "PersonalInformation".to_string(),
                    issuer_name: "Impierce Demo Portal".to_string(),
                    id: "62326539-3634-6536-6131-323731373463".to_string(),
                },
                HistoryCredential {
                    title: "Teamwork".to_string(),
                    issuer_name: "Impierce Demo Portal".to_string(),
                    id: "65323136-6535-3737-6463-386531323361".to_string(),
                },
            ],
        },
        HistoryEvent {
            connection_id: "424313e61e35ca4eeca44aac85dc4764c32d7cf9def83ba15f428c308bf1d181".to_string(),
            connection_name: "Impierce Demo Portal".to_string(),
            event_type: EventType::CredentialsShared,
            date: (chrono::Utc::now() - chrono::Duration::try_minutes(5).unwrap()).to_rfc3339(),
            credentials: vec![HistoryCredential {
                title: "Teamwork".to_string(),
                issuer_name: "Impierce Demo Portal".to_string(),
                id: "65323136-6535-3737-6463-386531323361".to_string(),
            }],
        },
    ];

    state.search_results.recent_credentials = vec![
        DRIVERS_LICENSE_CREDENTIAL.display_credential.id.clone(),
        OPEN_BADGE.display_credential.id.clone(),
    ];

    // Import trusted domains
    let default_trust_list: TrustList =
        serde_json::from_str(include_str!("../../../../resources/default_trust_list.json")).unwrap();

    state.trust_lists.insert(default_trust_list);

    state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    state.show_dev_mode_setting = true;
    state.dev_mode = DevMode::OnWithAutologin;

    Ok(state)
}

async fn load_predefined_images() -> Result<(), AppError> {
    // Issuers
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/issuer-university.png"),
        "university.png",
    )?;

    // Connections
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/impierce_white.png"),
        "424313e61e35ca4eeca44aac85dc4764c32d7cf9def83ba15f428c308bf1d181.png",
    )?;
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/iota-icon-dark.svg"),
        "a81a51b8ad26bdd333abd791a112bf0e0823d559cadc580218a240238a86c292.svg",
    )?;
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/kw1c-white.png"),
        "kw1c.png",
    )?;
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/ngdil.svg"),
        "352eaaf022a32cc315b4ac46bfa14bcad91e901bdf3aff3925d3a5a4c13bd611.svg",
    )?;

    // Credentials
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/cuddlyferris.svg"),
        &format!("{}.svg", PERSONAL_INFORMATION.display_credential.id),
    )?;
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/credential-driver-license.png"),
        &format!("{}.png", DRIVERS_LICENSE_CREDENTIAL.display_credential.id),
    )?;

    // Badges
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/badge-university-green.png"),
        &format!("{}.png", OPEN_BADGE.display_credential.id),
    )?;
    write_bytes_to_file(
        include_bytes!("../../../../resources/images/edubadge-1.png"),
        &format!("{}.png", EDU_BADGE.display_credential.id),
    )?;

    Ok(())
}

/// Helper function for load_predefined_images()
fn write_bytes_to_file(bytes: &[u8], file_name: &str) -> Result<File, std::io::Error> {
    let mut file = File::create(ASSETS_DIR.lock().unwrap().as_path().to_owned().join(file_name))?;
    file.write_all(bytes)?;
    Ok(file)
}
