use crate::error::AppError;
use identity_kit::sd_jwt_vc::{ClaimPath, PathComponent, Processed, SdJwt, ELLIPSIS_KEY};
use oid4vc_core::claim_path_pointer::{ClaimPathElement, ClaimPathPointer, ClaimPathProcessingError};
use oid4vp::dcql::dcql_query::{CredentialQuery, Format};
use serde_json::Value;
use std::collections::HashSet;

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
            .is_some_and(|claim| claim.path.get_values_from_json(credential_json).is_ok())
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
                return Err(AppError::MissingRequiredCredentialError(format!(
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
                AppError::MissingRequiredCredentialError(format!(
                    "No claim set of credential query {} can be satisfied",
                    credential_query.id.as_ref()
                ))
            }),
    }
}

/// The paths in `value` that `path` points to, where `null` matches every element of an array.
pub fn concrete_paths(path: &ClaimPathPointer, value: &Value) -> Vec<ClaimPath> {
    let mut selections: Vec<(Vec<PathComponent>, &Value)> = vec![(vec![], value)];
    for element in path.as_ref() {
        selections = selections
            .into_iter()
            .flat_map(|(prefix, value)| {
                let children: Vec<(PathComponent, &Value)> = match (element, value) {
                    (ClaimPathElement::String(key), Value::Object(object)) => object
                        .get(key)
                        .map(|child| (PathComponent::Key(key.clone()), child))
                        .into_iter()
                        .collect(),
                    (ClaimPathElement::Integer(index), Value::Array(array)) => array
                        .get(*index as usize)
                        .map(|child| (PathComponent::Index(*index as usize), child))
                        .into_iter()
                        .collect(),
                    (ClaimPathElement::Null, Value::Array(array)) => array
                        .iter()
                        .enumerate()
                        .map(|(index, child)| (PathComponent::Index(index), child))
                        .collect(),
                    _ => vec![],
                };
                children.into_iter().map(move |(component, child)| {
                    let mut path = prefix.clone();
                    path.push(component);
                    (path, child)
                })
            })
            .collect();
    }
    selections.into_iter().map(|(path, _)| ClaimPath::new(path)).collect()
}

/// The concrete paths that `path` points to in the processed payload, in the coordinates of the issued credential that
/// the disclosures' paths use.
fn issued_paths(path: &ClaimPathPointer, processed: &Processed) -> Vec<ClaimPath> {
    let payload = Value::Object(processed.payload.clone());
    let issued = Value::Object(processed.as_issued().clone());
    concrete_paths(path, &payload)
        .iter()
        .filter_map(|path| issued_path(path, &issued))
        .collect()
}

/// The path in `issued` of the claim at `path` in the processed payload. An array element the holder has no
/// disclosure for, such as a decoy, stays in the issued credential as `{"...": "<digest>"}` but not in the processed
/// payload, so the indices of the elements after it differ.
fn issued_path(path: &ClaimPath, issued: &Value) -> Option<ClaimPath> {
    let mut current = issued;
    let mut components = Vec::with_capacity(path.components().len());
    for component in path.components() {
        match (component, current) {
            (PathComponent::Key(key), Value::Object(object)) => {
                current = object.get(key)?;
                components.push(component.clone());
            }
            (PathComponent::Index(index), Value::Array(elements)) => {
                let (issued_index, element) = elements
                    .iter()
                    .enumerate()
                    .filter(|(_, element)| !is_undisclosed_element(element))
                    .nth(*index)?;
                current = element;
                components.push(PathComponent::Index(issued_index));
            }
            _ => return None,
        }
    }
    Some(ClaimPath::new(components))
}

fn is_undisclosed_element(element: &Value) -> bool {
    matches!(element, Value::Object(object) if object.len() == 1 && object.contains_key(ELLIPSIS_KEY))
}

fn parse(sd_jwt: &str) -> Result<(SdJwt, Processed), AppError> {
    let sd_jwt = SdJwt::parse(sd_jwt).map_err(|err| AppError::Error(format!("Failed to parse SD-JWT: {err}")))?;
    let processed = sd_jwt
        .process()
        .map_err(|err| AppError::Error(format!("Failed to decode SD-JWT: {err}")))?;
    Ok((sd_jwt, processed))
}

