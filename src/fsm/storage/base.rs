use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, collections::HashMap, error::Error as StdError, sync::Arc};

pub const DEFAULT_DESTINY: &str = "default";

/// Storage key is used to identify the state and data of the user in the storage
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StorageKey {
    pub bot_id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub destiny: &'static str,
}

impl StorageKey {
    #[must_use]
    pub fn new(bot_id: i64, chat_id: i64, user_id: i64) -> Self {
        Self {
            bot_id,
            chat_id,
            user_id,
            destiny: DEFAULT_DESTINY,
        }
    }

    #[must_use]
    pub fn destiny(self, destiny: &'static str) -> Self {
        Self { destiny, ..self }
    }
}

/// Storage is used to store state and data of the user
/// # Note
/// Storage is part of the FSM pattern,
/// don't use it for other purposes like database and store user data not related with state machine
#[async_trait]
pub trait Storage: Clone {
    type Error: StdError + Into<anyhow::Error>;

    /// Remove state for specified key
    /// # Arguments
    /// * `key` - Specified key to remove state
    async fn remove_state(&self, key: &StorageKey) -> Result<(), Self::Error>;

    /// Set state for specified key
    /// # Arguments
    /// * `key` - Specified key to set state
    /// * `state` - State for specified key
    async fn set_state<State>(&self, key: &StorageKey, state: State) -> Result<(), Self::Error>
    where
        State: Into<Cow<'static, str>> + Send;

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// State for specified key, if state is no exists, then [`None`] will be return
    async fn get_state(&self, key: &StorageKey) -> Result<Option<String>, Self::Error>;

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error>;

    /// Set data for specified key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `data` - Data for specified key, if empty, then data will be clear
    async fn set_data<Key, Value>(
        &self,
        key: &StorageKey,
        data: HashMap<Key, Value>,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send;

    /// Set value to the data for specified key and value key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `value_key` - Specified value key to set value to the data
    /// * `value` - Value for specified key and value key
    async fn set_value<Key, Value>(
        &self,
        key: &StorageKey,
        value_key: Key,
        value: Value,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send;

    /// Get data for specified key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// Data for specified key, if data is no exists, then empty [`HashMap`] will be return
    async fn get_data<Value>(
        &self,
        key: &StorageKey,
    ) -> Result<HashMap<String, Value>, Self::Error>
    where
        Value: DeserializeOwned;

    /// Get value from the data for specified key and value key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// * `value_key` - Specified value key to get value from data
    /// # Returns
    /// Value for specified key and value key, if value is no exists, then [`None`] will be return
    async fn get_value<Key, Value>(
        &self,
        key: &StorageKey,
        value_key: Key,
    ) -> Result<Option<Value>, Self::Error>
    where
        Value: DeserializeOwned,
        Key: Into<Cow<'static, str>> + Send;
}

#[async_trait]
impl<'a, S> Storage for &'a S
where
    S: Storage + Sync + 'a,
{
    type Error = S::Error;

    async fn remove_state(&self, key: &StorageKey) -> Result<(), Self::Error> {
        S::remove_state(self, key).await
    }

    async fn set_state<State>(&self, key: &StorageKey, state: State) -> Result<(), Self::Error>
    where
        State: Into<Cow<'static, str>> + Send,
    {
        S::set_state(self, key, state).await
    }

    async fn get_state(&self, key: &StorageKey) -> Result<Option<String>, Self::Error> {
        S::get_state(self, key).await
    }

    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error> {
        S::remove_data(self, key).await
    }

    async fn set_data<Key, Value>(
        &self,
        key: &StorageKey,
        data: HashMap<Key, Value>,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        S::set_data(self, key, data).await
    }

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
        S::set_value(self, key, value_key, value).await
    }

    async fn get_data<Value>(&self, key: &StorageKey) -> Result<HashMap<String, Value>, Self::Error>
    where
        Value: DeserializeOwned,
    {
        S::get_data(self, key).await
    }

    async fn get_value<Key, Value>(
        &self,
        key: &StorageKey,
        value_key: Key,
    ) -> Result<Option<Value>, Self::Error>
    where
        Value: DeserializeOwned,
        Key: Into<Cow<'static, str>> + Send,
    {
        S::get_value(self, key, value_key).await
    }
}

#[async_trait]
impl<S: ?Sized> Storage for Arc<S>
where
    S: Storage + Send + Sync,
{
    type Error = S::Error;

    async fn remove_state(&self, key: &StorageKey) -> Result<(), Self::Error> {
        S::remove_state(self, key).await
    }

    async fn set_state<State>(&self, key: &StorageKey, state: State) -> Result<(), Self::Error>
    where
        State: Into<Cow<'static, str>> + Send,
    {
        S::set_state(self, key, state).await
    }

    async fn get_state(&self, key: &StorageKey) -> Result<Option<String>, Self::Error> {
        S::get_state(self, key).await
    }

    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error> {
        S::remove_data(self, key).await
    }

    async fn set_data<Key, Value>(
        &self,
        key: &StorageKey,
        data: HashMap<Key, Value>,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        S::set_data(self, key, data).await
    }

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
        S::set_value(self, key, value_key, value).await
    }

    async fn get_data<Value>(&self, key: &StorageKey) -> Result<HashMap<String, Value>, Self::Error>
    where
        Value: DeserializeOwned,
    {
        S::get_data(self, key).await
    }

    async fn get_value<Key, Value>(
        &self,
        key: &StorageKey,
        value_key: Key,
    ) -> Result<Option<Value>, Self::Error>
    where
        Value: DeserializeOwned,
        Key: Into<Cow<'static, str>> + Send,
    {
        S::get_value(self, key, value_key).await
    }
}
