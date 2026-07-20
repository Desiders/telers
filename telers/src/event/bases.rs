use super::telegram::response::Response;
use crate::errors::HandlerError;

use std::fmt::{self, Debug, Display, Formatter};

/// Response, which can be returned from handlers, filters and middlewares by user.
/// This indicates how [`crate::dispatcher::Dispatcher`] should process response.
///
/// The meaning of each variant depends on where it is returned from;
/// see the [routing docs](crate::router) for the full picture.
/// # Notes
/// In some cases, some values may represent the same result
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EventReturn {
    /// In outer middlewares: skip the [`Request`](crate::Request) changes made in the middleware and continue.
    ///
    /// In inner middlewares and handlers: skip the current handler and go to the next one (and its filters).
    Skip,
    /// In outer middlewares: stop event propagation.
    ///
    /// In inner middlewares and handlers: stop event propagation for the current router and go to the next router.
    Cancel,
    /// In outer middlewares: save the [`Request`](crate::Request) changes made in the middleware and continue.
    ///
    /// In inner middlewares and handlers: finish event propagation.
    #[default]
    Finish,
}

impl Display for EventReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Skip => write!(f, "skip"),
            Self::Cancel => write!(f, "cancel"),
            Self::Finish => write!(f, "finish"),
        }
    }
}

/// Shortcut for `Ok(EventReturn::Skip)`
#[inline]
#[allow(clippy::missing_errors_doc)]
pub const fn skip_event() -> Result<EventReturn, HandlerError> {
    Ok(EventReturn::Skip)
}

/// Shortcut for `Ok(EventReturn::Cancel)`
#[inline]
#[allow(clippy::missing_errors_doc)]
pub const fn cancel_event() -> Result<EventReturn, HandlerError> {
    Ok(EventReturn::Cancel)
}

/// Shortcut for `Ok(EventReturn::Finish)`
#[inline]
#[allow(clippy::missing_errors_doc)]
pub const fn finish_event() -> Result<EventReturn, HandlerError> {
    Ok(EventReturn::Finish)
}

impl From<()> for EventReturn {
    #[inline]
    fn from(_value: ()) -> Self {
        EventReturn::default()
    }
}

/// Response, which can be returned from routers and observers by program.
/// This indicates [`crate::dispatcher::Dispatcher`] how propagate the event was processed.
pub enum PropagateEventResult<Client> {
    /// Event was rejected
    Rejected,
    /// No handler was processed
    Unhandled,
    /// Handler was processed with [`Response`]
    Handled(Response<Client>),
}

impl<Client> Debug for PropagateEventResult<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected => write!(f, "PropagateEventResult::Rejected"),
            Self::Unhandled => write!(f, "PropagateEventResult::Unhandled"),
            Self::Handled(response) => {
                write!(f, "PropagateEventResult::Handled({response:?})")
            }
        }
    }
}