/// The disclosures needed to reveal the claims at the concrete `paths`: those at or beneath a path, and those a path
/// lies inside. A disclosure's parents are among them too, since a parent's path is a prefix of its child's, so it is
/// either beneath the same path or one the path lies inside.
fn disclosures_revealing<'a>(sd_jwt: &'a SdJwt, processed: &Processed, paths: &[ClaimPath]) -> HashSet<&'a str> {
    processed
        .disclosed
        .iter()
        .filter(|claim| {
            paths
                .iter()
                .any(|path| path.is_prefix_of(&claim.path) || claim.path.is_prefix_of(path))
        })
        .map(|claim| sd_jwt.disclosures()[claim.disclosure].as_str())
        .collect()
}

/// For each claim path, whether the holder can leave it out of a presentation of `sd_jwt` while still disclosing all
/// other claims. A claim inside a disclosure that also holds other claims cannot be left out on its own.
pub fn concealable_claims(sd_jwt: &str, claim_paths: &[&ClaimPathPointer]) -> Vec<bool> {
    let Ok((sd_jwt, processed)) = parse(sd_jwt) else {
        return vec![false; claim_paths.len()];
    };
    // How many values each path selects: none when it selects nothing, and `None` when it does not fit the
    // payload's structure, which aborts its processing (OpenID4VP 1.0, Section 7.1.1) instead of making the claim absent.
    let value_counts = |value: &Value| -> Vec<Option<usize>> {
        claim_paths
            .iter()
            .map(|claim_path| match claim_path.get_values_from_json(value) {
                Ok(values) => Some(values.len()),
                Err(ClaimPathProcessingError::NothingSelected { .. }) => Some(0),
                Err(_) => None,
            })
            .collect()
    };
    let before = value_counts(&Value::Object(processed.payload.clone()));

    (0..claim_paths.len())
        .map(|index| {
            let paths = issued_paths(claim_paths[index], &processed);
            if paths.is_empty() {
                return false;
            }
            // Leave out the claim's own disclosure or, when it has none, the nearest disclosure it lies inside, together
            // with every disclosure beneath it.
            let concealed: HashSet<&str> = paths
                .iter()
                .flat_map(|path| {
                    let root = processed
                        .disclosed
                        .iter()
                        .filter(|claim| claim.path.is_prefix_of(path))
                        .max_by_key(|claim| claim.path.components().len())
                        .map_or(path, |claim| &claim.path);
                    processed
                        .disclosed
                        .iter()
                        .filter(move |claim| root.is_prefix_of(&claim.path))
                })
                .map(|claim| sd_jwt.disclosures()[claim.disclosure].as_str())
                .collect();
            let Ok(after) = sd_jwt
                .select(|disclosure| !concealed.contains(disclosure.as_str()))
                .and_then(|presented| presented.process())
            else {
                return false;
            };
            let after = value_counts(&Value::Object(after.payload));
            // Concealable only when the claim is then absent: a path that no longer fits would make a Verifier reject
            // the presentation, not find the claim missing.
            after[index] == Some(0)
                && (0..claim_paths.len()).all(|other| other == index || after[other] == before[other])
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
    let (sd_jwt, processed) = parse(sd_jwt)?;

    let mut kept: Vec<ClaimPath> = claim_paths
        .iter()
        .flat_map(|claim_path| issued_paths(claim_path, &processed))
        .collect();
    if matches!(format, Format::VcSdJwt) {
        kept.extend(
            processed
                .as_issued()
                .keys()
                .filter(|key| *key != "credentialSubject")
                .map(|key| ClaimPath::new([key.as_str()])),
        );
        kept.push(ClaimPath::new(["credentialSubject", "id"]));
    }

    let disclosed = disclosures_revealing(&sd_jwt, &processed, &kept);
    sd_jwt
        .select(|disclosure| disclosed.contains(disclosure.as_str()))
        .map(|presented| presented.to_string())
        .map_err(|err| AppError::Error(format!("Failed to create SD-JWT presentation: {err}")))
}

/// The payload of `sd_jwt` as presented with only the disclosures needed to reveal the claims at `claim_paths`.
pub fn presented_payload(sd_jwt: &str, format: &Format, claim_paths: &[&ClaimPathPointer]) -> Result<Value, AppError> {
    sd_jwt_payload(&conceal_unrequested_claims(sd_jwt, format, claim_paths)?)
}

/// The payload of `sd_jwt` with its disclosures applied.
pub fn sd_jwt_payload(sd_jwt: &str) -> Result<Value, AppError> {
    parse(sd_jwt).map(|(_, processed)| Value::Object(processed.payload))
}

#[cfg(test)]
pub(crate) mod tests {
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
            vec![
                ClaimPath::new([PathComponent::from("nationalities"), PathComponent::Index(0)]),
                ClaimPath::new([PathComponent::from("nationalities"), PathComponent::Index(1)])
            ]
        );
        assert_eq!(
            concrete_paths(&path(json!(["address", "city"])), &value),
            vec![ClaimPath::new(["address", "city"])]
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

    pub(crate) const DC_SD_JWT: &str = "eyJ0eXAiOiJkYytzZC1qd3QiLCJraWQiOiJkaWQ6d2ViOmxvY2FsaG9zdCUzQTMwMzMjQXM1cjRVeG9fWjREV19XLW9BUVNURTRLT1hhMnVWeUJRYUNna3VTUkhwWSIsImFsZyI6IkVTMjU2In0.eyJ2Y3QiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvdmN0L1UwUXRTbGRVLzAiLCJfc2QiOlsiNzZ0bGNVNi1kMlNaQUhHV3ZrVk5aV2hweS11QUFzczBuemdCdmx0X19QRSIsIlZsSVdHMVJNai1vckpSeTZIWUJqaXRLaDVUOGxLTUU5UkVlbmFUN2htRzgiLCJta19kazVuc0pXQ2hZYWpocW4yT2N2cXNYazlRdTdNcHdXeEpnTlBQQVp3Il0sImlzcyI6ImRpZDp3ZWI6bG9jYWxob3N0JTNBMzAzMyIsIm5iZiI6MTc3MDE0OTU2NSwiaWF0IjoxNzcwMTQ5NTY1LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMSIsImlkeCI6ODEyNH19LCJfc2RfYWxnIjoic2hhLTI1NiIsImNuZiI6eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVdHMUpPRmc1ZVhCU05WVTJlRE5ZTm5ab1NuUk9hM1ZwVkVWTk4zRklkbk5wVjBoa04zSkdSbTFxVVNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1aME4yUnZja1JLZDFwTE5GaG5hVGRsVDFJNGNXRkZMWE4yTUhoQ01qaFJOWFZLYTNJdFEzZEJNalFpTENKNUlqb2lSWFI0T1RWM1IxcFZZekE1VjJzMVJXTnZNR1JHZVhreGVXVkVlbUl6TFdsS1VuRTNOMDF4T1U5YVl5SjkjMCJ9fQ.8ZY3p-eeEtlFQjfEYBcRGjxHeq2DpqIHNgD4WKPLcF8qmK8bWOJxYKaw1cfrptx6WsNs5cqhZgtlztBV9HckMw~WyJaam1obE9QQzhpZWREOHJxTXItckxvVzFvRXFDSUR4SHFoQmJkRlpKIiwiZmlyc3RfbmFtZSIsIkZlcnJpcyJd~WyIyUkhteHh0cXpHNHRCeU1DdG9LWXhTRXdEaE92MTd0Z1FaR2hNVEUwIiwibGFzdF9uYW1lIiwiQ3JhYm1hbiJd~WyJQSlBPbjZaSG95Z0FlUHltWU5iRVYtVDJ0N3lwU2E0eE5uOVlDQ1ZBIiwiZG9iIiwiMTk4Mi0wMS0wMSJd~";
    const VC_SD_JWT: &str = "eyJ0eXAiOiJ2YytzZC1qd3QiLCJraWQiOiJkaWQ6d2ViOmxvY2FsaG9zdCUzQTMwMzMjQXM1cjRVeG9fWjREV19XLW9BUVNURTRLT1hhMnVWeUJRYUNna3VTUkhwWSIsImFsZyI6IkVTMjU2In0.eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvbnMvY3JlZGVudGlhbHMvdjIiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJfc2QiOlsiLU04ZmdVWlVuSkllWUpCcU45d2NjZVlwMzBMS21pZlZrR1Z1OFNacWFoQSIsIlZ5dW1yWUxPblJXaUxJMDZzTk1XY1J2MkswWGFoYWZnWjM0ekZMcDlDREEiLCJ2cnFyWW1yelRCTmtkSGlXLWllMnpURG1BamhjQkFURU5sZVNkaUI5VDN3Il19LCJpc3N1ZXIiOnsiaWQiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvIiwibmFtZSI6IlVuaUNvcmUifSwidmFsaWRGcm9tIjoiMjAyNi0wMi0wM1QyMDoxMToxOFoiLCJjcmVkZW50aWFsU3RhdHVzIjp7ImlkIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsInR5cGUiOiJzdGF0dXNsaXN0K2p3dCIsImlkeCI6MTExNCwidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCJ9LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsImlkeCI6MTExNH19LCJfc2RfYWxnIjoic2hhLTI1NiIsImNuZiI6eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVdHMUpPRmc1ZVhCU05WVTJlRE5ZTm5ab1NuUk9hM1ZwVkVWTk4zRklkbk5wVjBoa04zSkdSbTFxVVNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1aME4yUnZja1JLZDFwTE5GaG5hVGRsVDFJNGNXRkZMWE4yTUhoQ01qaFJOWFZLYTNJdFEzZEJNalFpTENKNUlqb2lSWFI0T1RWM1IxcFZZekE1VjJzMVJXTnZNR1JHZVhreGVXVkVlbUl6TFdsS1VuRTNOMDF4T1U5YVl5SjkjMCJ9fQ.ZxWYelfw39UScERquLEshscRMOBt0lyDrOaD9WzZgJ81j42wTVOYvqL1Ok_C3UcGgs-6-szu4o2V8uRksQc9jw~WyJZN1pKZGlqMnZPSDBWSnJmaEswY29zQVlPRGtibjlmcHNPMFNqTFduIiwiZG9iIiwiMTk4Mi0wMS0wMSJd~WyJoU1hxZEZWYzlYRnBoLWFWaUtQZUoxVVNBU3Z2c3RCOVBuaVdNSGFFIiwiZmlyc3RfbmFtZSIsIkZlcnJpcyJd~WyJjZFZ4cXJucWE2WF93SUhyOGN0QnlnZXdFU0dTSHNZNDJsSVg5MzZOIiwibGFzdF9uYW1lIiwiQ3JhYm1hbiJd~";

    pub(crate) fn disclosed_claim_names(sd_jwt: &str) -> Vec<String> {
        let mut names: Vec<String> = sd_jwt
            .parse::<SdJwt>()
            .unwrap()
            .disclosures()
            .iter()
            .filter_map(|disclosure| disclosure.name().map(str::to_owned))
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

    /// An SD-JWT of `claims` with the claims at `concealed` selectively disclosable, under a fake signature.
    pub(crate) fn unsigned_sd_jwt(claims: Value, concealed: Vec<ClaimPath>) -> String {
        use base64::Engine as _;
        let Value::Object(claims) = claims else {
            panic!("claims must be an object")
        };
        let encoded = concealed
            .into_iter()
            .fold(identity_kit::sd_jwt_vc::SdJwtBuilder::new(claims), |builder, path| {
                builder.conceal(path)
            })
            .encode()
            .unwrap();
        let encode = |value: &Value| base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(value.to_string());
        let jwt = format!(
            "{}.{}.c2lnbmF0dXJl",
            encode(&json!({ "alg": "ES256", "typ": "vc+sd-jwt" })),
            encode(&Value::Object(encoded.payload))
        );
        SdJwt::new(identity_kit::vc::Jwt::new(jwt).unwrap(), encoded.disclosures).to_string()
    }

    #[test]
    fn concealable_claims_rejects_claims_that_share_a_disclosure() {
        let sd_jwt = unsigned_sd_jwt(
            json!({
                "credentialSubject": { "achievement": { "name": "Badge", "description": "First" }, "nickname": "Ferris" }
            }),
            vec![
                ClaimPath::new(["credentialSubject", "achievement"]),
                ClaimPath::new(["credentialSubject", "nickname"]),
            ],
        );

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
    fn a_claim_without_a_disclosure_of_its_own_is_concealed_with_the_disclosure_it_lies_in() {
        let sd_jwt = unsigned_sd_jwt(
            json!({ "credentialSubject": { "achievement": { "name": "Badge" }, "nickname": "Ferris" } }),
            vec![
                ClaimPath::new(["credentialSubject", "achievement"]),
                ClaimPath::new(["credentialSubject", "nickname"]),
            ],
        );

        assert_eq!(
            concealable_claims(
                &sd_jwt,
                &[
                    &path(json!(["credentialSubject", "achievement", "name"])),
                    &path(json!(["credentialSubject", "nickname"]))
                ]
            ),
            vec![true, true]
        );
    }

    #[test]
    fn conceal_unrequested_claims_keeps_the_disclosures_a_requested_claim_lies_in() {
        let sd_jwt = unsigned_sd_jwt(
            json!({
                "credentialSubject": {
                    "achievement": { "name": "Badge", "description": "First" },
                    "nickname": "Ferris",
                    "nationalities": ["NL", "DE"]
                }
            }),
            vec![
                ClaimPath::new(["credentialSubject", "achievement", "name"]),
                ClaimPath::new(["credentialSubject", "achievement"]),
                ClaimPath::new(["credentialSubject", "nickname"]),
                ClaimPath::new([
                    PathComponent::from("credentialSubject"),
                    PathComponent::from("nationalities"),
                    PathComponent::Index(0),
                ]),
            ],
        );

        // `name` is disclosed on its own, inside the `achievement` disclosure, which must come along.
        let presented = conceal_unrequested_claims(
            &sd_jwt,
            &Format::DcSdJwt,
            &[&path(json!(["credentialSubject", "achievement", "name"]))],
        )
        .unwrap();
        assert_eq!(disclosed_claim_names(&presented), vec!["achievement", "name"]);

        // `description` is no disclosure of its own: the `achievement` disclosure holds it, without `name`.
        let presented = conceal_unrequested_claims(
            &sd_jwt,
            &Format::DcSdJwt,
            &[&path(json!(["credentialSubject", "achievement", "description"]))],
        )
        .unwrap();
        assert_eq!(disclosed_claim_names(&presented), vec!["achievement"]);

        // An array element is disclosed through the wildcard; `DE` was never concealable.
        let presented = conceal_unrequested_claims(
            &sd_jwt,
            &Format::DcSdJwt,
            &[&path(json!(["credentialSubject", "nationalities", null]))],
        )
        .unwrap();
        let presented = SdJwt::parse(&presented).unwrap();
        assert_eq!(presented.disclosures().len(), 1);
        assert_eq!(
            presented.process().unwrap().payload["credentialSubject"],
            json!({ "nationalities": ["NL", "DE"] })
        );
    }

    /// The verifier's paths index the processed payload. An array element the holder has no disclosure for stays in the
    /// issued credential as a placeholder, so the element a path selects is found past it.
    #[test]
    fn array_indices_count_the_elements_of_the_processed_payload() {
        let issued = unsigned_sd_jwt(
            json!({ "nationalities": ["NL", "DE"] }),
            vec![
                ClaimPath::new([PathComponent::from("nationalities"), PathComponent::Index(0)]),
                ClaimPath::new([PathComponent::from("nationalities"), PathComponent::Index(1)]),
            ],
        );
        // Without the disclosure of `NL`, its placeholder precedes `DE`, which the processed payload has at index 0.
        let issued = SdJwt::parse(&issued).unwrap();
        let disclosures = issued
            .disclosures()
            .iter()
            .filter(|disclosure| disclosure.value() != &json!("NL"))
            .cloned()
            .collect();
        let held = SdJwt::new(issued.jwt().clone(), disclosures).to_string();
        let first = path(json!(["nationalities", 0]));

        let presented = SdJwt::parse(&conceal_unrequested_claims(&held, &Format::DcSdJwt, &[&first]).unwrap()).unwrap();
        assert_eq!(presented.disclosures().len(), 1);
        assert_eq!(presented.process().unwrap().payload["nationalities"], json!(["DE"]));

        assert_eq!(concealable_claims(&held, &[&first]), vec![true]);
    }

    #[test]
    fn a_claim_whose_path_no_longer_fits_once_concealed_is_not_concealable() {
        // Concealing the first element of `items` leaves `["tail"]`: `["items", 0, "name"]` then applies a key to a
        // string, which aborts processing (OpenID4VP 1.0, Section 7.1.1) instead of finding the claim absent.
        let sd_jwt = unsigned_sd_jwt(
            json!({ "items": [{ "name": "Anna" }, "tail"] }),
            vec![ClaimPath::new([PathComponent::from("items"), PathComponent::Index(0)])],
        );

        assert_eq!(
            concealable_claims(&sd_jwt, &[&path(json!(["items", 0, "name"]))]),
            vec![false]
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
