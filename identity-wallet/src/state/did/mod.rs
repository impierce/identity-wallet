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

        // TODO: quick hack to solve the percent-encoding issue in did:web:localhost%3A3033 (localhost:3033)
        let url_decoded = url_str.replace("%3A", ":");

        if let Ok(url) = Url::parse(&format!("http://{url_decoded}")) {
            // TODO: the http:// hardcoded scheme is a hack to test with localhost
            return Some(url);
        }
    }
    None
}
