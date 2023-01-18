use super::session::{base::Session, Reqwest};

use crate::{
    error::session,
    methods::{GetUpdates, SendPhoto, TelegramMethod},
    types::{InputFile, Message, MessageEntity, ReplyMarkup, Update, User},
};

use std::fmt::{self, Debug, Formatter};

/// Hide token for privacy. \
/// If token length is less than 4, then it will be hidden as `*`. \
/// For example,
/// `1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11` will be hidden as `12********11`
fn hide_token(token: &str) -> String {
    let token_len = token.len();

    if token_len < 4 {
        return "*".repeat(token_len);
    }

    let mut hidden = String::with_capacity(token_len);
    hidden.push_str(&token[..2]);
    hidden.push_str(&"*".repeat(8));
    hidden.push_str(&token[token_len - 2..]);
    hidden
}

/// Represents a bot with a token for getting updates and sending requests to Telegram API
#[derive(Clone, Default)]
pub struct Bot {
    /// Bot token, which is used to receive updates and send requests to the Telegram API
    token: String,
    /// Bot token, which is used in `Debug` implementation for privacy
    hidden_token: String,
    /// Client for sending requests to Telegram API
    client: Reqwest,
}

impl Bot {
    #[must_use]
    pub fn new<T>(token: T, client: Reqwest) -> Self
    where
        T: Into<String>,
    {
        let token = token.into();
        let hidden_token = hide_token(&token);

        Self {
            token,
            hidden_token,
            client,
        }
    }

    #[must_use]
    pub fn builder() -> BotBuilder {
        BotBuilder::default()
    }
}

impl Debug for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bot")
            .field("token", &self.hidden_token)
            .field("client", &self.client)
            .finish()
    }
}

#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct BotBuilder {
    token: String,
    client: Reqwest,
}

impl BotBuilder {
    /// Set bot token, which is used to receive updates and send requests to the Telegram API
    #[must_use]
    pub fn token<T>(mut self, token: T) -> Self
    where
        T: Into<String>,
    {
        self.token = token.into();
        self
    }

    /// Set client for sending requests to Telegram API
    #[must_use]
    pub fn client(mut self, client: Reqwest) -> Self {
        self.client = client;
        self
    }

    #[must_use]
    pub fn build(self) -> Bot {
        let token = self.token;
        let hidden_token = hide_token(&token);

        Bot {
            token,
            hidden_token,
            client: self.client,
        }
    }
}

/// A block of unrelated methods with Telegram methods
impl Bot {
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[must_use]
    pub fn hidden_token(&self) -> &str {
        &self.hidden_token
    }
}

/// A block of Telegram methods
impl Bot {
    #[must_use]
    pub fn get_me(&self) -> User {
        todo!()
    }

    /// Use this method to making request to Telegram API
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
    ) -> Result<T::Return, session::ErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        self.client
            .make_request_and_get_result(self, method, request_timeout)
            .await
    }

    /// Use this method to receive incoming updates using long polling (`wiki <https://en.wikipedia.org/wiki/Push_technology#Long_polling>`). \
    /// # Arguments
    /// * `offset` - Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [`crate::methods::get_updates::GetUpdates`] is called with an `offset` higher than its `update_id`. The negative offset can be specified to retrieve updates starting from `-offset` update from the end of the updates queue. All previous updates will forgotten.
    /// * `limit` - Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    /// * `timeout` - Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` - A JSON-serialized list of the update types you want your bot to receive. For example, specify [`message`, `edited_channel_post`, `callback_query`] to only receive updates of these types. See [`crate::types::Update`] for a complete list of available update types. Specify an empty list to receive all update types except `chat_member` (default). If not specified, the previous setting will be used.
    /// * `request_timeout` - Request timeout
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_updates(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
        timeout: Option<i64>,
        allowed_updates: Option<Vec<String>>,
        request_timeout: Option<f32>,
    ) -> Result<Vec<Update>, session::ErrorKind> {
        self.send(
            &GetUpdates {
                offset,
                limit,
                timeout,
                allowed_updates,
            },
            request_timeout,
        )
        .await
    }

    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    /// * `message_thread_id` - Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    /// * `photo` - Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. See `more information on Sending Files <https://core.telegram.org/bots/api#sending-files>`.
    /// * `caption` - Photo caption (may also be used when resending photos by *file_id*), 0-1024 characters after entities parsing
    /// * `parse_mode` - Mode for parsing entities in the photo caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    /// * `caption_entities` - List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    /// * `disable_notification` - Sends the message silently. Users will receive a notification with no sound.
    /// * `reply_to_message_id` - If the message is a reply, ID of the original message
    /// * `allow_sending_without_reply` - Pass `True` if the message should be sent even if the specified replied-to message is not found
    /// * `reply_markup` - Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    /// * `request_timeout` - Request timeout
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    #[allow(clippy::too_many_arguments)]
    pub async fn send_photo<'a>(
        &self,
        chat_id: i64,
        message_thread_id: Option<i64>,
        photo: InputFile<'a>,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, session::ErrorKind> {
        self.send(
            &SendPhoto {
                chat_id,
                message_thread_id,
                photo,
                caption,
                parse_mode,
                caption_entities,
                disable_notification,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_hide_token() {
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"), "12********11");
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew1"), "12********w1");
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew"), "12********ew");
        assert_eq!(hide_token("123"), "***");
        assert_eq!(hide_token("1234"), "12********34");
    }
}
