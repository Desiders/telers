//! This module contains the [`Bot`] structure that represents a bot with its token and ID,
//! it also contains client for sending requests to Telegram API.
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
//! use telers::{Bot, methods::SendMessage};
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
//! use telers::{Bot, methods::SendMessage};
//!
//! async fn call_method(bot: Bot) {
//!     let chat_id = 1;
//!     let text = "Hello, world!";
//!     let timeout = 10.0; // 10 seconds
//!
//!     let _ = bot.send_with_timeout(SendMessage::new(chat_id, text), timeout).await;
//! }
//! ```

use super::{session::base::Session, Reqwest};

use crate::{errors::SessionErrorKind, methods::TelegramMethod, utils::token};

use std::fmt::{self, Debug, Display, Formatter};
use tracing::instrument;

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
pub struct Bot<Client: ?Sized = Reqwest> {
    /// Bot token, which is used to receive updates and send requests to the Telegram API
    pub token: String,
    /// Bot token, which is used in `Debug` implementation for privacy
    pub hidden_token: String,
    /// Bot id, extracted from the token
    pub bot_id: i64,
    /// Client for sending requests to Telegram API
    client: Client,
}

impl Bot<Reqwest> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_client(token, Reqwest::default())
    }
}

impl<Client> Bot<Client> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn with_client(token: impl Into<String>, client: Client) -> Self {
        let token = token.into();
        let bot_id = token::extract_bot_id(&token).expect(
            "This bot token is invalid, please check it. \
                If you test your bot, and you don't have a token, use `Bot::default` method instead of `Bot::new`.",
        );
        let hidden_token = token::hide(&token);

        Self {
            token,
            hidden_token,
            bot_id,
            client,
        }
    }
}

impl<Client> Debug for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bot")
            .field("token", &self.hidden_token)
            .field("bot_id", &self.bot_id)
            .finish_non_exhaustive()
    }
}

impl<Client> Display for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Displaying only bot id and hidden token, because the token is sensitive data
        write!(
            f,
            "Bot {{ bot_id: {}, token: {} }}",
            self.bot_id, self.hidden_token,
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
    #[instrument(skip(self, method))]
    pub async fn send<T, TRef>(&self, method: TRef) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
        TRef: AsRef<T>,
    {
        self.client
            .make_request_and_get_result(self, method.as_ref(), None)
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
    pub async fn send_with_timeout<T, TRef>(
        &self,
        method: TRef,
        request_timeout: f32,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
        TRef: AsRef<T>,
    {
        self.client
            .make_request_and_get_result(self, method.as_ref(), Some(request_timeout))
            .await
    }
}
