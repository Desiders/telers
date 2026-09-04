//! Errors that can be returned by the library.
//!
//! The errors you will meet most often:
//! - [`SessionErrorKind`] is returned by [`Bot::send`](crate::Bot::send): a network/client
//!   failure or an error response from the Telegram API ([`TelegramErrorKind`], e.g. flood
//!   control or a malformed request)
//! - [`DownloadErrorKind`] is returned by [`Bot::download`](crate::Bot::download) and related
//!   methods: a `getFile` failure, a missing file path, or an I/O error
//! - [`HandlerError`] is the error type your handlers return, wrapping any [`anyhow::Error`]
//! - [`ExtractionError`] means a handler argument couldn't be extracted from the request
//!   (see the [`extractor` module](crate::extractor))
//! - [`ConvertToTypeError`] means a conversion between an enum and one of its variants failed,
//!   e.g. extracting a [`Message`](crate::types::Message) from an update of another kind
//! - [`EventErrorKind`] groups the errors that can interrupt event propagation
//!   (filter, middleware and handler errors)

#![allow(clippy::module_name_repetitions)]

pub mod convert;
pub mod download;
pub mod event;
pub mod extractor;
pub mod filter;
pub mod handler;
pub mod middleware;
pub mod session;
pub mod telegram;

pub use convert::ConvertToType as ConvertToTypeError;
pub use download::ErrorKind as DownloadErrorKind;
pub use event::ErrorKind as EventErrorKind;
pub use extractor::Error as ExtractionError;
pub use filter::Error as FilterError;
pub use handler::Error as HandlerError;
pub use middleware::Error as MiddlewareError;
pub use session::ErrorKind as SessionErrorKind;
pub use telegram::ErrorKind as TelegramErrorKind;
