use super::{Storage, StorageKey};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::{hash_map::Entry, HashMap};
use tokio::sync::Mutex;

#[derive(Default)]
struct Record {
    state: Option<String>,
    data: HashMap<String, Vec<u8>>,
}

#[derive(Default)]
pub struct Memory {
    storage: Mutex<HashMap<StorageKey, Record>>,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Storage for Memory {
    type Error = bincode::Error;

    /// Remove state for specified key
    /// # Arguments
    /// * `key` - Specified key to remove state
    async fn remove_state(&self, key: &StorageKey) -> Result<(), Self::Error> {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().state = None;
            }
            Entry::Vacant(_) => {}
        }
        Ok(())
    }

    /// Set state for specified key
    /// # Arguments
    /// * `key` - Specified key to set state
    /// * `value` - Set state for specified key, if value is `None`, then state will be removed
    async fn set_sate<Value>(
        &self,
        key: &StorageKey,
        value: Option<Value>,
    ) -> Result<(), Self::Error>
    where
        Value: Into<String> + Send,
    {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().state = value.map(Into::into);
            }
            Entry::Vacant(entry) => {
                entry.insert(Record {
                    state: value.map(Into::into),
                    data: HashMap::new(),
                });
            }
        }
        Ok(())
    }

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// * State for specified key, if state is not exists, then `None` will be returned
    async fn get_state(&self, key: &StorageKey) -> Result<Option<String>, Self::Error> {
        Ok(self
            .storage
            .lock()
            .await
            .get(key)
            .map(|record| record.state.clone())
            .flatten())
    }

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error> {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().data.clear();
            }
            Entry::Vacant(_) => {}
        }
        Ok(())
    }

    /// Set data for specified key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `value` - Set data for specified key, if value is empty, then data will be removed
    async fn set_data<Key, Data>(
        &self,
        key: &StorageKey,
        value: HashMap<Key, Data>,
    ) -> Result<(), Self::Error>
    where
        Data: Serialize + Send,
        Key: Serialize + Into<String> + Send,
    {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().data.clear();
            }
            Entry::Vacant(entry) => {
                let mut data = HashMap::with_capacity(value.len());
                for (key, value) in value {
                    data.insert(key.into(), bincode::serialize(&value)?);
                }
                entry.insert(Record { state: None, data });
            }
        }
        Ok(())
    }

    /// Get data for specified key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// # Returns
    /// * Data for specified key, if data is not exists, then empty `HashMap` will be returned
    async fn get_data<Data>(&self, key: &StorageKey) -> Result<HashMap<String, Data>, Self::Error>
    where
        Data: DeserializeOwned,
    {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(entry) => {
                let entry_data = &entry.get().data;
                let mut data = HashMap::with_capacity(entry_data.len());
                for (key, value) in entry_data {
                    data.insert(key.clone(), bincode::deserialize(value)?);
                }
                Ok(data)
            }
            Entry::Vacant(_) => Ok(HashMap::new()),
        }
    }
}
