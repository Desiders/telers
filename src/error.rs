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
