use super::{Storage, StorageKey};

use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, collections::HashMap};

/// Context is used to manage state and data of the user in specified storage
pub struct Context<S> {
    storage: S,
    key: StorageKey,
}

impl<S> Context<S> {
    pub fn new(storage: S, key: StorageKey) -> Self {
        Self { storage, key }
    }
}

impl<S> Clone for Context<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            key: self.key.clone(),
        }
    }
}

impl<S> Context<S>
where
    S: Storage,
{
    /// Remove current state
    /// # Errors
    /// If storage error occurs, when remove state
    pub async fn remove_state(&self) -> Result<(), S::Error> {
        self.storage.remove_state(&self.key).await
    }

    /// Set current state
    /// # Arguments
    /// * `state` - State
    /// # Errors
    /// If storage error occurs, when set state
    pub async fn set_state<State>(&self, state: State) -> Result<(), S::Error>
    where
        State: Into<Cow<'static, str>> + Send,
    {
        self.storage.set_state(&self.key, state).await
    }

    /// Get current state
    /// # Returns
    /// State, if state is no exists, then [`None`] will be return
    /// # Errors
    /// If storage error occurs, when get state
    pub async fn get_state(&self) -> Result<Option<String>, S::Error> {
        self.storage.get_state(&self.key).await
    }

    /// Remove current data
    /// # Errors
    /// If storage error occurs, when remove data
    pub async fn remove_data(&self) -> Result<(), S::Error> {
        self.storage.remove_data(&self.key).await
    }

    /// Set current data
    /// # Arguments
    /// * `data` - Data, if empty, then data will be clear
    /// # Errors
    /// If storage error occurs, when set data
    pub async fn set_data<Key, Data>(&self, data: HashMap<Key, Data>) -> Result<(), S::Error>
    where
        Data: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        self.storage.set_data(&self.key, data).await
    }

    /// Set value to the data for value key
    /// # Arguments
    /// * `value_key` - Specified value key to set value to data
    /// * `value` - Value for value key
    /// # Errors
    /// If storage error occurs, when set value to data
    pub async fn set_value<Key, Value>(&self, value_key: Key, value: Value) -> Result<(), S::Error>
    where
        Value: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        self.storage.set_value(&self.key, value_key, value).await
    }

    /// Get current data
    /// # Returns
    /// Data, if data is no exists, then empty [`HashMap`] will be return
    /// # Errors
    /// If storage error occurs, when get data
    pub async fn get_data<Data>(&self) -> Result<HashMap<String, Data>, S::Error>
    where
        Data: DeserializeOwned,
    {
        self.storage.get_data(&self.key).await
    }

    /// Get value from data for value key
    /// # Arguments
    /// * `value_key` - Specified value key to get value from data
    /// # Returns
    /// Value, if value is no exists, then [`None`] will be return
    /// # Errors
    /// If storage error occurs, when get value from data
    pub async fn get_value<Key, Value>(&self, value_key: Key) -> Result<Option<Value>, S::Error>
    where
        Value: DeserializeOwned,
        Key: Into<Cow<'static, str>> + Send,
    {
        self.storage.get_value(&self.key, value_key).await
    }

    /// Finish current context by remove state and data.
    /// This method is just shortcut for [`Context::remove_state`] and [`Context::remove_data`] methods
    /// # Errors
    /// If storage error occurs, when remove state or data
    pub async fn finish(&self) -> Result<(), S::Error> {
        self.storage.remove_state(&self.key).await?;
        self.storage.remove_data(&self.key).await
    }
}
