mod error;

pub(crate) use error::format_error_report;

pub mod text;
pub mod token;

#[cfg(feature = "signal")]
pub mod signal;

#[cfg(feature = "signal")]
pub use signal::shutdown_signal;
