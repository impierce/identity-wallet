use crate::error::AppError;
use oid4vc::oid4vc_core::claim_path_pointer::{ClaimPathElement, ClaimPathPointer};
use oid4vc::oid4vp::dcql::dcql_query::{CredentialQuery, Format};
use sd_jwt::{SdJwt, SdJwtPresentationBuilder, Sha256Hasher};
use serde_json::Value;

/// Whether the claims of a credential in this format can be disclosed one by one.
pub fn supports_selective_disclosure(format: &Format) -> bool {
    matches!(format, Format::DcSdJwt | Format::VcSdJwt)
}

/// The claim sets of a credential query as indices into its `claims`. Empty when every claim is requested.
pub fn claim_sets(credential_query: &CredentialQuery) -> Vec<Vec<usize>> {
    let claims = credential_query.claims.as_deref().unwrap_or_default();
    credential_query
        .claim_sets
        .iter()
        .flatten()
        .map(|claim_set| {
            claim_set
                .iter()
                .filter_map(|claim_id| claims.iter().position(|claim| claim.id.as_ref() == Some(claim_id)))
                .collect()
        })
        .collect()
}

/// The indices of the requested claims to disclose: the holder's choice when it satisfies the query, otherwise
/// all claims, or the first claim set that the credential can satisfy.
pub fn claims_to_disclose(
    credential_query: &CredentialQuery,
    credential_json: &Value,
    chosen: Option<&[usize]>,
) -> Result<Vec<usize>, AppError> {
    let claims = credential_query.claims.as_deref().unwrap_or_default();
    let present = |index: &usize| {
        claims
            .get(*index)
            .is_some_and(|claim| !claim.path.get_values_from_json(credential_json).is_empty())
    };
    let claim_sets = claim_sets(credential_query);

    match chosen {
        Some(chosen) => {
            let satisfied = if claim_sets.is_empty() {
                (0..claims.len()).all(|index| chosen.contains(&index))
            } else {
                claim_sets
                    .iter()
                    .any(|claim_set| claim_set.iter().all(|index| chosen.contains(index) && present(index)))
            };
            if !satisfied || chosen.iter().any(|index| *index >= claims.len()) {
                return Err(AppError::Error(format!(
                    "The chosen claims do not satisfy credential query {}",
                    credential_query.id.as_ref()
                )));
            }
            Ok(chosen.to_vec())
        }
        None if claim_sets.is_empty() => Ok((0..claims.len()).collect()),
        None => claim_sets
            .into_iter()
            .find(|claim_set| claim_set.iter().all(present))
            .ok_or_else(|| {
                AppError::Error(format!(
                    "No claim set of credential query {} can be satisfied",
                    credential_query.id.as_ref()
                ))
            }),
    }
}

/// The paths in `value` that `path` points to, where `null` matches every element of an array.
pub fn concrete_paths(path: &ClaimPathPointer, value: &Value) -> Vec<Vec<String>> {
    let mut selections: Vec<(Vec<String>, &Value)> = vec![(vec![], value)];
    for element in path.as_ref() {
        selections = selections
            .into_iter()
            .flat_map(|(prefix, value)| {
                let children: Vec<(String, &Value)> = match (element, value) {
                    (ClaimPathElement::String(key), Value::Object(object)) => {
                        object.get(key).map(|child| (key.clone(), child)).into_iter().collect()
                    }
                    (ClaimPathElement::Integer(index), Value::Array(array)) => array
                        .get(*index as usize)
                        .map(|child| (index.to_string(), child))
                        .into_iter()
                        .collect(),
                    (ClaimPathElement::Null, Value::Array(array)) => array
                        .iter()
                        .enumerate()
                        .map(|(index, child)| (index.to_string(), child))
                        .collect(),
                    _ => vec![],
                };
                children.into_iter().map(move |(segment, child)| {
                    let mut path = prefix.clone();
                    path.push(segment);
                    (path, child)
                })
            })
            .collect();
    }
    selections.into_iter().map(|(path, _)| path).collect()
}

