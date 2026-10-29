use std::collections::HashMap;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use identity_wallet::{
    oid4vci::credential_format_profiles::CredentialFormats,
    state::credentials::{
        self_issued_migration::migrate_self_issued_credentials, CredentialMetadata, DisplayCredential,
        VerifiableCredentialRecord,
    },
    stronghold::StrongholdManager,
};
use serde_json::{json, Value};

use super::profile_fixture::{
    get_state, handle_actions, setup_profile_fixture, unlock_storage, PROFILE_V0_18_0, PROFILE_V0_18_0_PASSWORD,
};
use crate::common::test_managers;

const ISSUER_DID: &str = "did:example:ebfeb1f712ebc6f1c276e12ec21";
const LEGACY_VCT: &str = "https://www.ietf.org/archive/id/draft-terbu-oauth-sd-jwt-vc-00.html";

fn encode(value: Value) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_vec(&value).unwrap())
}

fn decode(segment: &str) -> Value {
    serde_json::from_slice(&URL_SAFE_NO_PAD.decode(segment).unwrap()).unwrap()
}

/// A record as UniMe up to 0.18 stored it after self-issuing, with nothing concealed. The signature is not checked by
/// the migration.
fn legacy_record(
    id: &str,
    issuer: &str,
    vct: &str,
    credential_type: &str,
    credential_subject: Value,
) -> VerifiableCredentialRecord {
    let header = json!({ "typ": "dc+sd-jwt", "alg": "EdDSA", "kid": format!("{issuer}#0") });
    let data = json!({
        "type": ["VerifiableCredential", credential_type],
        "issuanceDate": "2026-10-01T10:00:00.000Z",
        "name": "My Profile",
        "credentialSubject": credential_subject,
        "issuer": issuer,
    });
    let mut payload = data.clone();
    payload["vct"] = json!(vct);
    payload["iat"] = json!(1790848800);
    payload["iss"] = json!(issuer);
    payload["cnf"] = json!({ "kid": format!("{issuer}#0") });

    VerifiableCredentialRecord {
        verifiable_credential: json!(format!("{}.{}.c2lnbmF0dXJl~", encode(header), encode(payload))),
        display_credential: DisplayCredential {
            id: id.to_string(),
            format: CredentialFormats::DcSdJwt(()),
            issuer_name: "Ferris".to_string(),
            data,
            metadata: CredentialMetadata {
                is_favorite: true,
                date_added: "2026-10-01T10:00:00Z".to_string(),
                date_issued: "2026-10-01T10:00:00Z".to_string(),
                icon: Some("IdentificationBadge".to_string()),
                ..Default::default()
            },
            display_name: "My Profile".to_string(),
            ..Default::default()
        },
    }
}

