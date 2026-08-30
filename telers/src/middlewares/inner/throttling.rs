use super::base::{Middleware, Next};
use crate::{
    errors::EventErrorKind,
    event::{telegram::HandlerResponse, EventReturn},
    Request,
};

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tracing::{event, Level};

/// Which chat/user pair is used as the throttling key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Throttle per chat.
    Chat,
    /// Throttle per user.
    User,
    /// Throttle per chat and user pair.
    ChatUser,
}

/// Middleware that limits how often a handler is called for the same chat/user.
///
/// Updates without a message are passed through without throttling.
/// Requests that exceed the rate are skipped (the handler is not called) and a warning is logged.
#[derive(Debug, Clone)]
pub struct Throttling {
    key: Key,
    rate: Duration,
    timestamps: Arc<Mutex<HashMap<(i64, i64), Instant>>>,
}

impl Throttling {
    #[must_use]
    pub fn new(key: Key, rate: Duration) -> Self {
        Self {
            key,
            rate,
            timestamps: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Display for Throttling {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Throttling")
    }
}

impl<Client> Middleware<Client> for Throttling
where
    Client: Send + Sync + 'static,
{
    async fn call(
        &mut self,
        request: Request<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        let Some(message) = request.update.message() else {
            return next(request).await;
        };
        let chat_id = message.chat().id();
        let user_id = match message.from() {
            Some(user) => user.id,
            None => chat_id,
        };
        let key = match self.key {
            Key::Chat => (chat_id, chat_id),
            Key::User => (user_id, user_id),
            Key::ChatUser => (chat_id, user_id),
        };

        let now = Instant::now();
        let throttled = {
            let mut timestamps = self
                .timestamps
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if timestamps
                .get(&key)
                .is_some_and(|last| now.duration_since(*last) < self.rate)
            {
                true
            } else {
                timestamps.insert(key, now);
                false
            }
        };
        if throttled {
            event!(Level::WARN, "Request is throttled");
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
        types::{ChatPrivate, MessageText, Poll, PollOption, PollRegular, Update, UpdateMessage, UpdatePoll, User},
        Bot, Extensions,
    };

    use std::{
        convert::Infallible,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    fn request(user_id: i64, chat_id: i64) -> Request<Reqwest> {
        Request {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(chat_id), "")
                    .from(User::new(user_id, false, "user")),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
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
            context: crate::Context::default(),
            extensions: Extensions::default(),
        }
    }

    fn counting_service(calls: Arc<AtomicUsize>) -> crate::event::telegram::handler::BoxedCloneHandlerService<Reqwest> {
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
        let mut middleware = Throttling::new(Key::Chat, Duration::from_secs(10));

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
        let mut middleware = Throttling::new(Key::Chat, Duration::from_secs(10));

        middleware
            .call(request(1, 1), wrap_to_next(handler_service.clone(), [].into()))
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
    async fn test_different_keys_are_not_throttled() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware = Throttling::new(Key::ChatUser, Duration::from_secs(10));

        middleware
            .call(request(1, 1), wrap_to_next(handler_service.clone(), [].into()))
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
    async fn test_passes_through_without_message() {
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_service = counting_service(Arc::clone(&calls));
        let mut middleware = Throttling::new(Key::Chat, Duration::from_secs(10));

        let response = middleware
            .call(no_message_request(), wrap_to_next(handler_service, [].into()))
            .await;

        assert!(response.is_ok());
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }
}