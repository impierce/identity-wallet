use log::debug;
use tokio::{
    fs::{read, remove_file, File},
    io::AsyncWriteExt,
};

use crate::state::TransferState;
use crate::STATE_FILE;

/// Loads a [TransferState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
pub async fn load_state() -> anyhow::Result<TransferState> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let bytes = read(state_file).await?;
    let content = String::from_utf8(bytes)?;
    let transfer_state: TransferState = serde_json::from_str(&content)?;
    debug!("state loaded from disk");
    Ok(transfer_state)
}

/// Persists a [TransferState] to the app's data directory.
pub async fn save_state(transfer_state: TransferState) -> anyhow::Result<()> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let mut file = File::create(state_file).await?;
    file.write_all(serde_json::to_string(&transfer_state)?.as_bytes())
        .await?;
    debug!("state saved to disk");
    Ok(())
}

// Removes the state file from the app's data directory.
pub async fn delete_state_file() -> anyhow::Result<()> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    remove_file(state_file).await?;
    debug!("state deleted from disk");
    Ok(())
}

pub async fn delete_stronghold() -> anyhow::Result<()> {
    let stronghold_file = crate::STRONGHOLD.lock().unwrap().clone();
    remove_file(&stronghold_file).await?;
    remove_file(stronghold_file.join(".snapshot")).await?;
    debug!("stronghold deleted from disk");
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_load_state() {
        // TODO: how to mock the app_handle?
    }
}
