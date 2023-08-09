use super::telegram::handler::Response;

use crate::errors::HandlerError;

use std::fmt::{self, Debug, Display, Formatter};

/// Response, which can be returned from handlers, filters and middlewares by user.
/// This indicates how [`crate::dispatcher::Dispatcher`] should process response.
/// # Notes
/// In some cases, some values may represent the same result
#[derive(Debug, Default, Clone)]
pub enum EventReturn {
    /// Skip the event
    ///
    /// - In outer middleware, means that the middleware should be skipped, and next middleware should be run
    /// - In inner middleware, means that the middleware should be skipped, and next handler with inner middlewares should be run
    /// - In handler, means that the handler should be skipped, and next handler with inner middlewares should be run
    Skip,
    /// Cancel the event
    ///
    /// - In outer middleware, means that propagate the event should be stopped
    /// - In inner middleware, means that propagate the event should be stopped
    /// - In handler, means that the propagate event should return a response from handler or inner middleware
    Cancel,
    /// Finish the event (default). If you don't know what to return, use this.
    ///
    /// - In outer middleware, means that updated request from middleware should be passed to next middleware, and next middleware should be run
    /// - In inner middleware, means that the propagate event should return a response from handler or inner middleware
    /// - In handler, means that the propagate event should return a response from handler
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

/// Shortcut for [`Ok(EventReturn::Skip)`]
#[allow(clippy::missing_errors_doc)]
pub fn skip_event() -> Result<EventReturn, HandlerError> {
    Ok(EventReturn::Skip)
}

/// Shortcut for [`Ok(EventReturn::Cancel)`]
#[allow(clippy::missing_errors_doc)]
pub fn cancel_event() -> Result<EventReturn, HandlerError> {
    Ok(EventReturn::Cancel)
}

/// Shortcut for [`Ok(EventReturn::Finish)`]
#[allow(clippy::missing_errors_doc)]
pub fn finish_event() -> Result<EventReturn, HandlerError> {
    Ok(EventReturn::Finish)
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
            Self::Handled(response) => write!(f, "PropagateEventResult::Handled({response:?})"),
        }
    }
}
