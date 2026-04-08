use crate::{
    dialog::{Dialog, IntoDialog},
    entities::{DefaultAccessValidator, StackAccessValidator},
    errors::DialogError,
};
use std::{collections::BTreeMap, sync::Arc};
use tracing::warn;

pub struct DialogRegistry {
    dialogs: Vec<Arc<dyn Dialog>>,
    state_index: BTreeMap<String, usize>,
    access_validator: Arc<dyn StackAccessValidator>,
}

impl Default for DialogRegistry {
    fn default() -> Self {
        Self {
            dialogs: Vec::new(),
            state_index: BTreeMap::new(),
            access_validator: Arc::new(DefaultAccessValidator),
        }
    }
}

impl Clone for DialogRegistry {
    fn clone(&self) -> Self {
        Self {
            dialogs: self.dialogs.clone(),
            state_index: self.state_index.clone(),
            access_validator: self.access_validator.clone(),
        }
    }
}

impl DialogRegistry {
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

    #[must_use]
    pub fn with_access_validator(mut self, validator: impl StackAccessValidator + 'static) -> Self {
        self.access_validator = Arc::new(validator);
        self
    }

    #[must_use]
    pub fn find_by_state(&self, state: &str) -> Option<Arc<dyn Dialog>> {
        self.state_index
            .get(state)
            .and_then(|i| self.dialogs.get(*i).cloned())
    }

    #[must_use]
    pub fn access_validator(&self) -> &dyn StackAccessValidator {
        self.access_validator.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::DialogRegistry;
    use crate::{dialog, widgets::text, window, DialogError};

    #[test]
    fn registry_rejects_duplicate_states() {
        let registry = DialogRegistry::new()
            .register(dialog([window("state", [text("one")])]))
            .expect("first dialog");

        let err = registry
            .register(dialog([window("state", [text("two")])]))
            .err()
            .expect("duplicate state must fail");

        assert!(matches!(err, DialogError::DuplicateState(_)));
    }
}
