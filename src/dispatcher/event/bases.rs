use std::error::Error;

pub trait EventReturn {
    /// # Returns
    /// `true` if the event should skip
    /// `false` if the event shouldn't skip
    fn is_skip(&self) -> bool {
        false
    }

    /// # Returns
    /// `true` if the event should cancel
    /// `false` if the event shouldn't cancel
    fn is_cancel(&self) -> bool {
        false
    }
}

pub enum Action {
    /// Let possible skip handler and continue to next handler. Can be useful in middlewares and handlers.
    Skip,
    /// Let possible cancel event and stop to next handler. Can be useful in middlewares.
    Cancel,
}

impl EventReturn for Action {
    fn is_skip(&self) -> bool {
        matches!(self, Self::Skip)
    }

    fn is_cancel(&self) -> bool {
        matches!(self, Self::Cancel)
    }
}

impl<T> EventReturn for Option<T> {}

impl<T, E> EventReturn for Result<T, E> {}

impl<T> EventReturn for Box<T> {}

impl EventReturn for () {}

impl EventReturn for dyn Error {}
