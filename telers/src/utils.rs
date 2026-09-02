//! Helper utilities around the core routing flow.
//!
//! - [`text`] contains utilities for building, formatting and rendering message text
//!   with entities (HTML / `MarkdownV2`)
//! - [`chat_action`] keeps a chat action like "typing" alive during long-running
//!   operations via [`ChatActionSender`]
//! - [`deep_linking`] encodes and decodes payloads for Telegram deep links
//! - [`keyboard`] contains builders for inline and reply keyboards
//! - [`media_group`] contains a builder for media groups (albums)
//! - [`token`] validates a bot token and extracts the bot id from it
//! - [`signal`] (feature `signal`) provides a [`shutdown_signal`] future for graceful shutdown

mod error;

pub(crate) use error::format_error_report;

pub mod chat_action;
pub mod deep_linking;
pub mod keyboard;
pub mod media_group;
pub mod text;
pub mod token;

pub use chat_action::{ChatActionGuard, ChatActionSender};
pub use deep_linking::{decode_payload, encode_payload};
pub use keyboard::{InlineKeyboardBuilder, KeyboardBuilder, ReplyKeyboardBuilder};
pub use media_group::MediaGroupBuilder;

#[cfg(feature = "signal")]
pub mod signal;

#[cfg(feature = "signal")]
pub use signal::shutdown_signal;
