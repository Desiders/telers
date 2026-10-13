use crate::{
    dialog::{Dialog, IntoDialog},
    entities::{DefaultAccessValidator, EventContext, StackAccessValidator},
    errors::DialogError,
    widgets::media::{InMemoryMediaIdStorage, MediaIdStorage},
};
use std::{collections::BTreeMap, future::Future, sync::{Arc, Mutex, Weak}};
use tokio::sync::Mutex as AsyncMutex;
use tracing::warn;

/// Event serialization scope. Choose a scope at least as broad as FSM storage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EventIsolation {
    /// Serialize all dialog events for a bot; safe for any FSM strategy.
    #[default]
    Bot,
    /// Serialize a user's events across chats, matching global-user storage.
    User,
    /// Serialize all users in a chat, matching chat storage.
    Chat,
    /// Serialize each user/chat pair, matching user-in-chat storage.
    UserInChat,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct IsolationKey {
    bot_id: i64,
    chat_id: Option<i64>,
    user_id: Option<i64>,
}

#[derive(Clone, PartialEq, Eq)]
struct IsolationScope {
    registry_id: usize,
    key: IsolationKey,
}

tokio::task_local! {
    static ACTIVE_ISOLATION_SCOPES: Vec<IsolationScope>;
}

/// Registry of dialogs indexed by state id.
///
/// The registry is typically built once during startup and then injected into
/// the dispatcher as shared application state.
pub struct DialogRegistry {
    dialogs: Vec<Arc<dyn Dialog>>,
    state_index: BTreeMap<String, usize>,
    access_validator: Arc<dyn StackAccessValidator>,
    media_id_storage: Arc<dyn MediaIdStorage>,
    event_isolation: EventIsolation,
    event_locks: Arc<Mutex<BTreeMap<IsolationKey, Weak<AsyncMutex<()>>>>>,
}

impl Default for DialogRegistry {
    fn default() -> Self {
        Self {
            dialogs: Vec::new(),
            state_index: BTreeMap::new(),
            access_validator: Arc::new(DefaultAccessValidator),
            media_id_storage: Arc::new(InMemoryMediaIdStorage::default()),
            event_isolation: EventIsolation::default(),
            event_locks: Arc::default(),
        }
    }
}

impl Clone for DialogRegistry {
    fn clone(&self) -> Self {
        Self {
            dialogs: self.dialogs.clone(),
            state_index: self.state_index.clone(),
            access_validator: self.access_validator.clone(),
            media_id_storage: self.media_id_storage.clone(),
            event_isolation: self.event_isolation,
            event_locks: self.event_locks.clone(),
        }
    }
}

impl DialogRegistry {
    /// Create an empty dialog registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a dialog and index all of its states.
    ///
    /// # Errors
    /// Returns `DialogError::DuplicateState` when any state is already registered.
    pub fn register(mut self, dialog: impl IntoDialog) -> Result<Self, DialogError> {
        let dialog = dialog.into_dialog();
        let idx = self.dialogs.len();
        for state in dialog.states() {
            if self.state_index.contains_key(state) {
                warn!(state = %state, "Rejecting duplicate dialog state");
                return Err(DialogError::DuplicateState(state.clone()));
            }
            self.state_index.insert(state.to_owned(), idx);
        }
        self.dialogs.push(dialog);
        Ok(self)
    }

    /// Replace the access validator used by dialog managers created from this registry.
    #[must_use]
    pub fn with_access_validator(mut self, validator: impl StackAccessValidator + 'static) -> Self {
        self.access_validator = Arc::new(validator);
        self
    }

    /// Configure event serialization before cloning the registry for dispatch.
    ///
    /// The default is per bot. `UserInChat` allows independent conversations to
    /// run concurrently when FSM storage also uses user/chat pairs.
    #[must_use]
    pub fn with_event_isolation(mut self, isolation: EventIsolation) -> Self {
        self.event_isolation = isolation;
        self
    }

    fn isolation_key(&self, event: &EventContext) -> IsolationKey {
        let (chat_id, user_id) = match self.event_isolation {
            EventIsolation::Bot => (None, None),
            EventIsolation::User => (None, Some(event.user.id)),
            EventIsolation::Chat => (Some(event.chat.id()), None),
            EventIsolation::UserInChat => (Some(event.chat.id()), Some(event.user.id)),
        };
        IsolationKey { bot_id: event.bot_id, chat_id, user_id }
    }

    pub(crate) async fn run_isolated<F: Future>(&self, event: &EventContext, future: F) -> F::Output {
        let scope = IsolationScope {
            registry_id: Arc::as_ptr(&self.event_locks) as usize,
            key: self.isolation_key(event),
        };
        let mut scopes = ACTIVE_ISOLATION_SCOPES.try_with(Clone::clone).unwrap_or_default();
        if scopes.contains(&scope) {
            return future.await;
        }
        let gate = {
            let mut gates = self.event_locks.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            gates.retain(|_, gate| gate.strong_count() != 0);
            match gates.get(&scope.key).and_then(Weak::upgrade) {
                Some(gate) => gate,
                None => {
                    let gate = Arc::new(AsyncMutex::new(()));
                    gates.insert(scope.key.clone(), Arc::downgrade(&gate));
                    gate
                }
            }
        };
        let _guard = gate.lock_owned().await;
        scopes.push(scope);
        ACTIVE_ISOLATION_SCOPES.scope(scopes, future).await
    }

