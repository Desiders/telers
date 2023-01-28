use super::telegram::handler::Response;

use crate::error::{app, telegram};

/// Responses from events, that indicates how the dispatcher should process response
#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct EventReturn {
    /// - In outer middlewares, means that the middleware should be skipped, and next middleware should be run
    /// - In inner middlewares, means that the middleware should be skipped, and next handler should be run
    /// - In handler, means that the handler should be skipped, and next handler should be run
    is_skip: bool,
    /// - In outer middlewares, means that propagate the event should be stopped
    /// - In inner middlewares, means that propagate the event should be stopped
    /// - In handler, means that propagate the event should be stopped
    is_cancel: bool,
}

impl EventReturn {
    /// # Arguments
    /// * `is_skip` -
    ///     - In outer middlewares, means that the middleware should be skipped, and next middleware should be run
    ///     - In inner middlewares, means that the middleware should be skipped, and next handler should be run
    ///     - In handler, means that the handler should be skipped, and next handler should be run
    /// * `is_cancel` -
    ///     - In outer middlewares, means that propagate the event should be stopped
    ///     - In inner middlewares, means that propagate the event should be stopped
    ///     - In handler, means that propagate the event should be stopped
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

/// A wrapper to [`EventReturn`].
pub enum Action {
    /// - In outer middlewares, means that the middleware should be skipped, and next middleware should be run
    /// - In inner middlewares, means that the middleware should be skipped, and next handler should be run
    /// - In handler, means that the handler should be skipped, and next handler should be run
    Skip,
    /// - In outer middlewares, means that propagate the event should be stopped
    /// - In inner middlewares, means that propagate the event should be stopped
    /// - In handler, means that propagate the event should be stopped
    Cancel,
}

/// Responses from routers and observers
pub enum PropagateEventResult {
    Rejected,
    Unhandled,
    Handled(Response),
}

mod impl_from {
    use super::{app, telegram, Action, EventReturn};

    impl From<Action> for EventReturn {
        fn from(action: Action) -> Self {
            match action {
                Action::Skip => Self {
                    is_skip: true,
                    is_cancel: false,
                },
                Action::Cancel => Self {
                    is_skip: false,
                    is_cancel: true,
                },
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
        // Implement `From` for many `T` without lifetimes
        ($($T:ty),* $(,)?) => {
            $(
                impl From<$T> for EventReturn {
                    fn from(_: $T) -> Self {
                        Self::default()
                    }
                }
            )*
        };
    }

    // Implement `From` for `T` with one or more lifetimes
    default_impl_from!(Result<T, E>, T, E);
    default_impl_from!(Option<T>, T);
    default_impl_from!(Box<T>, T);
    // Implement `From` for many `T` without lifetimes
    default_impl_from!((), app::ErrorKind, telegram::ErrorKind);
}
