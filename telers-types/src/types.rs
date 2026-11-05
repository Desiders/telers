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
//! # Examples
//! ```rust
//! use telers_types::types::{ChatIdKind, InlineKeyboardButton, InlineKeyboardMarkup};
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
//! use telers_types::types::Message;
//!
//! fn inspect_message(message: &Message) {
//!     // `Message` is an enum, so helper methods provide unified access.
//!     let _chat = message.chat();
//!     let _message_id = message.message_id();
//!     let _maybe_text = message.text();
//! }
//! ```
#![allow(clippy::too_many_arguments)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::large_enum_variant)]
pub use telers_types_common::types::*;
pub use telers_types_input::types::*;
pub use telers_types_messages::types::*;
pub mod callback_query;
pub mod chat_full_info;
pub mod chat_full_info_channel;
pub mod chat_full_info_group;
pub mod chat_full_info_private;
pub mod chat_full_info_supergroup;
pub mod chat_full_info_unknown;
pub mod update;
pub mod update_business_message;
pub mod update_callback_query;
pub mod update_channel_post;
pub mod update_edited_business_message;
pub mod update_edited_channel_post;
pub mod update_edited_message;
pub mod update_guest_message;
pub mod update_message;
pub use callback_query::CallbackQuery;
pub use chat_full_info::ChatFullInfo;
pub use chat_full_info_channel::ChatFullInfoChannel;
pub use chat_full_info_group::ChatFullInfoGroup;
pub use chat_full_info_private::ChatFullInfoPrivate;
pub use chat_full_info_supergroup::ChatFullInfoSupergroup;
pub use chat_full_info_unknown::ChatFullInfoUnknown;
pub use update::Update;
pub use update_business_message::UpdateBusinessMessage;
pub use update_callback_query::UpdateCallbackQuery;
pub use update_channel_post::UpdateChannelPost;
pub use update_edited_business_message::UpdateEditedBusinessMessage;
pub use update_edited_channel_post::UpdateEditedChannelPost;
pub use update_edited_message::UpdateEditedMessage;
pub use update_guest_message::UpdateGuestMessage;
pub use update_message::UpdateMessage;
