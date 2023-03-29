use super::{Storage, StorageKey};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};
use tokio::sync::Mutex;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Record {
    state: Option<Cow<'static, str>>,
    data: HashMap<Cow<'static, str>, Vec<u8>>,
}

#[derive(Debug, Default, Clone)]
pub struct Memory {
    storage: Arc<Mutex<HashMap<StorageKey, Record>>>,
}

impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.storage, &other.storage)
    }
}

impl Memory {
    #[must_use]
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
    /// * `value` - State for specified key
    async fn set_state<Value>(&self, key: &StorageKey, value: Value) -> Result<(), Self::Error>
    where
        Value: Into<Cow<'static, str>> + Send,
    {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().state = Some(value.into());
            }
            Entry::Vacant(entry) => {
                entry.insert(Record {
                    state: Some(value.into()),
                    data: HashMap::default(),
                });
            }
        }
        Ok(())
    }

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// State for specified key, if state is no exists, then `None` will be return
    async fn get_state(&self, key: &StorageKey) -> Result<Option<Cow<'static, str>>, Self::Error> {
        Ok(self
            .storage
            .lock()
            .await
            .get(key)
            .and_then(|record| record.state.clone()))
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
    /// * `value` - Data for specified key, if empty, then data will be clear
    async fn set_data<Key, Data>(
        &self,
        key: &StorageKey,
        value: HashMap<Key, Data>,
    ) -> Result<(), Self::Error>
    where
        Data: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        let value_len = value.len();

        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                if value_len == 0 {
                    entry.get_mut().data.clear();
                    return Ok(());
                }

                let mut data = HashMap::with_capacity(value_len);

                for (key, value) in value {
                    data.insert(key.into(), bincode::serialize(&value)?);
                }

                entry.get_mut().data = data;
            }
            Entry::Vacant(entry) => {
                if value_len == 0 {
                    entry.insert(Record {
                        state: None,
                        data: HashMap::default(),
                    });
                    return Ok(());
                }

                let mut data = HashMap::with_capacity(value_len);

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
    /// Data for specified key, if data is no exists, then empty `HashMap` will be return
    async fn get_data<Data>(
        &self,
        key: &StorageKey,
    ) -> Result<HashMap<Cow<'static, str>, Data>, Self::Error>
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
            Entry::Vacant(_) => Ok(HashMap::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state() {
        let storage = Memory::default();

        let key1 = StorageKey::new(0, 1, 2);
        let key2 = StorageKey::new(2, 1, 0);

        assert_eq!(storage.get_state(&key1).await.unwrap(), None);
        assert_eq!(storage.get_state(&key2).await.unwrap(), None);

        storage.set_state(&key1, "state1").await.unwrap();
        storage.set_state(&key2, "state2").await.unwrap();

        assert_eq!(
            storage.get_state(&key1).await.unwrap(),
            Some("state1".into())
        );
        assert_eq!(
            storage.get_state(&key2).await.unwrap(),
            Some("state2".into())
        );

        storage.remove_state(&key1).await.unwrap();

        assert_eq!(storage.get_state(&key1).await.unwrap(), None);
        assert_eq!(
            storage.get_state(&key2).await.unwrap(),
            Some("state2".into())
        );

        storage.remove_state(&key2).await.unwrap();

        assert_eq!(storage.get_state(&key1).await.unwrap(), None);
        assert_eq!(storage.get_state(&key2).await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_data() {
        let storage = Memory::default();

        let key1 = StorageKey::new(0, 1, 2);
        let key2 = StorageKey::new(2, 1, 0);

        assert_eq!(
            storage.get_data::<String>(&key1).await.unwrap(),
            HashMap::default()
        );
        assert_eq!(
            storage.get_data::<String>(&key2).await.unwrap(),
            HashMap::default()
        );

        let mut data1 = HashMap::new();
        data1.insert("key1", "value1");
        data1.insert("key2", "value2");

        let mut data2 = HashMap::new();
        data2.insert("key3", "value3");
        data2.insert("key4", "value4");

        storage.set_data(&key1, data1).await.unwrap();
        storage.set_data(&key2, data2).await.unwrap();

        let get_data1 = storage.get_data::<String>(&key1).await.unwrap();
        let get_data2 = storage.get_data::<String>(&key2).await.unwrap();

        assert_eq!(get_data1.len(), 2);
        assert_eq!(get_data2.len(), 2);

        assert_eq!(get_data1.get("key1").unwrap(), &"value1");
        assert_eq!(get_data1.get("key2").unwrap(), &"value2");
        assert_eq!(get_data2.get("key3").unwrap(), &"value3");
        assert_eq!(get_data2.get("key4").unwrap(), &"value4");

        storage.remove_data(&key1).await.unwrap();

        assert_eq!(storage.get_data::<String>(&key1).await.unwrap().len(), 0);
        assert_eq!(storage.get_data::<String>(&key2).await.unwrap().len(), 2);

        storage.remove_data(&key2).await.unwrap();

        assert_eq!(storage.get_data::<String>(&key1).await.unwrap().len(), 0);
        assert_eq!(storage.get_data::<String>(&key2).await.unwrap().len(), 0);
    }
}
