use super::{Middleware, MiddlewareResult, Next};
use crate::{
    context::Context as RequestContext,
    errors::{EventErrorKind, MiddlewareError},
    fsm::{
        storage::base::{StorageKey, DEFAULT_DESTINY},
        strategy::Strategy,
        Context, Storage,
    },
    types::{Chat, User},
    Request,
};

use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    sync::{Arc, Mutex},
};
use tokio::sync::OwnedMutexGuard;
use tracing::instrument;

type KeyLock = tokio::sync::Mutex<()>;

/// One lock per state key, created on demand and shared by every clone of the middleware.
///
/// shortcut: entries are never removed, so a long-running process keeps one lock per key it has
/// ever seen. Add eviction if a bot serves millions of keys from a single process.
type Locks = Mutex<HashMap<StorageKey, Arc<KeyLock>>>;

/// State key an upstream `FSMContext` already locked, so a nested chain does not lock it twice.
const ISOLATED_KEY: &str = "fsm_isolated_key";

/// Middleware for creating FSM [`Context`]
///
/// With [`FSMContext::isolation`] enabled, updates resolving to the same state key are processed
/// one after another instead of concurrently.
#[derive(Clone)]
pub struct FSMContext<S> {
    storage: S,
    strategy: Strategy,
    destiny: &'static str,
    isolation: bool,
    locks: Arc<Locks>,
}

impl<S> FSMContext<S> {
    #[inline]
    #[must_use]
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            strategy: Strategy::default(),
            destiny: DEFAULT_DESTINY,
            isolation: false,
            locks: Arc::new(Locks::default()),
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
    /// # Notes
    /// Off by default. Enabled, a key stays locked for the whole update, including the parts that
    /// never touch the state.
    #[inline]
    #[must_use]
    pub fn isolation(self, val: bool) -> Self {
        Self {
            isolation: val,
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
            .field("isolation", &self.isolation)
            .finish_non_exhaustive()
    }
}

impl<S> FSMContext<S>
where
    S: Clone,
{
    #[must_use]
    fn resolve_event_context(
        &self,
        bot_id: i64,
        context: &RequestContext,
    ) -> Option<(StorageKey, Context<S>)> {
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
    ) -> Option<(StorageKey, Context<S>)> {
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
    ) -> (StorageKey, Context<S>) {
        let key = StorageKey {
            bot_id,
            chat_id,
            user_id,
            message_thread_id,
            business_connection_id,
            destiny: self.destiny,
        };

        (key.clone(), Context::new(self.storage.clone(), key))
    }

    /// Lock the key, unless an upstream middleware already locked the same one, which is what
    /// happens when this middleware is registered through `on_all`.
    async fn lock(
        &self,
        context: &mut RequestContext,
        key: StorageKey,
    ) -> Option<OwnedMutexGuard<()>> {
        if !self.isolation || context.get::<StorageKey>(ISOLATED_KEY) == Some(&key) {
            return None;
        }

        let lock = Arc::clone(
            self.locks
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .entry(key.clone())
                .or_insert_with(|| Arc::new(KeyLock::new(()))),
        );

        context.insert(ISOLATED_KEY, key);

        Some(lock.lock_owned().await)
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
        let _isolation_guard = match &fsm {
            Some((key, _)) => self.lock(&mut request.context, key.clone()).await,
            None => None,
        };

        if let Some((_, fsm_context)) = fsm {
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
        fsm::MemoryStorage,
        middlewares::outer::{
            base::BoxedCloneRoutingService, boxed_middleware_factory, wrap_to_next, UserContext,
        },
        router::Response,
        types::{ChatPrivate, MessageText, Update, UpdateMessage, User},
        Bot, Extensions,
    };

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

    fn fsm_middleware(isolation: bool) -> FSMContext<MemoryStorage> {
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
        let fsm = fsm_middleware(true);

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
        let fsm = fsm_middleware(false);

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
        let fsm = fsm_middleware(true);
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
