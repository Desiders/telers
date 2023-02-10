pub mod animation;
pub mod audio;
pub mod bot_command;
pub mod bot_command_scope;
pub mod bot_command_scope_all_chat_administrators;
pub mod bot_command_scope_all_group_chats;
pub mod bot_command_scope_all_private_chats;
pub mod bot_command_scope_chat;
pub mod bot_command_scope_chat_administrators;
pub mod bot_command_scope_chat_member;
pub mod bot_command_scope_default;
pub mod callback_game;
pub mod callback_query;
pub mod chat;
pub mod chat_administrator_rights;
pub mod chat_id_kind;
pub mod chat_invite_link;
pub mod chat_join_request;
pub mod chat_location;
pub mod chat_member;
pub mod chat_member_administrator;
pub mod chat_member_banned;
pub mod chat_member_left;
pub mod chat_member_member;
pub mod chat_member_owner;
pub mod chat_member_restricted;
pub mod chat_member_updated;
pub mod chat_permissions;
pub mod chat_photo;
pub mod chosen_inline_result;
pub mod contact;
pub mod dice;
pub mod document;
pub mod encrypted_credentials;
pub mod encrypted_passport_element;
pub mod file;
pub mod force_reply;
pub mod forum_topic;
pub mod forum_topic_closed;
pub mod forum_topic_created;
pub mod forum_topic_reopened;
pub mod game;
pub mod game_high_score;
pub mod inline_keyboard_button;
pub mod inline_keyboard_markup;
pub mod inline_query;
pub mod inline_query_result;
pub mod inline_query_result_article;
pub mod inline_query_result_audio;
pub mod inline_query_result_cached_audio;
pub mod inline_query_result_cached_document;
pub mod inline_query_result_cached_gif;
pub mod inline_query_result_cached_mpeg4_gif;
pub mod inline_query_result_cached_photo;
pub mod inline_query_result_cached_sticker;
pub mod inline_query_result_cached_video;
pub mod inline_query_result_cached_voice;
pub mod inline_query_result_contact;
pub mod inline_query_result_document;
pub mod inline_query_result_game;
pub mod inline_query_result_gif;
pub mod inline_query_result_location;
pub mod inline_query_result_mpeg4_gif;
pub mod inline_query_result_photo;
pub mod inline_query_result_venue;
pub mod inline_query_result_video;
pub mod inline_query_result_voice;
pub mod input_contact_message_content;
pub mod input_file;
pub mod input_invoice_message_content;
pub mod input_location_message_content;
pub mod input_media;
pub mod input_media_animation;
pub mod input_media_audio;
pub mod input_media_document;
pub mod input_media_photo;
pub mod input_media_video;
pub mod input_message_content;
pub mod input_text_message_content;
pub mod input_venue_message_content;
pub mod invoice;
pub mod keyboard_button;
pub mod keyboard_button_poll_type;
pub mod labeled_price;
pub mod location;
pub mod login_url;
pub mod mask_position;
pub mod menu_button;
pub mod menu_button_commands;
pub mod menu_button_default;
pub mod menu_button_web_app;
pub mod message;
pub mod message_auto_delete_timer_changed;
pub mod message_entity;
pub mod message_id;
pub mod message_or_true;
pub mod order_info;
pub mod passport_data;
pub mod passport_element_error;
pub mod passport_element_error_data_field;
pub mod passport_element_error_file;
pub mod passport_element_error_files;
pub mod passport_element_error_front_side;
pub mod passport_element_error_reverse_side;
pub mod passport_element_error_selfie;
pub mod passport_element_error_translation_file;
pub mod passport_element_error_translation_files;
pub mod passport_element_error_unspecified;
pub mod passport_file;
pub mod photo_size;
pub mod poll;
pub mod poll_answer;
pub mod poll_option;
pub mod pre_checkout_query;
pub mod proximity_alert_triggered;
pub mod reply_keyboard_markup;
pub mod reply_keyboard_remove;
pub mod reply_markup;
pub mod response_parameters;
pub mod sent_web_app_message;
pub mod shipping_address;
pub mod shipping_option;
pub mod shipping_query;
pub mod sticker;
pub mod sticker_set;
pub mod successful_payment;
pub mod update;
pub mod user;
pub mod user_profile_photos;
pub mod venue;
pub mod video;
pub mod video_chat_ended;
pub mod video_chat_participants_invited;
pub mod video_chat_scheduled;
pub mod video_chat_started;
pub mod video_note;
pub mod voice;
pub mod web_app_data;
pub mod web_app_info;
pub mod webhook_info;

