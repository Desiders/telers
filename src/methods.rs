//! This module contains telegram methods from the [Telegram Bot API](https://core.telegram.org/bots/api).
//! Each method has a description and a link to the official documentation.
//!
//! Telegram methods are represented by structs that implement the [`TelegramMethod`] trait.
//! Each method has `new` function that creates a new instance of the method and accepts required parameters,
//! for optional parameters there are builder-like methods that return a new instance of the method with the specified parameter.
//!
//! Some parameters that wrapped in an [`Option`] from the telegram types,
//! can be passed using `{parameter}_option` method, which accepts an [`Option`] and returns a new instance of the method.
//! These methods are useful when you have some optional parameters that you want to pass to the method from the types without boilerplate code.
//! For example:
//! ```ignore
//! async fn handler(bot: Bot, message: Message) -> HandlerResult {
//!     // Equivalent to:
//!     // let method = if let Some(message_thread_id) = message.message_thread_id {
//!     //     SendMessage::new(message.chat.id, "Hello world!").message_thread_id(message_thread_id);
//!     // } else {
//!     //     SendMessage::new(message.chat.id, "Hello world!");
//!     // };
//!
//!     bot.send(
//!         &SendMessage::new(message.chat.id, "Hello world!")
//!             .message_thread_id_option(message.message_thread_id),
//!         None,
//!     )
//!     .await?;
//!
//!     Ok(EventReturn::Finish)
//! }
//! ```
//!
//! You can check example of usage methods in the `examples` directory.

pub mod add_sticker_to_set;
pub mod answer_callback_query;
pub mod answer_inline_query;
pub mod answer_pre_checkout_query;
pub mod answer_shipping_query;
pub mod answer_web_app_query;
pub mod approve_chat_join_request;
pub mod ban_chat_member;
pub mod ban_chat_sender_chat;
pub mod base;
pub mod close_forum_topic;
pub mod close_general_forum_topic;
pub mod copy_message;
pub mod create_chat_invite_link;
pub mod create_forum_topic;
pub mod create_invoice_link;
pub mod create_new_sticker_set;
pub mod decline_chat_join_request;
pub mod delete_chat_photo;
pub mod delete_chat_sticker_set;
pub mod delete_forum_topic;
pub mod delete_message;
pub mod delete_my_commands;
pub mod delete_sticker_from_set;
pub mod delete_sticker_set;
pub mod edit_chat_invite_link;
pub mod edit_forum_topic;
pub mod edit_general_forum_topic;
pub mod edit_message_caption;
pub mod edit_message_live_location;
pub mod edit_message_media;
pub mod edit_message_reply_markup;
pub mod edit_message_text;
pub mod export_chat_invite_link;
pub mod forward_message;
pub mod get_chat;
pub mod get_chat_administrators;
pub mod get_chat_member;
pub mod get_chat_member_count;
pub mod get_chat_menu_button;
pub mod get_custom_emoji_stickers;
pub mod get_file;
pub mod get_forum_topic_icon_stickers;
pub mod get_game_high_scores;
pub mod get_me;
pub mod get_my_commands;
pub mod get_my_default_administrator_rights;
pub mod get_my_description;
pub mod get_my_name;
pub mod get_my_short_description;
pub mod get_sticker_set;
pub mod get_updates;
pub mod get_user_profile_photos;
pub mod hide_general_forum_topic;
pub mod leave_chat;
pub mod log_out;
pub mod pin_chat_message;
pub mod promote_chat_member;
pub mod reopen_forum_topic;
pub mod reopen_general_forum_topic;
pub mod restrict_chat_member;
pub mod revoke_chat_invite_link;
pub mod send_animation;
pub mod send_audio;
pub mod send_chat_action;
pub mod send_contact;
pub mod send_dice;
pub mod send_document;
pub mod send_game;
pub mod send_invoice;
pub mod send_location;
pub mod send_media_group;
pub mod send_message;
pub mod send_photo;
pub mod send_poll;
pub mod send_sticker;
pub mod send_venue;
pub mod send_video;
pub mod send_video_note;
pub mod send_voice;
pub mod set_chat_administrator_custom_title;
pub mod set_chat_description;
pub mod set_chat_menu_button;
pub mod set_chat_permissions;
pub mod set_chat_sticker_set;
pub mod set_chat_title;
pub mod set_custom_emoji_sticker_set_thumbnail;
pub mod set_game_score;
pub mod set_my_commands;
pub mod set_my_default_administrator_rights;
pub mod set_my_description;
pub mod set_my_name;
pub mod set_my_short_description;
pub mod set_passport_data_errors;
pub mod set_sticker_emoji_list;
pub mod set_sticker_keywords;
pub mod set_sticker_mask_position;
pub mod set_sticker_position_in_set;
pub mod set_sticker_set_thumbnail;
pub mod set_sticker_set_title;
pub mod stop_message_live_location;
pub mod stop_poll;
pub mod unban_chat_member;
pub mod unban_chat_sender_chat;
pub mod unhide_general_forum_topic;
pub mod unpin_all_chat_messages;
pub mod unpin_all_forum_topic_messages;
pub mod unpin_all_general_forum_topic_messages;
pub mod unpin_chat_message;
pub mod upload_sticker_file;

