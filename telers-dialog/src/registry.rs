use crate::{
    dialog::{Dialog, IntoDialog},
    errors::DialogError,
};
use std::{collections::BTreeMap, sync::Arc};
use tracing::warn;

#[derive(Default, Clone)]
pub struct DialogRegistry {
    dialogs: Vec<Arc<dyn Dialog>>,
    state_index: BTreeMap<String, usize>,
}

impl DialogRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
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
    pub fn find_by_state(&self, state: &str) -> Option<Arc<dyn Dialog>> {
        self.state_index
            .get(state)
            .and_then(|i| self.dialogs.get(*i).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::DialogRegistry;
    use crate::{widgets::WidgetKind, DialogError, DialogImpl, WindowImpl};

    #[test]
    fn registry_rejects_duplicate_states() {
        let registry = DialogRegistry::new()
            .register(DialogImpl::new(vec![WindowImpl::new(
                "state",
                [WidgetKind::text("one")],
            )]))
            .expect("first dialog");

        let err = registry
            .register(DialogImpl::new(vec![WindowImpl::new(
                "state",
                [WidgetKind::text("two")],
            )]))
            .err()
            .expect("duplicate state must fail");

        assert!(matches!(err, DialogError::DuplicateState(_)));
    }
}
