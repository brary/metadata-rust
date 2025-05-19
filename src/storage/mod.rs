mod tikv;

pub use tikv::TiKVStorage;

use anyhow::Result;
use bytes::Bytes;

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Get a value from storage
    async fn get(&self, key: &[u8]) -> Result<Option<Bytes>>;
    
    /// Put a value into storage
    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    
    /// Delete a value from storage
    async fn delete(&self, key: &[u8]) -> Result<()>;
    
    /// Scan a range of keys
    async fn scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Bytes, Bytes)>>;
} 