use iota_client::Client;

pub async fn create_iota_identity() -> anyhow::Result<()> {
    static API_ENDPOINT: &str = "http://127.0.0.1:14265";

    let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;
    Ok(())
}
