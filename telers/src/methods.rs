//! Telegram Bot API methods and their request builders.
//!
//! This module re-exports all generated method structs from the `crate::methods` module.
//! Each method follows a builder style:
//! - required arguments are passed to `new(...)`
//! - optional arguments are set by chainable builder methods
//! - optional fields also have `_option(...)` variants to pass `Option<T>` directly
//!   (including `None` to clear/unset a field)
//!
//! # Examples
//! ```rust
//! use telers::{errors::SessionErrorKind, methods::SendMessage, Bot};
//!
//! async fn send_text(bot: Bot) -> Result<(), SessionErrorKind> {
//!     let request = SendMessage::new("Hello world!")
//!         .chat_id(1_i64)
//!         // Regular builder setter.
//!         .disable_notification(true)
//!         // `_option(...)` variant for Option<T> values.
//!         .disable_notification_option(Some(true));
//!
//!     bot.send(request).await?;
//!     Ok(())
//! }
//! ```
pub use telers_methods::methods::*;

pub(crate) mod non_telegram;
pub use non_telegram::{base, TelegramMethod};