#[tokio::test]
#[serial_test::serial]
async fn test_migrate_self_issued_credentials() {
    let profile_id = "2db2ea17-337b-4ebb-bd56-d89229a9c9bd";
    let other_id = "34651718-4ba2-408e-b8c0-a78b1ab6c56e";
    let credential_subject = json!({
        "given_name": "Ferris",
        "family_name": "Rustacean",
        "birth_date": "2023-04-01T00:00:00.000Z",
        "birth_place": "Atlantic Ocean",
        "nationality": ["NL"],
    });
    let legacy_profile = legacy_record(
        profile_id,
        ISSUER_DID,
        LEGACY_VCT,
        "NaturalPersonCredential",
        credential_subject.clone(),
    );
    // A `dc+sd-jwt` that was not self-issued by UniMe stays untouched.
    let other = legacy_record(
        other_id,
        ISSUER_DID,
        "https://example.com/vct",
        "NaturalPersonCredential",
        credential_subject.clone(),
    );

    let managers = test_managers(vec![legacy_profile.clone(), other.clone()]).await;
    let managers = managers.lock().await;
    let stronghold_manager = managers.stronghold_manager.as_ref().unwrap();
    let subject = managers.identity_manager.as_ref().unwrap().subject.clone();
    let dids = HashMap::from([("did:jwk".to_string(), ISSUER_DID.to_string())]);

    migrate_self_issued_credentials(stronghold_manager, subject.clone(), &dids).await;

    let records = stronghold_manager.values().unwrap().unwrap();
    let record = |id: &str| records.iter().find(|r| r.display_credential.id == id).unwrap().clone();

    let migrated = record(profile_id);
    assert_eq!(migrated.display_credential.format, CredentialFormats::VcSdJwt(()));
    assert_eq!(migrated.display_credential.display_name, "My Profile");
    assert_eq!(migrated.display_credential.issuer_name, "Ferris");
    let metadata = &migrated.display_credential.metadata;
    assert!(metadata.is_favorite);
    assert_eq!(metadata.icon.as_deref(), Some("IdentificationBadge"));
    assert_eq!(metadata.date_added, "2026-10-01T10:00:00Z");
    assert_eq!(metadata.date_issued, "2026-10-01T10:00:00Z");

    let mut expected_subject = credential_subject.clone();
    expected_subject["id"] = json!(ISSUER_DID);
    assert_eq!(
        migrated.display_credential.data,
        json!({
            "name": "My Profile",
            "type": ["VerifiableCredential", "NaturalPersonCredential"],
            "issuer": ISSUER_DID,
            "credentialSubject": expected_subject,
        })
    );

    let sd_jwt = migrated.verifiable_credential.as_str().unwrap();
    let mut parts = sd_jwt.split('~');
    let mut jwt = parts.next().unwrap().split('.');
    assert_eq!(decode(jwt.next().unwrap())["typ"], "vc+sd-jwt");
    let payload = decode(jwt.next().unwrap());
    assert_eq!(
        payload["type"],
        json!(["VerifiableCredential", "NaturalPersonCredential"])
    );
    assert_eq!(payload["issuer"], ISSUER_DID);
    assert_eq!(payload["validFrom"], "2026-10-01T10:00:00Z");
    assert!(payload.get("vct").is_none());
    // Every claim is a disclosure; only the holder `id` stays visible.
    assert_eq!(payload["credentialSubject"]["_sd"].as_array().unwrap().len(), 5);
    let mut visible_claims: Vec<_> = payload["credentialSubject"].as_object().unwrap().keys().collect();
    visible_claims.sort();
    assert_eq!(visible_claims, ["_sd", "id"]);
    let mut disclosed: Vec<String> = parts
        .filter(|disclosure| !disclosure.is_empty())
        .map(|disclosure| decode(disclosure)[1].as_str().unwrap().to_string())
        .collect();
    disclosed.sort();
    assert_eq!(
        disclosed,
        ["birth_date", "birth_place", "family_name", "given_name", "nationality"]
    );

    assert_eq!(record(other_id), other);
    assert_eq!(record(other_id).verifiable_credential, other.verifiable_credential);

    // Unlocking again finds nothing to migrate.
    migrate_self_issued_credentials(stronghold_manager, subject, &dids).await;
    let records = stronghold_manager.values().unwrap().unwrap();
    let again = records.iter().find(|r| r.display_credential.id == profile_id).unwrap();
    assert_eq!(again.verifiable_credential, migrated.verifiable_credential);
}

#[tokio::test]
#[serial_test::serial]
async fn test_unlock_migrates_self_issued_credentials() {
    let data_dir = setup_profile_fixture(PROFILE_V0_18_0);
    let state_file = std::fs::File::open(data_dir.path().join("state.json")).unwrap();
    let state_json: Value = serde_json::from_reader(state_file).unwrap();
    let holder_did = state_json["dids"]["did:jwk"].as_str().unwrap();

    let profile_id = "2db2ea17-337b-4ebb-bd56-d89229a9c9bd";
    let legacy_profile = legacy_record(
        profile_id,
        holder_did,
        LEGACY_VCT,
        "NaturalPersonCredential",
        json!({ "given_name": "Ferris", "family_name": "Rustacean" }),
    );
    let stronghold_manager = StrongholdManager::load(PROFILE_V0_18_0_PASSWORD).unwrap();
    stronghold_manager
        .insert(
            profile_id.parse().unwrap(),
            json!(legacy_profile).to_string().into_bytes(),
        )
        .unwrap();
    drop(stronghold_manager);

    let state = handle_actions(vec![get_state(), unlock_storage(PROFILE_V0_18_0_PASSWORD)]).await;

    assert!(state.is_unlocked);
    assert_eq!(state.credentials.len(), 6);
    let migrated = state
        .credentials
        .iter()
        .find(|credential| credential.id == profile_id)
        .unwrap();
    assert_eq!(migrated.format, CredentialFormats::VcSdJwt(()));
    assert_eq!(migrated.display_name, "My Profile");
    assert!(migrated.metadata.is_favorite);
    assert_eq!(migrated.data["credentialSubject"]["id"], holder_did);
}
