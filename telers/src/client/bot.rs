//! This module contains the [`Bot`] structure that represents a bot with its token and ID,
//! it also contains client for sending requests to Telegram API.
//!
//! You can use [`Bot::send`] method, which accepts any type that implements [`TelegramMethod`].
//! Methods from [`methods`] module are implemented with builders, so you don't need to pass all parameters to it,
//! only required and optional by using builder methods. Builders yet can have some useful shortcuts.
//!
//! # Notes
//!
//! This structure is cheap to clone, because it contains only [`String`], [`i64`] fields and a client.
//! Default client is [`Reqwest`], which also is cheap to clone.
//!
//! You can use custom client by using [`Bot::with_client`] method.
//!
//! # Examples
//! ```rust
//! use telers::{methods::SendMessage, Bot};
//!
//! async fn call_method(bot: Bot) {
//!     let chat_id = 1;
//!     let text = "Hello, world!";
//!
//!     let _ = bot.send(SendMessage::new(chat_id, text)).await;
//! }
//! ```
//!
//! You also can use [`Bot::send_with_timeout`] method to send requests with timeout:
//!
//! ```rust
//! use telers::{methods::SendMessage, Bot};
//!
//! async fn call_method(bot: Bot) {
//!     let chat_id = 1;
//!     let text = "Hello, world!";
//!     let timeout = 10.0; // 10 seconds
//!
//!     let _ = bot
//!         .send_with_timeout(SendMessage::new(chat_id, text), timeout)
//!         .await;
//! }
//! ```
//!
//! More production examples can be found in [`examples`] directory.
//!
//! [`examples`]: https://github.com/Desiders/telers/tree/dev-1.x/examples
//! [`methods`]: telers::methods

use super::{session::base::Session, Reqwest};

use crate::{errors::SessionErrorKind, methods::TelegramMethod, utils::token};

use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
};

/// Represents a bot with its token and ID, also contains client for sending requests to Telegram API.
/// # Notes
/// This structure is cheap to clone, because it contains only [`String`], [`i64`] fields and a client.
///
/// Default client is [`Reqwest`], which also is cheap to clone.
///
/// You can use custom client by using [`Bot::with_client`] method.
///
/// Check [module docs](crate::client::bot) for examples.
#[derive(Clone, Default)]
pub struct Bot<Client = Reqwest> {
    /// Bot token, which is used to receive updates and send requests to the Telegram API
    pub token: String,
    /// Bot token, which is used in `Debug` implementation for privacy
    pub hidden_token: String,
    /// Bot id, extracted from the token
    pub id: i64,
    /// Client for sending requests to Telegram API
    client: Client,
}

impl<Client> Bot<Client> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn with_client(token: impl Into<String>, client: Client) -> Self {
        let token = token.into();
        let id = token::extract_bot_id(&token).expect(
            "This bot token is invalid, please check it. If you test your bot, and you don't have a token, use \
             `Bot::default` method instead of `Bot::new`.",
        );
        let hidden_token = token::hide(&token);

        Self {
            token,
            hidden_token,
            id,
            client,
        }
    }
}

impl Bot<Reqwest> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_client(token, Reqwest::default())
    }

    /// # Notes
    /// This method uses custom environment variable to get the token.
    /// If you want to use default environment variable, use [`Bot::from_env`] method instead.
    /// If you want to pass the token directly, use [`Bot::new`] method instead.
    /// # Panics
    /// Panics if the token is invalid or unset in the environment variables
    #[must_use]
    pub fn from_env_by_key(name: impl AsRef<str>) -> Self {
        Self::new(env::var(name.as_ref()).expect("This env variable is not set!"))
    }

    /// # Notes
    /// This method uses `BOT_TOKEN` environment variable to get the token.
    /// If you want to use custom environment variable, use [`Bot::from_env_by_key`] method instead.
    /// If you want to pass the token directly, use [`Bot::new`] method instead.
    /// # Panics
    /// Panics if the token is invalid or unset in the environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_by_key("BOT_TOKEN")
    }
}

impl<Client> Debug for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bot")
            .field("token", &self.hidden_token)
            .field("bot_id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<Client> Display for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Displaying only bot id and hidden token, because the token is sensitive data
        write!(
            f,
            "Bot {{ bot_id: {}, token: {} }}",
            self.id, self.hidden_token,
        )
    }
}

impl<Client: Session> Bot<Client> {
    /// Use this method to send requests to Telegram API
    /// # Arguments
    /// * `method` - Telegram API method
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses default timeout for requests, which is 30 seconds.
    /// If you want to use custom timeout, use [`Bot::send_with_timeout`] method.
    pub async fn send<T>(&self, method: T) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        self.client
            .make_request_and_get_result(self, method, None)
            .await
    }

    /// Use this method to send requests to Telegram API with timeout
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `request_timeout` - Request timeout
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses passed timeout for requests.
    /// If you want to use default timeout, use [`Bot::send`] method.
    pub async fn send_with_timeout<T>(
        &self,
        method: T,
        request_timeout: f32,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        self.client
            .make_request_and_get_result(self, method, Some(request_timeout))
            .await
    }
}
