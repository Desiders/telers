use super::{Middleware, MiddlewareResult, Next};
use crate::{
    context::Context as RequestContext,
    errors::{EventErrorKind, MiddlewareError},
    fsm::{
        strategy::Strategy, Context, DisabledEventIsolation, EventIsolation, IsolationGuard,
        Storage, StorageKey, DEFAULT_DESTINY,
    },
    types::{Chat, User},
    Request,
};

use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};
use tracing::instrument;

/// State key an upstream `FSMContext` already locked, so a nested chain does not lock it twice.
const LOCKED_KEY: &str = "fsm_locked_key";

/// Middleware for creating FSM [`Context`]
///
/// With [`FSMContext::isolation`] set, updates resolving to the same state key are processed one
/// after another instead of concurrently.
#[derive(Clone)]
pub struct FSMContext<S> {
    storage: S,
    strategy: Strategy,
    destiny: &'static str,
    isolation: Arc<dyn EventIsolation>,
}

impl<S> FSMContext<S> {
    #[inline]
    #[must_use]
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            strategy: Strategy::default(),
            destiny: DEFAULT_DESTINY,
            isolation: Arc::new(DisabledEventIsolation),
        }
    }

    #[inline]
    #[must_use]
    pub fn strategy(self, val: Strategy) -> Self {
        Self {
            strategy: val,
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn destiny(self, val: &'static str) -> Self {
        Self {
            destiny: val,
            ..self
        }
    }

    /// Process updates that resolve to the same state key one after another
    ///
    /// The lock is taken before the state is read and released once the handlers finish, so two
    /// updates from one key cannot interleave their reads and writes.
    ///
    /// # Examples
    /// ```
    /// use telers::{
    ///     fsm::{MemoryEventIsolation, MemoryStorage},
    ///     middlewares::outer::FSMContext,
    /// };
    ///
    /// let middleware = FSMContext::new(MemoryStorage::new()).isolation(MemoryEventIsolation::new());
    /// ```
    ///
    /// # Notes
    /// Defaults to [`DisabledEventIsolation`], which locks nothing. A key stays locked for the whole
    /// update, including the parts that never touch the state.
    #[inline]
    #[must_use]
    pub fn isolation(self, val: impl EventIsolation) -> Self {
        Self {
            isolation: Arc::new(val),
            ..self
        }
    }
}

impl<S> Default for FSMContext<S>
where
    S: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(S::default())
    }
}

impl<S> Debug for FSMContext<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FSMContext")
            .field("strategy", &self.strategy)
            .field("destiny", &self.destiny)
            .finish_non_exhaustive()
    }
}

impl<S> FSMContext<S>
where
    S: Clone,
{
    #[must_use]
    fn resolve_event_context(&self, bot_id: i64, context: &RequestContext) -> Option<Context<S>> {
        let user_id = context.get::<User>("event_user").map(|user| user.id);
        let chat_id = context.get::<Chat>("event_chat").map(Chat::id);
        let message_thread_id = context.get("event_message_thread_id").copied();
        let business_connection_id = context.get("event_business_connection_id").cloned();

        self.resolve_context(
            bot_id,
            chat_id,
            user_id,
            message_thread_id,
            business_connection_id,
        )
    }

    #[must_use]
    fn resolve_context(
        &self,
        bot_id: i64,
        chat_id: Option<i64>,
        user_id: Option<i64>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<String>,
    ) -> Option<Context<S>> {
        user_id.map(|user_id| {
            let id_pair = self.strategy.apply(
                chat_id.unwrap_or(user_id),
                user_id,
                message_thread_id,
                business_connection_id,
            );

            self.get_context(
                bot_id,
                id_pair.chat_id,
                id_pair.user_id,
                id_pair.message_thread_id,
                id_pair.business_connection_id,
            )
        })
    }

    #[must_use]
    fn get_context(
        &self,
        bot_id: i64,
        chat_id: i64,
        user_id: i64,
        message_thread_id: Option<i64>,
        business_connection_id: Option<String>,
    ) -> Context<S> {
        let key = StorageKey {
            bot_id,
            chat_id,
            user_id,
            message_thread_id,
            business_connection_id,
            destiny: self.destiny,
        };

        Context::new(self.storage.clone(), key)
    }

    /// Lock the key, unless this update is already locked
    ///
    /// `on_all` registers this middleware once per update type, so an update passes through it
    /// twice: once for its own type and once for `on_all`. Locking the key again on the second
    /// pass would wait for a lock this update already holds, so the key is remembered in the
    /// request context instead of taking it twice.
    ///
    /// # Errors
    /// If the isolation implementation can't take the lock
    async fn lock(
        &self,
        context: &mut RequestContext,
        key: &StorageKey,
    ) -> Result<Option<IsolationGuard>, EventErrorKind> {
        if context.get::<StorageKey>(LOCKED_KEY) == Some(key) {
            return Ok(None);
        }

        context.insert(LOCKED_KEY, key.clone());

        self.isolation
            .lock(key)
            .await
            .map(Some)
            .map_err(|err| EventErrorKind::Middleware(MiddlewareError::new(err)))
    }
}