    /// Find the dialog that owns a given state id.
    #[must_use]
    pub fn find_by_state(&self, state: &str) -> Option<Arc<dyn Dialog>> {
        self.state_index
            .get(state)
            .and_then(|i| self.dialogs.get(*i).cloned())
    }

    /// Access the validator used for stack and context access checks.
    #[must_use]
    pub fn access_validator(&self) -> &dyn StackAccessValidator {
        self.access_validator.as_ref()
    }

    /// Replace the storage used to cache uploaded media `file_id`s.
    #[must_use]
    pub fn with_media_id_storage(mut self, storage: impl MediaIdStorage + 'static) -> Self {
        self.media_id_storage = Arc::new(storage);
        self
    }

    /// Access the storage used to reuse uploaded media `file_id`s across renders.
    #[must_use]
    pub fn media_id_storage(&self) -> &dyn MediaIdStorage {
        self.media_id_storage.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{DialogRegistry, EventIsolation};
    use crate::{dialog, widgets::text, window, DialogError};
    use std::time::Duration;
    use tokio::{sync::oneshot, time::timeout};

    #[tokio::test]
    #[expect(
        clippy::async_yields_async,
        reason = "The spawned contender must be awaited after releasing the outer event gate"
    )]
    async fn inline_reentry_is_allowed_but_spawned_events_wait_for_the_outer_event() {
        let registry = DialogRegistry::new();
        let (_, event) = group_event();
        let mut other_event = event.clone();
        other_event.user.id += 1;
        let (entered_tx, mut entered_rx) = oneshot::channel();
        let contender = timeout(
            Duration::from_secs(5),
            registry.run_isolated(&event, async {
                assert_eq!(registry.clone().run_isolated(&event, async { 7 }).await, 7);
                let (attempted_tx, attempted_rx) = oneshot::channel();
                let registry = registry.clone();
                let contender = tokio::spawn(async move {
                    attempted_tx.send(()).expect("notify attempt");
                    registry
                        .run_isolated(&other_event, async {
                            entered_tx.send(()).expect("notify isolated operation");
                        })
                        .await;
                });
                attempted_rx.await.expect("contender attempted acquisition");
                assert!(matches!(
                    entered_rx.try_recv(),
                    Err(oneshot::error::TryRecvError::Empty)
                ));
                contender
            }),
        )
        .await
        .expect("inline reentry must not deadlock");

        timeout(Duration::from_secs(5), contender)
            .await
            .expect("waiting event eventually runs")
            .expect("contender task");
        entered_rx
            .await
            .expect("contender ran after outer event finished");
    }

    #[tokio::test]
    async fn user_in_chat_isolation_allows_independent_conversations_to_run_together() {
        let registry = DialogRegistry::new().with_event_isolation(EventIsolation::UserInChat);
        let (_, event) = group_event();
        let mut other_event = event.clone();
        other_event.user.id += 1;
        let (entered_tx, entered_rx) = oneshot::channel();
        timeout(
            Duration::from_secs(5),
            registry.run_isolated(&event, async {
                let registry = registry.clone();
                let contender = tokio::spawn(async move {
                    registry
                        .run_isolated(&other_event, async {
                            entered_tx.send(()).expect("independent conversation entered");
                        })
                        .await;
                });
                entered_rx
                    .await
                    .expect("different user/chat pair runs before outer event finishes");
                contender.await.expect("independent task");
            }),
        )
        .await
        .expect("independent conversations must not block each other");
    }

    #[tokio::test]
    async fn cancelling_isolated_work_releases_waiting_events() {
        let registry = DialogRegistry::new();
        let (_, event) = group_event();
        let (entered_tx, entered_rx) = oneshot::channel();
        let worker_registry = registry.clone();
        let worker_event = event.clone();
        let worker = tokio::spawn(async move {
            worker_registry
                .run_isolated(&worker_event, async {
                    entered_tx.send(()).expect("worker acquired isolation");
                    std::future::pending::<()>().await;
                })
                .await;
        });
        entered_rx.await.expect("worker running");
        worker.abort();
        assert!(worker.await.expect_err("worker aborted").is_cancelled());

        let result = timeout(
            Duration::from_secs(5),
            registry.run_isolated(&event, async { 11 }),
        )
        .await
        .expect("aborted worker releases isolation");
        assert_eq!(result, 11);
    }

    #[tokio::test]
    async fn registry_rejects_duplicate_states() {
        let registry = DialogRegistry::new()
            .register(dialog([window("state", [text("one")])]))
            .expect("first dialog");

        let err = registry
            .register(dialog([window("state", [text("two")])]))
            .err()
            .expect("duplicate state must fail");

        assert!(matches!(err, DialogError::DuplicateState(_)));
    }

    #[tokio::test]
    async fn registry_finds_registered_states() {
        let registry = DialogRegistry::new()
            .register(dialog([window("a", [text("x")]), window("b", [text("y")])]))
            .expect("dialog with two states");

        assert!(registry.find_by_state("a").is_some());
        assert!(registry.find_by_state("b").is_some());
        assert!(registry.find_by_state("missing").is_none());
    }

    #[tokio::test]
    async fn registry_states_resolve_to_same_dialog() {
        let registry = DialogRegistry::new()
            .register(dialog([window("a", [text("x")]), window("b", [text("y")])]))
            .expect("dialog with two states");

        let from_a = registry.find_by_state("a").expect("dialog for state a");
        let from_b = registry.find_by_state("b").expect("dialog for state b");

        let states_a = from_a.states();
        assert!(states_a.iter().any(|state| state == "a"));
        assert!(states_a.iter().any(|state| state == "b"));

        let states_b = from_b.states();
        assert!(states_b.iter().any(|state| state == "a"));
        assert!(states_b.iter().any(|state| state == "b"));
    }

    #[tokio::test]
    async fn registry_finds_states_across_multiple_dialogs() {
        let registry = DialogRegistry::new()
            .register(dialog([window("a", [text("x")])]))
            .expect("first dialog")
            .register(dialog([window("b", [text("y")])]))
            .expect("second dialog");

        let from_a = registry.find_by_state("a").expect("dialog for a");
        let from_b = registry.find_by_state("b").expect("dialog for b");
        assert!(from_a.states().iter().any(|s| s == "a"));
        assert!(!from_a.states().iter().any(|s| s == "b"));
        assert!(from_b.states().iter().any(|s| s == "b"));
        assert!(!from_b.states().iter().any(|s| s == "a"));
        assert!(registry.find_by_state("c").is_none());
    }

    fn group_event() -> (
        crate::entities::ChatEvent,
        crate::entities::EventContext,
    ) {
        use telers::{
            client::Reqwest,
            types::{ChatGroup, MessageText, User},
            Bot,
        };

        let event = crate::entities::ChatEvent::Message(
            MessageText::new(1, 1, ChatGroup::new(100), "/x")
                .from(User::new(8, false, "u"))
                .into(),
        );
        let event_ctx =
            crate::entities::EventContext::new(Bot::<Reqwest>::default(), event.clone());
        (event, event_ctx)
    }

    #[tokio::test]
    async fn default_access_validator_allows_without_settings() {
        use crate::entities::Stack;

        let registry = DialogRegistry::new();
        let (event, event_ctx) = group_event();

        assert!(registry
            .access_validator()
            .is_allowed(&Stack::new(), None, &event, &event_ctx));
    }

    #[tokio::test]
    async fn with_access_validator_replaces_default() {
        use crate::entities::{ChatEvent, Context, EventContext, Stack, StackAccessValidator};

        struct DenyAll;
        impl StackAccessValidator for DenyAll {
            fn is_allowed(
                &self,
                _: &Stack,
                _: Option<&Context>,
                _: &ChatEvent,
                _: &EventContext,
            ) -> bool {
                false
            }
        }

        let registry = DialogRegistry::new().with_access_validator(DenyAll);
        let (event, event_ctx) = group_event();

        assert!(!registry
            .access_validator()
            .is_allowed(&Stack::new(), None, &event, &event_ctx));
    }

    #[test]
    fn default_registry_has_working_media_id_storage() {
        use crate::widgets::media::{MediaContentType, MediaId};

        let registry = DialogRegistry::new();
        registry.media_id_storage().save_media_id(
            None,
            Some("u"),
            MediaContentType::Photo,
            MediaId::new("f"),
        );

        assert_eq!(
            registry
                .media_id_storage()
                .get_media_id(None, Some("u"), MediaContentType::Photo),
            Some(MediaId::new("f"))
        );
    }

    #[test]
    fn with_media_id_storage_replaces_default() {
        use crate::widgets::media::{MediaContentType, MediaId, MediaIdStorage};

        struct NoopStorage;
        impl MediaIdStorage for NoopStorage {
            fn get_media_id(
                &self,
                _: Option<&str>,
                _: Option<&str>,
                _: MediaContentType,
            ) -> Option<MediaId> {
                None
            }

            fn save_media_id(
                &self,
                _: Option<&str>,
                _: Option<&str>,
                _: MediaContentType,
                _: MediaId,
            ) {
            }
        }

        let registry = DialogRegistry::new().with_media_id_storage(NoopStorage);
        registry.media_id_storage().save_media_id(
            None,
            Some("u"),
            MediaContentType::Photo,
            MediaId::new("f"),
        );

        assert!(registry
            .media_id_storage()
            .get_media_id(None, Some("u"), MediaContentType::Photo)
            .is_none());
    }
}
