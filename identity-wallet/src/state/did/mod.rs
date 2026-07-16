use url::Url;

pub mod actions;
pub mod reducers;
pub mod validate_domain_linkage;
pub mod validate_linked_verifiable_presentations;

// Helper

pub fn extract_url_from_did_web(did_web: &str) -> Option<Url> {
    if let Some(did) = did_web.strip_prefix("did:web:") {
        let url_str = if let Some(index_colon) = did.find(':') {
            &did[..index_colon]
        } else {
            did
        };

        // When present in the URL, colons need to be percent-encoded, e.g., did:web:localhost%3A3033 (localhost:3033)
        // https://w3c-ccg.github.io/did-method-web/#method-specific-identifier
        let url_decoded = urlencoding::decode(url_str);

        if let Ok(url_decoded) = url_decoded {
            if let Ok(url) = Url::parse(&format!("http://{url_decoded}")) {
                // TODO: the http:// hardcoded scheme is a hack to test with localhost
                return Some(url);
            }
        }
    }
    None
}
