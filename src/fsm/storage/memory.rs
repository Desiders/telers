use super::{Error, Storage, StorageKey};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};
use tokio::sync::Mutex;
use tracing::{event, instrument, Level, Span};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Record {
    states: Vec<Cow<'static, str>>,
    data: HashMap<Cow<'static, str>, Vec<u8>>,
}

/// This is a simple thread-safe in-memory storage implementation used for testing purposes usually
/// # Warning
/// This storage isn't recommended for production use, because it doesn't persist data between restarts. \
/// It's recommended to use a database instead and other storage implementations, like [`super::Redis`]
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
    type Error = Error;

    /// Set state for specified key
    /// # Arguments
    /// * `key` - Specified key to set state
    /// * `state` - State for specified key
    async fn set_state<State>(&self, key: &StorageKey, state: State) -> Result<(), Self::Error>
    where
        State: Into<Cow<'static, str>> + Send,
    {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().states.push(state.into());
            }
            Entry::Vacant(entry) => {
                entry.insert(Record {
                    states: vec![state.into()],
                    data: HashMap::default(),
                });
            }
        }
        Ok(())
    }

    /// Set previous state as current state
    /// # Arguments
    /// * `key` - Specified key to set previous state
    /// # Notes
    /// States stack is used to store states history,
    /// when user set new state, then current state will be push to the states stack,
    /// so you can use this method to back to the previous state
    async fn previous_state(&self, key: &StorageKey) -> Result<(), Self::Error> {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().states.pop();
            }
            Entry::Vacant(_) => {}
        };
        Ok(())
    }

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// State for specified key, if state is no exists, then [`None`] will be return
    async fn get_state(&self, key: &StorageKey) -> Result<Option<String>, Self::Error> {
        Ok(self
            .storage
            .lock()
            .await
            .get(key)
            .and_then(|record| record.states.last().map(ToString::to_string)))
    }

    /// Get states stack for specified key
    /// # Arguments
    /// * `key` - Specified key to get states stack
    /// # Notes
    /// States stack is used to store states history,
    /// when user set new state, then current state will be push to the states stack,
    /// so you can use this method to get states history or back to the previous state
    /// # Returns
    /// States stack for specified key, if states stack is no exists, then empty [`Vec`] will be return
    async fn get_states(&self, key: &StorageKey) -> Result<Vec<String>, Self::Error> {
        Ok(self
            .storage
            .lock()
            .await
            .get(key)
            .map(|record| record.states.iter().map(ToString::to_string).collect())
            .unwrap_or_default())
    }

    /// Remove states stack for specified key
    /// # Arguments
    /// * `key` - Specified key to remove states stack
    /// # Notes
    /// States stack is used to store states history,
    /// when user set new state, then current state will be push to the states stack,
    /// so you can use this method to clear states history
    async fn remove_states(&self, key: &StorageKey) -> Result<(), Self::Error> {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                // We can't use `clear` method, because we don't need save allocated capacity
                entry.get_mut().states = vec![];
            }
            Entry::Vacant(_) => {}
        }
        Ok(())
    }

    /// Set data for specified key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `data` - Data for specified key, if empty, then data will be clear
    #[instrument(skip(self, data))]
    async fn set_data<Key, Value>(
        &self,
        key: &StorageKey,
        data: HashMap<Key, Value>,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        let data_len = data.len();

        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                if data_len == 0 {
                    // We can't use `clear` method, because we don't need save allocated capacity
                    entry.get_mut().data = HashMap::default();
                    return Ok(());
                }

                let mut new_data = HashMap::with_capacity(data_len);

                for (value_key, value) in data {
                    new_data.insert(
                        value_key.into(),
                        bincode::serialize(&value).map_err(|err| {
                            event!(Level::ERROR, "Failed to serialize value");

                            Error::new(
                                format!("Failed to serialize value. Storage key: `{key:?}`"),
                                err,
                            )
                        })?,
                    );
                }

                entry.get_mut().data = new_data;
            }
            Entry::Vacant(entry) => {
                if data_len == 0 {
                    entry.insert(Record {
                        states: vec![],
                        data: HashMap::default(),
                    });
                    return Ok(());
                }

                let mut new_data = HashMap::with_capacity(data_len);

                for (value_key, value) in data {
                    new_data.insert(
                        value_key.into(),
                        bincode::serialize(&value).map_err(|err| {
                            event!(Level::ERROR, "Failed to serialize value");

                            Error::new(
                                format!("Failed to serialize value. Storage key: `{key:?}`"),
                                err,
                            )
                        })?,
                    );
                }

                entry.insert(Record {
                    states: vec![],
                    data: new_data,
                });
            }
        }
        Ok(())
    }

    /// Set value to the data for specified key and value key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `value_key` - Specified value key to set value to the data
    /// * `value` - Value for specified key and value key
    #[instrument(skip(self, value_key, value), fields(value_key))]
    async fn set_value<Key, Value>(
        &self,
        key: &StorageKey,
        value_key: Key,
        value: Value,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        let value_key = value_key.into();

        Span::current().record("value_key", value_key.as_ref());

        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().data.insert(
                    value_key,
                    bincode::serialize(&value).map_err(|err| {
                        event!(Level::ERROR, "Failed to serialize value");

                        Error::new(
                            format!("Failed to serialize value. Storage key: `{key:?}`"),
                            err,
                        )
                    })?,
                );
            }
            Entry::Vacant(entry) => {
                entry.insert(Record {
                    states: vec![],
                    data: {
                        let mut new_data = HashMap::with_capacity(1);
                        new_data.insert(
                            value_key,
                            bincode::serialize(&value).map_err(|err| {
                                event!(Level::ERROR, "Failed to serialize value");

                                Error::new(
                                    format!("Failed to serialize value. Storage key: `{key:?}`"),
                                    err,
                                )
                            })?,
                        );
                        new_data
                    },
                });
            }
        }
        Ok(())
    }

    /// Get data for specified key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// # Returns
    /// Data for specified key, if data is no exists, then empty [`HashMap`] will be return
    #[instrument(skip(self))]
    async fn get_data<Value>(&self, key: &StorageKey) -> Result<HashMap<String, Value>, Self::Error>
    where
        Value: DeserializeOwned,
    {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(entry) => {
                let entry_data = &entry.get().data;
                let mut data = HashMap::with_capacity(entry_data.len());

                for (value_key, value) in entry_data {
                    data.insert(
                        value_key.as_ref().to_owned(),
                        bincode::deserialize(value).map_err(|err| {
                            event!(Level::ERROR, "Failed to deserialize value");

                            Error::new(
                                format!("Failed to deserialize value. Storage key: `{key:?}`"),
                                err,
                            )
                        })?,
                    );
                }

                Ok(data)
            }
            Entry::Vacant(_) => Ok(HashMap::default()),
        }
    }

    /// Get value from the data for specified key and value key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// * `value_key` - Specified value key to get value from the data
    /// # Returns
    /// Value for specified key and value key, if value is no exists, then [`None`] will be return
    #[instrument(skip(self, value_key), fields(value_key))]
    async fn get_value<Key, Value>(
        &self,
        key: &StorageKey,
        value_key: Key,
    ) -> Result<Option<Value>, Self::Error>
    where
        Value: DeserializeOwned,
        Key: Into<Cow<'static, str>> + Send,
    {
        let value_key = value_key.into();

        Span::current().record("value_key", value_key.as_ref());

        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(entry) => entry.get().data.get(&value_key).map_or(Ok(None), |value| {
                Ok(Some(bincode::deserialize(value).map_err(|err| {
                    event!(Level::ERROR, "Failed to deserialize value");

                    Error::new(
                        format!("Failed to deserialize value. Storage key: `{key:?}`"),
                        err,
                    )
                })?))
            }),
            Entry::Vacant(_) => Ok(None),
        }
    }

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error> {
        match self.storage.lock().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                // We can't use `clear` method, because we don't need save allocated capacity
                entry.get_mut().data = HashMap::default();
            }
            Entry::Vacant(_) => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state() {
        let storage = Memory::default();

        let key1 = StorageKey::new(0, 1, 2, None);
        let key2 = StorageKey::new(2, 1, 0, None);

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

        storage.previous_state(&key1).await.unwrap();

        assert_eq!(storage.get_state(&key1).await.unwrap(), None);
        assert_eq!(
            storage.get_state(&key2).await.unwrap(),
            Some("state2".into())
        );

        storage.previous_state(&key2).await.unwrap();

        assert_eq!(storage.get_state(&key1).await.unwrap(), None);
        assert_eq!(storage.get_state(&key2).await.unwrap(), None);

        storage.set_state(&key1, "state1").await.unwrap();
        storage.set_state(&key1, "state2").await.unwrap();

        assert_eq!(
            storage.get_states(&key1).await.unwrap(),
            vec!["state1".to_owned(), "state2".to_owned()]
        );

        storage.remove_states(&key1).await.unwrap();

        assert_eq!(storage.get_state(&key1).await.unwrap(), None);
        assert_eq!(
            storage.get_states(&key1).await.unwrap(),
            Vec::<String>::default(),
        );
    }

    #[tokio::test]
    async fn test_data() {
        let storage = Memory::default();

        let key1 = StorageKey::new(0, 1, 2, None);
        let key2 = StorageKey::new(2, 1, 0, None);

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

        assert_eq!(get_data1.get("key1").unwrap(), "value1");
        assert_eq!(get_data1.get("key2").unwrap(), "value2");
        assert_eq!(get_data2.get("key3").unwrap(), "value3");
        assert_eq!(get_data2.get("key4").unwrap(), "value4");
        assert_eq!(
            storage.get_value::<_, String>(&key1, "key1").await.unwrap(),
            Some("value1".into())
        );
        assert_eq!(
            storage.get_value::<_, String>(&key1, "key2").await.unwrap(),
            Some("value2".into())
        );
        assert_eq!(
            storage.get_value::<_, String>(&key2, "key3").await.unwrap(),
            Some("value3".into())
        );
        assert_eq!(
            storage.get_value::<_, String>(&key2, "key4").await.unwrap(),
            Some("value4".into())
        );

        storage.set_value(&key1, "key1", "value11").await.unwrap();
        storage.set_value(&key1, "key5", "value5").await.unwrap();

        assert_eq!(
            storage.get_value::<_, String>(&key1, "key1").await.unwrap(),
            Some("value11".into())
        );
        assert_eq!(
            storage.get_value::<_, String>(&key1, "key5").await.unwrap(),
            Some("value5".into())
        );

        storage.remove_data(&key1).await.unwrap();

        assert_eq!(storage.get_data::<String>(&key1).await.unwrap().len(), 0);
        assert_eq!(storage.get_data::<String>(&key2).await.unwrap().len(), 2);

        assert_eq!(
            storage.get_value::<_, String>(&key1, "key1").await.unwrap(),
            None
        );

        storage.remove_data(&key2).await.unwrap();

        assert_eq!(storage.get_data::<String>(&key1).await.unwrap().len(), 0);
        assert_eq!(storage.get_data::<String>(&key2).await.unwrap().len(), 0);

        storage.set_value(&key1, "key1", "value1").await.unwrap();

        assert_eq!(
            storage.get_value::<_, String>(&key1, "key1").await.unwrap(),
            Some("value1".into())
        );
    }
}
