use iota_client::Client;

// TODO: move this function to a folder that handles all did interactions with the resp. networks (such as "clients")
pub async fn create_iota_identity() -> anyhow::Result<()> {
    static API_ENDPOINT: &str = "http://127.0.0.1:14265";

    let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;
    Ok(())
}
