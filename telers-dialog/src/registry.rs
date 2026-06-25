use crate::{
    dialog::{Dialog, IntoDialog},
    entities::{DefaultAccessValidator, StackAccessValidator},
    errors::DialogError,
    widgets::media::{InMemoryMediaIdStorage, MediaIdStorage},
};
use std::{collections::BTreeMap, sync::Arc};
use tracing::warn;

/// Registry of dialogs indexed by state id.
///
/// The registry is typically built once during startup and then injected into
/// the dispatcher as shared application state.
pub struct DialogRegistry {
    dialogs: Vec<Arc<dyn Dialog>>,
    state_index: BTreeMap<String, usize>,
    access_validator: Arc<dyn StackAccessValidator>,
    media_id_storage: Arc<dyn MediaIdStorage>,
}

impl Default for DialogRegistry {
    fn default() -> Self {
        Self {
            dialogs: Vec::new(),
            state_index: BTreeMap::new(),
            access_validator: Arc::new(DefaultAccessValidator),
            media_id_storage: Arc::new(InMemoryMediaIdStorage::default()),
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
    use super::DialogRegistry;
    use crate::{dialog, widgets::text, window, DialogError};

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
        crate::entities::EventContext<telers::client::Reqwest>,
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
            crate::entities::EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
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