fn with_descendants(path: Vec<String>, value: &Value, paths: &mut Vec<Vec<String>>) {
    let children: Vec<(String, &Value)> = match value {
        Value::Object(object) => object.iter().map(|(key, child)| (key.clone(), child)).collect(),
        Value::Array(array) => array
            .iter()
            .enumerate()
            .map(|(index, child)| (index.to_string(), child))
            .collect(),
        _ => vec![],
    };
    for (segment, child) in children {
        let mut child_path = path.clone();
        child_path.push(segment);
        with_descendants(child_path, child, paths);
    }
    paths.push(path);
}

fn pointer(path: &[String]) -> String {
    format!("/{}", path.join("/"))
}

fn decoded_object(sd_jwt: &SdJwt) -> Result<Value, AppError> {
    sd_jwt
        .clone()
        .into_disclosed_object(&Sha256Hasher::new())
        .map(Value::Object)
        .map_err(|err| AppError::Error(format!("Failed to decode SD-JWT: {err}")))
}

fn presentation_builder(sd_jwt: SdJwt) -> Result<SdJwtPresentationBuilder, AppError> {
    sd_jwt
        .into_presentation(&Sha256Hasher::new())
        .map_err(|err| AppError::Error(format!("Failed to create SD-JWT presentation: {err}")))
}

/// For each claim path, whether the holder can leave it out of a presentation of `sd_jwt` while still disclosing all
/// other claims. A claim inside a disclosure that also holds other claims cannot be left out on its own.
pub fn concealable_claims(sd_jwt: &str, claim_paths: &[&ClaimPathPointer]) -> Vec<bool> {
    let Ok(sd_jwt) = sd_jwt.parse::<SdJwt>() else {
        return vec![false; claim_paths.len()];
    };
    let (Ok(decoded), Ok(builder)) = (decoded_object(&sd_jwt), presentation_builder(sd_jwt)) else {
        return vec![false; claim_paths.len()];
    };
    let value_counts = |value: &Value| -> Vec<usize> {
        claim_paths
            .iter()
            .map(|claim_path| claim_path.get_values_from_json(value).len())
            .collect()
    };
    let before = value_counts(&decoded);

    (0..claim_paths.len())
        .map(|index| {
            let paths = concrete_paths(claim_paths[index], &decoded);
            if paths.is_empty() {
                return false;
            }
            let concealed = paths
                .iter()
                .try_fold(builder.clone(), |builder, path| builder.conceal(&pointer(path)));
            let Ok(after) = concealed
                .map(|builder| builder.finish().0)
                .and_then(|sd_jwt| sd_jwt.into_disclosed_object(&Sha256Hasher::new()).map(Value::Object))
            else {
                return false;
            };
            let after = value_counts(&after);
            after[index] == 0 && (0..claim_paths.len()).all(|other| other == index || after[other] == before[other])
        })
        .collect()
}

