//! This module contains all the errors that can be returned by the library.
//!
//! This module contains errors:
//! - [`AppErrorKind`]: Main error type that is returned by the library.
//! This enum represents all possible errors that can be returned by the library in processing updates.
//! All user errors are wrapped in the [`AppErrorKind::User`] variant for possible to determine type of errors
//! in higher levels of the application.
//! For example, [`EventError`], which is used to represent
//! all possible errors that can be returned from handlers, converts to [`AppErrorKind::User`].
//! So, if you want to handle user errors, you should match [`AppErrorKind::User`] variant in middlewares,
//! but if you want to handle extraction errors, you should match [`AppErrorKind::Extraction`] variant the same.
//! - [`EventError`]: Error type that can occur in the handlers.
//! Usually, in handlers returns [`SessionErrorKind`] error or [`TelegramErrorKind`] error,
//! but you can return any error that implements [`Into<anyhow::Error>`] trait.
//! - [`ExtractionError`]: Error type represents error that can occur in the extraction process.
//! Check [`crate::extract`] module for more information about extraction process.
//! - [`SessionErrorKind`]: Error type that represents all possible errors that can occur in the process
//! of sending requests to the Telegram API and parsing responses.
//! - [`TelegramErrorKind`]: Error type that represents all possible errors that can be returned from Telegram Bot API.
//! This enum isn't complete. If you find a new error, please open an issue or pull request.
//! All possible errors aren't documented in the official Telegram API documentation and usually
//! defined by messages in the responses, but these messages can be changed in the future (frequent situation).
//! So, many errors are represents as [`TelegramErrorKind::BadRequest`], and we are not trying to distinguish them
//! for stability. Thanks Telegram Bot API for this ^_^.
//! - [`ConvertUpdateToTypeError`]: Error occurs when the update type cannot be converted to the desired type.
//! For example, if you try to convert an update to a [`crate::types::Message`] type,
//! but the update represents a [`crate::types::CallbackQuery`], you will get this error.
//! - [`UnknownUpdateTypeError`]: Error to possible handle incorrect update types or unsupported update types,
//! that can be added in the new versions of the Telegram Bot API.
//! This is necessary to support the "old" API if the "new" API has released a new update type.

#![allow(clippy::module_name_repetitions)]

pub mod app;
pub mod event;
pub mod extractor;
pub mod session;
pub mod telegram;
pub mod update;
pub mod update_type;

pub use app::ErrorKind as AppErrorKind;
pub use event::Error as EventError;
pub use extractor::Error as ExtractionError;
pub use session::ErrorKind as SessionErrorKind;
pub use telegram::ErrorKind as TelegramErrorKind;
pub use update::ConvertUpdateToType as ConvertUpdateToTypeError;
pub use update_type::UnknownUpdateType as UnknownUpdateTypeError;
