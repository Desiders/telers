//! Telegram Bot API data types and helper models.
//!
//! This module re-exports all generated types from the `crate::types` module, including
//! shared helper types from `non_telegram`.
//! Generated type builders follow the same conventions as method builders:
//! - optional fields can be set with normal chainable builder methods
//! - optional fields also expose `_option(...)` variants to pass `Option<T>` directly
//!   (including `None` to clear/unset a field)
//!
//! For polymorphic API objects (for example [`crate::types::Message`]), telers uses enums with
//! split subtypes. In these cases, use generated helper methods like `message.chat()`
//! and `message.text()` instead of field access like `message.chat`.
//!
//! Messages, their subtypes and callback queries also have shortcuts that create methods for them
//! with the fields filled from the object, for example `message.answer(text)` creates
//! [`crate::methods::SendMessage`] to the chat of the message, `message.reply(text)` also as a reply to it,
//! and `callback_query.answer()` creates [`crate::methods::AnswerCallbackQuery`].
//! Import [`CallbackQueryShortcuts`] to use callback query shortcuts, and [`MessageShortcuts`]
//! to use message shortcuts. The data objects are also available independently in `telers-types`.
//! Optional fields are set with the builders of the method, as usual.
//! The text and the caption of a message are rendered back to HTML or `MarkdownV2` with their entities
//! by the shortcuts of [`crate::types::MessageRenderers`].
//!
//! # Examples
//! ```rust
//! use telers::types::{ChatIdKind, InlineKeyboardButton, InlineKeyboardMarkup};
//!
//! let chat_id = ChatIdKind::id(1);
//! let keyboard =
//!     InlineKeyboardMarkup::new([[InlineKeyboardButton::new("Open Telegram API docs")
//!         // Regular builder setter.
//!         .url("https://core.telegram.org/bots/api")
//!         // `_option(...)` variant for Option<T> values.
//!         .url_option(Some("https://core.telegram.org/bots/api"))]]);
//!
//! assert!(matches!(chat_id, ChatIdKind::Id(_)));
//! assert_eq!(keyboard.inline_keyboard.len(), 1);
//! ```
//!
//! ```rust
//! use telers::types::Message;
//!
//! fn inspect_message(message: &Message) {
//!     // `Message` is an enum, so helper methods provide unified access.
//!     let _chat = message.chat();
//!     let _message_id = message.message_id();
//!     let _maybe_text = message.text();
//! }
//! ```
pub use telers_types::types::*;

pub(crate) mod to_methods;
pub use to_methods::{CallbackQueryShortcuts, MessageRenderers, MessageShortcuts};
