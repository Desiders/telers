//! This module contains the [`Bot`] structure that represents a bot with its token and ID,
//! it also contains client for sending requests to Telegram API.
//!
//! You can use [`Bot::send`] method, which accepts any type that implements [`TelegramMethod`].
//! Methods from [`methods`] module are implemented with builders, so you don't need to pass all parameters to it,
//! only the required ones, and set optional ones using builder methods. Builders can also have some useful shortcuts.
//!
//! # Notes
//!
//! This structure is cheap to clone: the token is shared behind an [`Arc`] and the id is an
//! [`i64`]. The default client is [`Reqwest`], which is also cheap to clone.
//!
//! You can use a custom client by using the [`Bot::with_client`] method.
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
//! You can also use the [`Bot::send_with_timeout`] method to send requests with a timeout:
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
//! Files can be downloaded with the [`Bot::download`] method, which accepts a file ID or any object
//! that represents a file, for example [`PhotoSize`](crate::types::PhotoSize).
//! Check the [`download`] module docs for more information:
//!
//! ```rust
//! use telers::{types::PhotoSize, Bot};
//!
//! async fn save_photo(bot: Bot, photo: &PhotoSize) {
//!     let _ = bot
//!         .download(photo)
//!         .await
//!         .unwrap()
//!         .to_path("photo.jpg")
//!         .await;
//! }
//! ```
//!
//! More production examples can be found in the [`examples`] directory.
//!
//! [`examples`]: https://github.com/Desiders/telers/tree/dev-1.x/examples
//! [`methods`]: telers::methods

pub mod download;

use super::{session::base::Session, Reqwest};

use crate::{errors::SessionErrorKind, methods::TelegramMethod, utils::token};

use secrecy::SecretString;
use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};

/// Represents a bot with its token and ID, also contains client for sending requests to Telegram API.
/// # Notes
/// This structure is cheap to clone, because the token is shared behind an [`Arc`] and the id is an
/// [`i64`].
///
/// Default client is [`Reqwest`], which also is cheap to clone.
///
/// You can use a custom client by using the [`Bot::with_client`] method.
///
/// Check [module docs](crate::client::bot) for examples.
#[derive(Clone)]
pub struct Bot<Client = Reqwest> {
    /// Bot token, which is used to receive updates and send requests to the Telegram API.
    ///
    /// Wrapped in a [`SecretString`] so it is redacted in `Debug`/`Display` output and zeroized when
    /// the last clone is dropped. Read it with [`Bot::token`].
    token: Arc<SecretString>,
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
            "This bot token is invalid, please check it. If you test your bot, and you don't have \
             a token, use `Bot::default` method instead of `Bot::new`.",
        );

        Self {
            token: Arc::new(SecretString::from(token.into_boxed_str())),
            id,
            client,
        }
    }

    /// Bot token, which is used to receive updates and send requests to the Telegram API.
    ///
    /// # Notes
    /// The token is a [`SecretString`], so it is redacted in `Debug`/`Display` output and zeroized
    /// once the last [`Bot`] clone is dropped. Reading it is deliberate and auditable — call
    /// [`ExposeSecret::expose_secret`] on the returned value:
    ///
    /// ```rust
    /// use telers::{client::ExposeSecret as _, Bot};
    ///
    /// let bot = Bot::new("123:token");
    ///
    /// assert_eq!(bot.token().expose_secret(), "123:token");
    /// ```
    ///
    /// [`ExposeSecret::expose_secret`]: secrecy::ExposeSecret::expose_secret
    #[must_use]
    pub fn token(&self) -> &SecretString {
        &self.token
    }
}

impl<Client: Default> Default for Bot<Client> {
    /// Creates a bot with an empty token and a zero id, for tests and other cases where no real
    /// token is available
    fn default() -> Self {
        Self {
            token: Arc::new(SecretString::from(Box::<str>::from(""))),
            id: 0,
            client: Client::default(),
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
            .field("token", &self.token)
            .field("bot_id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<Client> Display for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Bot {{ bot_id: {}, token: {:?} }}", self.id, self.token)
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
