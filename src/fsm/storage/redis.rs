use super::{Storage, StorageKey};

use async_trait::async_trait;
use redis::{aio::Connection, Client, RedisError};
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, collections::HashMap, sync::Arc};
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

impl<T: ?Sized> KeyBuilder for Arc<T>
where
    T: KeyBuilder,
{
    fn build(&self, key: &StorageKey, part: Part) -> String {
        T::build(self, key, part)
    }
}

#[derive(Debug)]
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

/// This is a thread-safe storage implementation for redis
#[derive(Clone)]
pub struct Redis {
    client: Arc<Mutex<Client>>,
    /// Key builder for redis keys, used to build redis keys for specified key and part
    key_builder: Arc<dyn KeyBuilder>,
    /// TTL for state, if [`None`] then state will not be deleted
    state_ttl: Option<u64>,
    /// TTL for data, if [`None`] then data will not be deleted
    data_ttl: Option<u64>,
}

impl Redis {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            key_builder: Arc::<DefaultKeyBuilder>::default(),
            state_ttl: None,
            data_ttl: None,
        }
    }

    #[must_use]
    pub fn key_builder<T>(self, key_builder: T) -> Self
    where
        T: KeyBuilder + 'static,
    {
        Self {
            key_builder: Arc::new(key_builder),
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
        let key = self.key_builder.build(key, Part::State);
        let mut connection = self.get_connection().await?;

        redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut connection)
            .await?;
        Ok(())
    }

    /// Set state for specified key
    /// # Arguments
    /// * `key` - Specified key to set state
    /// * `state` - State for specified key
    async fn set_state<State>(&self, key: &StorageKey, state: State) -> Result<(), Self::Error>
    where
        State: Into<Cow<'static, str>> + Send,
    {
        let key = self.key_builder.build(key, Part::State);
        let state = state.into();
        let mut connection = self.get_connection().await?;

        if let Some(state_ttl) = self.state_ttl {
            redis::cmd("SETEX")
                .arg(&key)
                .arg(state_ttl)
                .arg(state.as_ref())
                .query_async(&mut connection)
                .await?;
        } else {
            redis::cmd("SET")
                .arg(&key)
                .arg(state.as_ref())
                .query_async(&mut connection)
                .await?;
        }
        Ok(())
    }

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// State for specified key, if state is no exists, then [`None`] will be return
    async fn get_state(&self, key: &StorageKey) -> Result<Option<String>, Self::Error> {
        let key = self.key_builder.build(key, Part::State);
        let mut connection = self.get_connection().await?;

        redis::cmd("GET")
            .arg(&key)
            .query_async(&mut connection)
            .await
            .map_err(Into::into)
    }

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error> {
        let key = self.key_builder.build(key, Part::Data);
        let mut connection = self.get_connection().await?;

        redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut connection)
            .await?;
        Ok(())
    }

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
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        let key = self.key_builder.build(key, Part::Data);
        let plain_json = serde_json::to_string(&data)?;
        let mut connection = self.get_connection().await?;

        if let Some(data_ttl) = self.data_ttl {
            redis::cmd("SETEX")
                .arg(&key)
                .arg(data_ttl)
                .arg(&plain_json)
                .query_async(&mut connection)
                .await?;
        } else {
            redis::cmd("SET")
                .arg(&key)
                .arg(&plain_json)
                .query_async(&mut connection)
                .await?;
        }
        Ok(())
    }

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
        Key: Serialize + Into<Cow<'static, str>> + Send,
    {
        let key = self.key_builder.build(key, Part::Data);
        let mut connection = self.get_connection().await?;

        let plain_json: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut connection)
            .await?;

        let mut data = match plain_json {
            Some(plain_json) => serde_json::from_str(&plain_json)?,
            None => HashMap::with_capacity(1),
        };

        data.insert(value_key.into(), serde_json::to_value(value)?);

        let plain_json = serde_json::to_string(&data)?;
        if let Some(data_ttl) = self.data_ttl {
            redis::cmd("SETEX")
                .arg(&key)
                .arg(data_ttl)
                .arg(&plain_json)
                .query_async(&mut connection)
                .await?;
        } else {
            redis::cmd("SET")
                .arg(&key)
                .arg(&plain_json)
                .query_async(&mut connection)
                .await?;
        }
        Ok(())
    }

    /// Get data for specified key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// # Returns
    /// Data for specified key, if data is no exists, then empty [`HashMap`] will be return
    async fn get_data<Value>(&self, key: &StorageKey) -> Result<HashMap<String, Value>, Self::Error>
    where
        Value: DeserializeOwned,
    {
        let key = self.key_builder.build(key, Part::Data);
        let mut connection = self.get_connection().await?;

        let plain_json: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut connection)
            .await?;

        match plain_json {
            Some(plain_json) => Ok(serde_json::from_str(&plain_json)?),
            None => Ok(HashMap::default()),
        }
    }

    /// Get value from the data for specified key and value key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// * `value_key` - Specified value key to get value from the data
    /// # Returns
    /// Value for specified key and value key, if value is no exists, then [`None`] will be return
    async fn get_value<Key, Value>(
        &self,
        key: &StorageKey,
        value_key: Key,
    ) -> Result<Option<Value>, Self::Error>
    where
        Value: DeserializeOwned,
        Key: Into<Cow<'static, str>> + Send,
    {
        let key = self.key_builder.build(key, Part::Data);
        let mut connection = self.get_connection().await?;

        let plain_json: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut connection)
            .await?;

        match plain_json {
            Some(plain_json) => {
                let data: HashMap<Cow<'static, str>, serde_json::Value> =
                    serde_json::from_str(&plain_json)?;

                match data.get(&value_key.into()) {
                    Some(value) => Ok(Some(serde_json::from_value(value.clone())?)),
                    None => Ok(None),
                }
            }
            None => Ok(None),
        }
    }
}
