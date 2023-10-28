use super::{Error, Storage, StorageKey};

use async_trait::async_trait;
use redis::{aio::Connection, Client, RedisError};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};
use tokio::sync::Mutex;
use tracing::{event, field, instrument, Level, Span};

const DEFAULT_PREFIX: &str = "fsm";
const DEFAULT_SEPARATOR: &str = ":";

#[derive(Debug)]
pub enum Part {
    States,
    Data,
}

impl Part {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Part::States => "states",
            Part::Data => "data",
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
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
    fn build(&self, key: &StorageKey, part: Part) -> Box<str>;
}

impl<T: ?Sized> KeyBuilder for Arc<T>
where
    T: KeyBuilder,
{
    fn build(&self, key: &StorageKey, part: Part) -> Box<str> {
        T::build(self, key, part)
    }
}

#[derive(Debug)]
pub struct DefaultKeyBuilder {
    prefix: &'static str,
    separator: &'static str,
    with_bot_id: bool,
    with_destiny: bool,
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
    fn build(&self, key: &StorageKey, part: Part) -> Box<str> {
        let bot_id = key.bot_id.to_string();
        let chat_id = key.chat_id.to_string();
        let user_id = key.user_id.to_string();
        let message_thread_id = key
            .message_thread_id
            .map(|message_thread_id| message_thread_id.to_string());

        let mut parts = vec![];

        parts.push(self.prefix);
        if self.with_destiny {
            parts.push(key.destiny);
        }
        if self.with_bot_id {
            parts.push(&bot_id);
        }

        parts.push(&chat_id);
        if let Some(message_thread_id) = &message_thread_id {
            parts.push(message_thread_id);
        }
        parts.push(&user_id);
        parts.push(part.as_str());

        parts.join(self.separator).into_boxed_str()
    }
}

/// This is a thread-safe storage implementation for redis
#[derive(Clone)]
pub struct Redis {
    client: Arc<Mutex<Client>>,
    /// Key builder for redis keys, used to build redis keys for specified key and part
    key_builder: Arc<dyn KeyBuilder>,
}

impl Redis {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            key_builder: Arc::<DefaultKeyBuilder>::default(),
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
}

impl Redis {
    async fn get_connection(&self) -> Result<Connection, RedisError> {
        self.client.lock().await.get_async_connection().await
    }
}

#[async_trait]
impl Storage for Redis {
    type Error = Error;

