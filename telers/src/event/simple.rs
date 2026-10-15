//! Handlers and observer for events that are not Telegram updates, like startup and shutdown.
//!
//! A [`Handler`] stores the arguments it is called with, so the observer triggers it with an empty
//! request, and every handler returns [`HandlerResult`].

pub mod handler;
pub mod observer;

pub use handler::{Handler, HandlerFn, HandlerResult};
pub use observer::Observer;
