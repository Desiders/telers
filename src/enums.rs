//! This module contains enums used in the library for simple way to use it instead of using strings (constants),
//! which not so comfortable and not so safe, because you can make a typo in string and it will compile without errors.
//!
//! This module contains enums for:
//! - [`BotCommandScopeType`]: Scope of bot commands, which is used in [`crate::types::BotCommandScope`].
//! - [`ChatAction`]: Type of action to broadcast.
//! - [`ChatMemberStatus`]: Status of a chat member.
//! - [`ChatType`]: Type of a chat.
//! - [`ContentType`]: Type of a content.
//! - [`DiceEmoji`]: Emoji on which the dice throw animation is based.
//! - [`InlineQueryResultType`]: Type of the result, which is used in [`crate::types::InlineQueryResult`].
//! - [`InputMediaType`]: Type of the media to send, which is used in [`crate::types::InputMedia`].
//! - [`MaskPositionPoint`]: Part of the face, relative to which the mask should be placed.
//! - [`MenuButtonType`]: Type of a button in a custom keyboard, which is used in [`crate::types::MenuButton`].
//! - [`MessageEntityType`]: Type of a message entity.
//! - [`ParseMode`]: Mode for parsing entities in the message text.
//! - [`PollType`]: Type of a poll.
//! - [`StickerFormat`]: Format of a sticker, represented as a string.
//! - [`StickerType`]: Type of a sticker.
//! - [`TopicIconColor`]: Color of a topic icon.
//! - [`UpdateType`]: Type of an incoming update.
//!
//! You can pass these enums to methods, because they implement [`std::convert::Into`] trait for [`String`].
//!
//! Every enum has a `all` method that returns a list of all possible variants of the enum.

pub mod bot_command_scope_type;
pub mod chat_action;
pub mod chat_member_status;
pub mod chat_type;
pub mod content_type;
pub mod dice_emoji;
pub mod inline_query_result_type;
pub mod input_media_type;
pub mod mask_position_point;
pub mod menu_button_type;
pub mod message_entity_type;
pub mod observer_name;
pub mod parse_mode;
pub mod poll_type;
pub mod sticker_format;
pub mod sticker_type;
pub mod topic_icon_color;
pub mod update_type;

pub use bot_command_scope_type::BotCommandScopeType;
pub use chat_action::ChatAction;
pub use chat_member_status::ChatMemberStatus;
pub use chat_type::ChatType;
pub use content_type::ContentType;
pub use dice_emoji::DiceEmoji;
pub use inline_query_result_type::InlineQueryResultType;
pub use input_media_type::InputMediaType;
pub use mask_position_point::MaskPositionPoint;
pub use menu_button_type::MenuButtonType;
pub use message_entity_type::MessageEntityType;
pub use observer_name::{Simple as SimpleObserverName, Telegram as TelegramObserverName};
pub use parse_mode::ParseMode;
pub use poll_type::PollType;
pub use sticker_format::StickerFormat;
pub use sticker_type::StickerType;
pub use topic_icon_color::TopicIconColor;
pub use update_type::UpdateType;
