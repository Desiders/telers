//! This module contains inner middlewares.
//!
//! Middlewares are called `inner` if they called before handler, but after outer middlewares and filers.
//! These middlewares have access to the middlewares/handler-chain
//! and can modify the [`request`] (with [`context`] in it)/[`response`].
//!
//! Prefer to use inner middlewares over outer middlewares in some cases:
//! - If you need to call middlewares after filters and before handlers
//! - If you need to manipulate with call of next middleware or handler
//! - If you need to manipulate with [`request`] or [`response`]
//!
//! You can check example of using inner middlewares in `examples/stats_incoming_updates_middleware`.
//!
//! [`request`]: crate::event::telegram::HandlerRequest
//! [`response`]: crate::event::telegram::HandlerResponse
//! [`context`]: crate::context::Context

pub mod base;
pub mod logging;
pub mod manager;

pub use base::{wrap_handler_and_middlewares_to_next, Middleware, Next};
pub use logging::Logging;
pub use manager::Manager;