impl<Client, S> Middleware<Client> for FSMContext<S>
where
    Client: Send + Sync + 'static,
    S: Storage + Send + Sync + 'static,
{
    #[instrument(skip_all)]
    async fn call(
        &mut self,
        mut request: Request<Client>,
        next: Next<Client>,
    ) -> MiddlewareResult<Client> {
        let fsm = self.resolve_event_context(request.bot.id, &request.context);
        // Dropped when this function returns, so the handlers run inside the lock.
        let _isolation_guard = match fsm.as_ref() {
            Some(fsm_context) => match self.lock(&mut request.context, fsm_context.key()).await {
                Ok(guard) => guard,
                Err(err) => return Err((err, request)),
            },
            None => None,
        };

        if let Some(fsm_context) = fsm {
            let state = match fsm_context.get_state().await {
                Ok(state) => state,
                Err(err) => {
                    return Err((
                        EventErrorKind::Middleware(MiddlewareError::new(err.into())),
                        request,
                    ));
                }
            };
            if let Some(state) = state {
                request.context.insert("fsm_state", state);
            }

            request.context.insert("fsm_context", fsm_context);
        }

        request.context.insert("fsm_storage", self.storage.clone());

        next(request).await
    }
}

#[cfg(all(test, feature = "memory-storage"))]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        event::{
            bases::PropagateEventResult,
            service::{service_fn, BoxCloneService},
        },
        fsm::{MemoryEventIsolation, MemoryStorage},
        middlewares::outer::{
            base::BoxedCloneRoutingService, boxed_middleware_factory, wrap_to_next, UserContext,
        },
        router::Response,
        types::{ChatPrivate, MessageText, Update, UpdateMessage, User},
        Bot, Extensions,
    };
    use std::sync::Mutex;

    const USER_ID: i64 = 3;

    fn test_request() -> Request<Reqwest> {
        Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), "").from(User::new(USER_ID, false, "")),
            ))),
            bot: Bot::default(),
            context: RequestContext::default(),
            extensions: Extensions::default(),
        }
    }

    /// Reads the state, yields to let a concurrent update through, then writes, recording what it
    /// saw on the way in.
    fn recording_service(
        reads: Arc<Mutex<Vec<Option<Box<str>>>>>,
    ) -> BoxedCloneRoutingService<Reqwest> {
        BoxCloneService::new(service_fn(move |request: Request<Reqwest>| {
            let reads = reads.clone();

            async move {
                let fsm = request
                    .context
                    .get::<Context<MemoryStorage>>("fsm_context")
                    .expect("the middleware put the fsm context in place")
                    .clone();

                let state = fsm
                    .get_state()
                    .await
                    .map_err(anyhow::Error::from)
                    .expect("memory storage never fails");
                reads.lock().unwrap().push(state);

                tokio::task::yield_now().await;

                fsm.set_state("done")
                    .await
                    .expect("memory storage never fails");

                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                })
            }
        }))
    }

    fn fsm_middleware<I: EventIsolation>(isolation: I) -> FSMContext<MemoryStorage> {
        FSMContext::new(MemoryStorage::new()).isolation(isolation)
    }

    /// `Next` is `FnOnce`, so every concurrent update gets its own chain over the same middleware
    /// clones, which is what the router does per update.
    fn chain(
        reads: &Arc<Mutex<Vec<Option<Box<str>>>>>,
        fsm: &FSMContext<MemoryStorage>,
    ) -> Next<Reqwest> {
        wrap_to_next(
            recording_service(reads.clone()),
            vec![
                boxed_middleware_factory(UserContext),
                boxed_middleware_factory(fsm.clone()),
            ]
            .into_boxed_slice(),
        )
    }

    #[tokio::test]
    async fn isolation_serializes_updates_of_the_same_key() {
        let reads = Arc::new(Mutex::new(Vec::new()));
        let fsm = fsm_middleware(MemoryEventIsolation::new());

        let (first, second) = tokio::join!(
            chain(&reads, &fsm)(test_request()),
            chain(&reads, &fsm)(test_request()),
        );
        first.unwrap();
        second.unwrap();

        let reads = reads.lock().unwrap();
        assert_eq!(reads.len(), 2, "both updates must reach the handler");
        assert_eq!(
            reads.iter().filter(|state| state.is_none()).count(),
            1,
            "the first update reads no state, the second one reads what it wrote: {reads:?}"
        );
        assert_eq!(
            reads
                .iter()
                .filter(|state| state.as_deref() == Some("done"))
                .count(),
            1,
            "{reads:?}"
        );
    }

    #[tokio::test]
    async fn without_isolation_updates_of_the_same_key_overlap() {
        let reads = Arc::new(Mutex::new(Vec::new()));
        let fsm = fsm_middleware(DisabledEventIsolation);

        let (first, second) = tokio::join!(
            chain(&reads, &fsm)(test_request()),
            chain(&reads, &fsm)(test_request()),
        );
        first.unwrap();
        second.unwrap();

        let reads = reads.lock().unwrap();
        assert_eq!(reads.len(), 2, "both updates must reach the handler");
        assert_eq!(
            reads.iter().filter(|state| state.is_none()).count(),
            2,
            "both updates read before either wrote: {reads:?}"
        );
    }

    #[tokio::test]
    async fn a_nested_middleware_does_not_lock_the_same_key_twice() {
        let reads = Arc::new(Mutex::new(Vec::new()));
        // Both middlewares share one lock registry, which is what a clone registered through
        // `on_all` gives. Separate instances would have separate registries and never collide.
        let fsm = fsm_middleware(MemoryEventIsolation::new());
        let middlewares = vec![
            boxed_middleware_factory(UserContext),
            boxed_middleware_factory(fsm.clone()),
            boxed_middleware_factory(fsm.clone()),
        ]
        .into_boxed_slice();

        wrap_to_next(recording_service(reads.clone()), middlewares)(test_request())
            .await
            .unwrap();

        assert_eq!(reads.lock().unwrap().len(), 1);
    }
}
