use super::base::{Middleware, Next};
use crate::{
    client::Reqwest,
    errors::EventErrorKind,
    event::{telegram::HandlerResponse, EventReturn},
    types::{Chat, User},
    Request,
};

use self::strategy::IdPair;
pub use self::strategy::Strategy;

use futures_util::future::BoxFuture;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    future::Future,
    sync::{Arc, Mutex, PoisonError},
    time::{Duration, Instant},
};
use tracing::{event, Level};

pub mod strategy;

/// Information about a throttled request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThrottledInfo {
    /// How many times this key exceeded the rate in a row.
    pub exceeded_count: u64,
    /// How much time is left until the request would be allowed.
    pub time_left: Duration,
}

type Timestamps = Arc<Mutex<HashMap<IdPair, (Instant, u64)>>>;
type OnThrottledCallback<Client> =
    dyn Fn(&Request<Client>, ThrottledInfo) -> BoxFuture<'static, ()> + Send + Sync;

/// Middleware that limits how often a handler is called for the same chat/user/thread.
///
/// Peer IDs are resolved from the request context (`event_user`, `event_chat`,
/// `event_message_thread_id`, `event_business_connection_id`) populated by
/// [`UserContextMiddleware`], which is included in the default middleware config.
/// Updates without a user and a chat are passed through without throttling.
/// The strategy controls which peer IDs form the throttling key, e.g. [`Strategy::Chat`]
/// throttles per chat, [`Strategy::UserInThread`] per user and thread pair.
///
/// Requests that exceed the rate are skipped (the handler is not called) and a warning is logged.
/// Timestamps of expired keys are pruned when a new key is inserted, so the memory usage is
/// bounded by the number of keys inserted since the last prune.
///
/// [`UserContextMiddleware`]: crate::middlewares::outer::UserContextMiddleware
pub struct Throttling<Client = Reqwest> {
    strategy: Strategy,
    rate: Duration,
    timestamps: Timestamps,
    on_throttled: Option<Arc<OnThrottledCallback<Client>>>,
}

impl<Client: Send + Sync + 'static> Throttling<Client> {
    #[must_use]
    pub fn new(rate: Duration) -> Self {
        Self {
            strategy: Strategy::UserInChat,
            rate,
            timestamps: Arc::new(Mutex::new(HashMap::new())),
            on_throttled: None,
        }
    }

    #[must_use]
    pub fn strategy(mut self, strategy: Strategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Call `callback` when a request is throttled.
    ///
    /// ```ignore
    /// .on_throttled(|request, info| async move {
    ///     let _ = request.bot.send(/* "Too many requests" */).await;
    /// })
    /// ```
    #[must_use]
    pub fn on_throttled<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(&Request<Client>, ThrottledInfo) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_throttled = Some(Arc::new(move |request, info| {
            Box::pin(callback(request, info))
        }));
        self
    }
}

impl<Client> Clone for Throttling<Client> {
    fn clone(&self) -> Self {
        Self {
            strategy: self.strategy,
            rate: self.rate,
            timestamps: Arc::clone(&self.timestamps),
            on_throttled: self.on_throttled.clone(),
        }
    }
}

impl<Client> fmt::Debug for Throttling<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Throttling")
            .field("strategy", &self.strategy)
            .field("rate", &self.rate)
            .field("timestamps", &self.timestamps)
            .field("on_throttled", &self.on_throttled.is_some())
            .finish()
    }
}

impl<Client> Display for Throttling<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Throttling")
    }
}

