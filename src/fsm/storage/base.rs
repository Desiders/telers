use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, error::Error as StdError, hash::BuildHasher};

const DEFAULT_DESTINY: &'static str = "default";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StorageKey {
    pub bot_id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub destiny: &'static str,
}

impl StorageKey {
    pub fn new(bot_id: i64, chat_id: i64, user_id: i64) -> Self {
        Self {
            bot_id,
            chat_id,
            user_id,
            destiny: DEFAULT_DESTINY,
        }
    }

    pub fn destiny(self, destiny: &'static str) -> Self {
        Self { destiny, ..self }
    }
}

#[async_trait]
pub trait Storage {
    type Error: StdError;

    /// Remove state for specified key
    /// # Arguments
    /// * `key` - Specified key to remove state
    async fn remove_state(&self, key: &StorageKey) -> Result<(), Self::Error>;

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
        Value: Into<String> + Send;

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// * State for specified key, if state is not exists, then `None` will be returned
    async fn get_state(&self, key: &StorageKey) -> Result<Option<String>, Self::Error>;

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error>;

    /// Set data for specified key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `value` - Set data for specified key, if value is empty, then data will be removed
    async fn set_data<Key, Data, S>(
        &self,
        key: &StorageKey,
        value: HashMap<Key, Data, S>,
    ) -> Result<(), Self::Error>
    where
        Data: Serialize + Send,
        Key: Serialize + Into<String> + Send,
        S: BuildHasher + Send;

    /// Get data for specified key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// # Returns
    /// * Data for specified key, if data is not exists, then empty `HashMap` will be returned
    async fn get_data<Data, S>(
        &self,
        key: &StorageKey,
    ) -> Result<HashMap<String, Data, S>, Self::Error>
    where
        Data: DeserializeOwned,
        S: BuildHasher + Default + Send;
}
