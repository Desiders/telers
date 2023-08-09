use crate::{
    client::{telegram::APIServer, Bot},
    errors::{SessionErrorKind, TelegramErrorKind},
    methods::{Response, TelegramMethod},
};

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::{
    fmt::{self, Display, Formatter},
    ops::RangeInclusive,
};
use tracing::{event, instrument, Level, Span};

pub const DEFAULT_TIMEOUT: f32 = 60.0;

#[derive(Debug)]
pub struct StatusCode(u16);

impl StatusCode {
    const SUCESS_STATUS_CODE_RANGE: RangeInclusive<u16> = 200..=226;

    #[must_use]
    pub fn new(status_code: u16) -> Self {
        Self(status_code)
    }

    #[must_use]
    pub fn is_success(&self) -> bool {
        Self::SUCESS_STATUS_CODE_RANGE.contains(&self.0)
    }

    #[must_use]
    pub fn is_error(&self) -> bool {
        !self.is_success()
    }

    #[must_use]
    pub const fn as_u16(&self) -> u16 {
        self.0
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl From<u16> for StatusCode {
    fn from(status_code: u16) -> Self {
        Self::new(status_code)
    }
}

#[derive(Debug)]
pub struct ClientResponse {
    pub status_code: StatusCode,
    pub content: String,
}

impl ClientResponse {
    #[must_use]
    pub fn new(status_code: impl Into<StatusCode>, content: String) -> Self {
        Self {
            status_code: status_code.into(),
            content,
        }
    }
}

#[async_trait]
pub trait Session: Send + Sync {
    /// Get configuration of Telegram Bot API server endpoints and local mode
    #[must_use]
    fn api(&self) -> &APIServer;

    /// Makes a request to Telegram API
    /// # Arguments
    /// * `bot` - Bot instance for building request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building request
    /// * `timeout` - Request timeout.
    /// If `None`, then client timeout will be used, which is [`DEFAULT_TIMEOUT`] by default.
    /// # Errors
    /// If the request cannot be send or decoded
    #[must_use]
    async fn send_request<Client, T>(
        &self,
        bot: &Bot<Client>,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync;

    /// Checks a response from Telegram API
    /// # Arguments
    /// * `method` - Telegram method
    /// * `status_code` - HTTP status code
    /// * `content` - Response content
    /// # Errors
    /// If the response represents an telegram api error
    #[allow(clippy::redundant_else)]
    #[instrument(skip(self, response, status_code), fields(ok, error_message))]
    fn check_response(
        &self,
        response: &Response<impl DeserializeOwned>,
        status_code: &StatusCode,
    ) -> Result<(), TelegramErrorKind> {
        if status_code.is_success() && response.ok {
            Span::current().record("ok", true);

            if response.result.is_none() {
                event!(
                    Level::ERROR,
                    "Contract violation: result is empty in success response"
                );

                let err: TelegramErrorKind =
                    anyhow::Error::msg("Contract violation: result is empty in success response")
                        .into();

                return Err(err);
            }

            return Ok(());
        } else {
            Span::current().record("ok", false);
        }

        let message = if let Some(ref description) = response.description {
            description.to_string()
        } else {
            // Descriptions for every error mentioned in errors (https://core.telegram.org/api/errors)
            event!(
                Level::ERROR,
                description = ?response.description,
                "Contract violation: description is empty in error response"
            );

            let err: TelegramErrorKind =
                anyhow::Error::msg("Contract violation: description is empty in error response")
                    .into();

            return Err(err);
        };

        Span::current().record("error_message", &message);

        if let Some(ref parameters) = response.parameters {
            if let Some(retry_after) = parameters.retry_after {
                return Err(TelegramErrorKind::RetryAfter {
                    url: "https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this",
                    message,
                    retry_after,
                });
            }
            if let Some(migrate_to_chat_id) = parameters.migrate_to_chat_id {
                return Err(TelegramErrorKind::MigrateToChat {
                    url: "https://core.telegram.org/bots/api#responseparameters",
                    message,
                    migrate_to_chat_id,
                });
            }
        }

        let err = match status_code.as_u16() {
            400 => TelegramErrorKind::BadRequest { message },
            401 => TelegramErrorKind::Unauthorized { message },
            403 => TelegramErrorKind::Forbidden { message },
            404 => TelegramErrorKind::NotFound { message },
            409 => TelegramErrorKind::ConflictError { message },
            413 => TelegramErrorKind::EntityTooLarge {
                url: "https://core.telegram.org/bots/api#sending-files",
                message,
            },
            500 => {
                if message.contains("restart") {
                    TelegramErrorKind::RestartingTelegram { message }
                } else {
                    TelegramErrorKind::ServerError { message }
                }
            }
            _ => {
                event!(Level::ERROR, "Unknown status code",);

                anyhow::Error::msg(message).into()
            }
        };

        Err(err)
    }

    /// Makes a request to Telegram API
    /// # Arguments
    /// * `bot` - Bot instance for building and sending request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building and sending request
    /// * `timeout` - Request timeout.
    /// If `None`, then client timeout will be used, which is [`DEFAULT_TIMEOUT`] by default.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    #[instrument(skip(self, bot, method, timeout), fields(bot_id))]
    async fn make_request<Client, T>(
        &self,
        bot: &Bot<Client>,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<Response<T::Return>, SessionErrorKind>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        Span::current().record("bot_id", bot.bot_id);

        let response = self
            .send_request(bot, method, timeout)
            .await
            .map_err(|err| {
                event!(
                    Level::ERROR,
                    error = %err,
                    "Cannot send request to Telegram API",
                );

                err
            })?;

        let telegram_response = method.build_response(&response.content).map_err(|err| {
            event!(
                Level::ERROR,
                error = %err,
                response_content = ?response.content,
                "Cannot parse response content",
            );

            err
        })?;

        self.check_response(&telegram_response, &response.status_code)
            .map_err(|err| {
                event!(
                    Level::ERROR,
                    error = %err,
                    "Response represents an telegram api error",
                );

                err
            })?;

        Ok(telegram_response)
    }

    /// Makes a request to Telegram API and get result from it
    /// # Arguments
    /// * `bot` - Bot instance for building and sending request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building and sending request
    /// * `timeout` - Request timeout.
    /// If `None`, then client timeout will be used, which is [`DEFAULT_TIMEOUT`] by default.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    async fn make_request_and_get_result<Client, T>(
        &self,
        bot: &Bot<Client>,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<T::Return, SessionErrorKind>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let response = self.make_request(bot, method, timeout).await?;

        // Unwrap safe because we checked it in `check_response`
        Ok(response.result.unwrap())
    }

    /// Close client session. Default implementation does nothing.
    async fn close(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
