//! This module contains outer middlewares.
//!
//! Middlewares are called `outer` if they called before filters, inner middlewares and handlers.
//! These middlewares have access to the [`request`] (with [`context`] in it),
//! but don't have access to the middlewares/handler-chain and the [`response`] (for these purposes, use [`inner middlewares`]).
//!
//! Prefer to use outer middlewares over inner middlewares in some cases:
//! - If you need to call middlewares before filters, inner middlewares and handlers
//! - If you need to manipulate with [`request`] and [`context`] in it
//!
//! You can check example of using outer middlewares in `examples/stats_incoming_updates_middleware`.
//!
//! [`request`]: crate::event::telegram::HandlerRequest
//! [`response`]: crate::event::telegram::HandlerResponse
//! [`context`]: crate::context::Context
//! [`inner middlewares`]: crate::middlewares::inner

pub mod base;
pub mod fsm_context;
pub mod manager;
pub mod user_context;

pub use base::{Middleware, MiddlewareResponse};
pub use fsm_context::FSMContext;
pub use manager::Manager;
pub use user_context::UserContext;
