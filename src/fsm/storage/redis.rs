use super::{Storage, StorageKey};

use async_trait::async_trait;
use redis::{aio::Connection, Client, RedisError};
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, collections::HashMap};
use thiserror;
use tokio::sync::Mutex;

const DEFAULT_PREFIX: &str = "fsm";
const DEFAULT_SEPARATOR: &str = ":";

pub enum Part {
    State,
    Data,
}

impl Part {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Part::State => "state",
            Part::Data => "data",
        }
    }
}

pub trait KeyBuilder: Send + Sync {
    /// Build redis key for specified key and part
    /// # Arguments
    /// * `key` - Specified key to build key
    /// * `part` - Specified part to build key
    /// # Returns
    /// Redis key for specified key and part
    #[must_use]
    fn build(&self, key: &StorageKey, part: Part) -> String;
}

pub struct DefaultKeyBuilder {
    prefix: &'static str,
    separator: &'static str,
    with_bot_id: bool,
    with_destiny: bool,

    /// Capacity of parts, used to pre-allocate memory
    parts_capacity: usize,
}

impl DefaultKeyBuilder {
    #[must_use]
    pub fn new(
        prefix: &'static str,
        separator: &'static str,
        with_bot_id: bool,
        with_destiny: bool,
    ) -> Self {
        Self {
            prefix,
            separator,
            with_bot_id,
            with_destiny,
            parts_capacity: {
                let mut count = 4;
                if with_destiny {
                    count += 1;
                }
                if with_bot_id {
                    count += 1;
                }
                count
            },
        }
    }
}

impl Default for DefaultKeyBuilder {
    #[must_use]
    fn default() -> Self {
        Self::new(DEFAULT_PREFIX, DEFAULT_SEPARATOR, true, true)
    }
}

impl KeyBuilder for DefaultKeyBuilder {
    fn build(&self, key: &StorageKey, part: Part) -> String {
        let bot_id = key.bot_id.to_string();
        let chat_id = key.chat_id.to_string();
        let user_id = key.user_id.to_string();

        let mut parts = Vec::with_capacity(self.parts_capacity);

        parts.push(self.prefix);
        if self.with_destiny {
            parts.push(key.destiny);
        }
        if self.with_bot_id {
            parts.push(&bot_id);
        }

        parts.push(&chat_id);
        parts.push(&user_id);
        parts.push(part.as_str());

        parts.join(self.separator)
    }
}

pub struct Redis {
    client: Mutex<Client>,
    key_builder: Box<dyn KeyBuilder>,
    state_ttl: Option<u64>,
    data_ttl: Option<u64>,
}

impl Redis {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client: Mutex::new(client),
            key_builder: Box::<DefaultKeyBuilder>::default(),
            state_ttl: None,
            data_ttl: None,
        }
    }

    #[must_use]
    pub fn key_builder(self, key_builder: Box<dyn KeyBuilder>) -> Self {
        Self {
            key_builder,
            ..self
        }
    }

    #[must_use]
    pub fn state_ttl(self, state_ttl: u64) -> Self {
        Self {
            state_ttl: Some(state_ttl),
            ..self
        }
    }

    #[must_use]
    pub fn data_ttl(self, data_ttl: u64) -> Self {
        Self {
            data_ttl: Some(data_ttl),
            ..self
        }
    }
}

impl Redis {
    async fn get_connection(&self) -> Result<Connection, RedisError> {
        self.client.lock().await.get_async_connection().await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    Redis(#[from] RedisError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

#[async_trait]
impl Storage for Redis {
    type Error = StorageError;

    /// Remove state for specified key
    /// # Arguments
    /// * `key` - Specified key to remove state
    async fn remove_state(&self, key: &StorageKey) -> Result<(), Self::Error> {
        let mut connection = self.get_connection().await?;
        let key = self.key_builder.build(key, Part::State);

        redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut connection)
            .await?;
        Ok(())
    }

    /// Set state for specified key
    /// # Arguments
    /// * `key` - Specified key to set state
    /// * `value` - Set state for specified key
    async fn set_sate<Value>(&self, key: &StorageKey, value: Value) -> Result<(), Self::Error>
    where
        Value: Into<Cow<'static, str>> + Send,
    {
        let mut connection = self.get_connection().await?;
        let key = self.key_builder.build(key, Part::State);
        let value = value.into();

        if let Some(state_ttl) = self.state_ttl {
            redis::cmd("SETEX")
                .arg(&key)
                .arg(state_ttl)
                .arg(value.as_ref())
                .query_async(&mut connection)
                .await?;
        } else {
            redis::cmd("SET")
                .arg(&key)
                .arg(value.as_ref())
                .query_async(&mut connection)
                .await?;
        }
        Ok(())
    }

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// * State for specified key, if state is no exists, then `None` will be return
    async fn get_state(&self, key: &StorageKey) -> Result<Option<Cow<'static, str>>, Self::Error> {
        let mut connection = self.get_connection().await?;
        let key = self.key_builder.build(key, Part::State);

        let value: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut connection)
            .await?;
        Ok(value.map(Cow::Owned))
    }

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error> {
        let mut connection = self.get_connection().await?;
        let key = self.key_builder.build(key, Part::Data);

        redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut connection)
            .await?;
        Ok(())
    }

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
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        let mut connection = self.get_connection().await?;
        let key = self.key_builder.build(key, Part::Data);
        let value = serde_json::to_string(&value)?;

        if let Some(data_ttl) = self.data_ttl {
            redis::cmd("SETEX")
                .arg(&key)
                .arg(data_ttl)
                .arg(&value)
                .query_async(&mut connection)
                .await?;
        } else {
            redis::cmd("SET")
                .arg(&key)
                .arg(&value)
                .query_async(&mut connection)
                .await?;
        }
        Ok(())
    }

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
        Data: DeserializeOwned,
    {
        let mut connection = self.get_connection().await?;
        let key = self.key_builder.build(key, Part::Data);

        let value: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut connection)
            .await?;

        match value {
            Some(value) => Ok(serde_json::from_str(&value)?),
            None => Ok(HashMap::default()),
        }
    }
}
