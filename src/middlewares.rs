//! This module contains inner and outer middlewares.
//!
//! Middlewares are designed to be `inner` and `outer`.
//! You can read more about each middleware type in their modules:
//! - [`inner module`]
//! - [`outer module`]
//!
//! [`inner module`]: inner
//! [`outer module`]: outer

#![allow(clippy::module_name_repetitions)]

pub mod inner;
pub mod outer;

pub use inner::{Middleware as InnerMiddleware, Middlewares as InnerMiddlewares, Next};
pub use outer::{Middleware as OuterMiddleware, Middlewares as OuterMiddlewares};
