use crate::{
    client::Bot,
    error::{session, telegram},
    methods::{Response, TelegramMethod},
};

use async_trait::async_trait;
use std::ops::RangeInclusive;

pub const DEFAULT_TIMEOUT: f32 = 60.0;
pub const SUCESS_STATUS_CODE_RANGE: RangeInclusive<u16> = 200..=226;

#[async_trait]
pub trait Session: Sized {
    /// Checks a response from Telegram API
    /// # Arguments
    /// * `method` - Telegram method
    /// * `status_code` - HTTP status code
    /// * `content` - Response content
    /// # Errors
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    fn check_response<Method>(
        &self,
        method: Method,
        status_code: u16,
        content: &str,
    ) -> Result<Response<Method::Return>, session::ErrorKind>
    where
        Method: TelegramMethod,
    {
        let response = method.build_response(content)?;

        // Check if the response isn't an error
        if SUCESS_STATUS_CODE_RANGE.contains(&status_code) && response.ok() {
            if response.result().is_none() {
                log::error!("Contract violation: result is empty in success response");

                let err: telegram::ErrorKind =
                    anyhow::Error::msg("Contract violation: result is empty in success response")
                        .into();

                return Err(err.into());
            }

            return Ok(response);
        }

        let message = if let Some(description) = response.description() {
            description.to_string()
        } else {
            // Descriptions for every error mentioned in errors (https://core.telegram.org/api/errors)
            log::error!("Contract violation: description is empty in error response");

            let err: telegram::ErrorKind =
                anyhow::Error::msg("Contract violation: description is empty in error response")
                    .into();

            return Err(err.into());
        };

        if let Some(parameters) = response.parameters() {
            if let Some(retry_after) = parameters.retry_after {
                return Err(telegram::ErrorKind::RetryAfter {
                    url: "https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this",
                    message,
                    retry_after,
                }.into());
            }
            if let Some(migrate_to_chat_id) = parameters.migrate_to_chat_id {
                return Err(telegram::ErrorKind::MigrateToChat {
                    url: "https://core.telegram.org/bots/api#responseparameters",
                    message,
                    migrate_to_chat_id,
                }
                .into());
            }
        }

        let err = match status_code {
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

        Err(err.into())
    }

    /// Close client session. Default implementation does nothing.
    async fn close(&self) -> Result<(), session::ErrorKind> {
        Ok(())
    }

    /// Makes a request to Telegram API
    /// # Arguments
    /// * `bot` - Bot instance for building request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building request and parsing response
    /// * `timeout` - *Optional*. Request timeout.
    /// If `None`, then client timeout will be used, which is [`DEFAULT_TIMEOUT`] by default.
    #[must_use]
    async fn make_request<T>(
        &self,
        bot: &Bot,
        method: T,
        timeout: Option<f32>,
    ) -> Result<T::Return, session::ErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync;

    // TODO: Implement streaming
    // async fn stream_content(&self) ->;
}
