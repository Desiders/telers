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
//! let request = SendMessage::new(1_i64, "*Hello world!*").parse_mode(ParseMode::Markdown);
//!
//! assert_eq!(request.parse_mode.as_deref(), Some("Markdown"));
//! ```
pub mod background_fill;
pub mod background_type;
pub mod bot_command_scope;
pub mod chat;
pub mod chat_boost_source;
pub mod chat_full_info;
pub mod chat_member;
pub mod encrypted_passport_element;
pub mod external_reply_info;
pub mod giveaway;
pub mod giveaway_winners;
pub mod inline_query_result;
pub mod inline_query_result_audio_kind;
pub mod inline_query_result_document_kind;
pub mod inline_query_result_gif_kind;
pub mod inline_query_result_video_kind;
pub mod inline_query_result_voice_kind;
pub mod input_media;
pub mod input_message_content;
pub mod input_paid_media;
pub mod input_profile_photo;
pub mod input_story_content;
pub mod maybe_inaccessible_message;
pub mod menu_button;
pub mod message;
pub mod message_entity;
pub mod message_origin;
pub mod owned_gift;
pub mod paid_media;
pub mod parse_mode;
pub mod passport_element_error;
pub mod poll;
pub mod reaction_type;
pub mod reply_markup;
pub mod revenue_withdrawal_state;
pub mod star_transaction;
pub mod sticker;
pub mod story_area_type;
pub mod telegram_observer_type;
pub mod transaction_partner;
pub mod transaction_partner_user;
pub mod update;
pub use background_fill::BackgroundFillType;
pub use background_type::BackgroundTypeType;
pub use bot_command_scope::BotCommandScopeType;
pub use chat::ChatType;
pub use chat_boost_source::ChatBoostSourceType;
pub use chat_full_info::ChatFullInfoType;
pub use chat_member::ChatMemberType;
pub use encrypted_passport_element::EncryptedPassportElementType;
pub use external_reply_info::ExternalReplyInfoType;
pub use giveaway::GiveawayType;
pub use giveaway_winners::GiveawayWinnersType;
pub use inline_query_result::InlineQueryResultType;
pub use inline_query_result_audio_kind::InlineQueryResultAudioKindType;
pub use inline_query_result_document_kind::InlineQueryResultDocumentKindType;
pub use inline_query_result_gif_kind::InlineQueryResultGifKindType;
pub use inline_query_result_video_kind::InlineQueryResultVideoKindType;
pub use inline_query_result_voice_kind::InlineQueryResultVoiceKindType;
pub use input_media::InputMediaType;
pub use input_message_content::InputMessageContentType;
pub use input_paid_media::InputPaidMediaType;
pub use input_profile_photo::InputProfilePhotoType;
pub use input_story_content::InputStoryContentType;
pub use maybe_inaccessible_message::MaybeInaccessibleMessageType;
pub use menu_button::MenuButtonType;
pub use message::MessageType;
pub use message_entity::MessageEntityType;
pub use message_origin::MessageOriginType;
pub use owned_gift::OwnedGiftType;
pub use paid_media::PaidMediaType;
pub use parse_mode::ParseMode;
pub use passport_element_error::PassportElementErrorType;
pub use poll::PollType;
pub use reaction_type::ReactionTypeType;
pub use reply_markup::ReplyMarkupType;
pub use revenue_withdrawal_state::RevenueWithdrawalStateType;
pub use star_transaction::StarTransactionType;
pub use sticker::StickerType;
pub use story_area_type::StoryAreaTypeType;
pub use telegram_observer_type::TelegramObserverType;
pub use transaction_partner::TransactionPartnerType;
pub use transaction_partner_user::TransactionPartnerUserType;
pub use update::UpdateType;
