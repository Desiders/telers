use std::error::Error;

pub trait EventReturn {
    fn is_skip(&self) -> bool {
        false
    }

    fn is_cancel(&self) -> bool {
        false
    }
}

/// Let possible skip handler and continue to next handler. Can be useful in middlewares and handlers.
pub enum SkipHandler {}

impl EventReturn for SkipHandler {
    fn is_skip(&self) -> bool {
        true
    }
}

/// Let possible cancel event and stop to next handler. Can be useful in middlewares.
pub enum CancelHandler {}

impl EventReturn for CancelHandler {
    fn is_cancel(&self) -> bool {
        true
    }
}

impl<T> EventReturn for Option<T> {}

impl<T, E> EventReturn for Result<T, E> {}

impl EventReturn for () {}

impl EventReturn for dyn Error {}
