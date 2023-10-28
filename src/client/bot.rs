use super::{session::base::Session, Reqwest};

use crate::{errors::SessionErrorKind, methods::TelegramMethod, utils::token};

use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter},
};
use tracing::instrument;

/// Represents a bot with a token for getting updates and sending requests to Telegram API
#[derive(Clone, Default)]
pub struct Bot<Client: ?Sized = Reqwest> {
    /// Bot token, which is used to receive updates and send requests to the Telegram API
    pub token: Cow<'static, str>,
    /// Bot token, which is used in `Debug` implementation for privacy
    pub hidden_token: Box<str>,
    /// Bot id, extracted from the token
    pub bot_id: i64,
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
            .finish()
    }
}

impl<Client> Display for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hidden_token)
    }
}

impl<Client: Session> Bot<Client> {
    /// Use this method to send requests to Telegram API
    /// # Arguments
    /// * `method` - Telegram API method
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
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
    /// - If the response represents an telegram api error
    #[instrument(skip(self, method, request_timeout))]
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
