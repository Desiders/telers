//! This module contains inner middlewares.
//!
//! Middlewares are called `inner` if they are called before the handler, but after outer middlewares and filters.
//! These middlewares have access to the middlewares/handler-chain
//! and can modify the [`request`] (with [`context`] in it)/[`response`].
//!
//! Prefer to use inner middlewares over outer middlewares in some cases:
//! - If you need to call middlewares after filters and before handlers
//! - If you need to manipulate the call of the next middleware or handler
//! - If you need to manipulate the [`request`] or the [`response`]
//!
//! You can check an example of using inner middlewares in `examples/stats_incoming_updates_middleware`.
//!
//! [`request`]: telers::Request
//! [`response`]: telers::event::telegram::HandlerResponse
//! [`context`]: telers::context::Context

pub mod base;
pub mod logging;
pub mod manager;
pub mod throttling;

pub(crate) use base::{boxed_middleware_factory, BoxedCloneMiddlewareService};

pub use base::{wrap_to_next, Middleware, Next};
pub use logging::Logging;
pub use manager::Manager;
pub use throttling::{Strategy, Throttling};
