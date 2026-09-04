//! Event observers, handlers and propagation control.
//!
//! - [`telegram`] contains handlers and observers for Telegram events (messages, callback
//!   queries, etc.); handler functions return [`telegram::HandlerResult`]
//! - [`simple`] contains handlers and observers for simple events like startup and shutdown
//! - [`error`] contains handlers and observers for error events (see the module docs)
//! - [`EventReturn`] controls how an event propagates further; the [`finish_event`],
//!   [`skip_event`] and [`cancel_event`] shortcuts wrap it in `Ok(...)`
//!
//! See the [`router` module](crate::router) for how observers fit into event routing.

#![allow(clippy::module_name_repetitions)]

pub mod bases;
pub mod error;
pub mod service;
pub mod simple;
pub mod telegram;

pub use bases::{cancel_event, finish_event, skip_event, EventReturn};
pub use error::{ErrorEvent, PropagateErrorResult};
