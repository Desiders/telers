use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, collections::HashMap, error::Error as StdError};

const DEFAULT_DESTINY: &str = "default";

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
    /// * `value` - Set state for specified key
    async fn set_state<Value>(&self, key: &StorageKey, value: Value) -> Result<(), Self::Error>
    where
        Value: Into<Cow<'static, str>> + Send;

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// * State for specified key, if state is no exists, then `None` will be return
    async fn get_state(&self, key: &StorageKey) -> Result<Option<Cow<'static, str>>, Self::Error>;

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error>;

    /// Set data for specified key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `value` - Set data for specified key, if empty, then data will be clear
    async fn set_data<Key, Data>(
        &self,
        key: &StorageKey,
        value: HashMap<Key, Data>,
    ) -> Result<(), Self::Error>
    where
        Data: Serialize + Send,
        Key: Serialize + Into<Cow<'static, str>> + Send;

    /// Get data for specified key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// # Returns
    /// * Data for specified key, if data is no exists, then empty `HashMap` will be return
    async fn get_data<Data>(
        &self,
        key: &StorageKey,
    ) -> Result<HashMap<Cow<'static, str>, Data>, Self::Error>
    where
        Data: DeserializeOwned;
}
