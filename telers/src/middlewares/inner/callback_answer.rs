//! Middleware that automatically answers callback queries.

use super::base::{Middleware, Next};
use crate::{
    client::{Bot, Session},
    errors::{EventErrorKind, SessionErrorKind},
    event::telegram::HandlerResponse,
    methods::AnswerCallbackQuery,
    Request,
};

use tracing::{event, Level};

/// Inner middleware that automatically answers callback queries.
///
/// Register it on the `callback_query` observer to answer every callback query
/// without repeating the same API call in every handler. With [`pre`](Self::pre)
/// the answer is sent before the handler runs, otherwise right after it.
/// Answers are sent even if the handler returns an error.
#[derive(Debug, Clone, Default)]
pub struct CallbackAnswer {
    pre: bool,
    text: Option<Box<str>>,
    show_alert: Option<bool>,
    url: Option<Box<str>>,
    cache_time: Option<i64>,
}

impl CallbackAnswer {
    /// Creates a middleware that answers callback queries without extra parameters.
    #[must_use]
    pub fn new() -> Self {
        Self {
            pre: false,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    /// Answers before the handler runs instead of after it.
    #[must_use]
    pub fn pre(mut self, val: bool) -> Self {
        self.pre = val;
        self
    }

    /// Answers with the given text.
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = Some(val.into());
        self
    }

    /// Shows the answer as an alert.
    #[must_use]
    pub fn show_alert(mut self, val: bool) -> Self {
        self.show_alert = Some(val);
        self
    }

    /// Answers with a game URL.
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = Some(val.into());
        self
    }

    /// Caches the answer for the given number of seconds.
    #[must_use]
    pub fn cache_time<T: Into<i64>>(mut self, val: T) -> Self {
        self.cache_time = Some(val.into());
        self
    }
}

impl CallbackAnswer {
    async fn answer<Client>(
        &self,
        callback_query_id: &str,
        bot: &Bot<Client>,
    ) -> Result<(), SessionErrorKind>
    where
        Client: Session,
    {
        let method = AnswerCallbackQuery::new(callback_query_id)
            .text_option(self.text.as_deref())
            .show_alert_option(self.show_alert)
            .url_option(self.url.as_deref())
            .cache_time_option(self.cache_time);
        bot.send(method).await?;
        Ok(())
    }
}

impl<Client: Send + Sync + Clone + 'static + Session> Middleware<Client> for CallbackAnswer {
    async fn call(
        &mut self,
        request: Request<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        let callback_query_id = request
            .update
            .callback_query()
            .map(|callback_query| callback_query.id.clone());

        let bot = request.bot.clone();

        if self.pre {
            if let Some(id) = &callback_query_id {
                if let Err(err) = self.answer(id, &bot).await {
                    event!(Level::ERROR, error = %err, "Failed to answer callback query");
                }
            }
        }

        let response = next(request).await;

        if !self.pre {
            if let Some(id) = &callback_query_id {
                if let Err(err) = self.answer(id, &bot).await {
                    event!(Level::ERROR, error = %err, "Failed to answer callback query");
                }
            }
        }

        response
    }
}
