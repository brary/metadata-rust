use anyhow::{Result, Context};
use bytes::Bytes;
use tikv_client::{Config, TransactionClient, Key};
use async_trait::async_trait;

use super::Storage;

pub struct TiKVStorage {
    client: TransactionClient,
}

impl TiKVStorage {
    pub async fn new(pd_endpoints: Vec<String>) -> Result<Self> {
        let config = Config::new(pd_endpoints);
        let client = TransactionClient::new(config)
            .await
            .context("Failed to create TiKV client")?;
        
        Ok(Self { client })
    }
}

#[async_trait]
impl Storage for TiKVStorage {
    async fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {
        let txn = self.client.begin_optimistic()
            .await
            .context("Failed to begin transaction")?;
        
        let key = Key::from(key);
        let value = txn.get(key)
            .await
            .context("Failed to get value")?;
        
        Ok(value.map(|v| Bytes::from(v)))
    }

    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let txn = self.client.begin_optimistic()
            .await
            .context("Failed to begin transaction")?;
        
        let key = Key::from(key);
        txn.put(key, value)
            .await
            .context("Failed to put value")?;
        
        txn.commit()
            .await
            .context("Failed to commit transaction")?;
        
        Ok(())
    }

    async fn delete(&self, key: &[u8]) -> Result<()> {
        let txn = self.client.begin_optimistic()
            .await
            .context("Failed to begin transaction")?;
        
        let key = Key::from(key);
        txn.delete(key)
            .await
            .context("Failed to delete value")?;
        
        txn.commit()
            .await
            .context("Failed to commit transaction")?;
        
        Ok(())
    }

    async fn scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Bytes, Bytes)>> {
        let txn = self.client.begin_optimistic()
            .await
            .context("Failed to begin transaction")?;
        
        let start_key = Key::from(start);
        let end_key = Key::from(end);
        
        let pairs = txn.scan(start_key..end_key, 100)
            .await
            .context("Failed to scan range")?;
        
        let result = pairs
            .map(|pair| {
                let (key, value) = pair.context("Failed to get key-value pair")?;
                Ok((Bytes::from(key.into_raw()?), Bytes::from(value)))
            })
            .collect::<Result<Vec<_>>>()?;
        
        Ok(result)
    }
} 