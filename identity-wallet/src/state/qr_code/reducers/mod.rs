pub mod read_authorization_request;
pub mod read_credential_offer;

fn encode(url: reqwest::Url) -> String {
    base64::encode_config(url.as_str(), base64::URL_SAFE)
}
