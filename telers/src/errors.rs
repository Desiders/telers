//! This module contains errors that can be returned by the library.
//!
//! This module contains errors:
//! - [`EventErrorKind`]
//! - [`HandlerError`]
//! - [`ExtractionError`]
//! - [`SessionErrorKind`]
//! - [`TelegramErrorKind`]
//! - [`ConvertToTypeError`]
//! - [`UnknownUpdateTypeError`]
//! Check the documentation for each error to see what it means.

#![allow(clippy::module_name_repetitions)]

pub mod convert;
pub mod event;
pub mod extractor;
pub mod handler;
pub mod middleware;
pub mod session;
pub mod telegram;
pub mod update_type;

pub use convert::ConvertToType as ConvertToTypeError;
pub use event::ErrorKind as EventErrorKind;
pub use extractor::Error as ExtractionError;
pub use handler::Error as HandlerError;
pub use middleware::Error as MiddlewareError;
pub use session::ErrorKind as SessionErrorKind;
pub use telegram::ErrorKind as TelegramErrorKind;
pub use update_type::UnknownUpdateType as UnknownUpdateTypeError;