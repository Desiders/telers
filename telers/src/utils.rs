mod error;

pub(crate) use error::format_error_report;

pub mod chat_action;
pub mod text;
pub mod token;

pub use chat_action::{ChatActionGuard, ChatActionSender};

#[cfg(feature = "signal")]
pub mod signal;

#[cfg(feature = "signal")]
pub use signal::shutdown_signal;