pub use add_sticker_to_set::AddStickerToSet;
pub use answer_callback_query::AnswerCallbackQuery;
pub use answer_inline_query::AnswerInlineQuery;
pub use answer_pre_checkout_query::AnswerPreCheckoutQuery;
pub use answer_shipping_query::AnswerShippingQuery;
pub use answer_web_app_query::AnswerWebAppQuery;
pub use approve_chat_join_request::ApproveChatJoinRequest;
pub use ban_chat_member::BanChatMember;
pub use ban_chat_sender_chat::BanChatSenderChat;
pub use base::{Request, Response, TelegramMethod};
pub use close_forum_topic::CloseForumTopic;
pub use close_general_forum_topic::CloseGeneralForumTopic;
pub use copy_message::CopyMessage;
pub use create_chat_invite_link::CreateChatInviteLink;
pub use create_forum_topic::CreateForumTopic;
pub use create_invoice_link::CreateInvoiceLink;
pub use create_new_sticker_set::CreateNewStickerSet;
pub use decline_chat_join_request::DeclineChatJoinRequest;
pub use delete_chat_photo::DeleteChatPhoto;
pub use delete_chat_sticker_set::DeleteChatStickerSet;
pub use delete_forum_topic::DeleteForumTopic;
pub use delete_message::DeleteMessage;
pub use delete_my_commands::DeleteMyCommands;
pub use delete_sticker_from_set::DeleteStickerFromSet;
pub use delete_sticker_set::DeleteStickerSet;
pub use edit_chat_invite_link::EditChatInviteLink;
pub use edit_forum_topic::EditForumTopic;
pub use edit_general_forum_topic::EditGeneralForumTopic;
pub use edit_message_caption::EditMessageCaption;
pub use edit_message_live_location::EditMessageLiveLocation;
pub use edit_message_media::EditMessageMedia;
pub use edit_message_reply_markup::EditMessageReplyMarkup;
pub use edit_message_text::EditMessageText;
pub use export_chat_invite_link::ExportChatInviteLink;
pub use forward_message::ForwardMessage;
pub use get_chat::GetChat;
pub use get_chat_administrators::GetChatAdministrators;
pub use get_chat_member::GetChatMember;
pub use get_chat_member_count::GetChatMemberCount;
pub use get_chat_menu_button::GetChatMenuButton;
pub use get_custom_emoji_stickers::GetCustomEmojiStickers;
pub use get_file::GetFile;
pub use get_forum_topic_icon_stickers::GetForumTopicIconStickers;
pub use get_game_high_scores::GetGameHighScores;
pub use get_me::GetMe;
pub use get_my_commands::GetMyCommands;
pub use get_my_default_administrator_rights::GetMyDefaultAdministratorRights;
pub use get_my_description::GetMyDescription;
pub use get_my_name::GetMyName;
pub use get_my_short_description::GetMyShortDescription;
pub use get_sticker_set::GetStickerSet;
pub use get_updates::GetUpdates;
pub use get_user_profile_photos::GetUserProfilePhotos;
pub use hide_general_forum_topic::HideGeneralForumTopic;
pub use leave_chat::LeaveChat;
pub use log_out::LogOut;
pub use pin_chat_message::PinChatMessage;
pub use promote_chat_member::PromoteChatMember;
pub use reopen_forum_topic::ReopenForumTopic;
pub use reopen_general_forum_topic::ReopenGeneralForumTopic;
pub use restrict_chat_member::RestrictChatMember;
pub use revoke_chat_invite_link::RevokeChatInviteLink;
pub use send_animation::SendAnimation;
pub use send_audio::SendAudio;
pub use send_chat_action::SendChatAction;
pub use send_contact::SendContact;
pub use send_dice::SendDice;
pub use send_document::SendDocument;
pub use send_game::SendGame;
pub use send_invoice::SendInvoice;
pub use send_location::SendLocation;
pub use send_media_group::SendMediaGroup;
pub use send_message::SendMessage;
pub use send_photo::SendPhoto;
pub use send_poll::SendPoll;
pub use send_sticker::SendSticker;
pub use send_venue::SendVenue;
pub use send_video::SendVideo;
pub use send_video_note::SendVideoNote;
pub use send_voice::SendVoice;
pub use set_chat_administrator_custom_title::SetChatAdministratorCustomTitle;
pub use set_chat_description::SetChatDescription;
pub use set_chat_menu_button::SetChatMenuButton;
pub use set_chat_permissions::SetChatPermissions;
pub use set_chat_sticker_set::SetChatStickerSet;
pub use set_chat_title::SetChatTitle;
pub use set_custom_emoji_sticker_set_thumbnail::SetCustomEmojiStickerSetThumbnail;
pub use set_game_score::SetGameScore;
pub use set_my_commands::SetMyCommands;
pub use set_my_default_administrator_rights::SetMyDefaultAdministratorRights;
pub use set_my_description::SetMyDescription;
pub use set_my_name::SetMyName;
pub use set_my_short_description::SetMyShortDescription;
pub use set_passport_data_errors::SetPassportDataErrors;
pub use set_sticker_emoji_list::SetStickerEmojiList;
pub use set_sticker_keywords::SetStickerKeywords;
pub use set_sticker_mask_position::SetStickerMaskPosition;
pub use set_sticker_position_in_set::SetStickerPositionInSet;
pub use set_sticker_set_thumbnail::SetStickerSetThumbnail;
pub use set_sticker_set_title::SetStickerSetTitle;
pub use stop_message_live_location::StopMessageLiveLocation;
pub use stop_poll::StopPoll;
pub use unban_chat_member::UnbanChatMember;
pub use unban_chat_sender_chat::UnbanChatSenderChat;
pub use unhide_general_forum_topic::UnhideGeneralForumTopic;
pub use unpin_all_chat_messages::UnpinAllChatMessages;
pub use unpin_all_forum_topic_messages::UnpinAllForumTopicMessages;
pub use unpin_all_general_forum_topic_messages::UnpinAllGeneralForumTopicMessages;
pub use unpin_chat_message::UnpinChatMessage;
pub use upload_sticker_file::UploadStickerFile;
