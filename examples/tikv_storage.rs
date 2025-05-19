use metadata_rust::storage::TiKVStorage;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Connect to TiKV (using the new PD endpoint)
    let pd_endpoints = vec!["127.0.0.1:2381".to_string()];
    let storage = TiKVStorage::new(pd_endpoints).await?;

    // Example operations
    let key = b"test_key";
    let value = b"test_value";

    // Put a value
    storage.put(key, value).await?;
    println!("Put value successfully");

    // Get the value
    let retrieved = storage.get(key).await?;
    println!("Retrieved value: {:?}", retrieved);

    // Delete the value
    storage.delete(key).await?;
    println!("Deleted value successfully");

    // Verify deletion
    let retrieved = storage.get(key).await?;
    println!("Value after deletion: {:?}", retrieved);

    Ok(())
} 