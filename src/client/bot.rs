use super::{session::base::Session, Reqwest};

use crate::{error::SessionErrorKind, methods::TelegramMethod, utils::token};

use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter},
};

/// Represents a bot with a token for getting updates and sending requests to Telegram API
/// # Warning
/// Using `default` method isn't recommended, because it doesn't check the token for validity.
/// This method is only for testing purposes.
#[derive(Clone, Default)]
pub struct Bot<Client: ?Sized = Reqwest> {
    /// Bot token, which is used to receive updates and send requests to the Telegram API
    token: Cow<'static, str>,
    /// Bot token, which is used in `Debug` implementation for privacy
    hidden_token: String,
    /// Bot id, extracted from the token
    bot_id: i64,
    /// Client for sending requests to Telegram API
    client: Client,
}

impl Bot<Reqwest> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn new(token: impl Into<Cow<'static, str>>) -> Self {
        Self::with_client(token, Reqwest::default())
    }
}

impl<Client> Bot<Client> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn with_client(token: impl Into<Cow<'static, str>>, client: Client) -> Self {
        let token = token.into();
        let bot_id =
            token::extract_bot_id(&token).expect(
                "This bot token is invalid, please check it. \
                If you test your bot, and you don't have a token, use `Bot::default` method instead of `Bot::new`."
            );
        let hidden_token = token::hide(&token);

        Self {
            token,
            hidden_token,
            bot_id,
            client,
        }
    }

    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[must_use]
    pub fn hidden_token(&self) -> &str {
        &self.hidden_token
    }

    #[must_use]
    pub const fn id(&self) -> i64 {
        self.bot_id
    }
}

impl<Client> Debug for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bot")
            .field("token", &self.hidden_token)
            .field("bot_id", &self.bot_id)
            .finish()
    }
}

impl<Client> Display for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hidden_token)
    }
}

/// A block of Telegram methods
impl<Client: Session> Bot<Client> {
    /// Use this method to send requests to Telegram API
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `request_timeout` - Request timeout
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send<T>(
        &self,
        method: &T,
        request_timeout: Option<f32>,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        self.client
            .make_request_and_get_result(self, method, request_timeout)
            .await
    }
}
