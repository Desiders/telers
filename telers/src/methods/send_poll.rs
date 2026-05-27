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
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Polls can't be sent to channel direct messages chats.
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
    /// A JSON-serialized list of 1-12 answer options
    pub options: Box<[crate::types::InputPollOption]>,
    /// `true`, if the poll needs to be anonymous, defaults to `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, `quiz` or `regular`, defaults to `regular`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
    /// Pass `true`, if the poll allows multiple answers, defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// Pass `true`, if the poll allows to change chosen answer options, defaults to `false` for quizzes and to `true` for regular polls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_revoting: Option<bool>,
    /// Pass `true`, if the poll options must be shown in random order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shuffle_options: Option<bool>,
    /// Pass `true`, if answer options can be added to the poll after creation; not supported for anonymous polls and quizzes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_adding_options: Option<bool>,
    /// Pass `true`, if poll results must be shown only after the poll closes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_results_until_closes: Option<bool>,
    /// Pass `true`, if voting is limited to users who have been members of the chat where the poll is being sent for more than 24 hours; for channel chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_only: Option<bool>,
    /// A JSON-serialized list of 0-12 two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which users can vote in the poll; for channel chats only. If omitted or empty, then users from any country can participate in the poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Box<[Box<str>]>>,
    /// A JSON-serialized list of monotonically increasing 0-based identifiers of the correct answer options, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_ids: Option<Box<[i64]>>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<str>>,
    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of `explanation_parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Media added to the quiz explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_media: Option<crate::types::InputPollMedia>,
    /// Amount of time in seconds the poll will be active after creation, 5-2628000. Can't be used together with `close_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 2628000 seconds in the future. Can't be used together with `open_period`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// Pass `true` if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// Description of the poll to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Mode for parsing entities in the poll description. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the poll description, which can be specified instead of `description_parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Media added to the poll description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<crate::types::InputPollMedia>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
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
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Polls can't be sent to channel direct messages chats.
    /// * `question` - Poll question, 1-300 characters
    /// * `options` - A JSON-serialized list of 1-12 answer options
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
            allows_revoting: None,
            shuffle_options: None,
            allow_adding_options: None,
            hide_results_until_closes: None,
            members_only: None,
            country_codes: None,
            correct_option_ids: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            explanation_media: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            description: None,
            description_parse_mode: None,
            description_entities: None,
            media: None,
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
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.business_connection_id = val.map(Into::into);
        self
    }

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Polls can't be sent to channel direct messages chats.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = Some(val.into());
        self
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_thread_id = val.map(Into::into);
        self
    }

    /// Poll question, 1-300 characters
    #[must_use]
    pub fn question<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.question = val.into();
        self
    }

    /// Mode for parsing entities in the question. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[must_use]
    pub fn question_parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.question_parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the question. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[must_use]
    pub fn question_parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.question_parse_mode = val.map(Into::into);
        self
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
        mut self,
        val: T,
    ) -> Self {
        self.question_entities = Some(
            self.question_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of `question_parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn question_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.question_entities = Some(
            self.question_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
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
        mut self,
        val: Option<T>,
    ) -> Self {
        self.question_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// A JSON-serialized list of 1-12 answer options
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn options<TItem: Into<crate::types::InputPollOption>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.options = self
            .options
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        self
    }

    /// A JSON-serialized list of 1-12 answer options
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option<T: Into<crate::types::InputPollOption>>(mut self, val: T) -> Self {
        self.options = self
            .options
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// `true`, if the poll needs to be anonymous, defaults to `true`
    #[must_use]
    pub fn is_anonymous<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_anonymous = Some(val.into());
        self
    }

    /// `true`, if the poll needs to be anonymous, defaults to `true`
    #[must_use]
    pub fn is_anonymous_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_anonymous = val.map(Into::into);
        self
    }

    /// Poll type, `quiz` or `regular`, defaults to `regular`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = Some(val.into());
        self
    }

    /// Poll type, `quiz` or `regular`, defaults to `regular`
    #[must_use]
    pub fn type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.r#type = val.map(Into::into);
        self
    }

    /// Pass `true`, if the poll allows multiple answers, defaults to `false`
    #[must_use]
    pub fn allows_multiple_answers<T: Into<bool>>(mut self, val: T) -> Self {
        self.allows_multiple_answers = Some(val.into());
        self
    }

    /// Pass `true`, if the poll allows multiple answers, defaults to `false`
    #[must_use]
    pub fn allows_multiple_answers_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allows_multiple_answers = val.map(Into::into);
        self
    }

    /// Pass `true`, if the poll allows to change chosen answer options, defaults to `false` for quizzes and to `true` for regular polls
    #[must_use]
    pub fn allows_revoting<T: Into<bool>>(mut self, val: T) -> Self {
        self.allows_revoting = Some(val.into());
        self
    }

    /// Pass `true`, if the poll allows to change chosen answer options, defaults to `false` for quizzes and to `true` for regular polls
    #[must_use]
    pub fn allows_revoting_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allows_revoting = val.map(Into::into);
        self
    }

    /// Pass `true`, if the poll options must be shown in random order
    #[must_use]
    pub fn shuffle_options<T: Into<bool>>(mut self, val: T) -> Self {
        self.shuffle_options = Some(val.into());
        self
    }

    /// Pass `true`, if the poll options must be shown in random order
    #[must_use]
    pub fn shuffle_options_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.shuffle_options = val.map(Into::into);
        self
    }

    /// Pass `true`, if answer options can be added to the poll after creation; not supported for anonymous polls and quizzes
    #[must_use]
    pub fn allow_adding_options<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_adding_options = Some(val.into());
        self
    }

    /// Pass `true`, if answer options can be added to the poll after creation; not supported for anonymous polls and quizzes
    #[must_use]
    pub fn allow_adding_options_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_adding_options = val.map(Into::into);
        self
    }

    /// Pass `true`, if poll results must be shown only after the poll closes
    #[must_use]
    pub fn hide_results_until_closes<T: Into<bool>>(mut self, val: T) -> Self {
        self.hide_results_until_closes = Some(val.into());
        self
    }

    /// Pass `true`, if poll results must be shown only after the poll closes
    #[must_use]
    pub fn hide_results_until_closes_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.hide_results_until_closes = val.map(Into::into);
        self
    }

    /// Pass `true`, if voting is limited to users who have been members of the chat where the poll is being sent for more than 24 hours; for channel chats only
    #[must_use]
    pub fn members_only<T: Into<bool>>(mut self, val: T) -> Self {
        self.members_only = Some(val.into());
        self
    }

    /// Pass `true`, if voting is limited to users who have been members of the chat where the poll is being sent for more than 24 hours; for channel chats only
    #[must_use]
    pub fn members_only_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.members_only = val.map(Into::into);
        self
    }

    /// A JSON-serialized list of 0-12 two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which users can vote in the poll; for channel chats only. If omitted or empty, then users from any country can participate in the poll.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn country_codes<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.country_codes = Some(
            self.country_codes
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of 0-12 two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which users can vote in the poll; for channel chats only. If omitted or empty, then users from any country can participate in the poll.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn country_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.country_codes = Some(
            self.country_codes
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of 0-12 two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which users can vote in the poll; for channel chats only. If omitted or empty, then users from any country can participate in the poll.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn country_codes_option<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.country_codes = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// A JSON-serialized list of monotonically increasing 0-based identifiers of the correct answer options, required for polls in quiz mode
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn correct_option_ids<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.correct_option_ids = Some(
            self.correct_option_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of monotonically increasing 0-based identifiers of the correct answer options, required for polls in quiz mode
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn correct_option_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.correct_option_ids = Some(
            self.correct_option_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of monotonically increasing 0-based identifiers of the correct answer options, required for polls in quiz mode
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn correct_option_ids_option<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.correct_option_ids = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[must_use]
    pub fn explanation<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.explanation = Some(val.into());
        self
    }

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[must_use]
    pub fn explanation_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.explanation = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[must_use]
    pub fn explanation_parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.explanation_parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[must_use]
    pub fn explanation_parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.explanation_parse_mode = val.map(Into::into);
        self
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
        mut self,
        val: T,
    ) -> Self {
        self.explanation_entities = Some(
            self.explanation_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of `explanation_parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn explanation_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.explanation_entities = Some(
            self.explanation_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
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
        mut self,
        val: Option<T>,
    ) -> Self {
        self.explanation_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// Media added to the quiz explanation
    #[must_use]
    pub fn explanation_media<T: Into<crate::types::InputPollMedia>>(mut self, val: T) -> Self {
        self.explanation_media = Some(val.into());
        self
    }

    /// Media added to the quiz explanation
    #[must_use]
    pub fn explanation_media_option<T: Into<crate::types::InputPollMedia>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.explanation_media = val.map(Into::into);
        self
    }

    /// Amount of time in seconds the poll will be active after creation, 5-2628000. Can't be used together with `close_date`.
    #[must_use]
    pub fn open_period<T: Into<u32>>(mut self, val: T) -> Self {
        self.open_period = Some(val.into());
        self
    }

    /// Amount of time in seconds the poll will be active after creation, 5-2628000. Can't be used together with `close_date`.
    #[must_use]
    pub fn open_period_option<T: Into<u32>>(mut self, val: Option<T>) -> Self {
        self.open_period = val.map(Into::into);
        self
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 2628000 seconds in the future. Can't be used together with `open_period`.
    #[must_use]
    pub fn close_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.close_date = Some(val.into());
        self
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 2628000 seconds in the future. Can't be used together with `open_period`.
    #[must_use]
    pub fn close_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.close_date = val.map(Into::into);
        self
    }

    /// Pass `true` if the poll needs to be immediately closed. This can be useful for poll preview.
    #[must_use]
    pub fn is_closed<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_closed = Some(val.into());
        self
    }

    /// Pass `true` if the poll needs to be immediately closed. This can be useful for poll preview.
    #[must_use]
    pub fn is_closed_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_closed = val.map(Into::into);
        self
    }

    /// Description of the poll to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
        self
    }

    /// Description of the poll to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn description_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.description = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the poll description. See formatting options for more details.
    #[must_use]
    pub fn description_parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description_parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the poll description. See formatting options for more details.
    #[must_use]
    pub fn description_parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.description_parse_mode = val.map(Into::into);
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll description, which can be specified instead of `description_parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn description_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: T,
    ) -> Self {
        self.description_entities = Some(
            self.description_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll description, which can be specified instead of `description_parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn description_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.description_entities = Some(
            self.description_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll description, which can be specified instead of `description_parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn description_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.description_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// Media added to the poll description
    #[must_use]
    pub fn media<T: Into<crate::types::InputPollMedia>>(mut self, val: T) -> Self {
        self.media = Some(val.into());
        self
    }

    /// Media added to the poll description
    #[must_use]
    pub fn media_option<T: Into<crate::types::InputPollMedia>>(mut self, val: Option<T>) -> Self {
        self.media = val.map(Into::into);
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.disable_notification = val.map(Into::into);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.protect_content = Some(val.into());
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.protect_content = val.map(Into::into);
        self
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_paid_broadcast = Some(val.into());
        self
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_paid_broadcast = val.map(Into::into);
        self
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.message_effect_id = val.map(Into::into);
        self
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters<T: Into<crate::types::ReplyParameters>>(mut self, val: T) -> Self {
        self.reply_parameters = Some(val.into());
        self
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters_option<T: Into<crate::types::ReplyParameters>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_parameters = val.map(Into::into);
        self
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::ReplyMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::ReplyMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SendPoll {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendPoll", self, None)
    }
}
