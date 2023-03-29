use super::{Storage, StorageKey};

use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, collections::HashMap};

pub struct FSMContext<S> {
    storage: S,
    key: StorageKey,
}

impl<S> FSMContext<S> {
    pub fn new(storage: S, key: StorageKey) -> Self {
        Self { storage, key }
    }
}

impl<S> FSMContext<S>
where
    S: Storage,
{
    /// Remove current state
    pub async fn remove_state(&self) -> Result<(), S::Error> {
        self.storage.remove_state(&self.key).await
    }

    /// Set current state
    /// # Arguments
    /// * `value` - State
    pub async fn set_state<Value>(&self, value: Value) -> Result<(), S::Error>
    where
        Value: Into<Cow<'static, str>> + Send,
    {
        self.storage.set_state(&self.key, value).await
    }

    /// Get current state
    /// # Returns
    /// State, if state is no exists, then `None` will be return
    pub async fn get_state(&self) -> Result<Option<Cow<'static, str>>, S::Error> {
        self.storage.get_state(&self.key).await
    }

    /// Remove current data
    pub async fn remove_data(&self) -> Result<(), S::Error> {
        self.storage.remove_data(&self.key).await
    }

    /// Set current data
    /// # Arguments
    /// * `value` - Data, if empty, then data will be clear
    pub async fn set_data<Key, Data>(&self, value: HashMap<Key, Data>) -> Result<(), S::Error>
    where
        Data: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        self.storage.set_data(&self.key, value).await
    }

    /// Get current data
    /// # Returns
    /// Data, if data is no exists, then empty `HashMap` will be return
    pub async fn get_data<Data>(&self) -> Result<HashMap<Cow<'static, str>, Data>, S::Error>
    where
        Data: DeserializeOwned,
    {
        self.storage.get_data(&self.key).await
    }
}