impl<Client> Middleware<Client> for Throttling<Client>
where
    Client: Send + Sync + 'static,
{
    async fn call(
        &mut self,
        request: Request<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        let context = &request.context;
        let chat_id = context.get::<Chat>("event_chat").map(Chat::id);
        let user_id = context.get::<User>("event_user").map(|user| user.id);
        let message_thread_id = context.get::<i64>("event_message_thread_id").copied();
        let business_connection_id = context
            .get::<String>("event_business_connection_id")
            .cloned();
        let (chat_id, user_id) = match (chat_id, user_id) {
            (Some(chat_id), Some(user_id)) => (chat_id, user_id),
            (Some(chat_id), None) => (chat_id, chat_id),
            (None, Some(user_id)) => (user_id, user_id),
            (None, None) => return next(request).await,
        };
        let key = self
            .strategy
            .apply(chat_id, user_id, message_thread_id, business_connection_id);

        let now = Instant::now();
        let throttled = {
            let mut timestamps = self
                .timestamps
                .lock()
                .unwrap_or_else(PoisonError::into_inner);
            if !timestamps.contains_key(&key) {
                timestamps.retain(|_, (last, _)| now.duration_since(*last) < self.rate);
            }
            match timestamps.get_mut(&key) {
                Some((last, exceeded_count)) if now.duration_since(*last) < self.rate => {
                    *exceeded_count += 1;
                    Some(ThrottledInfo {
                        exceeded_count: *exceeded_count,
                        time_left: self.rate.saturating_sub(now.duration_since(*last)),
                    })
                }
                _ => {
                    timestamps.insert(key, (now, 0));
                    None
                }
            }
        };
        if let Some(info) = throttled {
            event!(Level::WARN, "Request is throttled");
            if let Some(callback) = &self.on_throttled {
                callback(&request, info).await;
            }
            return Ok(HandlerResponse {
                request,
                result: Ok(EventReturn::Skip),
            });
        }
        next(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        event::telegram::handler::boxed_handler_factory,
        middlewares::inner::wrap_to_next,
        types::{
            CallbackQuery, ChatPrivate, MessageText, Poll, PollOption, PollRegular, Update,
            UpdateCallbackQuery, UpdateMessage, UpdatePoll, User,
        },
        Bot, Context, Extensions,
    };

    use std::{
        convert::Infallible,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    fn request(user_id: i64, chat_id: i64) -> Request<Reqwest> {
        let mut context = Context::default();
        context.insert("event_user", User::new(user_id, false, "user"));
        context.insert("event_chat", Chat::from(ChatPrivate::new(chat_id)));
        Request {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(chat_id), "")
                    .from(User::new(user_id, false, "user")),
            ))),
            bot: Bot::default(),
            context,
            extensions: Extensions::default(),
        }
    }

    fn thread_request(user_id: i64, chat_id: i64, thread_id: i64) -> Request<Reqwest> {
        let mut context = Context::default();
        context.insert("event_user", User::new(user_id, false, "user"));
        context.insert("event_chat", Chat::from(ChatPrivate::new(chat_id)));
        context.insert("event_message_thread_id", thread_id);
        Request {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(chat_id), "")
                    .from(User::new(user_id, false, "user"))
                    .message_thread_id(thread_id),
            ))),
            bot: Bot::default(),
            context,
            extensions: Extensions::default(),
        }
    }

    fn callback_query_request(user_id: i64) -> Request<Reqwest> {
        let mut context = Context::default();
        context.insert("event_user", User::new(user_id, false, "user"));
        Request {
            update: Arc::new(Update::CallbackQuery(UpdateCallbackQuery::new(
                0,
                CallbackQuery::new("id", User::new(user_id, false, "user"), "chat_instance"),
            ))),
            bot: Bot::default(),
            context,
            extensions: Extensions::default(),
        }
    }

    fn no_message_request() -> Request<Reqwest> {
        Request {
            update: Arc::new(Update::Poll(UpdatePoll::new(
                0,
                Poll::Regular(PollRegular::new(
                    "id",
                    "question",
                    [PollOption::new("opt", "opt", 0)],
                    0,
                    false,
                    true,
                    false,
                    false,
                    false,
                )),
            ))),
            bot: Bot::default(),
            context: Context::default(),
            extensions: Extensions::default(),
        }
    }

    fn counting_service(
        calls: Arc<AtomicUsize>,
    ) -> crate::event::telegram::handler::BoxedCloneHandlerService<Reqwest> {
        boxed_handler_factory(move || {
            let calls = Arc::clone(&calls);
            async move {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<_, Infallible>(EventReturn::Finish)
            }
        })
    }

    #[tokio::test]
    async fn test_allows_first_request() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware = Throttling::new(Duration::from_secs(10));

        let response = middleware
            .call(request(1, 1), wrap_to_next(handler_service, [].into()))
            .await;

        assert!(response.is_ok());
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_skips_throttled_request() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware = Throttling::new(Duration::from_secs(10));

        middleware
            .call(
                request(1, 1),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        let response = middleware
            .call(request(1, 1), wrap_to_next(handler_service, [].into()))
            .await
            .unwrap();

        assert_eq!(response.result.unwrap(), EventReturn::Skip);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_calls_callback_on_throttled_request() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let infos = Arc::new(Mutex::new(Vec::new()));
        let callback_infos = Arc::clone(&infos);
        let mut middleware =
            Throttling::new(Duration::from_secs(10)).on_throttled(move |_, info| {
                let callback_infos = Arc::clone(&callback_infos);
                async move {
                    callback_infos.lock().unwrap().push(info);
                }
            });

        middleware
            .call(
                request(1, 1),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        let response = middleware
            .call(request(1, 1), wrap_to_next(handler_service, [].into()))
            .await
            .unwrap();

        assert_eq!(response.result.unwrap(), EventReturn::Skip);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        let infos = infos.lock().unwrap();
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0].exceeded_count, 1);
        assert!(infos[0].time_left <= Duration::from_secs(10));
        assert!(infos[0].time_left > Duration::ZERO);
    }

    #[tokio::test]
    async fn test_exceeded_count_increments() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let infos = Arc::new(Mutex::new(Vec::new()));
        let callback_infos = Arc::clone(&infos);
        let mut middleware =
            Throttling::new(Duration::from_secs(10)).on_throttled(move |_, info| {
                let callback_infos = Arc::clone(&callback_infos);
                async move {
                    callback_infos.lock().unwrap().push(info);
                }
            });

        middleware
            .call(
                request(1, 1),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        middleware
            .call(
                request(1, 1),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        let response = middleware
            .call(request(1, 1), wrap_to_next(handler_service, [].into()))
            .await
            .unwrap();

        assert_eq!(response.result.unwrap(), EventReturn::Skip);
        let infos = infos.lock().unwrap();
        assert_eq!(infos.len(), 2);
        assert_eq!(infos[0].exceeded_count, 1);
        assert_eq!(infos[1].exceeded_count, 2);
    }

    #[tokio::test]
    async fn test_different_keys_are_not_throttled() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware = Throttling::new(Duration::from_secs(10));

        middleware
            .call(
                request(1, 1),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        let response = middleware
            .call(request(2, 1), wrap_to_next(handler_service, [].into()))
            .await
            .unwrap();

        assert_eq!(response.result.unwrap(), EventReturn::Finish);
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn test_threads_are_throttled_separately() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware =
            Throttling::new(Duration::from_secs(10)).strategy(Strategy::UserInThread);

        middleware
            .call(
                thread_request(1, 1, 1),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        middleware
            .call(
                thread_request(1, 1, 2),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        let response = middleware
            .call(
                thread_request(1, 1, 2),
                wrap_to_next(handler_service, [].into()),
            )
            .await
            .unwrap();

        assert_eq!(response.result.unwrap(), EventReturn::Skip);
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn test_throttles_callback_query() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware = Throttling::new(Duration::from_secs(10));

        middleware
            .call(
                callback_query_request(1),
                wrap_to_next(handler_service.clone(), [].into()),
            )
            .await
            .unwrap();
        let response = middleware
            .call(
                callback_query_request(1),
                wrap_to_next(handler_service, [].into()),
            )
            .await
            .unwrap();

        assert_eq!(response.result.unwrap(), EventReturn::Skip);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_passes_through_without_message() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware = Throttling::new(Duration::from_secs(10));

        let response = middleware
            .call(
                no_message_request(),
                wrap_to_next(handler_service, [].into()),
            )
            .await;

        assert!(response.is_ok());
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }
}
