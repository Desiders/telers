use crate::client::Bot;
use serde::Serialize;
/// Use this method to send a native poll. On success, the sent Message is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendpoll>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct SendPoll {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername). Polls can't be sent to channel direct messages chats.
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Poll question, 1-300 characters
    pub question: Box<str>,
    /// Mode for parsing entities in the question. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of `question_parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// A JSON-serialized list of 2-12 answer options
    pub options: Box<[crate::types::InputPollOption]>,
    /// `true`, if the poll needs to be anonymous, defaults to `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, `quiz` or `regular`, defaults to `regular`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
    /// `true`, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<str>>,
    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of `explanation_parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with `close_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u16>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with `open_period`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// Pass `true` if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<Box<str>>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<crate::types::ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::ReplyMarkup>,
}
impl SendPoll {
    /// Creates a new `SendPoll`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername). Polls can't be sent to channel direct messages chats.
    /// * `question` - Poll question, 1-300 characters
    /// * `options` - A JSON-serialized list of 2-12 answer options
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::ChatIdKind>,
        T1: Into<Box<str>>,
        T2Item: Into<crate::types::InputPollOption>,
        T2: IntoIterator<Item = T2Item>,
    >(
        chat_id: T0,
        question: T1,
        options: T2,
    ) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            question: question.into(),
            question_parse_mode: None,
            question_entities: None,
            options: options.into_iter().map(Into::into).collect(),
            is_anonymous: None,
            r#type: None,
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
            allow_paid_broadcast: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = Some(val.into());
        this
    }

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.business_connection_id = val.map(Into::into);
        this
    }

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername). Polls can't be sent to channel direct messages chats.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = Some(val.into());
        this
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_thread_id = val.map(Into::into);
        this
    }

    /// Poll question, 1-300 characters
    #[must_use]
    pub fn question<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.question = val.into();
        this
    }

    /// Mode for parsing entities in the question. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[must_use]
    pub fn question_parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.question_parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the question. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[must_use]
    pub fn question_parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.question_parse_mode = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of `question_parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn question_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.question_entities = Some(
            this.question_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of `question_parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn question_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.question_entities = Some(
            this.question_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of `question_parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn question_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.question_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }

    /// A JSON-serialized list of 2-12 answer options
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn options<TItem: Into<crate::types::InputPollOption>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.options = this
            .options
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        this
    }

    /// A JSON-serialized list of 2-12 answer options
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option<T: Into<crate::types::InputPollOption>>(self, val: T) -> Self {
        let mut this = self;
        this.options = this
            .options
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// `true`, if the poll needs to be anonymous, defaults to `true`
    #[must_use]
    pub fn is_anonymous<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_anonymous = Some(val.into());
        this
    }

    /// `true`, if the poll needs to be anonymous, defaults to `true`
    #[must_use]
    pub fn is_anonymous_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_anonymous = val.map(Into::into);
        this
    }

    /// Poll type, `quiz` or `regular`, defaults to `regular`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.r#type = Some(val.into());
        this
    }

    /// Poll type, `quiz` or `regular`, defaults to `regular`
    #[must_use]
    pub fn type_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.r#type = val.map(Into::into);
        this
    }

    /// `true`, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to `false`
    #[must_use]
    pub fn allows_multiple_answers<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allows_multiple_answers = Some(val.into());
        this
    }

    /// `true`, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to `false`
    #[must_use]
    pub fn allows_multiple_answers_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allows_multiple_answers = val.map(Into::into);
        this
    }

    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[must_use]
    pub fn correct_option_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.correct_option_id = Some(val.into());
        this
    }

    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[must_use]
    pub fn correct_option_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.correct_option_id = val.map(Into::into);
        this
    }

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[must_use]
    pub fn explanation<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.explanation = Some(val.into());
        this
    }

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[must_use]
    pub fn explanation_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.explanation = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[must_use]
    pub fn explanation_parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.explanation_parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[must_use]
    pub fn explanation_parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.explanation_parse_mode = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of `explanation_parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn explanation_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.explanation_entities = Some(
            this.explanation_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of `explanation_parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn explanation_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.explanation_entities = Some(
            this.explanation_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of `explanation_parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn explanation_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.explanation_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }

    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with `close_date`.
    #[must_use]
    pub fn open_period<T: Into<u16>>(self, val: T) -> Self {
        let mut this = self;
        this.open_period = Some(val.into());
        this
    }

    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with `close_date`.
    #[must_use]
    pub fn open_period_option<T: Into<u16>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.open_period = val.map(Into::into);
        this
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with `open_period`.
    #[must_use]
    pub fn close_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.close_date = Some(val.into());
        this
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with `open_period`.
    #[must_use]
    pub fn close_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.close_date = val.map(Into::into);
        this
    }

    /// Pass `true` if the poll needs to be immediately closed. This can be useful for poll preview.
    #[must_use]
    pub fn is_closed<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_closed = Some(val.into());
        this
    }

    /// Pass `true` if the poll needs to be immediately closed. This can be useful for poll preview.
    #[must_use]
    pub fn is_closed_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_closed = val.map(Into::into);
        this
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.disable_notification = Some(val.into());
        this
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.disable_notification = val.map(Into::into);
        this
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.protect_content = Some(val.into());
        this
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.protect_content = val.map(Into::into);
        this
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[must_use]
    pub fn allow_paid_broadcast<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allow_paid_broadcast = Some(val.into());
        this
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[must_use]
    pub fn allow_paid_broadcast_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allow_paid_broadcast = val.map(Into::into);
        this
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.message_effect_id = Some(val.into());
        this
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_effect_id = val.map(Into::into);
        this
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters<T: Into<crate::types::ReplyParameters>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_parameters = Some(val.into());
        this
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters_option<T: Into<crate::types::ReplyParameters>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_parameters = val.map(Into::into);
        this
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::ReplyMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::ReplyMarkup>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SendPoll {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendPoll", self, None)
    }
}
