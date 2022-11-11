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

impl Default for EventReturn {
    fn default() -> Self {
        Self {
            is_skip: false,
            is_cancel: false,
        }
    }
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
/// This is wrapper to [`EventReturn`].
pub enum Action {
    /// Let possible skip handler and continue to next handler. Can be useful in middlewares and handlers
    Skip,
    /// Let possible cancel event and stop to next handler. Can be useful in middlewares
    /// This is useless in handlers
    Cancel,
}

/// A special enumeration containing all possible responses from observers
pub enum PropagateEventResult {
    /// Event was rejected
    Rejected,
    /// Event was unhandled
    Unhandled,
    /// Event was been handled and retured [`TelegramHandlerResponse`]
    Handled(TelegramHandlerResponse),
}

mod impl_from {
    use super::{app, telegram, Action, EventReturn};

    impl From<Action> for EventReturn {
        fn from(action: Action) -> Self {
            Self {
                is_skip: matches!(action, Action::Skip),
                is_cancel: matches!(action, Action::Cancel),
            }
        }
    }

    macro_rules! default_impl_from {
        // Implement `From` for `T` with one or more lifetimes
        ($T:ty, $($lifetime:tt),* $(,)?) => {
            impl<$($lifetime,)*> From<$T> for EventReturn {
                fn from(_: $T) -> Self {
                    Self::default()
                }
            }
        };
        // Implement `From` for `T` without lifetimes
        ($T:ty) => {
            impl From<$T> for EventReturn {
                fn from(_: $T) -> Self {
                    Self::default()
                }
            }
        };
        // Implement `From` for many `T` without lifetimes
        ($($T:ty),* $(,)?) => {
            $(
                default_impl_from!($T);
            )*
        };
    }

    default_impl_from!(Result<T, E>, T, E);
    default_impl_from!(Option<T>, T);
    default_impl_from!(Box<T>, T);
    default_impl_from!((), app::Error, telegram::Error);
}
