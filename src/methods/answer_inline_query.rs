use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::InlineQueryResult};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send answers to an inline query. No more than 50 results per query are allowed.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerinlinequery>
/// # Returns
/// On success, `True` is returned
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    pub cache_time: Option<i32>,
    /// Pass `True` if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don‘t support pagination. Offset length can’t exceed 64 bytes.
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a private chat with the bot and sends the bot a start message with the parameter `switch_pm_parameter`
    pub switch_pm_text: Option<String>,
    /// [Deep-linking](https://core.telegram.org/bots/features#deep-linking) parameter for the /start message sent to the bot when user presses the switch button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    /// Example: An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a ‘Connect your YouTube account’ button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an oauth link. Once done, the bot can offer a switch_inline button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    pub switch_pm_parameter: Option<String>,
}

impl AnswerInlineQuery {
    #[must_use]
    pub fn new<T: Into<String>, E: Into<InlineQueryResult>>(
        inline_query_id: T,
        results: Vec<E>,
    ) -> Self {
        Self {
            inline_query_id: inline_query_id.into(),
            results: results.into_iter().map(Into::into).collect(),
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    #[must_use]
    pub fn results<T: Into<InlineQueryResult>>(mut self, val: Vec<T>) -> Self {
        self.results = val.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn result<T: Into<InlineQueryResult>>(mut self, val: T) -> Self {
        self.results.push(val.into());
        self
    }

    #[must_use]
    pub fn cache_time(mut self, val: i32) -> Self {
        self.cache_time = Some(val);
        self
    }

    #[must_use]
    pub fn is_personal(mut self, val: bool) -> Self {
        self.is_personal = Some(val);
        self
    }

    #[must_use]
    pub fn next_offset<T: Into<String>>(mut self, val: T) -> Self {
        self.next_offset = Some(val.into());
        self
    }

    #[must_use]
    pub fn switch_pm_text<T: Into<String>>(mut self, val: T) -> Self {
        self.switch_pm_text = Some(val.into());
        self
    }

    #[must_use]
    pub fn switch_pm_parameter<T: Into<String>>(mut self, val: T) -> Self {
        self.switch_pm_parameter = Some(val.into());
        self
    }
}

impl TelegramMethod for AnswerInlineQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("answerInlineQuery", self, None)
    }
}