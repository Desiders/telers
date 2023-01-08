use crate::{
    client::Bot,
    error::{session, telegram},
    methods::{Request, Response, TelegramMethod},
};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::ops::RangeInclusive;

pub const DEFAULT_TIMEOUT: f32 = 60.0;

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
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl From<u16> for StatusCode {
    fn from(status_code: u16) -> Self {
        Self::new(status_code)
    }
}

pub struct ClientResponse {
    status_code: StatusCode,
    content: String,
}

impl ClientResponse {
    #[must_use]
    pub fn new(status_code: impl Into<StatusCode>, content: String) -> Self {
        Self {
            status_code: status_code.into(),
            content,
        }
    }

    #[must_use]
    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[async_trait]
pub trait Session {
    #[must_use]
    async fn send_json<'a, T>(
        &self,
        request: Request<'a, T>,
        url: &str,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        T: Serialize + Send + Sync;

    #[must_use]
    async fn send_multipart<'a, T>(
        &self,
        request: Request<'a, T>,
        url: &str,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        T: Serialize + Send + Sync;

    /// Makes a request to Telegram API
    /// # Arguments
    /// * `bot` - Bot instance for building request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building request
    /// * `timeout` - *Optional*. Request timeout.
    /// If `None`, then client timeout will be used, which is [`DEFAULT_TIMEOUT`] by default.
    /// # Errors
    /// If the request cannot be send or decoded
    #[must_use]
    async fn send_request<T>(
        &self,
        bot: &Bot,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync;

    /// Checks a response from Telegram API
    /// # Arguments
    /// * `method` - Telegram method
    /// * `status_code` - HTTP status code
    /// * `content` - Response content
    /// # Errors
    /// If the response represents an telegram api error
    fn check_response<T>(
        &self,
        response: &Response<T>,
        status_code: &StatusCode,
    ) -> Result<(), telegram::ErrorKind>
    where
        T: DeserializeOwned,
    {
        if status_code.is_success() && response.ok() {
            if response.result().is_none() {
                log::error!("Contract violation: result is empty in success response");

                let err: telegram::ErrorKind =
                    anyhow::Error::msg("Contract violation: result is empty in success response")
                        .into();

                return Err(err);
            }

            return Ok(());
        }

        let message = if let Some(description) = response.description() {
            description.to_string()
        } else {
            // Descriptions for every error mentioned in errors (https://core.telegram.org/api/errors)
            log::error!("Contract violation: description is empty in error response");

            let err: telegram::ErrorKind =
                anyhow::Error::msg("Contract violation: description is empty in error response")
                    .into();

            return Err(err);
        };

        if let Some(parameters) = response.parameters() {
            if let Some(retry_after) = parameters.retry_after {
                return Err(telegram::ErrorKind::RetryAfter {
                    url: "https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this",
                    message,
                    retry_after,
                });
            }
            if let Some(migrate_to_chat_id) = parameters.migrate_to_chat_id {
                return Err(telegram::ErrorKind::MigrateToChat {
                    url: "https://core.telegram.org/bots/api#responseparameters",
                    message,
                    migrate_to_chat_id,
                });
            }
        }

        let err = match status_code.as_u16() {
            400 => telegram::ErrorKind::BadRequest { message },
            401 => telegram::ErrorKind::Unauthorized { message },
            403 => telegram::ErrorKind::Forbidden { message },
            404 => telegram::ErrorKind::NotFound { message },
            409 => telegram::ErrorKind::ConflictError { message },
            413 => telegram::ErrorKind::EntityTooLarge {
                url: "https://core.telegram.org/bots/api#sending-files",
                message,
            },
            500 => {
                if message.contains("restart") {
                    telegram::ErrorKind::RestartingTelegram { message }
                } else {
                    telegram::ErrorKind::ServerError { message }
                }
            }
            _ => {
                log::error!("Contract violation: unknown status code");

                anyhow::Error::msg(message).into()
            }
        };

        Err(err)
    }

    /// Makes a request to Telegram API
    /// # Arguments
    /// * `bot` - Bot instance for building and sending request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building and sending request
    /// * `timeout` - *Optional*. Request timeout.
    /// If `None`, then client timeout will be used, which is [`DEFAULT_TIMEOUT`] by default.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    async fn make_request<T>(
        &self,
        bot: &Bot,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<Response<T::Return>, session::ErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let client_response = self
            .send_request(bot, method, timeout)
            .await
            .map_err(session::ErrorKind::Client)?;

        let response = method
            .build_response(client_response.content())
            .map_err(|err| {
                log::error!("Cannot parse response: {err}");

                session::ErrorKind::Parse(err)
            })?;

        self.check_response(&response, client_response.status_code())?;

        Ok(response)
    }

    /// Makes a request to Telegram API and get result from it
    /// # Arguments
    /// * `bot` - Bot instance for building and sending request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building and sending request
    /// * `timeout` - *Optional*. Request timeout.
    /// If `None`, then client timeout will be used, which is [`DEFAULT_TIMEOUT`] by default.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    async fn make_request_and_get_result<T>(
        &self,
        bot: &Bot,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<T::Return, session::ErrorKind>
    where
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