    /// Set state for specified key
    /// # Arguments
    /// * `key` - Specified key to set state
    /// * `state` - State for specified key
    #[instrument(skip(self, key, state), fields(key, state))]
    async fn set_state<State>(&self, key: &StorageKey, state: State) -> Result<(), Self::Error>
    where
        State: Into<Cow<'static, str>> + Send,
    {
        let key = self.key_builder.build(key, Part::States);
        let key_ref = key.as_ref();
        let state = state.into();
        let state_ref = state.as_ref();

        Span::current()
            .record("key", key_ref)
            .record("state", state_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        redis::cmd("RPUSH")
            .arg(key_ref)
            .arg(state_ref)
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to set state");

                Error::new(format!("Failed to set state. Storage key: {key}"), err)
            })
    }

    /// Set previous state as current state
    /// # Arguments
    /// * `key` - Specified key to set previous state
    /// # Notes
    /// States stack is used to store states history,
    /// when user set new state, then current state will be push to the states stack,
    /// so you can use this method to back to the previous state
    #[instrument(skip(self, key), fields(key))]
    async fn set_previous_state(&self, key: &StorageKey) -> Result<(), Self::Error> {
        let key = self.key_builder.build(key, Part::States);
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        redis::cmd("RPOP")
            .arg(key_ref)
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to remove state");

                Error::new(format!("Failed to remove state. Storage key: {key}"), err)
            })
    }

    /// Get state for specified key
    /// # Arguments
    /// * `key` - Specified key to get state
    /// # Returns
    /// State for specified key, if state is no exists, then `None` will be return
    #[instrument(skip(self, key), fields(key))]
    async fn get_state(&self, key: &StorageKey) -> Result<Option<Box<str>>, Self::Error> {
        let key = self.key_builder.build(key, Part::States);
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        redis::cmd("LINDEX")
            .arg(key_ref)
            .arg(-1)
            .query_async::<_, Option<String>>(&mut connection)
            .await
            .map(|state| state.map(Into::into))
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to get state");

                Error::new(format!("Failed to get state. Storage key: {key}"), err)
            })
    }

    /// Get states stack for specified key
    /// # Arguments
    /// * `key` - Specified key to get states stack
    /// # Note
    /// States stack is used to store states history,
    /// when user set new state, then current state will be push to the states stack,
    /// so you can use this method to get states history or back to the previous state
    /// # Returns
    /// States stack for specified key, if states stack is no exists, then empty slice will be return
    #[instrument(skip(self, key), fields(key))]
    async fn get_states(&self, key: &StorageKey) -> Result<Box<[Box<str>]>, Self::Error> {
        let key = self.key_builder.build(key, Part::States);
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        redis::cmd("LRANGE")
            .arg(key_ref)
            .arg(0)
            .arg(-1)
            .query_async::<_, Vec<String>>(&mut connection)
            .await
            .map(|states| states.into_iter().map(Into::into).collect())
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to get states");

                Error::new(format!("Failed to get states. Storage key: {key}"), err)
            })
    }

    /// Remove states stack for specified key
    /// # Arguments
    /// * `key` - Specified key to remove states stack
    /// # Note
    /// States stack is used to store states history,
    /// when user set new state, then current state will be push to the states stack,
    /// so you can use this method to clear states history
    #[instrument(skip(self, key), fields(key))]
    async fn remove_states(&self, key: &StorageKey) -> Result<(), Self::Error> {
        let key = self.key_builder.build(key, Part::States);
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        redis::cmd("DEL")
            .arg(key_ref)
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to remove states");

                Error::new(format!("Failed to remove states. Storage key: {key}"), err)
            })
    }

    /// Set data for specified key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `data` - Data for specified key, if empty, then data will be clear
    #[instrument(skip(self, key, data), fields(key))]
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
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let plain_json = serde_json::to_string(&data).map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to serialize data");

            Error::new(format!("Failed to serialize data. Storage key: {key}"), err)
        })?;
        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        redis::cmd("SET")
            .arg(key_ref)
            .arg(plain_json.as_str())
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to set data");

                Error::new(format!("Failed to set data. Storage key: {key}"), err)
            })
    }

    /// Set value to the data for specified key and value key
    /// # Arguments
    /// * `key` - Specified key to set data
    /// * `value_key` - Specified value key to set value to the data
    /// * `value` - Value for specified key and value key
    #[instrument(skip(self, key, value_key, value), fields(key, value_key, data))]
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
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        let plain_json: Option<String> = redis::cmd("GET")
            .arg(key_ref)
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to get data");

                Error::new(format!("Failed to get data. Storage key: {key}"), err)
            })?;

        let mut data = match plain_json {
            Some(plain_json) => serde_json::from_str(plain_json.as_str()).map_err(|err| {
                event!(
                    Level::ERROR,
                    error = %err,
                    json = %plain_json,
                    "Failed to deserialize data",
                );

                Error::new(
                    format!("Failed to deserialize data. Storage key: {key}"),
                    err,
                )
            })?,
            None => HashMap::with_capacity(1),
        };

        let value_key = value_key.into();

        Span::current().record("value_key", value_key.as_ref());

        data.insert(
            value_key,
            serde_json::to_value(value).map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to convert value to `serde_json::Value`");

                Error::new(
                    format!("Failed to convert value to `serde_json::Value`. Storage key: {key}"),
                    err,
                )
            })?,
        );

        Span::current().record("data", field::debug(&data));

        let plain_json = serde_json::to_string(&data).map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to serialize data");

            Error::new(format!("Failed to serialize data. Storage key: {key}"), err)
        })?;

        redis::cmd("SET")
            .arg(key_ref)
            .arg(plain_json.as_str())
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to set data");

                Error::new(format!("Failed to set data. Storage key: {key}"), err)
            })
    }

    /// Get data for specified key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// # Returns
    /// Data for specified key, if data is no exists, then empty [`HashMap`] will be return
    #[instrument(skip(self, key))]
    async fn get_data<Value>(
        &self,
        key: &StorageKey,
    ) -> Result<HashMap<Box<str>, Value>, Self::Error>
    where
        Value: DeserializeOwned,
    {
        let key = self.key_builder.build(key, Part::Data);
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        let plain_json: Option<String> = redis::cmd("GET")
            .arg(key_ref)
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to get data");

                Error::new(format!("Failed to get data. Storage key: {key}"), err)
            })?;

        match plain_json {
            Some(plain_json) => serde_json::from_str(plain_json.as_str()).map_err(|err| {
                event!(
                    Level::ERROR,
                    error = %err,
                    json = %plain_json,
                    "Failed to deserialize data",
                );

                Error::new(
                    format!("Failed to deserialize data. Storage key: {key}"),
                    err,
                )
            }),
            None => Ok(HashMap::default()),
        }
    }

    /// Get value from the data for specified key and value key
    /// # Arguments
    /// * `key` - Specified key to get data
    /// * `value_key` - Specified value key to get value from the data
    /// # Returns
    /// Value for specified key and value key, if value is no exists, then `None` will be return
    #[instrument(skip(self, key, value_key), fields(key))]
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
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        let plain_json: Option<String> = redis::cmd("GET")
            .arg(key_ref)
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to get data");

                Error::new(format!("Failed to get data. Storage key: {key}"), err)
            })?;

        match plain_json {
            Some(plain_json) => {
                let data: HashMap<Box<str>, serde_json::Value> =
                    serde_json::from_str(plain_json.as_str()).map_err(|err| {
                        event!(
                            Level::ERROR,
                            error = %err,
                            json = %plain_json,
                            "Failed to deserialize data",
                        );

                        Error::new(
                            format!("Failed to deserialize data. Storage key: {key}"),
                            err,
                        )
                    })?;

                match data.get(value_key.into().as_ref()) {
                    Some(value) => serde_json::from_value(value.clone()).map_err(
                        |err| {
                            event!(
                                Level::ERROR,
                                error = %err,
                                value = %value,
                                "Failed to convert `serde_json::Value` to value",
                            );

                            Error::new(
                                format!(
                                    "Failed to convert `serde_json::Value` to value. Storage key: {key}"
                                ),
                                err,
                            )
                        },
                    ).map(Some),
                    None => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    /// Remove data for specified key
    /// # Arguments
    /// * `key` - Specified key to remove data
    #[instrument(skip(self, key), fields(key))]
    async fn remove_data(&self, key: &StorageKey) -> Result<(), Self::Error> {
        let key = self.key_builder.build(key, Part::Data);
        let key_ref = key.as_ref();

        Span::current().record("key", key_ref);

        let mut connection = self.get_connection().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to get redis connection");

            Error::new(
                format!("Failed to get redis connection. Storage key: {key}"),
                err,
            )
        })?;

        redis::cmd("DEL")
            .arg(key_ref)
            .query_async(&mut connection)
            .await
            .map_err(|err| {
                event!(Level::ERROR, error = %err, "Failed to remove data");

                Error::new(format!("Failed to remove data. Storage key: {key}"), err)
            })
    }
}