pub use animation::Animation;
pub use audio::Audio;
pub use bot_command::BotCommand;
pub use bot_command_scope::BotCommandScope;
pub use bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;
pub use bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;
pub use bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;
pub use bot_command_scope_chat::BotCommandScopeChat;
pub use bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;
pub use bot_command_scope_chat_member::BotCommandScopeChatMember;
pub use bot_command_scope_default::BotCommandScopeDefault;
pub use callback_game::CallbackGame;
pub use callback_query::CallbackQuery;
pub use chat::Chat;
pub use chat_administrator_rights::ChatAdministratorRights;
pub use chat_id_kind::ChatIdKind;
pub use chat_invite_link::ChatInviteLink;
pub use chat_join_request::ChatJoinRequest;
pub use chat_location::ChatLocation;
pub use chat_member::ChatMember;
pub use chat_member_administrator::ChatMemberAdministrator;
pub use chat_member_banned::ChatMemberBanned;
pub use chat_member_left::ChatMemberLeft;
pub use chat_member_member::ChatMemberMember;
pub use chat_member_owner::ChatMemberOwner;
pub use chat_member_restricted::ChatMemberRestricted;
pub use chat_member_updated::ChatMemberUpdated;
pub use chat_permissions::ChatPermissions;
pub use chat_photo::ChatPhoto;
pub use chosen_inline_result::ChosenInlineResult;
pub use contact::Contact;
pub use dice::{Dice, Emoji as DiceEmoji};
pub use document::Document;
pub use encrypted_credentials::EncryptedCredentials;
pub use encrypted_passport_element::EncryptedPassportElement;
pub use file::File;
pub use force_reply::ForceReply;
pub use forum_topic::ForumTopic;
pub use forum_topic_closed::ForumTopicClosed;
pub use forum_topic_created::ForumTopicCreated;
pub use forum_topic_reopened::ForumTopicReopened;
pub use game::Game;
pub use game_high_score::GameHighScore;
pub use inline_keyboard_button::InlineKeyboardButton;
pub use inline_keyboard_markup::InlineKeyboardMarkup;
pub use inline_query::InlineQuery;
pub use inline_query_result::InlineQueryResult;
pub use inline_query_result_article::InlineQueryResultArticle;
pub use inline_query_result_audio::InlineQueryResultAudio;
pub use inline_query_result_cached_audio::InlineQueryResultCachedAudio;
pub use inline_query_result_cached_document::InlineQueryResultCachedDocument;
pub use inline_query_result_cached_gif::InlineQueryResultCachedGif;
pub use inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
pub use inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
pub use inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
pub use inline_query_result_cached_video::InlineQueryResultCachedVideo;
pub use inline_query_result_cached_voice::InlineQueryResultCachedVoice;
pub use inline_query_result_contact::InlineQueryResultContact;
pub use inline_query_result_document::InlineQueryResultDocument;
pub use inline_query_result_game::InlineQueryResultGame;
pub use inline_query_result_gif::InlineQueryResultGif;
pub use inline_query_result_location::InlineQueryResultLocation;
pub use inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
pub use inline_query_result_photo::InlineQueryResultPhoto;
pub use inline_query_result_venue::InlineQueryResultVenue;
pub use inline_query_result_video::InlineQueryResultVideo;
pub use inline_query_result_voice::InlineQueryResultVoice;
pub use input_contact_message_content::InputContactMessageContent;
pub use input_file::{
    FSFile as InputFSFile, FileId as InputFileId, FileKind as InputFileKind, InputFile,
    UrlFile as InputUrlFile,
};
pub use input_invoice_message_content::InputInvoiceMessageContent;
pub use input_location_message_content::InputLocationMessageContent;
pub use input_media::InputMedia;
pub use input_media_animation::InputMediaAnimation;
pub use input_media_audio::InputMediaAudio;
pub use input_media_document::InputMediaDocument;
pub use input_media_photo::InputMediaPhoto;
pub use input_media_video::InputMediaVideo;
pub use input_message_content::InputMessageContent;
pub use input_text_message_content::InputTextMessageContent;
pub use input_venue_message_content::InputVenueMessageContent;
pub use invoice::Invoice;
pub use keyboard_button::KeyboardButton;
pub use keyboard_button_poll_type::KeyboardButtonPollType;
pub use labeled_price::LabeledPrice;
pub use location::Location;
pub use login_url::LoginUrl;
pub use mask_position::MaskPosition;
pub use menu_button::MenuButton;
pub use menu_button_commands::MenuButtonCommands;
pub use menu_button_default::MenuButtonDefault;
pub use menu_button_web_app::MenuButtonWebApp;
pub use message::Message;
pub use message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
pub use message_entity::MessageEntity;
pub use message_id::MessageId;
pub use message_or_true::MessageOrTrue;
pub use order_info::OrderInfo;
pub use passport_data::PassportData;
pub use passport_element_error::PassportElementError;
pub use passport_element_error_data_field::PassportElementErrorDataField;
pub use passport_element_error_file::PassportElementErrorFile;
pub use passport_element_error_files::PassportElementErrorFiles;
pub use passport_element_error_front_side::PassportElementErrorFrontSide;
pub use passport_element_error_reverse_side::PassportElementErrorReverseSide;
pub use passport_element_error_selfie::PassportElementErrorSelfie;
pub use passport_element_error_translation_file::PassportElementErrorTranslationFile;
pub use passport_element_error_translation_files::PassportElementErrorTranslationFiles;
pub use passport_element_error_unspecified::PassportElementErrorUnspecified;
pub use passport_file::PassportFile;
pub use photo_size::PhotoSize;
pub use poll::Poll;
pub use poll_answer::PollAnswer;
pub use poll_option::PollOption;
pub use pre_checkout_query::PreCheckoutQuery;
pub use proximity_alert_triggered::ProximityAlertTriggered;
pub use reply_keyboard_markup::ReplyKeyboardMarkup;
pub use reply_keyboard_remove::ReplyKeyboardRemove;
pub use reply_markup::ReplyMarkup;
pub use response_parameters::ResponseParameters;
pub use sent_web_app_message::SentWebAppMessage;
pub use shipping_address::ShippingAddress;
pub use shipping_option::ShippingOption;
pub use shipping_query::ShippingQuery;
pub use sticker::Sticker;
pub use sticker_set::StickerSet;
pub use successful_payment::SuccessfulPayment;
pub use update::Update;
pub use user::User;
pub use user_profile_photos::UserProfilePhotos;
pub use venue::Venue;
pub use video::Video;
pub use video_chat_ended::VideoChatEnded;
pub use video_chat_participants_invited::VideoChatParticipantsInvited;
pub use video_chat_scheduled::VideoChatScheduled;
pub use video_chat_started::VideoChatStarted;
pub use video_note::VideoNote;
pub use voice::Voice;
pub use web_app_data::WebAppData;
pub use web_app_info::WebAppInfo;
pub use webhook_info::WebhookInfo;
