use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InlineQueryResult, InlineQueryResultsButton},
};

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
    /// A JSON-serialized object describing a button to be shown above inline query results
    pub button: Option<InlineQueryResultsButton>,
}

impl AnswerInlineQuery {
    #[must_use]
    pub fn new<T, R, I>(inline_query_id: T, results: I) -> Self
    where
        T: Into<String>,
        R: Into<InlineQueryResult>,
        I: IntoIterator<Item = R>,
    {
        Self {
            inline_query_id: inline_query_id.into(),
            results: results.into_iter().map(Into::into).collect(),
            cache_time: None,
            is_personal: None,
            next_offset: None,
            button: None,
        }
    }

    #[must_use]
    pub fn result(self, val: impl Into<InlineQueryResult>) -> Self {
        Self {
            results: self.results.into_iter().chain(Some(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn results<T, I>(self, val: I) -> Self
    where
        T: Into<InlineQueryResult>,
        I: IntoIterator<Item = T>,
    {
        Self {
            results: self
                .results
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn cache_time(self, val: i32) -> Self {
        Self {
            cache_time: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn is_personal(self, val: bool) -> Self {
        Self {
            is_personal: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn next_offset(self, val: impl Into<String>) -> Self {
        Self {
            next_offset: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn button(self, val: InlineQueryResultsButton) -> Self {
        Self {
            button: Some(val),
            ..self
        }
    }
}

impl AnswerInlineQuery {
    #[must_use]
    pub fn cache_time_option(self, val: Option<i32>) -> Self {
        Self {
            cache_time: val,
            ..self
        }
    }

    #[must_use]
    pub fn is_personal_option(self, val: Option<bool>) -> Self {
        Self {
            is_personal: val,
            ..self
        }
    }

    #[must_use]
    pub fn next_offset_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            next_offset: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn button_option(self, val: Option<InlineQueryResultsButton>) -> Self {
        Self {
            button: val,
            ..self
        }
    }
}

impl TelegramMethod for AnswerInlineQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("answerInlineQuery", self, None)
    }
}