/// Presents `sd_jwt` with only the disclosures needed to reveal the claims at `claim_paths`. For a VCDM 2.0 SD-JWT
/// everything outside `credentialSubject`, and the subject's `id`, stays disclosed.
pub fn conceal_unrequested_claims(
    sd_jwt: &str,
    format: &Format,
    claim_paths: &[&ClaimPathPointer],
) -> Result<String, AppError> {
    let sd_jwt = sd_jwt
        .parse::<SdJwt>()
        .map_err(|err| AppError::Error(format!("Failed to parse SD-JWT: {err}")))?;
    let decoded = decoded_object(&sd_jwt)?;

    let mut kept: Vec<Vec<String>> = claim_paths
        .iter()
        .flat_map(|claim_path| concrete_paths(claim_path, &decoded))
        .collect();
    if matches!(format, Format::VcSdJwt) {
        if let Value::Object(object) = &decoded {
            kept.extend(
                object
                    .keys()
                    .filter(|key| *key != "credentialSubject")
                    .map(|key| vec![key.clone()]),
            );
        }
        kept.push(vec!["credentialSubject".to_string(), "id".to_string()]);
    }

    let mut disclosed = vec![];
    for path in kept {
        let mut current = &decoded;
        for segment in &path {
            current = match current {
                Value::Object(object) => object.get(segment).unwrap_or(&Value::Null),
                Value::Array(array) => segment
                    .parse::<usize>()
                    .ok()
                    .and_then(|index| array.get(index))
                    .unwrap_or(&Value::Null),
                _ => &Value::Null,
            };
        }
        with_descendants(path, current, &mut disclosed);
    }

    let mut builder = presentation_builder(sd_jwt)?.conceal_all();
    for path in disclosed {
        // A path that holds no disclosable value is disclosed already.
        if let Ok(next) = builder.clone().disclose(&pointer(&path)) {
            builder = next;
        }
    }
    let (sd_jwt, _) = builder.finish();
    Ok(sd_jwt.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn path(value: Value) -> ClaimPathPointer {
        serde_json::from_value(value).unwrap()
    }

    fn query(value: Value) -> CredentialQuery {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn concrete_paths_expand_array_wildcards() {
        let value = json!({ "nationalities": ["NL", "DE"], "address": { "city": "Utrecht" } });

        assert_eq!(
            concrete_paths(&path(json!(["nationalities", null])), &value),
            vec![vec!["nationalities", "0"], vec!["nationalities", "1"]]
        );
        assert_eq!(
            concrete_paths(&path(json!(["address", "city"])), &value),
            vec![vec!["address", "city"]]
        );
        assert!(concrete_paths(&path(json!(["address", "street"])), &value).is_empty());
    }

    #[test]
    fn claims_to_disclose_follows_claim_sets() {
        let credential_query = query(json!({
            "id": "pid",
            "format": "dc+sd-jwt",
            "meta": { "vct_values": ["pid"] },
            "claims": [
                { "id": "given_name", "path": ["given_name"] },
                { "id": "nationality", "path": ["nationality"] },
                { "id": "address", "path": ["address"] }
            ],
            "claim_sets": [["given_name", "address"], ["given_name", "nationality"]]
        }));
        let credential = json!({ "given_name": "Anna", "nationality": "NL" });

        assert_eq!(
            claims_to_disclose(&credential_query, &credential, None).unwrap(),
            vec![0, 1]
        );
        assert_eq!(
            claims_to_disclose(&credential_query, &credential, Some(&[0, 1])).unwrap(),
            vec![0, 1]
        );
        assert!(claims_to_disclose(&credential_query, &credential, Some(&[0])).is_err());
        assert!(claims_to_disclose(&credential_query, &credential, Some(&[0, 2])).is_err());
    }

    #[test]
    fn claims_to_disclose_requires_all_claims_without_claim_sets() {
        let credential_query = query(json!({
            "id": "pid",
            "format": "dc+sd-jwt",
            "meta": { "vct_values": ["pid"] },
            "claims": [{ "path": ["given_name"] }, { "path": ["family_name"] }]
        }));
        let credential = json!({ "given_name": "Anna", "family_name": "de Vries" });

        assert_eq!(
            claims_to_disclose(&credential_query, &credential, None).unwrap(),
            vec![0, 1]
        );
        assert!(claims_to_disclose(&credential_query, &credential, Some(&[1])).is_err());
    }

    const DC_SD_JWT: &str = "eyJ0eXAiOiJkYytzZC1qd3QiLCJraWQiOiJkaWQ6d2ViOmxvY2FsaG9zdCUzQTMwMzMjQXM1cjRVeG9fWjREV19XLW9BUVNURTRLT1hhMnVWeUJRYUNna3VTUkhwWSIsImFsZyI6IkVTMjU2In0.eyJ2Y3QiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvdmN0L1UwUXRTbGRVLzAiLCJfc2QiOlsiNzZ0bGNVNi1kMlNaQUhHV3ZrVk5aV2hweS11QUFzczBuemdCdmx0X19QRSIsIlZsSVdHMVJNai1vckpSeTZIWUJqaXRLaDVUOGxLTUU5UkVlbmFUN2htRzgiLCJta19kazVuc0pXQ2hZYWpocW4yT2N2cXNYazlRdTdNcHdXeEpnTlBQQVp3Il0sImlzcyI6ImRpZDp3ZWI6bG9jYWxob3N0JTNBMzAzMyIsIm5iZiI6MTc3MDE0OTU2NSwiaWF0IjoxNzcwMTQ5NTY1LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMSIsImlkeCI6ODEyNH19LCJfc2RfYWxnIjoic2hhLTI1NiIsImNuZiI6eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVdHMUpPRmc1ZVhCU05WVTJlRE5ZTm5ab1NuUk9hM1ZwVkVWTk4zRklkbk5wVjBoa04zSkdSbTFxVVNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1aME4yUnZja1JLZDFwTE5GaG5hVGRsVDFJNGNXRkZMWE4yTUhoQ01qaFJOWFZLYTNJdFEzZEJNalFpTENKNUlqb2lSWFI0T1RWM1IxcFZZekE1VjJzMVJXTnZNR1JHZVhreGVXVkVlbUl6TFdsS1VuRTNOMDF4T1U5YVl5SjkjMCJ9fQ.8ZY3p-eeEtlFQjfEYBcRGjxHeq2DpqIHNgD4WKPLcF8qmK8bWOJxYKaw1cfrptx6WsNs5cqhZgtlztBV9HckMw~WyJaam1obE9QQzhpZWREOHJxTXItckxvVzFvRXFDSUR4SHFoQmJkRlpKIiwiZmlyc3RfbmFtZSIsIkZlcnJpcyJd~WyIyUkhteHh0cXpHNHRCeU1DdG9LWXhTRXdEaE92MTd0Z1FaR2hNVEUwIiwibGFzdF9uYW1lIiwiQ3JhYm1hbiJd~WyJQSlBPbjZaSG95Z0FlUHltWU5iRVYtVDJ0N3lwU2E0eE5uOVlDQ1ZBIiwiZG9iIiwiMTk4Mi0wMS0wMSJd~";
    const VC_SD_JWT: &str = "eyJ0eXAiOiJ2YytzZC1qd3QiLCJraWQiOiJkaWQ6d2ViOmxvY2FsaG9zdCUzQTMwMzMjQXM1cjRVeG9fWjREV19XLW9BUVNURTRLT1hhMnVWeUJRYUNna3VTUkhwWSIsImFsZyI6IkVTMjU2In0.eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvbnMvY3JlZGVudGlhbHMvdjIiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJfc2QiOlsiLU04ZmdVWlVuSkllWUpCcU45d2NjZVlwMzBMS21pZlZrR1Z1OFNacWFoQSIsIlZ5dW1yWUxPblJXaUxJMDZzTk1XY1J2MkswWGFoYWZnWjM0ekZMcDlDREEiLCJ2cnFyWW1yelRCTmtkSGlXLWllMnpURG1BamhjQkFURU5sZVNkaUI5VDN3Il19LCJpc3N1ZXIiOnsiaWQiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvIiwibmFtZSI6IlVuaUNvcmUifSwidmFsaWRGcm9tIjoiMjAyNi0wMi0wM1QyMDoxMToxOFoiLCJjcmVkZW50aWFsU3RhdHVzIjp7ImlkIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsInR5cGUiOiJzdGF0dXNsaXN0K2p3dCIsImlkeCI6MTExNCwidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCJ9LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsImlkeCI6MTExNH19LCJfc2RfYWxnIjoic2hhLTI1NiIsImNuZiI6eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVdHMUpPRmc1ZVhCU05WVTJlRE5ZTm5ab1NuUk9hM1ZwVkVWTk4zRklkbk5wVjBoa04zSkdSbTFxVVNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1aME4yUnZja1JLZDFwTE5GaG5hVGRsVDFJNGNXRkZMWE4yTUhoQ01qaFJOWFZLYTNJdFEzZEJNalFpTENKNUlqb2lSWFI0T1RWM1IxcFZZekE1VjJzMVJXTnZNR1JHZVhreGVXVkVlbUl6TFdsS1VuRTNOMDF4T1U5YVl5SjkjMCJ9fQ.ZxWYelfw39UScERquLEshscRMOBt0lyDrOaD9WzZgJ81j42wTVOYvqL1Ok_C3UcGgs-6-szu4o2V8uRksQc9jw~WyJZN1pKZGlqMnZPSDBWSnJmaEswY29zQVlPRGtibjlmcHNPMFNqTFduIiwiZG9iIiwiMTk4Mi0wMS0wMSJd~WyJoU1hxZEZWYzlYRnBoLWFWaUtQZUoxVVNBU3Z2c3RCOVBuaVdNSGFFIiwiZmlyc3RfbmFtZSIsIkZlcnJpcyJd~WyJjZFZ4cXJucWE2WF93SUhyOGN0QnlnZXdFU0dTSHNZNDJsSVg5MzZOIiwibGFzdF9uYW1lIiwiQ3JhYm1hbiJd~";

    fn disclosed_claim_names(sd_jwt: &str) -> Vec<String> {
        let mut names: Vec<String> = sd_jwt
            .parse::<SdJwt>()
            .unwrap()
            .disclosures()
            .iter()
            .filter_map(|disclosure| disclosure.claim_name.clone())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn conceal_unrequested_claims_keeps_only_requested_disclosures() {
        let presented =
            conceal_unrequested_claims(DC_SD_JWT, &Format::DcSdJwt, &[&path(json!(["first_name"]))]).unwrap();
        assert_eq!(disclosed_claim_names(&presented), vec!["first_name"]);

        let presented = conceal_unrequested_claims(
            VC_SD_JWT,
            &Format::VcSdJwt,
            &[
                &path(json!(["credentialSubject", "dob"])),
                &path(json!(["credentialSubject", "last_name"])),
            ],
        )
        .unwrap();
        assert_eq!(disclosed_claim_names(&presented), vec!["dob", "last_name"]);

        let presented = conceal_unrequested_claims(DC_SD_JWT, &Format::DcSdJwt, &[]).unwrap();
        assert!(disclosed_claim_names(&presented).is_empty());
    }

    struct UnsignedSigner;

    #[async_trait::async_trait]
    impl sd_jwt::JwsSigner for UnsignedSigner {
        type Error = String;
        async fn sign(&self, header: &sd_jwt::JsonObject, payload: &sd_jwt::JsonObject) -> Result<Vec<u8>, String> {
            use base64::Engine as _;
            let encode = |object: &sd_jwt::JsonObject| {
                base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_vec(object).unwrap())
            };
            Ok(format!("{}.{}.c2lnbmF0dXJl", encode(header), encode(payload)).into_bytes())
        }
    }

    #[tokio::test]
    async fn concealable_claims_rejects_claims_that_share_a_disclosure() {
        let sd_jwt = sd_jwt::SdJwtBuilder::new(json!({
            "credentialSubject": { "achievement": { "name": "Badge", "description": "First" }, "nickname": "Ferris" }
        }))
        .unwrap()
        .make_concealable("/credentialSubject/achievement")
        .unwrap()
        .make_concealable("/credentialSubject/nickname")
        .unwrap()
        .finish(&UnsignedSigner, "ES256")
        .await
        .unwrap()
        .to_string();

        assert_eq!(
            concealable_claims(
                &sd_jwt,
                &[
                    &path(json!(["credentialSubject", "achievement", "name"])),
                    &path(json!(["credentialSubject", "achievement", "description"])),
                    &path(json!(["credentialSubject", "nickname"]))
                ]
            ),
            vec![false, false, true]
        );
    }

    #[test]
    fn concealable_claims_detects_disclosable_claims() {
        assert_eq!(
            concealable_claims(
                VC_SD_JWT,
                &[
                    &path(json!(["credentialSubject", "first_name"])),
                    &path(json!(["issuer", "name"])),
                    &path(json!(["credentialSubject", "missing"]))
                ]
            ),
            vec![true, false, false]
        );
    }
}
