use crate::client::Bot;
use serde::Serialize;
/// Use this method to send answers to an inline query. On success, `true` is returned.
/// No more than 50 results per query are allowed.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerinlinequery>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: Box<str>,
    /// A JSON-serialized Array of results for the inline query
    pub results: Box<[crate::types::InlineQueryResult]>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    /// Pass `true` if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<Box<str>>,
    /// A JSON-serialized object describing a button to be shown above inline query results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<crate::types::InlineQueryResultsButton>,
}
impl AnswerInlineQuery {
    /// Creates a new `AnswerInlineQuery`.
    ///
    /// # Arguments
    /// * `inline_query_id` - Unique identifier for the answered query
    /// * `results` - A JSON-serialized Array of results for the inline query
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1Item: Into<crate::types::InlineQueryResult>,
        T1: IntoIterator<Item = T1Item>,
    >(
        inline_query_id: T0,
        results: T1,
    ) -> Self {
        Self {
            inline_query_id: inline_query_id.into(),
            results: results.into_iter().map(Into::into).collect(),
            cache_time: None,
            is_personal: None,
            next_offset: None,
            button: None,
        }
    }

    /// Unique identifier for the answered query
    #[must_use]
    pub fn inline_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_query_id = val.into();
        self
    }

    /// A JSON-serialized Array of results for the inline query
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn results<TItem: Into<crate::types::InlineQueryResult>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.results = self
            .results
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        self
    }

    /// A JSON-serialized Array of results for the inline query
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn result<T: Into<crate::types::InlineQueryResult>>(mut self, val: T) -> Self {
        self.results = self
            .results
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[must_use]
    pub fn cache_time<T: Into<i64>>(mut self, val: T) -> Self {
        self.cache_time = Some(val.into());
        self
    }

    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[must_use]
    pub fn cache_time_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.cache_time = val.map(Into::into);
        self
    }

    /// Pass `true` if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    #[must_use]
    pub fn is_personal<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_personal = Some(val.into());
        self
    }

    /// Pass `true` if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    #[must_use]
    pub fn is_personal_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_personal = val.map(Into::into);
        self
    }

    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[must_use]
    pub fn next_offset<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.next_offset = Some(val.into());
        self
    }

    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[must_use]
    pub fn next_offset_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.next_offset = val.map(Into::into);
        self
    }

    /// A JSON-serialized object describing a button to be shown above inline query results
    #[must_use]
    pub fn button<T: Into<crate::types::InlineQueryResultsButton>>(mut self, val: T) -> Self {
        self.button = Some(val.into());
        self
    }

    /// A JSON-serialized object describing a button to be shown above inline query results
    #[must_use]
    pub fn button_option<T: Into<crate::types::InlineQueryResultsButton>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.button = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for AnswerInlineQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_inline_query_results(&mut files, self.results.iter_mut().collect());
        super::Request::new("answerInlineQuery", self, Some(files))
    }
}
