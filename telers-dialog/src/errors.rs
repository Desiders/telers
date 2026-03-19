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
}
