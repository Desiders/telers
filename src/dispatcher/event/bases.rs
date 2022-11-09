use super::TelegramHandlerResponse;

use crate::error::{app, telegram};

/// Response from handlers or middlewares, that indicates how the dispatcher should process response
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReturn {
    /// Let possible skip handler and continue to next handler. Can be useful in middlewares and handlers
    is_skip: bool,
    /// Let possible cancel event and stop to next handler. Can be useful in middlewares.
    /// This is useless in handlers.
    is_cancel: bool,
}

impl EventReturn {
    /// Create a new event return
    #[must_use]
    pub fn new(is_skip: bool, is_cancel: bool) -> Self {
        Self { is_skip, is_cancel }
    }

    #[must_use]
    pub fn is_skip(&self) -> bool {
        self.is_skip
    }

    #[must_use]
    pub fn is_cancel(&self) -> bool {
        self.is_cancel
    }
}

/// A special enumeration containing all possible responses from events.
/// This is wrapper to `Event Return`.
pub enum Action {
    /// Let possible skip handler and continue to next handler. Can be useful in middlewares and handlers
    Skip,
    /// Let possible cancel event and stop to next handler. Can be useful in middlewares
    /// This is useless in handlers
    Cancel,
}

impl From<Action> for EventReturn {
    fn from(action: Action) -> Self {
        Self {
            is_skip: matches!(action, Action::Skip),
            is_cancel: matches!(action, Action::Cancel),
        }
    }
}

impl<T, E> From<Result<T, E>> for EventReturn {
    fn from(_: Result<T, E>) -> Self {
        Self {
            is_skip: false,
            is_cancel: false,
        }
    }
}

impl<T> From<Option<T>> for EventReturn {
    fn from(_: Option<T>) -> Self {
        Self {
            is_skip: false,
            is_cancel: false,
        }
    }
}

impl<T> From<Box<T>> for EventReturn {
    fn from(_: Box<T>) -> Self {
        Self {
            is_skip: false,
            is_cancel: false,
        }
    }
}

impl From<()> for EventReturn {
    fn from(_: ()) -> Self {
        Self {
            is_skip: false,
            is_cancel: false,
        }
    }
}

impl From<app::Error> for EventReturn {
    fn from(_: app::Error) -> Self {
        Self {
            is_skip: false,
            is_cancel: false,
        }
    }
}

impl From<telegram::Error> for EventReturn {
    fn from(_: telegram::Error) -> Self {
        Self {
            is_skip: false,
            is_cancel: false,
        }
    }
}

pub enum PropagateEventResult {
    /// Event was rejected
    Rejected,
    /// Event was unhandled
    Unhandled,
    /// Event was been handled and retured [`TelegramHandlerResponse`]
    Handled(TelegramHandlerResponse),
}
