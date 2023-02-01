use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, Message, MessageEntity, ReplyMarkup},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send a native poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendpoll>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Poll question, 1-300 characters
    pub question: String,
    /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    /// `True`, if the poll needs to be anonymous, defaults to `True`
    pub is_anonymous: Option<bool>,
    /// Poll type, `quiz` or `regular`, defaults to `regular`
    #[serde(rename = "type")]
    pub poll_type: Option<String>,
    /// `True`, if the poll allows multiple answers, ignored for polls in `quiz` mode, defaults to `False`
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in `quiz` mode
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub explanation_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll explanation, which can be specified instead of `parse_mode`
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with `close_date`
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with `open_period`
    pub close_date: Option<i64>,
    /// Pass `True`, if the poll needs to be immediately closed. This can be useful for poll preview
    pub is_closed: Option<bool>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `True`, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendPoll {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, T: Into<String>>(
        chat_id: C,
        question: T,
        options: Vec<T>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            question: question.into(),
            options: options.into_iter().map(Into::into).collect(),
            is_anonymous: None,
            poll_type: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }

    #[must_use]
    pub fn question<T: Into<String>>(mut self, val: T) -> Self {
        self.question = val.into();
        self
    }

    #[must_use]
    pub fn options<T: Into<String>>(mut self, val: Vec<T>) -> Self {
        self.options = val.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn is_anonymous(mut self, val: bool) -> Self {
        self.is_anonymous = Some(val);
        self
    }

    #[must_use]
    pub fn poll_type<T: Into<String>>(mut self, val: T) -> Self {
        self.poll_type = Some(val.into());
        self
    }

    #[must_use]
    pub fn allows_multiple_answers(mut self, val: bool) -> Self {
        self.allows_multiple_answers = Some(val);
        self
    }

    #[must_use]
    pub fn correct_option_id(mut self, val: i64) -> Self {
        self.correct_option_id = Some(val);
        self
    }

    #[must_use]
    pub fn explanation<T: Into<String>>(mut self, val: T) -> Self {
        self.explanation = Some(val.into());
        self
    }

    #[must_use]
    pub fn explanation_parse_mode<T: Into<String>>(mut self, val: T) -> Self {
        self.explanation_parse_mode = Some(val.into());
        self
    }

    #[must_use]
    pub fn explanation_entities(mut self, val: Vec<MessageEntity>) -> Self {
        self.explanation_entities = Some(val);
        self
    }

    #[must_use]
    pub fn open_period(mut self, val: i64) -> Self {
        self.open_period = Some(val);
        self
    }

    #[must_use]
    pub fn close_date(mut self, val: i64) -> Self {
        self.close_date = Some(val);
        self
    }

    #[must_use]
    pub fn is_closed(mut self, val: bool) -> Self {
        self.is_closed = Some(val);
        self
    }

    #[must_use]
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    #[must_use]
    pub fn reply_to_message_id(mut self, val: i64) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, val: bool) -> Self {
        self.allow_sending_without_reply = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<ReplyMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl TelegramMethod for SendPoll {
    type Method = Self;
    type Return = Message;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("sendPoll", self, None)
    }
}
