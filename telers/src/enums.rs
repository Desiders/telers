//! Enum helpers and discriminator types for Telegram objects.
//!
//! This module contains:
//! - generated `*Type` enums for polymorphic Telegram objects (for example `MessageType`)
//! - hand-authored enums like [`ParseMode`]
//!
//! # Examples
//! ```rust
//! use telers::{enums::ParseMode, methods::SendMessage};
//!
//! let request = SendMessage::new("*Hello world!*")
//!     .chat_id(1_i64)
//!     .parse_mode(ParseMode::Markdown);
//!
//! assert_eq!(request.parse_mode.as_deref(), Some("Markdown"));
//! ```
pub use telers_types::enums::*;

pub mod telegram_observer_type;
pub use telegram_observer_type::TelegramObserverType;
