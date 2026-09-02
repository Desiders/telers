//! Middleware that automatically answers callback queries.

use super::base::{Middleware, Next};
use crate::{
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

    async fn answer<Client>(
        &self,
        callback_query_id: &str,
        bot: &crate::client::Bot<Client>,
    ) -> Result<(), SessionErrorKind>
    where
        Client: crate::client::Session,
    {
        let mut method = AnswerCallbackQuery::new(callback_query_id);
        if let Some(text) = &self.text {
            method = method.text(text.clone());
        }
        if let Some(show_alert) = self.show_alert {
            method = method.show_alert(show_alert);
        }
        if let Some(url) = &self.url {
            method = method.url(url.clone());
        }
        if let Some(cache_time) = self.cache_time {
            method = method.cache_time(cache_time);
        }
        bot.send(method).await?;
        Ok(())
    }

    async fn try_answer<Client>(&self, callback_query_id: &str, bot: &crate::client::Bot<Client>)
    where
        Client: crate::client::Session,
    {
        if let Err(err) = self.answer(callback_query_id, bot).await {
            event!(Level::ERROR, error = %err, "Failed to answer callback query");
        }
    }
}

impl<Client: Send + Sync + Clone + 'static + crate::client::Session> Middleware<Client>
    for CallbackAnswer
{
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
                self.try_answer(id, &bot).await;
            }
        }

        let response = next(request).await;

        if !self.pre {
            if let Some(id) = &callback_query_id {
                self.try_answer(id, &bot).await;
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{
            session::{ClientResponse, Session, StatusCode},
            telegram::{APIServer, PRODUCTION},
            Bot,
        },
        event::{
            telegram::handler::{boxed_handler_factory, BoxedCloneHandlerService},
            EventReturn,
        },
        methods::TelegramMethod,
        middlewares::inner::base::{boxed_middleware_factory, wrap_to_next},
        types::{CallbackQuery, Update, UpdateCallbackQuery, User},
    };

    use std::{
        convert::Infallible,
        future::Future,
        sync::{Arc, Mutex},
    };

    #[derive(Clone)]
    struct StubSession {
        calls: Arc<Mutex<Vec<String>>>,
        fail: bool,
    }

    impl Session for StubSession {
        fn api(&self) -> &APIServer {
            &PRODUCTION
        }

        fn send_request<Client, T>(
            &self,
            _bot: &Bot<Client>,
            _method: T,
            _timeout: Option<f32>,
        ) -> impl Future<Output = Result<ClientResponse, anyhow::Error>> + Send
        where
            Client: Session,
            T: TelegramMethod + Send + Sync,
            T::Method: Send + Sync,
        {
            async move {
                if self.fail {
                    return Err(anyhow::anyhow!("boom"));
                }
                self.calls
                    .lock()
                    .unwrap()
                    .push(std::any::type_name::<T>().to_owned());
                Ok(ClientResponse {
                    status_code: StatusCode::from(200),
                    content: r#"{"ok":true,"result":true}"#.into(),
                })
            }
        }
    }

    fn request_with(update: Update, calls: Arc<Mutex<Vec<String>>>) -> Request<StubSession> {
        Request {
            update: Arc::new(update),
            bot: Bot::with_client(
                "123456:ABC",
                StubSession {
                    calls,
                    fail: false,
                },
            ),
            context: crate::Context::default(),
            extensions: crate::Extensions::default(),
        }
    }

    fn callback_request(calls: Arc<Mutex<Vec<String>>>) -> Request<StubSession> {
        request_with(
            Update::CallbackQuery(UpdateCallbackQuery::new(
                0,
                CallbackQuery::new("query_id", User::new(1, false, "user"), "chat_instance"),
            )),
            calls,
        )
    }

    fn message_request(calls: Arc<Mutex<Vec<String>>>) -> Request<StubSession> {
        use crate::types::{ChatPrivate, MessageText, UpdateMessage};

        request_with(
            Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            )),
            calls,
        )
    }

    async fn run_middleware_with(
        request: Request<StubSession>,
        middleware: CallbackAnswer,
        handler_service: BoxedCloneHandlerService<StubSession>,
    ) -> Result<HandlerResponse<StubSession>, EventErrorKind> {
        let middlewares = vec![boxed_middleware_factory(middleware)].into_boxed_slice();
        wrap_to_next(handler_service, middlewares)(request).await
    }

    async fn run_middleware(
        request: Request<StubSession>,
        middleware: CallbackAnswer,
    ) -> Result<HandlerResponse<StubSession>, EventErrorKind> {
        run_middleware_with(
            request,
            middleware,
            boxed_handler_factory(|| async { Ok::<_, Infallible>(EventReturn::Finish) }),
        )
        .await
    }

    #[tokio::test]
    async fn answers_after_handler() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let response = run_middleware(callback_request(calls.clone()), CallbackAnswer::new())
            .await
            .unwrap();

        assert!(matches!(response.result, Ok(EventReturn::Finish)));
        assert_eq!(calls.lock().unwrap().len(), 1);
        assert!(calls.lock().unwrap()[0].contains("AnswerCallbackQuery"));
    }

    #[tokio::test]
    async fn pre_answers_before_handler() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let handler_service = boxed_handler_factory({
            let calls = calls.clone();
            move || {
                let calls = calls.clone();
                async move {
                    calls.lock().unwrap().push("handler".to_owned());
                    Ok::<_, Infallible>(EventReturn::Finish)
                }
            }
        });
        let request = callback_request(calls.clone());
        let response =
            run_middleware_with(request, CallbackAnswer::new().pre(true), handler_service)
                .await
                .unwrap();

        assert!(matches!(response.result, Ok(EventReturn::Finish)));
        let calls = calls.lock().unwrap();
        assert_eq!(calls.len(), 2);
        assert!(calls[0].contains("AnswerCallbackQuery"));
        assert_eq!(calls[1], "handler");
    }

    #[tokio::test]
    async fn answers_even_when_handler_errors() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let request = callback_request(calls.clone());
        let handler_service = boxed_handler_factory(|| async {
            Err::<EventReturn, anyhow::Error>(anyhow::anyhow!("boom"))
        });

        let response = run_middleware_with(request, CallbackAnswer::new(), handler_service).await;

        assert!(response.is_err());
        let calls = calls.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert!(calls[0].contains("AnswerCallbackQuery"));
    }

    #[tokio::test]
    async fn answer_failure_does_not_fail_the_chain() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let mut request = callback_request(calls.clone());
        request.bot = Bot::with_client(
            "123456:ABC",
            StubSession {
                calls: calls.clone(),
                fail: true,
            },
        );

        let response = run_middleware(request, CallbackAnswer::new())
            .await
            .unwrap();

        assert!(matches!(response.result, Ok(EventReturn::Finish)));
    }

    #[tokio::test]
    async fn passes_through_non_callback_updates() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let response = run_middleware(message_request(calls.clone()), CallbackAnswer::new())
            .await
            .unwrap();

        assert!(matches!(response.result, Ok(EventReturn::Finish)));
        assert!(calls.lock().unwrap().is_empty());
    }
}
