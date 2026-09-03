use telers::{errors::SessionErrorKind, fsm};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DialogError {
    #[error(transparent)]
    Storage(#[from] fsm::storage::Error),
    #[error(transparent)]
    SessionErrorKind(#[from] SessionErrorKind),
    #[error("No active dialog context")]
    NoContext,
    #[error("Dialog not found for state")]
    DialogNotFound,
    #[error("State `{0}` is not registered in the current dialog")]
    InvalidState(String),
    #[error("Dialog state `{0}` is already registered")]
    DuplicateState(String),
    #[error("No matching state transition from `{0}`")]
    TransitionNotFound(String),
    #[error("Cannot start a new dialog on top of an exclusive dialog")]
    ExclusiveDialogActive,
    #[error("Dialog stack overflow")]
    StackOverflow,
    #[error("Access denied for user {user_id}")]
    AccessDenied { user_id: i64 },
}

#[cfg(test)]
mod tests {
    use super::DialogError;

    #[test]
    fn no_context_message() {
        assert_eq!(
            DialogError::NoContext.to_string(),
            "No active dialog context"
        );
    }

    #[test]
    fn dialog_not_found_message() {
        assert_eq!(
            DialogError::DialogNotFound.to_string(),
            "Dialog not found for state"
        );
    }

    #[test]
    fn invalid_state_message() {
        assert_eq!(
            DialogError::InvalidState("foo".into()).to_string(),
            "State `foo` is not registered in the current dialog"
        );
    }

    #[test]
    fn duplicate_state_message() {
        assert_eq!(
            DialogError::DuplicateState("bar".into()).to_string(),
            "Dialog state `bar` is already registered"
        );
    }

    #[test]
    fn transition_not_found_message() {
        assert_eq!(
            DialogError::TransitionNotFound("s".into()).to_string(),
            "No matching state transition from `s`"
        );
    }

    #[test]
    fn access_denied_message() {
        assert_eq!(
            DialogError::AccessDenied { user_id: 5 }.to_string(),
            "Access denied for user 5"
        );
    }

    #[test]
    fn exclusive_dialog_active_message() {
        assert_eq!(
            DialogError::ExclusiveDialogActive.to_string(),
            "Cannot start a new dialog on top of an exclusive dialog"
        );
    }

    #[test]
    fn stack_overflow_message() {
        assert_eq!(
            DialogError::StackOverflow.to_string(),
            "Dialog stack overflow"
        );
    }
}
