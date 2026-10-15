//! Handlers, observer and response types for Telegram events.
//!
//! Register a [`Handler`] in the [`Observer`] of an update type; its filters are checked before
//! the handler runs and what the handler returns ([`HandlerResult`]) controls the propagation.

pub mod handler;
pub mod observer;
pub mod response;

pub use handler::{Handler, HandlerFn};
pub use observer::Observer;
pub use response::{HandlerResult, IntoHandlerResult, Response as HandlerResponse};
