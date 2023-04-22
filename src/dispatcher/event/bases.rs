use super::telegram::handler::Response;

use crate::error::EventError;

/// Response, which can be returned from handlers, filters and middlewares by user.
/// This indicates how [`crate::dispatcher::Dispatcher`] should process response.
/// # Notes
/// In some cases, some values may represent the same result
/// # Shortcuts
/// - [`SkipEvent`] - [`EventReturn::Skip`]
/// - [`CancelEvent`] - [`EventReturn::Cancel`]
/// - [`FinishEvent`] - [`EventReturn::Finish`]
#[derive(Debug, Default)]
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

/// Shortcut for [`Ok(EventReturn::Skip)`]
#[allow(clippy::missing_errors_doc)]
pub fn skip_event() -> Result<EventReturn, EventError> {
    Ok(EventReturn::Skip)
}

/// Shortcut for [`Ok(EventReturn::Cancel)`]
#[allow(clippy::missing_errors_doc)]
pub fn cancel_event() -> Result<EventReturn, EventError> {
    Ok(EventReturn::Cancel)
}

/// Shortcut for [`Ok(EventReturn::Finish)`]
#[allow(clippy::missing_errors_doc)]
pub fn finish_event() -> Result<EventReturn, EventError> {
    Ok(EventReturn::Finish)
}

/// Response, which can be returned from routers and observers by program.
/// This indicates [`crate::dispatcher::Dispatcher`] how propagate the event was processed.
#[derive(Debug)]
pub enum PropagateEventResult<Client> {
    /// Event was rejected
    Rejected,
    /// No handler was processed
    Unhandled,
    /// Handler was processed with [`Response`]
    Handled(Response<Client>),
}

mod impl_from {
    use super::EventReturn;

    macro_rules! default_impl_event_return_from {
        ($($t:ty),*) => {
            $(
                impl From<$t> for EventReturn {
                    fn from(_: $t) -> Self {
                        <Self as Default>::default()
                    }
                }
            )*
        };
    }

    default_impl_event_return_from! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, (), usize,
        f32, f64, bool,
        char, &str, String
    }
}
