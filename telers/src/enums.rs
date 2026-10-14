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
use crate::types::{
    BackgroundFill, BackgroundType, BotCommandScope, Chat, ChatBoostSource, ChatFullInfo,
    ChatMember, EncryptedPassportElement, ExternalReplyInfo, Giveaway, GiveawayWinners,
    InlineQueryResult, InlineQueryResultAudioKind, InlineQueryResultDocumentKind,
    InlineQueryResultGifKind, InlineQueryResultMpeg4GifKind, InlineQueryResultPhotoKind,
    InlineQueryResultVideoKind, InlineQueryResultVoiceKind, InputMedia, InputMessageContent,
    InputPaidMedia, InputPollMedia, InputPollOptionMedia, InputProfilePhoto, InputRichBlock,
    InputRichMessageMediaContent, InputStoryContent, MaybeInaccessibleMessage, MenuButton, Message,
    MessageEntity, MessageOrigin, OwnedGift, PaidMedia, PassportElementError, Poll, PollMedia,
    ReactionType, ReplyMarkup, RevenueWithdrawalState, RichBlock, RichText, StarTransaction,
    Sticker, StoryAreaType, TransactionPartner, TransactionPartnerUser, Update,
};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents an incoming update.
/// At most one of the optional fields can be present in any given update.
/// Currently, it can be one of
/// - [`crate::types::UpdateBusinessConnection`]
/// - [`crate::types::UpdateBusinessMessage`]
/// - [`crate::types::UpdateCallbackQuery`]
/// - [`crate::types::UpdateChannelPost`]
/// - [`crate::types::UpdateChatBoost`]
/// - [`crate::types::UpdateChatJoinRequest`]
/// - [`crate::types::UpdateChatMember`]
/// - [`crate::types::UpdateChosenInlineResult`]
/// - [`crate::types::UpdateDeletedBusinessMessages`]
/// - [`crate::types::UpdateEditedBusinessMessage`]
/// - [`crate::types::UpdateEditedChannelPost`]
/// - [`crate::types::UpdateEditedMessage`]
/// - [`crate::types::UpdateGuestMessage`]
/// - [`crate::types::UpdateInlineQuery`]
/// - [`crate::types::UpdateManagedBot`]
/// - [`crate::types::UpdateMessage`]
/// - [`crate::types::UpdateMessageReaction`]
/// - [`crate::types::UpdateMessageReactionCount`]
/// - [`crate::types::UpdateMyChatMember`]
/// - [`crate::types::UpdatePoll`]
/// - [`crate::types::UpdatePollAnswer`]
/// - [`crate::types::UpdatePreCheckoutQuery`]
/// - [`crate::types::UpdatePurchasedPaidMedia`]
/// - [`crate::types::UpdateRemovedChatBoost`]
/// - [`crate::types::UpdateShippingQuery`]
/// - [`crate::types::UpdateStoppedMessageGeneration`]
/// - [`crate::types::UpdateSubscription`]
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum UpdateType {
    #[strum(serialize = "business_connection")]
    BusinessConnection,
    #[strum(serialize = "business_message")]
    BusinessMessage,
    #[strum(serialize = "callback_query")]
    CallbackQuery,
    #[strum(serialize = "channel_post")]
    ChannelPost,
    #[strum(serialize = "chat_boost")]
    ChatBoost,
    #[strum(serialize = "chat_join_request")]
    ChatJoinRequest,
    #[strum(serialize = "chat_member")]
    ChatMember,
    #[strum(serialize = "chosen_inline_result")]
    ChosenInlineResult,
    #[strum(serialize = "deleted_business_messages")]
    DeletedBusinessMessages,
    #[strum(serialize = "edited_business_message")]
    EditedBusinessMessage,
    #[strum(serialize = "edited_channel_post")]
    EditedChannelPost,
    #[strum(serialize = "edited_message")]
    EditedMessage,
    #[strum(serialize = "guest_message")]
    GuestMessage,
    #[strum(serialize = "inline_query")]
    InlineQuery,
    #[strum(serialize = "managed_bot")]
    ManagedBot,
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "message_reaction")]
    MessageReaction,
    #[strum(serialize = "message_reaction_count")]
    MessageReactionCount,
    #[strum(serialize = "my_chat_member")]
    MyChatMember,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "poll_answer")]
    PollAnswer,
    #[strum(serialize = "pre_checkout_query")]
    PreCheckoutQuery,
    #[strum(serialize = "purchased_paid_media")]
    PurchasedPaidMedia,
    #[strum(serialize = "removed_chat_boost")]
    RemovedChatBoost,
    #[strum(serialize = "shipping_query")]
    ShippingQuery,
    #[strum(serialize = "stopped_message_generation")]
    StoppedMessageGeneration,
    #[strum(serialize = "subscription")]
    Subscription,
}
impl UpdateType {
    #[must_use]
    pub const fn all() -> [UpdateType; 27usize] {
        [
            UpdateType::BusinessConnection,
            UpdateType::BusinessMessage,
            UpdateType::CallbackQuery,
            UpdateType::ChannelPost,
            UpdateType::ChatBoost,
            UpdateType::ChatJoinRequest,
            UpdateType::ChatMember,
            UpdateType::ChosenInlineResult,
            UpdateType::DeletedBusinessMessages,
            UpdateType::EditedBusinessMessage,
            UpdateType::EditedChannelPost,
            UpdateType::EditedMessage,
            UpdateType::GuestMessage,
            UpdateType::InlineQuery,
            UpdateType::ManagedBot,
            UpdateType::Message,
            UpdateType::MessageReaction,
            UpdateType::MessageReactionCount,
            UpdateType::MyChatMember,
            UpdateType::Poll,
            UpdateType::PollAnswer,
            UpdateType::PreCheckoutQuery,
            UpdateType::PurchasedPaidMedia,
            UpdateType::RemovedChatBoost,
            UpdateType::ShippingQuery,
            UpdateType::StoppedMessageGeneration,
            UpdateType::Subscription,
        ]
    }
}
impl From<UpdateType> for Box<str> {
    fn from(val: UpdateType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<UpdateType> for String {
    fn from(val: UpdateType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for UpdateType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Update> for UpdateType {
    fn from(val: &'a Update) -> Self {
        match val {
            Update::BusinessConnection(_) => UpdateType::BusinessConnection,
            Update::BusinessMessage(_) => UpdateType::BusinessMessage,
            Update::CallbackQuery(_) => UpdateType::CallbackQuery,
            Update::ChannelPost(_) => UpdateType::ChannelPost,
            Update::ChatBoost(_) => UpdateType::ChatBoost,
            Update::ChatJoinRequest(_) => UpdateType::ChatJoinRequest,
            Update::ChatMember(_) => UpdateType::ChatMember,
            Update::ChosenInlineResult(_) => UpdateType::ChosenInlineResult,
            Update::DeletedBusinessMessages(_) => UpdateType::DeletedBusinessMessages,
            Update::EditedBusinessMessage(_) => UpdateType::EditedBusinessMessage,
            Update::EditedChannelPost(_) => UpdateType::EditedChannelPost,
            Update::EditedMessage(_) => UpdateType::EditedMessage,
            Update::GuestMessage(_) => UpdateType::GuestMessage,
            Update::InlineQuery(_) => UpdateType::InlineQuery,
            Update::ManagedBot(_) => UpdateType::ManagedBot,
            Update::Message(_) => UpdateType::Message,
            Update::MessageReaction(_) => UpdateType::MessageReaction,
            Update::MessageReactionCount(_) => UpdateType::MessageReactionCount,
            Update::MyChatMember(_) => UpdateType::MyChatMember,
            Update::Poll(_) => UpdateType::Poll,
            Update::PollAnswer(_) => UpdateType::PollAnswer,
            Update::PreCheckoutQuery(_) => UpdateType::PreCheckoutQuery,
            Update::PurchasedPaidMedia(_) => UpdateType::PurchasedPaidMedia,
            Update::RemovedChatBoost(_) => UpdateType::RemovedChatBoost,
            Update::ShippingQuery(_) => UpdateType::ShippingQuery,
            Update::StoppedMessageGeneration(_) => UpdateType::StoppedMessageGeneration,
            Update::Subscription(_) => UpdateType::Subscription,
        }
    }
}
impl From<Update> for UpdateType {
    fn from(val: Update) -> Self {
        UpdateType::from(&val)
    }
}
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedVoice`] and [`crate::types::InlineQueryResultVoice`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultVoiceKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultVoiceKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultVoiceKindType; 2usize] {
        [
            InlineQueryResultVoiceKindType::Uncached,
            InlineQueryResultVoiceKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultVoiceKindType> for Box<str> {
    fn from(val: InlineQueryResultVoiceKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultVoiceKindType> for String {
    fn from(val: InlineQueryResultVoiceKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultVoiceKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultVoiceKind> for InlineQueryResultVoiceKindType {
    fn from(val: &'a InlineQueryResultVoiceKind) -> Self {
        match val {
            InlineQueryResultVoiceKind::Uncached(_) => InlineQueryResultVoiceKindType::Uncached,
            InlineQueryResultVoiceKind::Cached(_) => InlineQueryResultVoiceKindType::Cached,
        }
    }
}
impl From<InlineQueryResultVoiceKind> for InlineQueryResultVoiceKindType {
    fn from(val: InlineQueryResultVoiceKind) -> Self {
        InlineQueryResultVoiceKindType::from(&val)
    }
}
/// This object describes a profile photo to set. Currently, it can be one of
/// - [`crate::types::InputProfilePhotoStatic`]
/// - [`crate::types::InputProfilePhotoAnimated`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephoto>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputProfilePhotoType {
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "animated")]
    Animated,
}
impl InputProfilePhotoType {
    #[must_use]
    pub const fn all() -> [InputProfilePhotoType; 2usize] {
        [
            InputProfilePhotoType::Static,
            InputProfilePhotoType::Animated,
        ]
    }
}
impl From<InputProfilePhotoType> for Box<str> {
    fn from(val: InputProfilePhotoType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputProfilePhotoType> for String {
    fn from(val: InputProfilePhotoType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputProfilePhotoType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputProfilePhoto> for InputProfilePhotoType {
    fn from(val: &'a InputProfilePhoto) -> Self {
        match val {
            InputProfilePhoto::Static(_) => InputProfilePhotoType::Static,
            InputProfilePhoto::Animated(_) => InputProfilePhotoType::Animated,
        }
    }
}
impl From<InputProfilePhoto> for InputProfilePhotoType {
    fn from(val: InputProfilePhoto) -> Self {
        InputProfilePhotoType::from(&val)
    }
}
/// This object contains information about a poll.
/// Currently, it can be one of
/// - [`crate::types::PollQuiz`]
/// - [`crate::types::PollRegular`]
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum PollType {
    #[strum(serialize = "regular")]
    Regular,
    #[strum(serialize = "quiz")]
    Quiz,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl PollType {
    #[must_use]
    pub const fn all() -> [PollType; 3usize] {
        [PollType::Regular, PollType::Quiz, PollType::Unknown]
    }
}
impl From<PollType> for Box<str> {
    fn from(val: PollType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PollType> for String {
    fn from(val: PollType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PollType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Poll> for PollType {
    fn from(val: &'a Poll) -> Self {
        match val {
            Poll::Regular(_) => PollType::Regular,
            Poll::Quiz(_) => PollType::Quiz,
            Poll::Unknown(_) => PollType::Unknown,
        }
    }
}
impl From<Poll> for PollType {
    fn from(val: Poll) -> Self {
        PollType::from(&val)
    }
}
/// Describes the type of a clickable area on a story. Currently, it can be one of
/// - [`crate::types::StoryAreaTypeLocation`]
/// - [`crate::types::StoryAreaTypeSuggestedReaction`]
/// - [`crate::types::StoryAreaTypeLink`]
/// - [`crate::types::StoryAreaTypeWeather`]
/// - [`crate::types::StoryAreaTypeUniqueGift`]
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatype>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum StoryAreaTypeType {
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "suggested_reaction")]
    SuggestedReaction,
    #[strum(serialize = "link")]
    Link,
    #[strum(serialize = "weather")]
    Weather,
    #[strum(serialize = "unique_gift")]
    UniqueGift,
}
impl StoryAreaTypeType {
    #[must_use]
    pub const fn all() -> [StoryAreaTypeType; 5usize] {
        [
            StoryAreaTypeType::Location,
            StoryAreaTypeType::SuggestedReaction,
            StoryAreaTypeType::Link,
            StoryAreaTypeType::Weather,
            StoryAreaTypeType::UniqueGift,
        ]
    }
}
impl From<StoryAreaTypeType> for Box<str> {
    fn from(val: StoryAreaTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<StoryAreaTypeType> for String {
    fn from(val: StoryAreaTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for StoryAreaTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a StoryAreaType> for StoryAreaTypeType {
    fn from(val: &'a StoryAreaType) -> Self {
        match val {
            StoryAreaType::Location(_) => StoryAreaTypeType::Location,
            StoryAreaType::SuggestedReaction(_) => StoryAreaTypeType::SuggestedReaction,
            StoryAreaType::Link(_) => StoryAreaTypeType::Link,
            StoryAreaType::Weather(_) => StoryAreaTypeType::Weather,
            StoryAreaType::UniqueGift(_) => StoryAreaTypeType::UniqueGift,
        }
    }
}
impl From<StoryAreaType> for StoryAreaTypeType {
    fn from(val: StoryAreaType) -> Self {
        StoryAreaTypeType::from(&val)
    }
}
/// This object represents available reply markup variants.
/// Currently, it can be one of
/// - [`crate::types::ForceReply`]
/// - [`crate::types::InlineKeyboardMarkup`]
/// - [`crate::types::ReplyKeyboardMarkup`]
/// - [`crate::types::ReplyKeyboardRemove`]
/// # Documentation
/// <https://core.telegram.org/bots/api>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ReplyMarkupType {
    #[strum(serialize = "reply_keyboard_markup")]
    ReplyKeyboardMarkup,
    #[strum(serialize = "force_reply")]
    ForceReply,
    #[strum(serialize = "inline_keyboard_markup")]
    InlineKeyboardMarkup,
    #[strum(serialize = "reply_keyboard_remove")]
    ReplyKeyboardRemove,
}
impl ReplyMarkupType {
    #[must_use]
    pub const fn all() -> [ReplyMarkupType; 4usize] {
        [
            ReplyMarkupType::ReplyKeyboardMarkup,
            ReplyMarkupType::ForceReply,
            ReplyMarkupType::InlineKeyboardMarkup,
            ReplyMarkupType::ReplyKeyboardRemove,
        ]
    }
}
impl From<ReplyMarkupType> for Box<str> {
    fn from(val: ReplyMarkupType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ReplyMarkupType> for String {
    fn from(val: ReplyMarkupType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ReplyMarkupType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ReplyMarkup> for ReplyMarkupType {
    fn from(val: &'a ReplyMarkup) -> Self {
        match val {
            ReplyMarkup::ReplyKeyboardMarkup(_) => ReplyMarkupType::ReplyKeyboardMarkup,
            ReplyMarkup::ForceReply(_) => ReplyMarkupType::ForceReply,
            ReplyMarkup::InlineKeyboardMarkup(_) => ReplyMarkupType::InlineKeyboardMarkup,
            ReplyMarkup::ReplyKeyboardRemove(_) => ReplyMarkupType::ReplyKeyboardRemove,
        }
    }
}
impl From<ReplyMarkup> for ReplyMarkupType {
    fn from(val: ReplyMarkup) -> Self {
        ReplyMarkupType::from(&val)
    }
}
/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
/// Currently, it can be one of
/// - [`crate::types::MessageEntityBlockquote`]
/// - [`crate::types::MessageEntityBold`]
/// - [`crate::types::MessageEntityBotCommand`]
/// - [`crate::types::MessageEntityCashtag`]
/// - [`crate::types::MessageEntityCode`]
/// - [`crate::types::MessageEntityCustomEmoji`]
/// - [`crate::types::MessageEntityDateTime`]
/// - [`crate::types::MessageEntityEmail`]
/// - [`crate::types::MessageEntityExpandableBlockquote`]
/// - [`crate::types::MessageEntityHashtag`]
/// - [`crate::types::MessageEntityItalic`]
/// - [`crate::types::MessageEntityMention`]
/// - [`crate::types::MessageEntityPhoneNumber`]
/// - [`crate::types::MessageEntityPre`]
/// - [`crate::types::MessageEntitySpoiler`]
/// - [`crate::types::MessageEntityStrikethrough`]
/// - [`crate::types::MessageEntityTextLink`]
/// - [`crate::types::MessageEntityTextMention`]
/// - [`crate::types::MessageEntityUnderline`]
/// - [`crate::types::MessageEntityUrl`]
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum MessageEntityType {
    #[strum(serialize = "mention")]
    Mention,
    #[strum(serialize = "hashtag")]
    Hashtag,
    #[strum(serialize = "cashtag")]
    Cashtag,
    #[strum(serialize = "bot_command")]
    BotCommand,
    #[strum(serialize = "url")]
    Url,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "bold")]
    Bold,
    #[strum(serialize = "italic")]
    Italic,
    #[strum(serialize = "underline")]
    Underline,
    #[strum(serialize = "strikethrough")]
    Strikethrough,
    #[strum(serialize = "spoiler")]
    Spoiler,
    #[strum(serialize = "blockquote")]
    Blockquote,
    #[strum(serialize = "expandable_blockquote")]
    ExpandableBlockquote,
    #[strum(serialize = "code")]
    Code,
    #[strum(serialize = "pre")]
    Pre,
    #[strum(serialize = "text_link")]
    TextLink,
    #[strum(serialize = "text_mention")]
    TextMention,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "date_time")]
    DateTime,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl MessageEntityType {
    #[must_use]
    pub const fn all() -> [MessageEntityType; 21usize] {
        [
            MessageEntityType::Mention,
            MessageEntityType::Hashtag,
            MessageEntityType::Cashtag,
            MessageEntityType::BotCommand,
            MessageEntityType::Url,
            MessageEntityType::Email,
            MessageEntityType::PhoneNumber,
            MessageEntityType::Bold,
            MessageEntityType::Italic,
            MessageEntityType::Underline,
            MessageEntityType::Strikethrough,
            MessageEntityType::Spoiler,
            MessageEntityType::Blockquote,
            MessageEntityType::ExpandableBlockquote,
            MessageEntityType::Code,
            MessageEntityType::Pre,
            MessageEntityType::TextLink,
            MessageEntityType::TextMention,
            MessageEntityType::CustomEmoji,
            MessageEntityType::DateTime,
            MessageEntityType::Unknown,
        ]
    }
}
impl From<MessageEntityType> for Box<str> {
    fn from(val: MessageEntityType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MessageEntityType> for String {
    fn from(val: MessageEntityType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MessageEntityType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MessageEntity> for MessageEntityType {
    fn from(val: &'a MessageEntity) -> Self {
        match val {
            MessageEntity::Mention(_) => MessageEntityType::Mention,
            MessageEntity::Hashtag(_) => MessageEntityType::Hashtag,
            MessageEntity::Cashtag(_) => MessageEntityType::Cashtag,
            MessageEntity::BotCommand(_) => MessageEntityType::BotCommand,
            MessageEntity::Url(_) => MessageEntityType::Url,
            MessageEntity::Email(_) => MessageEntityType::Email,
            MessageEntity::PhoneNumber(_) => MessageEntityType::PhoneNumber,
            MessageEntity::Bold(_) => MessageEntityType::Bold,
            MessageEntity::Italic(_) => MessageEntityType::Italic,
            MessageEntity::Underline(_) => MessageEntityType::Underline,
            MessageEntity::Strikethrough(_) => MessageEntityType::Strikethrough,
            MessageEntity::Spoiler(_) => MessageEntityType::Spoiler,
            MessageEntity::Blockquote(_) => MessageEntityType::Blockquote,
            MessageEntity::ExpandableBlockquote(_) => MessageEntityType::ExpandableBlockquote,
            MessageEntity::Code(_) => MessageEntityType::Code,
            MessageEntity::Pre(_) => MessageEntityType::Pre,
            MessageEntity::TextLink(_) => MessageEntityType::TextLink,
            MessageEntity::TextMention(_) => MessageEntityType::TextMention,
            MessageEntity::CustomEmoji(_) => MessageEntityType::CustomEmoji,
            MessageEntity::DateTime(_) => MessageEntityType::DateTime,
            MessageEntity::Unknown(_) => MessageEntityType::Unknown,
        }
    }
}
impl From<MessageEntity> for MessageEntityType {
    fn from(val: MessageEntity) -> Self {
        MessageEntityType::from(&val)
    }
}
/// This object represents a block in a rich formatted message. Currently, it can be any of the following types:
/// - [`crate::types::RichBlockParagraph`]
/// - [`crate::types::RichBlockSectionHeading`]
/// - [`crate::types::RichBlockPreformatted`]
/// - [`crate::types::RichBlockFooter`]
/// - [`crate::types::RichBlockDivider`]
/// - [`crate::types::RichBlockMathematicalExpression`]
/// - [`crate::types::RichBlockAnchor`]
/// - [`crate::types::RichBlockList`]
/// - [`crate::types::RichBlockBlockQuotation`]
/// - [`crate::types::RichBlockExpandableBlockQuotation`]
/// - [`crate::types::RichBlockPullQuotation`]
/// - [`crate::types::RichBlockCollage`]
/// - [`crate::types::RichBlockSlideshow`]
/// - [`crate::types::RichBlockTable`]
/// - [`crate::types::RichBlockDetails`]
/// - [`crate::types::RichBlockMap`]
/// - [`crate::types::RichBlockButtons`]
/// - [`crate::types::RichBlockAnimation`]
/// - [`crate::types::RichBlockAudio`]
/// - [`crate::types::RichBlockDocument`]
/// - [`crate::types::RichBlockPhoto`]
/// - [`crate::types::RichBlockVideo`]
/// - [`crate::types::RichBlockVoiceNote`]
/// - [`crate::types::RichBlockThinking`]
/// # Documentation
/// <https://core.telegram.org/bots/api#richblock>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum RichBlockType {
    #[strum(serialize = "paragraph")]
    Paragraph,
    #[strum(serialize = "heading")]
    Heading,
    #[strum(serialize = "pre")]
    Pre,
    #[strum(serialize = "footer")]
    Footer,
    #[strum(serialize = "divider")]
    Divider,
    #[strum(serialize = "mathematical_expression")]
    MathematicalExpression,
    #[strum(serialize = "anchor")]
    Anchor,
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "blockquote")]
    Blockquote,
    #[strum(serialize = "expandable_blockquote")]
    ExpandableBlockquote,
    #[strum(serialize = "pullquote")]
    Pullquote,
    #[strum(serialize = "collage")]
    Collage,
    #[strum(serialize = "slideshow")]
    Slideshow,
    #[strum(serialize = "table")]
    Table,
    #[strum(serialize = "details")]
    Details,
    #[strum(serialize = "map")]
    Map,
    #[strum(serialize = "buttons")]
    Buttons,
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice_note")]
    VoiceNote,
    #[strum(serialize = "thinking")]
    Thinking,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl RichBlockType {
    #[must_use]
    pub const fn all() -> [RichBlockType; 25usize] {
        [
            RichBlockType::Paragraph,
            RichBlockType::Heading,
            RichBlockType::Pre,
            RichBlockType::Footer,
            RichBlockType::Divider,
            RichBlockType::MathematicalExpression,
            RichBlockType::Anchor,
            RichBlockType::List,
            RichBlockType::Blockquote,
            RichBlockType::ExpandableBlockquote,
            RichBlockType::Pullquote,
            RichBlockType::Collage,
            RichBlockType::Slideshow,
            RichBlockType::Table,
            RichBlockType::Details,
            RichBlockType::Map,
            RichBlockType::Buttons,
            RichBlockType::Animation,
            RichBlockType::Audio,
            RichBlockType::Document,
            RichBlockType::Photo,
            RichBlockType::Video,
            RichBlockType::VoiceNote,
            RichBlockType::Thinking,
            RichBlockType::Unknown,
        ]
    }
}
impl From<RichBlockType> for Box<str> {
    fn from(val: RichBlockType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<RichBlockType> for String {
    fn from(val: RichBlockType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for RichBlockType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a RichBlock> for RichBlockType {
    fn from(val: &'a RichBlock) -> Self {
        match val {
            RichBlock::Paragraph(_) => RichBlockType::Paragraph,
            RichBlock::Heading(_) => RichBlockType::Heading,
            RichBlock::Pre(_) => RichBlockType::Pre,
            RichBlock::Footer(_) => RichBlockType::Footer,
            RichBlock::Divider(_) => RichBlockType::Divider,
            RichBlock::MathematicalExpression(_) => RichBlockType::MathematicalExpression,
            RichBlock::Anchor(_) => RichBlockType::Anchor,
            RichBlock::List(_) => RichBlockType::List,
            RichBlock::Blockquote(_) => RichBlockType::Blockquote,
            RichBlock::ExpandableBlockquote(_) => RichBlockType::ExpandableBlockquote,
            RichBlock::Pullquote(_) => RichBlockType::Pullquote,
            RichBlock::Collage(_) => RichBlockType::Collage,
            RichBlock::Slideshow(_) => RichBlockType::Slideshow,
            RichBlock::Table(_) => RichBlockType::Table,
            RichBlock::Details(_) => RichBlockType::Details,
            RichBlock::Map(_) => RichBlockType::Map,
            RichBlock::Buttons(_) => RichBlockType::Buttons,
            RichBlock::Animation(_) => RichBlockType::Animation,
            RichBlock::Audio(_) => RichBlockType::Audio,
            RichBlock::Document(_) => RichBlockType::Document,
            RichBlock::Photo(_) => RichBlockType::Photo,
            RichBlock::Video(_) => RichBlockType::Video,
            RichBlock::VoiceNote(_) => RichBlockType::VoiceNote,
            RichBlock::Thinking(_) => RichBlockType::Thinking,
            RichBlock::Unknown(_) => RichBlockType::Unknown,
        }
    }
}
impl From<RichBlock> for RichBlockType {
    fn from(val: RichBlock) -> Self {
        RichBlockType::from(&val)
    }
}
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedAudio`] and [`crate::types::InlineQueryResultAudio`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultAudioKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultAudioKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultAudioKindType; 2usize] {
        [
            InlineQueryResultAudioKindType::Uncached,
            InlineQueryResultAudioKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultAudioKindType> for Box<str> {
    fn from(val: InlineQueryResultAudioKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultAudioKindType> for String {
    fn from(val: InlineQueryResultAudioKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultAudioKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultAudioKind> for InlineQueryResultAudioKindType {
    fn from(val: &'a InlineQueryResultAudioKind) -> Self {
        match val {
            InlineQueryResultAudioKind::Uncached(_) => InlineQueryResultAudioKindType::Uncached,
            InlineQueryResultAudioKind::Cached(_) => InlineQueryResultAudioKindType::Cached,
        }
    }
}
impl From<InlineQueryResultAudioKind> for InlineQueryResultAudioKindType {
    fn from(val: InlineQueryResultAudioKind) -> Self {
        InlineQueryResultAudioKindType::from(&val)
    }
}
/// This object describes a message that can be inaccessible to the bot. It can be one of
/// - Message
/// - [`crate::types::InaccessibleMessage`]
/// # Documentation
/// <https://core.telegram.org/bots/api#maybeinaccessiblemessage>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum MaybeInaccessibleMessageType {
    #[strum(serialize = "inaccessible_message")]
    InaccessibleMessage,
    #[strum(serialize = "message")]
    Message,
}
impl MaybeInaccessibleMessageType {
    #[must_use]
    pub const fn all() -> [MaybeInaccessibleMessageType; 2usize] {
        [
            MaybeInaccessibleMessageType::InaccessibleMessage,
            MaybeInaccessibleMessageType::Message,
        ]
    }
}
impl From<MaybeInaccessibleMessageType> for Box<str> {
    fn from(val: MaybeInaccessibleMessageType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MaybeInaccessibleMessageType> for String {
    fn from(val: MaybeInaccessibleMessageType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MaybeInaccessibleMessageType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MaybeInaccessibleMessage> for MaybeInaccessibleMessageType {
    fn from(val: &'a MaybeInaccessibleMessage) -> Self {
        match val {
            MaybeInaccessibleMessage::InaccessibleMessage(_) => {
                MaybeInaccessibleMessageType::InaccessibleMessage
            }
            MaybeInaccessibleMessage::Message(_) => MaybeInaccessibleMessageType::Message,
        }
    }
}
impl From<MaybeInaccessibleMessage> for MaybeInaccessibleMessageType {
    fn from(val: MaybeInaccessibleMessage) -> Self {
        MaybeInaccessibleMessageType::from(&val)
    }
}
/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
/// Currently, it can be one of
/// - [`crate::types::StarTransactionIncoming`]
/// - [`crate::types::StarTransactionOutgoing`]
/// # Documentation
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum StarTransactionType {
    #[strum(serialize = "incoming")]
    Incoming,
    #[strum(serialize = "outgoing")]
    Outgoing,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl StarTransactionType {
    #[must_use]
    pub const fn all() -> [StarTransactionType; 3usize] {
        [
            StarTransactionType::Incoming,
            StarTransactionType::Outgoing,
            StarTransactionType::Unknown,
        ]
    }
}
impl From<StarTransactionType> for Box<str> {
    fn from(val: StarTransactionType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<StarTransactionType> for String {
    fn from(val: StarTransactionType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for StarTransactionType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a StarTransaction> for StarTransactionType {
    fn from(val: &'a StarTransaction) -> Self {
        match val {
            StarTransaction::Incoming(_) => StarTransactionType::Incoming,
            StarTransaction::Outgoing(_) => StarTransactionType::Outgoing,
            StarTransaction::Unknown(_) => StarTransactionType::Unknown,
        }
    }
}
impl From<StarTransaction> for StarTransactionType {
    fn from(val: StarTransaction) -> Self {
        StarTransactionType::from(&val)
    }
}
/// This object describes the content of a story to post. Currently, it can be one of
/// - [`crate::types::InputStoryContentPhoto`]
/// - [`crate::types::InputStoryContentVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputstorycontent>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputStoryContentType {
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
}
impl InputStoryContentType {
    #[must_use]
    pub const fn all() -> [InputStoryContentType; 2usize] {
        [InputStoryContentType::Photo, InputStoryContentType::Video]
    }
}
impl From<InputStoryContentType> for Box<str> {
    fn from(val: InputStoryContentType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputStoryContentType> for String {
    fn from(val: InputStoryContentType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputStoryContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputStoryContent> for InputStoryContentType {
    fn from(val: &'a InputStoryContent) -> Self {
        match val {
            InputStoryContent::Photo(_) => InputStoryContentType::Photo,
            InputStoryContent::Video(_) => InputStoryContentType::Video,
        }
    }
}
impl From<InputStoryContent> for InputStoryContentType {
    fn from(val: InputStoryContent) -> Self {
        InputStoryContentType::from(&val)
    }
}
/// This object represents a message.
/// Currently, it can be one of
/// - [`crate::types::MessageAnimation`]
/// - [`crate::types::MessageAudio`]
/// - [`crate::types::MessageBoostAdded`]
/// - [`crate::types::MessageChannelChatCreated`]
/// - [`crate::types::MessageChatBackgroundSet`]
/// - [`crate::types::MessageChatOwnerChanged`]
/// - [`crate::types::MessageChatOwnerLeft`]
/// - [`crate::types::MessageChatShared`]
/// - [`crate::types::MessageChecklist`]
/// - [`crate::types::MessageChecklistTasksAdded`]
/// - [`crate::types::MessageChecklistTasksDone`]
/// - [`crate::types::MessageCommunityChatAdded`]
/// - [`crate::types::MessageCommunityChatJoined`]
/// - [`crate::types::MessageCommunityChatRemoved`]
/// - [`crate::types::MessageConnectedWebsite`]
/// - [`crate::types::MessageContact`]
/// - [`crate::types::MessageDeleteChatPhoto`]
/// - [`crate::types::MessageDice`]
/// - [`crate::types::MessageDirectMessagePriceChanged`]
/// - [`crate::types::MessageDocument`]
/// - [`crate::types::MessageForumTopicClosed`]
/// - [`crate::types::MessageForumTopicCreated`]
/// - [`crate::types::MessageForumTopicEdited`]
/// - [`crate::types::MessageForumTopicReopened`]
/// - [`crate::types::MessageGame`]
/// - [`crate::types::MessageGeneralForumTopicHidden`]
/// - [`crate::types::MessageGeneralForumTopicUnhidden`]
/// - [`crate::types::MessageGift`]
/// - [`crate::types::MessageGiftUpgradeSent`]
/// - [`crate::types::MessageGiveaway`]
/// - [`crate::types::MessageGiveawayCompleted`]
/// - [`crate::types::MessageGiveawayCreated`]
/// - [`crate::types::MessageGiveawayWinners`]
/// - [`crate::types::MessageGroupChatCreated`]
/// - [`crate::types::MessageInvoice`]
/// - [`crate::types::MessageLeftChatMember`]
/// - [`crate::types::MessageLivePhoto`]
/// - [`crate::types::MessageLocation`]
/// - [`crate::types::MessageManagedBotCreated`]
/// - [`crate::types::MessageMessageAutoDeleteTimerChanged`]
/// - [`crate::types::MessageMigrateFromChatId`]
/// - [`crate::types::MessageMigrateToChatId`]
/// - [`crate::types::MessageNewChatMembers`]
/// - [`crate::types::MessageNewChatPhoto`]
/// - [`crate::types::MessageNewChatTitle`]
/// - [`crate::types::MessagePaidMedia`]
/// - [`crate::types::MessagePaidMessagePriceChanged`]
/// - [`crate::types::MessagePassportData`]
/// - [`crate::types::MessagePhoto`]
/// - [`crate::types::MessagePinnedMessage`]
/// - [`crate::types::MessagePoll`]
/// - [`crate::types::MessagePollOptionAdded`]
/// - [`crate::types::MessagePollOptionDeleted`]
/// - [`crate::types::MessageProximityAlertTriggered`]
/// - [`crate::types::MessageRefundedPayment`]
/// - [`crate::types::MessageRichMessage`]
/// - [`crate::types::MessageSticker`]
/// - [`crate::types::MessageStory`]
/// - [`crate::types::MessageSuccessfulPayment`]
/// - [`crate::types::MessageSuggestedPostApprovalFailed`]
/// - [`crate::types::MessageSuggestedPostApproved`]
/// - [`crate::types::MessageSuggestedPostDeclined`]
/// - [`crate::types::MessageSuggestedPostPaid`]
/// - [`crate::types::MessageSuggestedPostRefunded`]
/// - [`crate::types::MessageSupergroupChatCreated`]
/// - [`crate::types::MessageText`]
/// - [`crate::types::MessageUniqueGift`]
/// - [`crate::types::MessageUsersShared`]
/// - [`crate::types::MessageVenue`]
/// - [`crate::types::MessageVideo`]
/// - [`crate::types::MessageVideoChatEnded`]
/// - [`crate::types::MessageVideoChatParticipantsInvited`]
/// - [`crate::types::MessageVideoChatScheduled`]
/// - [`crate::types::MessageVideoChatStarted`]
/// - [`crate::types::MessageVideoNote`]
/// - [`crate::types::MessageVoice`]
/// - [`crate::types::MessageWebAppData`]
/// - [`crate::types::MessageWriteAccessAllowed`]
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum MessageType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "boost_added")]
    BoostAdded,
    #[strum(serialize = "channel_chat_created")]
    ChannelChatCreated,
    #[strum(serialize = "chat_background_set")]
    ChatBackgroundSet,
    #[strum(serialize = "chat_owner_changed")]
    ChatOwnerChanged,
    #[strum(serialize = "chat_owner_left")]
    ChatOwnerLeft,
    #[strum(serialize = "chat_shared")]
    ChatShared,
    #[strum(serialize = "checklist")]
    Checklist,
    #[strum(serialize = "checklist_tasks_added")]
    ChecklistTasksAdded,
    #[strum(serialize = "checklist_tasks_done")]
    ChecklistTasksDone,
    #[strum(serialize = "community_chat_added")]
    CommunityChatAdded,
    #[strum(serialize = "community_chat_joined")]
    CommunityChatJoined,
    #[strum(serialize = "community_chat_removed")]
    CommunityChatRemoved,
    #[strum(serialize = "connected_website")]
    ConnectedWebsite,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "delete_chat_photo")]
    DeleteChatPhoto,
    #[strum(serialize = "dice")]
    Dice,
    #[strum(serialize = "direct_message_price_changed")]
    DirectMessagePriceChanged,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "forum_topic_closed")]
    ForumTopicClosed,
    #[strum(serialize = "forum_topic_created")]
    ForumTopicCreated,
    #[strum(serialize = "forum_topic_edited")]
    ForumTopicEdited,
    #[strum(serialize = "forum_topic_reopened")]
    ForumTopicReopened,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "general_forum_topic_hidden")]
    GeneralForumTopicHidden,
    #[strum(serialize = "general_forum_topic_unhidden")]
    GeneralForumTopicUnhidden,
    #[strum(serialize = "gift")]
    Gift,
    #[strum(serialize = "gift_upgrade_sent")]
    GiftUpgradeSent,
    #[strum(serialize = "giveaway")]
    Giveaway,
    #[strum(serialize = "giveaway_completed")]
    GiveawayCompleted,
    #[strum(serialize = "giveaway_created")]
    GiveawayCreated,
    #[strum(serialize = "giveaway_winners")]
    GiveawayWinners,
    #[strum(serialize = "group_chat_created")]
    GroupChatCreated,
    #[strum(serialize = "invoice")]
    Invoice,
    #[strum(serialize = "left_chat_member")]
    LeftChatMember,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "managed_bot_created")]
    ManagedBotCreated,
    #[strum(serialize = "message_auto_delete_timer_changed")]
    MessageAutoDeleteTimerChanged,
    #[strum(serialize = "migrate_from_chat_id")]
    MigrateFromChatId,
    #[strum(serialize = "migrate_to_chat_id")]
    MigrateToChatId,
    #[strum(serialize = "new_chat_members")]
    NewChatMembers,
    #[strum(serialize = "new_chat_photo")]
    NewChatPhoto,
    #[strum(serialize = "new_chat_title")]
    NewChatTitle,
    #[strum(serialize = "paid_media")]
    PaidMedia,
    #[strum(serialize = "paid_message_price_changed")]
    PaidMessagePriceChanged,
    #[strum(serialize = "passport_data")]
    PassportData,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "pinned_message")]
    PinnedMessage,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "poll_option_added")]
    PollOptionAdded,
    #[strum(serialize = "poll_option_deleted")]
    PollOptionDeleted,
    #[strum(serialize = "proximity_alert_triggered")]
    ProximityAlertTriggered,
    #[strum(serialize = "refunded_payment")]
    RefundedPayment,
    #[strum(serialize = "rich_message")]
    RichMessage,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "story")]
    Story,
    #[strum(serialize = "successful_payment")]
    SuccessfulPayment,
    #[strum(serialize = "suggested_post_approval_failed")]
    SuggestedPostApprovalFailed,
    #[strum(serialize = "suggested_post_approved")]
    SuggestedPostApproved,
    #[strum(serialize = "suggested_post_declined")]
    SuggestedPostDeclined,
    #[strum(serialize = "suggested_post_paid")]
    SuggestedPostPaid,
    #[strum(serialize = "suggested_post_refunded")]
    SuggestedPostRefunded,
    #[strum(serialize = "supergroup_chat_created")]
    SupergroupChatCreated,
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "unique_gift")]
    UniqueGift,
    #[strum(serialize = "users_shared")]
    UsersShared,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "video_chat_ended")]
    VideoChatEnded,
    #[strum(serialize = "video_chat_participants_invited")]
    VideoChatParticipantsInvited,
    #[strum(serialize = "video_chat_scheduled")]
    VideoChatScheduled,
    #[strum(serialize = "video_chat_started")]
    VideoChatStarted,
    #[strum(serialize = "video_note")]
    VideoNote,
    #[strum(serialize = "voice")]
    Voice,
    #[strum(serialize = "web_app_data")]
    WebAppData,
    #[strum(serialize = "write_access_allowed")]
    WriteAccessAllowed,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl MessageType {
    #[must_use]
    pub const fn all() -> [MessageType; 79usize] {
        [
            MessageType::Animation,
            MessageType::LivePhoto,
            MessageType::Venue,
            MessageType::Audio,
            MessageType::BoostAdded,
            MessageType::ChannelChatCreated,
            MessageType::ChatBackgroundSet,
            MessageType::ChatOwnerChanged,
            MessageType::ChatOwnerLeft,
            MessageType::ChatShared,
            MessageType::Checklist,
            MessageType::ChecklistTasksAdded,
            MessageType::ChecklistTasksDone,
            MessageType::CommunityChatAdded,
            MessageType::CommunityChatJoined,
            MessageType::CommunityChatRemoved,
            MessageType::ConnectedWebsite,
            MessageType::Contact,
            MessageType::DeleteChatPhoto,
            MessageType::Dice,
            MessageType::DirectMessagePriceChanged,
            MessageType::Document,
            MessageType::ForumTopicClosed,
            MessageType::ForumTopicCreated,
            MessageType::ForumTopicEdited,
            MessageType::ForumTopicReopened,
            MessageType::Game,
            MessageType::GeneralForumTopicHidden,
            MessageType::GeneralForumTopicUnhidden,
            MessageType::Gift,
            MessageType::GiftUpgradeSent,
            MessageType::Giveaway,
            MessageType::GiveawayCompleted,
            MessageType::GiveawayCreated,
            MessageType::GiveawayWinners,
            MessageType::GroupChatCreated,
            MessageType::Invoice,
            MessageType::LeftChatMember,
            MessageType::Location,
            MessageType::ManagedBotCreated,
            MessageType::MessageAutoDeleteTimerChanged,
            MessageType::MigrateFromChatId,
            MessageType::MigrateToChatId,
            MessageType::NewChatMembers,
            MessageType::NewChatPhoto,
            MessageType::NewChatTitle,
            MessageType::PaidMedia,
            MessageType::PaidMessagePriceChanged,
            MessageType::PassportData,
            MessageType::Photo,
            MessageType::PinnedMessage,
            MessageType::Poll,
            MessageType::PollOptionAdded,
            MessageType::PollOptionDeleted,
            MessageType::ProximityAlertTriggered,
            MessageType::RefundedPayment,
            MessageType::RichMessage,
            MessageType::Sticker,
            MessageType::Story,
            MessageType::SuccessfulPayment,
            MessageType::SuggestedPostApprovalFailed,
            MessageType::SuggestedPostApproved,
            MessageType::SuggestedPostDeclined,
            MessageType::SuggestedPostPaid,
            MessageType::SuggestedPostRefunded,
            MessageType::SupergroupChatCreated,
            MessageType::Text,
            MessageType::UniqueGift,
            MessageType::UsersShared,
            MessageType::Video,
            MessageType::VideoChatEnded,
            MessageType::VideoChatParticipantsInvited,
            MessageType::VideoChatScheduled,
            MessageType::VideoChatStarted,
            MessageType::VideoNote,
            MessageType::Voice,
            MessageType::WebAppData,
            MessageType::WriteAccessAllowed,
            MessageType::Unknown,
        ]
    }
}
impl From<MessageType> for Box<str> {
    fn from(val: MessageType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MessageType> for String {
    fn from(val: MessageType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MessageType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Message> for MessageType {
    fn from(val: &'a Message) -> Self {
        match val {
            Message::Animation(_) => MessageType::Animation,
            Message::LivePhoto(_) => MessageType::LivePhoto,
            Message::Venue(_) => MessageType::Venue,
            Message::Audio(_) => MessageType::Audio,
            Message::BoostAdded(_) => MessageType::BoostAdded,
            Message::ChannelChatCreated(_) => MessageType::ChannelChatCreated,
            Message::ChatBackgroundSet(_) => MessageType::ChatBackgroundSet,
            Message::ChatOwnerChanged(_) => MessageType::ChatOwnerChanged,
            Message::ChatOwnerLeft(_) => MessageType::ChatOwnerLeft,
            Message::ChatShared(_) => MessageType::ChatShared,
            Message::Checklist(_) => MessageType::Checklist,
            Message::ChecklistTasksAdded(_) => MessageType::ChecklistTasksAdded,
            Message::ChecklistTasksDone(_) => MessageType::ChecklistTasksDone,
            Message::CommunityChatAdded(_) => MessageType::CommunityChatAdded,
            Message::CommunityChatJoined(_) => MessageType::CommunityChatJoined,
            Message::CommunityChatRemoved(_) => MessageType::CommunityChatRemoved,
            Message::ConnectedWebsite(_) => MessageType::ConnectedWebsite,
            Message::Contact(_) => MessageType::Contact,
            Message::DeleteChatPhoto(_) => MessageType::DeleteChatPhoto,
            Message::Dice(_) => MessageType::Dice,
            Message::DirectMessagePriceChanged(_) => MessageType::DirectMessagePriceChanged,
            Message::Document(_) => MessageType::Document,
            Message::ForumTopicClosed(_) => MessageType::ForumTopicClosed,
            Message::ForumTopicCreated(_) => MessageType::ForumTopicCreated,
            Message::ForumTopicEdited(_) => MessageType::ForumTopicEdited,
            Message::ForumTopicReopened(_) => MessageType::ForumTopicReopened,
            Message::Game(_) => MessageType::Game,
            Message::GeneralForumTopicHidden(_) => MessageType::GeneralForumTopicHidden,
            Message::GeneralForumTopicUnhidden(_) => MessageType::GeneralForumTopicUnhidden,
            Message::Gift(_) => MessageType::Gift,
            Message::GiftUpgradeSent(_) => MessageType::GiftUpgradeSent,
            Message::Giveaway(_) => MessageType::Giveaway,
            Message::GiveawayCompleted(_) => MessageType::GiveawayCompleted,
            Message::GiveawayCreated(_) => MessageType::GiveawayCreated,
            Message::GiveawayWinners(_) => MessageType::GiveawayWinners,
            Message::GroupChatCreated(_) => MessageType::GroupChatCreated,
            Message::Invoice(_) => MessageType::Invoice,
            Message::LeftChatMember(_) => MessageType::LeftChatMember,
            Message::Location(_) => MessageType::Location,
            Message::ManagedBotCreated(_) => MessageType::ManagedBotCreated,
            Message::MessageAutoDeleteTimerChanged(_) => MessageType::MessageAutoDeleteTimerChanged,
            Message::MigrateFromChatId(_) => MessageType::MigrateFromChatId,
            Message::MigrateToChatId(_) => MessageType::MigrateToChatId,
            Message::NewChatMembers(_) => MessageType::NewChatMembers,
            Message::NewChatPhoto(_) => MessageType::NewChatPhoto,
            Message::NewChatTitle(_) => MessageType::NewChatTitle,
            Message::PaidMedia(_) => MessageType::PaidMedia,
            Message::PaidMessagePriceChanged(_) => MessageType::PaidMessagePriceChanged,
            Message::PassportData(_) => MessageType::PassportData,
            Message::Photo(_) => MessageType::Photo,
            Message::PinnedMessage(_) => MessageType::PinnedMessage,
            Message::Poll(_) => MessageType::Poll,
            Message::PollOptionAdded(_) => MessageType::PollOptionAdded,
            Message::PollOptionDeleted(_) => MessageType::PollOptionDeleted,
            Message::ProximityAlertTriggered(_) => MessageType::ProximityAlertTriggered,
            Message::RefundedPayment(_) => MessageType::RefundedPayment,
            Message::RichMessage(_) => MessageType::RichMessage,
            Message::Sticker(_) => MessageType::Sticker,
            Message::Story(_) => MessageType::Story,
            Message::SuccessfulPayment(_) => MessageType::SuccessfulPayment,
            Message::SuggestedPostApprovalFailed(_) => MessageType::SuggestedPostApprovalFailed,
            Message::SuggestedPostApproved(_) => MessageType::SuggestedPostApproved,
            Message::SuggestedPostDeclined(_) => MessageType::SuggestedPostDeclined,
            Message::SuggestedPostPaid(_) => MessageType::SuggestedPostPaid,
            Message::SuggestedPostRefunded(_) => MessageType::SuggestedPostRefunded,
            Message::SupergroupChatCreated(_) => MessageType::SupergroupChatCreated,
            Message::Text(_) => MessageType::Text,
            Message::UniqueGift(_) => MessageType::UniqueGift,
            Message::UsersShared(_) => MessageType::UsersShared,
            Message::Video(_) => MessageType::Video,
            Message::VideoChatEnded(_) => MessageType::VideoChatEnded,
            Message::VideoChatParticipantsInvited(_) => MessageType::VideoChatParticipantsInvited,
            Message::VideoChatScheduled(_) => MessageType::VideoChatScheduled,
            Message::VideoChatStarted(_) => MessageType::VideoChatStarted,
            Message::VideoNote(_) => MessageType::VideoNote,
            Message::Voice(_) => MessageType::Voice,
            Message::WebAppData(_) => MessageType::WebAppData,
            Message::WriteAccessAllowed(_) => MessageType::WriteAccessAllowed,
            Message::Unknown(_) => MessageType::Unknown,
        }
    }
}
impl From<Message> for MessageType {
    fn from(val: Message) -> Self {
        MessageType::from(&val)
    }
}
/// This object represents the content of a media message to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaAudio`]
/// - [`crate::types::InputMediaDocument`]
/// - [`crate::types::InputMediaLivePhoto`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
}
impl InputMediaType {
    #[must_use]
    pub const fn all() -> [InputMediaType; 6usize] {
        [
            InputMediaType::Animation,
            InputMediaType::Audio,
            InputMediaType::Document,
            InputMediaType::LivePhoto,
            InputMediaType::Photo,
            InputMediaType::Video,
        ]
    }
}
impl From<InputMediaType> for Box<str> {
    fn from(val: InputMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputMediaType> for String {
    fn from(val: InputMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputMedia> for InputMediaType {
    fn from(val: &'a InputMedia) -> Self {
        match val {
            InputMedia::Animation(_) => InputMediaType::Animation,
            InputMedia::Audio(_) => InputMediaType::Audio,
            InputMedia::Document(_) => InputMediaType::Document,
            InputMedia::LivePhoto(_) => InputMediaType::LivePhoto,
            InputMedia::Photo(_) => InputMediaType::Photo,
            InputMedia::Video(_) => InputMediaType::Video,
        }
    }
}
impl From<InputMedia> for InputMediaType {
    fn from(val: InputMedia) -> Self {
        InputMediaType::from(&val)
    }
}
/// This object represents the content of a poll description or a quiz explanation to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaAudio`]
/// - [`crate::types::InputMediaDocument`]
/// - [`crate::types::InputMediaLivePhoto`]
/// - [`crate::types::InputMediaLocation`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaVenue`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpollmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputPollMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "video")]
    Video,
}
impl InputPollMediaType {
    #[must_use]
    pub const fn all() -> [InputPollMediaType; 8usize] {
        [
            InputPollMediaType::Animation,
            InputPollMediaType::Audio,
            InputPollMediaType::Document,
            InputPollMediaType::LivePhoto,
            InputPollMediaType::Location,
            InputPollMediaType::Photo,
            InputPollMediaType::Venue,
            InputPollMediaType::Video,
        ]
    }
}
impl From<InputPollMediaType> for Box<str> {
    fn from(val: InputPollMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputPollMediaType> for String {
    fn from(val: InputPollMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputPollMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputPollMedia> for InputPollMediaType {
    fn from(val: &'a InputPollMedia) -> Self {
        match val {
            InputPollMedia::Animation(_) => InputPollMediaType::Animation,
            InputPollMedia::Audio(_) => InputPollMediaType::Audio,
            InputPollMedia::Document(_) => InputPollMediaType::Document,
            InputPollMedia::LivePhoto(_) => InputPollMediaType::LivePhoto,
            InputPollMedia::Location(_) => InputPollMediaType::Location,
            InputPollMedia::Photo(_) => InputPollMediaType::Photo,
            InputPollMedia::Venue(_) => InputPollMediaType::Venue,
            InputPollMedia::Video(_) => InputPollMediaType::Video,
        }
    }
}
impl From<InputPollMedia> for InputPollMediaType {
    fn from(val: InputPollMedia) -> Self {
        InputPollMediaType::from(&val)
    }
}
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedMpeg4Gif`] and [`crate::types::InlineQueryResultMpeg4Gif`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultMpeg4GifKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultMpeg4GifKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultMpeg4GifKindType; 2usize] {
        [
            InlineQueryResultMpeg4GifKindType::Uncached,
            InlineQueryResultMpeg4GifKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultMpeg4GifKindType> for Box<str> {
    fn from(val: InlineQueryResultMpeg4GifKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultMpeg4GifKindType> for String {
    fn from(val: InlineQueryResultMpeg4GifKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultMpeg4GifKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultMpeg4GifKind> for InlineQueryResultMpeg4GifKindType {
    fn from(val: &'a InlineQueryResultMpeg4GifKind) -> Self {
        match val {
            InlineQueryResultMpeg4GifKind::Uncached(_) => {
                InlineQueryResultMpeg4GifKindType::Uncached
            }
            InlineQueryResultMpeg4GifKind::Cached(_) => InlineQueryResultMpeg4GifKindType::Cached,
        }
    }
}
impl From<InlineQueryResultMpeg4GifKind> for InlineQueryResultMpeg4GifKindType {
    fn from(val: InlineQueryResultMpeg4GifKind) -> Self {
        InlineQueryResultMpeg4GifKindType::from(&val)
    }
}
/// This object describes the source of a chat boost. It can be one of
/// - [`crate::types::ChatBoostSourcePremium`]
/// - [`crate::types::ChatBoostSourceGiftCode`]
/// - [`crate::types::ChatBoostSourceGiveaway`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsource>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ChatBoostSourceType {
    #[strum(serialize = "premium")]
    Premium,
    #[strum(serialize = "gift_code")]
    GiftCode,
    #[strum(serialize = "giveaway")]
    Giveaway,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ChatBoostSourceType {
    #[must_use]
    pub const fn all() -> [ChatBoostSourceType; 4usize] {
        [
            ChatBoostSourceType::Premium,
            ChatBoostSourceType::GiftCode,
            ChatBoostSourceType::Giveaway,
            ChatBoostSourceType::Unknown,
        ]
    }
}
impl From<ChatBoostSourceType> for Box<str> {
    fn from(val: ChatBoostSourceType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatBoostSourceType> for String {
    fn from(val: ChatBoostSourceType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatBoostSourceType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ChatBoostSource> for ChatBoostSourceType {
    fn from(val: &'a ChatBoostSource) -> Self {
        match val {
            ChatBoostSource::Premium(_) => ChatBoostSourceType::Premium,
            ChatBoostSource::GiftCode(_) => ChatBoostSourceType::GiftCode,
            ChatBoostSource::Giveaway(_) => ChatBoostSourceType::Giveaway,
            ChatBoostSource::Unknown(_) => ChatBoostSourceType::Unknown,
        }
    }
}
impl From<ChatBoostSource> for ChatBoostSourceType {
    fn from(val: ChatBoostSource) -> Self {
        ChatBoostSourceType::from(&val)
    }
}
/// This object describes the paid media to be sent. Currently, it can be one of
/// - [`crate::types::InputPaidMediaLivePhoto`]
/// - [`crate::types::InputPaidMediaPhoto`]
/// - [`crate::types::InputPaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputPaidMediaType {
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
}
impl InputPaidMediaType {
    #[must_use]
    pub const fn all() -> [InputPaidMediaType; 3usize] {
        [
            InputPaidMediaType::LivePhoto,
            InputPaidMediaType::Photo,
            InputPaidMediaType::Video,
        ]
    }
}
impl From<InputPaidMediaType> for Box<str> {
    fn from(val: InputPaidMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputPaidMediaType> for String {
    fn from(val: InputPaidMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputPaidMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputPaidMedia> for InputPaidMediaType {
    fn from(val: &'a InputPaidMedia) -> Self {
        match val {
            InputPaidMedia::LivePhoto(_) => InputPaidMediaType::LivePhoto,
            InputPaidMedia::Photo(_) => InputPaidMediaType::Photo,
            InputPaidMedia::Video(_) => InputPaidMediaType::Video,
        }
    }
}
impl From<InputPaidMedia> for InputPaidMediaType {
    fn from(val: InputPaidMedia) -> Self {
        InputPaidMediaType::from(&val)
    }
}
/// This object describes the origin of a message. It can be one of
/// - [`crate::types::MessageOriginUser`]
/// - [`crate::types::MessageOriginHiddenUser`]
/// - [`crate::types::MessageOriginChat`]
/// - [`crate::types::MessageOriginChannel`]
/// # Documentation
/// <https://core.telegram.org/bots/api#messageorigin>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum MessageOriginType {
    #[strum(serialize = "user")]
    User,
    #[strum(serialize = "hidden_user")]
    HiddenUser,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "channel")]
    Channel,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl MessageOriginType {
    #[must_use]
    pub const fn all() -> [MessageOriginType; 5usize] {
        [
            MessageOriginType::User,
            MessageOriginType::HiddenUser,
            MessageOriginType::Chat,
            MessageOriginType::Channel,
            MessageOriginType::Unknown,
        ]
    }
}
impl From<MessageOriginType> for Box<str> {
    fn from(val: MessageOriginType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MessageOriginType> for String {
    fn from(val: MessageOriginType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MessageOriginType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MessageOrigin> for MessageOriginType {
    fn from(val: &'a MessageOrigin) -> Self {
        match val {
            MessageOrigin::User(_) => MessageOriginType::User,
            MessageOrigin::HiddenUser(_) => MessageOriginType::HiddenUser,
            MessageOrigin::Chat(_) => MessageOriginType::Chat,
            MessageOrigin::Channel(_) => MessageOriginType::Channel,
            MessageOrigin::Unknown(_) => MessageOriginType::Unknown,
        }
    }
}
impl From<MessageOrigin> for MessageOriginType {
    fn from(val: MessageOrigin) -> Self {
        MessageOriginType::from(&val)
    }
}
/// This object represents the content of a poll option to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaLink`]
/// - [`crate::types::InputMediaLivePhoto`]
/// - [`crate::types::InputMediaLocation`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaSticker`]
/// - [`crate::types::InputMediaVenue`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpolloptionmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputPollOptionMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "link")]
    Link,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "video")]
    Video,
}
impl InputPollOptionMediaType {
    #[must_use]
    pub const fn all() -> [InputPollOptionMediaType; 8usize] {
        [
            InputPollOptionMediaType::Animation,
            InputPollOptionMediaType::Link,
            InputPollOptionMediaType::LivePhoto,
            InputPollOptionMediaType::Location,
            InputPollOptionMediaType::Photo,
            InputPollOptionMediaType::Sticker,
            InputPollOptionMediaType::Venue,
            InputPollOptionMediaType::Video,
        ]
    }
}
impl From<InputPollOptionMediaType> for Box<str> {
    fn from(val: InputPollOptionMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputPollOptionMediaType> for String {
    fn from(val: InputPollOptionMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputPollOptionMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputPollOptionMedia> for InputPollOptionMediaType {
    fn from(val: &'a InputPollOptionMedia) -> Self {
        match val {
            InputPollOptionMedia::Animation(_) => InputPollOptionMediaType::Animation,
            InputPollOptionMedia::Link(_) => InputPollOptionMediaType::Link,
            InputPollOptionMedia::LivePhoto(_) => InputPollOptionMediaType::LivePhoto,
            InputPollOptionMedia::Location(_) => InputPollOptionMediaType::Location,
            InputPollOptionMedia::Photo(_) => InputPollOptionMediaType::Photo,
            InputPollOptionMedia::Sticker(_) => InputPollOptionMediaType::Sticker,
            InputPollOptionMedia::Venue(_) => InputPollOptionMediaType::Venue,
            InputPollOptionMedia::Video(_) => InputPollOptionMediaType::Video,
        }
    }
}
impl From<InputPollOptionMedia> for InputPollOptionMediaType {
    fn from(val: InputPollOptionMedia) -> Self {
        InputPollOptionMediaType::from(&val)
    }
}
/// This object represents a sticker.
/// Currently, it can be one of
/// - [`crate::types::StickerCustomEmoji`]
/// - [`crate::types::StickerMask`]
/// - [`crate::types::StickerRegular`]
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum StickerType {
    #[strum(serialize = "regular")]
    Regular,
    #[strum(serialize = "mask")]
    Mask,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl StickerType {
    #[must_use]
    pub const fn all() -> [StickerType; 4usize] {
        [
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
            StickerType::Unknown,
        ]
    }
}
impl From<StickerType> for Box<str> {
    fn from(val: StickerType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<StickerType> for String {
    fn from(val: StickerType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for StickerType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Sticker> for StickerType {
    fn from(val: &'a Sticker) -> Self {
        match val {
            Sticker::Regular(_) => StickerType::Regular,
            Sticker::Mask(_) => StickerType::Mask,
            Sticker::CustomEmoji(_) => StickerType::CustomEmoji,
            Sticker::Unknown(_) => StickerType::Unknown,
        }
    }
}
impl From<Sticker> for StickerType {
    fn from(val: Sticker) -> Self {
        StickerType::from(&val)
    }
}
/// This object describes the type of a reaction. Currently, it can be one of
/// - [`crate::types::ReactionTypeEmoji`]
/// - [`crate::types::ReactionTypeCustomEmoji`]
/// - [`crate::types::ReactionTypePaid`]
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontype>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ReactionTypeType {
    #[strum(serialize = "emoji")]
    Emoji,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "paid")]
    Paid,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ReactionTypeType {
    #[must_use]
    pub const fn all() -> [ReactionTypeType; 4usize] {
        [
            ReactionTypeType::Emoji,
            ReactionTypeType::CustomEmoji,
            ReactionTypeType::Paid,
            ReactionTypeType::Unknown,
        ]
    }
}
impl From<ReactionTypeType> for Box<str> {
    fn from(val: ReactionTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ReactionTypeType> for String {
    fn from(val: ReactionTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ReactionTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ReactionType> for ReactionTypeType {
    fn from(val: &'a ReactionType) -> Self {
        match val {
            ReactionType::Emoji(_) => ReactionTypeType::Emoji,
            ReactionType::CustomEmoji(_) => ReactionTypeType::CustomEmoji,
            ReactionType::Paid(_) => ReactionTypeType::Paid,
            ReactionType::Unknown(_) => ReactionTypeType::Unknown,
        }
    }
}
impl From<ReactionType> for ReactionTypeType {
    fn from(val: ReactionType) -> Self {
        ReactionTypeType::from(&val)
    }
}
/// This object represents a block in a rich formatted message to be sent. Currently, it can be any of the following types:
/// - [`crate::types::InputRichBlockParagraph`]
/// - [`crate::types::InputRichBlockSectionHeading`]
/// - [`crate::types::InputRichBlockPreformatted`]
/// - [`crate::types::InputRichBlockFooter`]
/// - [`crate::types::InputRichBlockDivider`]
/// - [`crate::types::InputRichBlockMathematicalExpression`]
/// - [`crate::types::InputRichBlockAnchor`]
/// - [`crate::types::InputRichBlockList`]
/// - [`crate::types::InputRichBlockBlockQuotation`]
/// - [`crate::types::InputRichBlockExpandableBlockQuotation`]
/// - [`crate::types::InputRichBlockPullQuotation`]
/// - [`crate::types::InputRichBlockCollage`]
/// - [`crate::types::InputRichBlockSlideshow`]
/// - [`crate::types::InputRichBlockTable`]
/// - [`crate::types::InputRichBlockDetails`]
/// - [`crate::types::InputRichBlockMap`]
/// - [`crate::types::InputRichBlockButtons`]
/// - [`crate::types::InputRichBlockAnimation`]
/// - [`crate::types::InputRichBlockAudio`]
/// - [`crate::types::InputRichBlockDocument`]
/// - [`crate::types::InputRichBlockPhoto`]
/// - [`crate::types::InputRichBlockVideo`]
/// - [`crate::types::InputRichBlockVoiceNote`]
/// - [`crate::types::InputRichBlockThinking`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblock>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputRichBlockType {
    #[strum(serialize = "paragraph")]
    Paragraph,
    #[strum(serialize = "heading")]
    Heading,
    #[strum(serialize = "pre")]
    Pre,
    #[strum(serialize = "footer")]
    Footer,
    #[strum(serialize = "divider")]
    Divider,
    #[strum(serialize = "mathematical_expression")]
    MathematicalExpression,
    #[strum(serialize = "anchor")]
    Anchor,
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "blockquote")]
    Blockquote,
    #[strum(serialize = "expandable_blockquote")]
    ExpandableBlockquote,
    #[strum(serialize = "pullquote")]
    Pullquote,
    #[strum(serialize = "collage")]
    Collage,
    #[strum(serialize = "slideshow")]
    Slideshow,
    #[strum(serialize = "table")]
    Table,
    #[strum(serialize = "details")]
    Details,
    #[strum(serialize = "map")]
    Map,
    #[strum(serialize = "buttons")]
    Buttons,
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice_note")]
    VoiceNote,
    #[strum(serialize = "thinking")]
    Thinking,
}
impl InputRichBlockType {
    #[must_use]
    pub const fn all() -> [InputRichBlockType; 24usize] {
        [
            InputRichBlockType::Paragraph,
            InputRichBlockType::Heading,
            InputRichBlockType::Pre,
            InputRichBlockType::Footer,
            InputRichBlockType::Divider,
            InputRichBlockType::MathematicalExpression,
            InputRichBlockType::Anchor,
            InputRichBlockType::List,
            InputRichBlockType::Blockquote,
            InputRichBlockType::ExpandableBlockquote,
            InputRichBlockType::Pullquote,
            InputRichBlockType::Collage,
            InputRichBlockType::Slideshow,
            InputRichBlockType::Table,
            InputRichBlockType::Details,
            InputRichBlockType::Map,
            InputRichBlockType::Buttons,
            InputRichBlockType::Animation,
            InputRichBlockType::Audio,
            InputRichBlockType::Document,
            InputRichBlockType::Photo,
            InputRichBlockType::Video,
            InputRichBlockType::VoiceNote,
            InputRichBlockType::Thinking,
        ]
    }
}
impl From<InputRichBlockType> for Box<str> {
    fn from(val: InputRichBlockType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputRichBlockType> for String {
    fn from(val: InputRichBlockType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputRichBlockType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputRichBlock> for InputRichBlockType {
    fn from(val: &'a InputRichBlock) -> Self {
        match val {
            InputRichBlock::Paragraph(_) => InputRichBlockType::Paragraph,
            InputRichBlock::Heading(_) => InputRichBlockType::Heading,
            InputRichBlock::Pre(_) => InputRichBlockType::Pre,
            InputRichBlock::Footer(_) => InputRichBlockType::Footer,
            InputRichBlock::Divider(_) => InputRichBlockType::Divider,
            InputRichBlock::MathematicalExpression(_) => InputRichBlockType::MathematicalExpression,
            InputRichBlock::Anchor(_) => InputRichBlockType::Anchor,
            InputRichBlock::List(_) => InputRichBlockType::List,
            InputRichBlock::Blockquote(_) => InputRichBlockType::Blockquote,
            InputRichBlock::ExpandableBlockquote(_) => InputRichBlockType::ExpandableBlockquote,
            InputRichBlock::Pullquote(_) => InputRichBlockType::Pullquote,
            InputRichBlock::Collage(_) => InputRichBlockType::Collage,
            InputRichBlock::Slideshow(_) => InputRichBlockType::Slideshow,
            InputRichBlock::Table(_) => InputRichBlockType::Table,
            InputRichBlock::Details(_) => InputRichBlockType::Details,
            InputRichBlock::Map(_) => InputRichBlockType::Map,
            InputRichBlock::Buttons(_) => InputRichBlockType::Buttons,
            InputRichBlock::Animation(_) => InputRichBlockType::Animation,
            InputRichBlock::Audio(_) => InputRichBlockType::Audio,
            InputRichBlock::Document(_) => InputRichBlockType::Document,
            InputRichBlock::Photo(_) => InputRichBlockType::Photo,
            InputRichBlock::Video(_) => InputRichBlockType::Video,
            InputRichBlock::VoiceNote(_) => InputRichBlockType::VoiceNote,
            InputRichBlock::Thinking(_) => InputRichBlockType::Thinking,
        }
    }
}
impl From<InputRichBlock> for InputRichBlockType {
    fn from(val: InputRichBlock) -> Self {
        InputRichBlockType::from(&val)
    }
}
/// This object represents the media content of a rich message to be sent.
/// Currently, it can be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaAudio`]
/// - [`crate::types::InputMediaDocument`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaVideo`]
/// - [`crate::types::InputMediaVoiceNote`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichmessagemedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputRichMessageMediaContentType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice_note")]
    VoiceNote,
}
impl InputRichMessageMediaContentType {
    #[must_use]
    pub const fn all() -> [InputRichMessageMediaContentType; 6usize] {
        [
            InputRichMessageMediaContentType::Animation,
            InputRichMessageMediaContentType::Audio,
            InputRichMessageMediaContentType::Document,
            InputRichMessageMediaContentType::Photo,
            InputRichMessageMediaContentType::Video,
            InputRichMessageMediaContentType::VoiceNote,
        ]
    }
}
impl From<InputRichMessageMediaContentType> for Box<str> {
    fn from(val: InputRichMessageMediaContentType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputRichMessageMediaContentType> for String {
    fn from(val: InputRichMessageMediaContentType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputRichMessageMediaContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputRichMessageMediaContent> for InputRichMessageMediaContentType {
    fn from(val: &'a InputRichMessageMediaContent) -> Self {
        match val {
            InputRichMessageMediaContent::Animation(_) => {
                InputRichMessageMediaContentType::Animation
            }
            InputRichMessageMediaContent::Audio(_) => InputRichMessageMediaContentType::Audio,
            InputRichMessageMediaContent::Document(_) => InputRichMessageMediaContentType::Document,
            InputRichMessageMediaContent::Photo(_) => InputRichMessageMediaContentType::Photo,
            InputRichMessageMediaContent::Video(_) => InputRichMessageMediaContentType::Video,
            InputRichMessageMediaContent::VoiceNote(_) => {
                InputRichMessageMediaContentType::VoiceNote
            }
        }
    }
}
impl From<InputRichMessageMediaContent> for InputRichMessageMediaContentType {
    fn from(val: InputRichMessageMediaContent) -> Self {
        InputRichMessageMediaContentType::from(&val)
    }
}
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedGif`] and [`crate::types::InlineQueryResultGif`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultGifKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultGifKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultGifKindType; 2usize] {
        [
            InlineQueryResultGifKindType::Uncached,
            InlineQueryResultGifKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultGifKindType> for Box<str> {
    fn from(val: InlineQueryResultGifKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultGifKindType> for String {
    fn from(val: InlineQueryResultGifKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultGifKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultGifKind> for InlineQueryResultGifKindType {
    fn from(val: &'a InlineQueryResultGifKind) -> Self {
        match val {
            InlineQueryResultGifKind::Uncached(_) => InlineQueryResultGifKindType::Uncached,
            InlineQueryResultGifKind::Cached(_) => InlineQueryResultGifKindType::Cached,
        }
    }
}
impl From<InlineQueryResultGifKind> for InlineQueryResultGifKindType {
    fn from(val: InlineQueryResultGifKind) -> Self {
        InlineQueryResultGifKindType::from(&val)
    }
}
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
/// Currently, it can be one of
/// - [`crate::types::EncryptedPassportElementAddress`]
/// - [`crate::types::EncryptedPassportElementBankStatement`]
/// - [`crate::types::EncryptedPassportElementDriverLicense`]
/// - [`crate::types::EncryptedPassportElementEmail`]
/// - [`crate::types::EncryptedPassportElementIdentityCard`]
/// - [`crate::types::EncryptedPassportElementInternalPassport`]
/// - [`crate::types::EncryptedPassportElementPassport`]
/// - [`crate::types::EncryptedPassportElementPassportRegistration`]
/// - [`crate::types::EncryptedPassportElementPersonalDetails`]
/// - [`crate::types::EncryptedPassportElementPhoneNumber`]
/// - [`crate::types::EncryptedPassportElementRentalAgreement`]
/// - [`crate::types::EncryptedPassportElementTemporaryRegistration`]
/// - [`crate::types::EncryptedPassportElementUtilityBill`]
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum EncryptedPassportElementType {
    #[strum(serialize = "personal_details")]
    PersonalDetails,
    #[strum(serialize = "passport")]
    Passport,
    #[strum(serialize = "driver_license")]
    DriverLicense,
    #[strum(serialize = "identity_card")]
    IdentityCard,
    #[strum(serialize = "internal_passport")]
    InternalPassport,
    #[strum(serialize = "address")]
    Address,
    #[strum(serialize = "utility_bill")]
    UtilityBill,
    #[strum(serialize = "bank_statement")]
    BankStatement,
    #[strum(serialize = "rental_agreement")]
    RentalAgreement,
    #[strum(serialize = "passport_registration")]
    PassportRegistration,
    #[strum(serialize = "temporary_registration")]
    TemporaryRegistration,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl EncryptedPassportElementType {
    #[must_use]
    pub const fn all() -> [EncryptedPassportElementType; 14usize] {
        [
            EncryptedPassportElementType::PersonalDetails,
            EncryptedPassportElementType::Passport,
            EncryptedPassportElementType::DriverLicense,
            EncryptedPassportElementType::IdentityCard,
            EncryptedPassportElementType::InternalPassport,
            EncryptedPassportElementType::Address,
            EncryptedPassportElementType::UtilityBill,
            EncryptedPassportElementType::BankStatement,
            EncryptedPassportElementType::RentalAgreement,
            EncryptedPassportElementType::PassportRegistration,
            EncryptedPassportElementType::TemporaryRegistration,
            EncryptedPassportElementType::PhoneNumber,
            EncryptedPassportElementType::Email,
            EncryptedPassportElementType::Unknown,
        ]
    }
}
impl From<EncryptedPassportElementType> for Box<str> {
    fn from(val: EncryptedPassportElementType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<EncryptedPassportElementType> for String {
    fn from(val: EncryptedPassportElementType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for EncryptedPassportElementType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a EncryptedPassportElement> for EncryptedPassportElementType {
    fn from(val: &'a EncryptedPassportElement) -> Self {
        match val {
            EncryptedPassportElement::PersonalDetails(_) => {
                EncryptedPassportElementType::PersonalDetails
            }
            EncryptedPassportElement::Passport(_) => EncryptedPassportElementType::Passport,
            EncryptedPassportElement::DriverLicense(_) => {
                EncryptedPassportElementType::DriverLicense
            }
            EncryptedPassportElement::IdentityCard(_) => EncryptedPassportElementType::IdentityCard,
            EncryptedPassportElement::InternalPassport(_) => {
                EncryptedPassportElementType::InternalPassport
            }
            EncryptedPassportElement::Address(_) => EncryptedPassportElementType::Address,
            EncryptedPassportElement::UtilityBill(_) => EncryptedPassportElementType::UtilityBill,
            EncryptedPassportElement::BankStatement(_) => {
                EncryptedPassportElementType::BankStatement
            }
            EncryptedPassportElement::RentalAgreement(_) => {
                EncryptedPassportElementType::RentalAgreement
            }
            EncryptedPassportElement::PassportRegistration(_) => {
                EncryptedPassportElementType::PassportRegistration
            }
            EncryptedPassportElement::TemporaryRegistration(_) => {
                EncryptedPassportElementType::TemporaryRegistration
            }
            EncryptedPassportElement::PhoneNumber(_) => EncryptedPassportElementType::PhoneNumber,
            EncryptedPassportElement::Email(_) => EncryptedPassportElementType::Email,
            EncryptedPassportElement::Unknown(_) => EncryptedPassportElementType::Unknown,
        }
    }
}
impl From<EncryptedPassportElement> for EncryptedPassportElementType {
    fn from(val: EncryptedPassportElement) -> Self {
        EncryptedPassportElementType::from(&val)
    }
}
/// This object describes the bot's menu button in a private chat. It should be one of
/// - [`crate::types::MenuButtonCommands`]
/// - [`crate::types::MenuButtonWebApp`]
/// - [`crate::types::MenuButtonDefault`]
///
/// If a menu button other than [`crate::types::MenuButtonDefault`] is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubutton>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum MenuButtonType {
    #[strum(serialize = "commands")]
    Commands,
    #[strum(serialize = "web_app")]
    WebApp,
    #[strum(serialize = "default")]
    Default,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl MenuButtonType {
    #[must_use]
    pub const fn all() -> [MenuButtonType; 4usize] {
        [
            MenuButtonType::Commands,
            MenuButtonType::WebApp,
            MenuButtonType::Default,
            MenuButtonType::Unknown,
        ]
    }
}
impl From<MenuButtonType> for Box<str> {
    fn from(val: MenuButtonType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MenuButtonType> for String {
    fn from(val: MenuButtonType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MenuButtonType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MenuButton> for MenuButtonType {
    fn from(val: &'a MenuButton) -> Self {
        match val {
            MenuButton::Commands(_) => MenuButtonType::Commands,
            MenuButton::WebApp(_) => MenuButtonType::WebApp,
            MenuButton::Default(_) => MenuButtonType::Default,
            MenuButton::Unknown(_) => MenuButtonType::Unknown,
        }
    }
}
impl From<MenuButton> for MenuButtonType {
    fn from(val: MenuButton) -> Self {
        MenuButtonType::from(&val)
    }
}
/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// - [`crate::types::BotCommandScopeDefault`]
/// - [`crate::types::BotCommandScopeAllPrivateChats`]
/// - [`crate::types::BotCommandScopeAllGroupChats`]
/// - [`crate::types::BotCommandScopeAllChatAdministrators`]
/// - [`crate::types::BotCommandScopeChat`]
/// - [`crate::types::BotCommandScopeChatAdministrators`]
/// - [`crate::types::BotCommandScopeChatMember`]
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscope>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum BotCommandScopeType {
    #[strum(serialize = "default")]
    Default,
    #[strum(serialize = "all_private_chats")]
    AllPrivateChats,
    #[strum(serialize = "all_group_chats")]
    AllGroupChats,
    #[strum(serialize = "all_chat_administrators")]
    AllChatAdministrators,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "chat_administrators")]
    ChatAdministrators,
    #[strum(serialize = "chat_member")]
    ChatMember,
}
impl BotCommandScopeType {
    #[must_use]
    pub const fn all() -> [BotCommandScopeType; 7usize] {
        [
            BotCommandScopeType::Default,
            BotCommandScopeType::AllPrivateChats,
            BotCommandScopeType::AllGroupChats,
            BotCommandScopeType::AllChatAdministrators,
            BotCommandScopeType::Chat,
            BotCommandScopeType::ChatAdministrators,
            BotCommandScopeType::ChatMember,
        ]
    }
}
impl From<BotCommandScopeType> for Box<str> {
    fn from(val: BotCommandScopeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<BotCommandScopeType> for String {
    fn from(val: BotCommandScopeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for BotCommandScopeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a BotCommandScope> for BotCommandScopeType {
    fn from(val: &'a BotCommandScope) -> Self {
        match val {
            BotCommandScope::Default(_) => BotCommandScopeType::Default,
            BotCommandScope::AllPrivateChats(_) => BotCommandScopeType::AllPrivateChats,
            BotCommandScope::AllGroupChats(_) => BotCommandScopeType::AllGroupChats,
            BotCommandScope::AllChatAdministrators(_) => BotCommandScopeType::AllChatAdministrators,
            BotCommandScope::Chat(_) => BotCommandScopeType::Chat,
            BotCommandScope::ChatAdministrators(_) => BotCommandScopeType::ChatAdministrators,
            BotCommandScope::ChatMember(_) => BotCommandScopeType::ChatMember,
        }
    }
}
impl From<BotCommandScope> for BotCommandScopeType {
    fn from(val: BotCommandScope) -> Self {
        BotCommandScopeType::from(&val)
    }
}
/// Describes a transaction with a user.
/// Currently, it can be one of
/// - [`crate::types::TransactionPartnerUserBusinessAccountTransfer`]
/// - [`crate::types::TransactionPartnerUserGiftPurchase`]
/// - [`crate::types::TransactionPartnerUserInvoicePayment`]
/// - [`crate::types::TransactionPartnerUserPaidMediaPayment`]
/// - [`crate::types::TransactionPartnerUserPremiumPurchase`]
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum TransactionPartnerUserType {
    #[strum(serialize = "invoice_payment")]
    InvoicePayment,
    #[strum(serialize = "paid_media_payment")]
    PaidMediaPayment,
    #[strum(serialize = "gift_purchase")]
    GiftPurchase,
    #[strum(serialize = "premium_purchase")]
    PremiumPurchase,
    #[strum(serialize = "business_account_transfer")]
    BusinessAccountTransfer,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl TransactionPartnerUserType {
    #[must_use]
    pub const fn all() -> [TransactionPartnerUserType; 6usize] {
        [
            TransactionPartnerUserType::InvoicePayment,
            TransactionPartnerUserType::PaidMediaPayment,
            TransactionPartnerUserType::GiftPurchase,
            TransactionPartnerUserType::PremiumPurchase,
            TransactionPartnerUserType::BusinessAccountTransfer,
            TransactionPartnerUserType::Unknown,
        ]
    }
}
impl From<TransactionPartnerUserType> for Box<str> {
    fn from(val: TransactionPartnerUserType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<TransactionPartnerUserType> for String {
    fn from(val: TransactionPartnerUserType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for TransactionPartnerUserType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a TransactionPartnerUser> for TransactionPartnerUserType {
    fn from(val: &'a TransactionPartnerUser) -> Self {
        match val {
            TransactionPartnerUser::InvoicePayment(_) => TransactionPartnerUserType::InvoicePayment,
            TransactionPartnerUser::PaidMediaPayment(_) => {
                TransactionPartnerUserType::PaidMediaPayment
            }
            TransactionPartnerUser::GiftPurchase(_) => TransactionPartnerUserType::GiftPurchase,
            TransactionPartnerUser::PremiumPurchase(_) => {
                TransactionPartnerUserType::PremiumPurchase
            }
            TransactionPartnerUser::BusinessAccountTransfer(_) => {
                TransactionPartnerUserType::BusinessAccountTransfer
            }
            TransactionPartnerUser::Unknown(_) => TransactionPartnerUserType::Unknown,
        }
    }
}
impl From<TransactionPartnerUser> for TransactionPartnerUserType {
    fn from(val: TransactionPartnerUser) -> Self {
        TransactionPartnerUserType::from(&val)
    }
}
/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// - [`crate::types::PassportElementErrorDataField`]
/// - [`crate::types::PassportElementErrorFrontSide`]
/// - [`crate::types::PassportElementErrorReverseSide`]
/// - [`crate::types::PassportElementErrorSelfie`]
/// - [`crate::types::PassportElementErrorFile`]
/// - [`crate::types::PassportElementErrorFiles`]
/// - [`crate::types::PassportElementErrorTranslationFile`]
/// - [`crate::types::PassportElementErrorTranslationFiles`]
/// - [`crate::types::PassportElementErrorUnspecified`]
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerror>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum PassportElementErrorType {
    #[strum(serialize = "data")]
    Data,
    #[strum(serialize = "front_side")]
    FrontSide,
    #[strum(serialize = "reverse_side")]
    ReverseSide,
    #[strum(serialize = "selfie")]
    Selfie,
    #[strum(serialize = "file")]
    File,
    #[strum(serialize = "files")]
    Files,
    #[strum(serialize = "translation_file")]
    TranslationFile,
    #[strum(serialize = "translation_files")]
    TranslationFiles,
    #[strum(serialize = "unspecified")]
    Unspecified,
}
impl PassportElementErrorType {
    #[must_use]
    pub const fn all() -> [PassportElementErrorType; 9usize] {
        [
            PassportElementErrorType::Data,
            PassportElementErrorType::FrontSide,
            PassportElementErrorType::ReverseSide,
            PassportElementErrorType::Selfie,
            PassportElementErrorType::File,
            PassportElementErrorType::Files,
            PassportElementErrorType::TranslationFile,
            PassportElementErrorType::TranslationFiles,
            PassportElementErrorType::Unspecified,
        ]
    }
}
impl From<PassportElementErrorType> for Box<str> {
    fn from(val: PassportElementErrorType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PassportElementErrorType> for String {
    fn from(val: PassportElementErrorType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PassportElementErrorType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a PassportElementError> for PassportElementErrorType {
    fn from(val: &'a PassportElementError) -> Self {
        match val {
            PassportElementError::Data(_) => PassportElementErrorType::Data,
            PassportElementError::FrontSide(_) => PassportElementErrorType::FrontSide,
            PassportElementError::ReverseSide(_) => PassportElementErrorType::ReverseSide,
            PassportElementError::Selfie(_) => PassportElementErrorType::Selfie,
            PassportElementError::File(_) => PassportElementErrorType::File,
            PassportElementError::Files(_) => PassportElementErrorType::Files,
            PassportElementError::TranslationFile(_) => PassportElementErrorType::TranslationFile,
            PassportElementError::TranslationFiles(_) => PassportElementErrorType::TranslationFiles,
            PassportElementError::Unspecified(_) => PassportElementErrorType::Unspecified,
        }
    }
}
impl From<PassportElementError> for PassportElementErrorType {
    fn from(val: PassportElementError) -> Self {
        PassportElementErrorType::from(&val)
    }
}
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedPhoto`] and [`crate::types::InlineQueryResultPhoto`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultPhotoKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultPhotoKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultPhotoKindType; 2usize] {
        [
            InlineQueryResultPhotoKindType::Uncached,
            InlineQueryResultPhotoKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultPhotoKindType> for Box<str> {
    fn from(val: InlineQueryResultPhotoKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultPhotoKindType> for String {
    fn from(val: InlineQueryResultPhotoKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultPhotoKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultPhotoKind> for InlineQueryResultPhotoKindType {
    fn from(val: &'a InlineQueryResultPhotoKind) -> Self {
        match val {
            InlineQueryResultPhotoKind::Uncached(_) => InlineQueryResultPhotoKindType::Uncached,
            InlineQueryResultPhotoKind::Cached(_) => InlineQueryResultPhotoKindType::Cached,
        }
    }
}
impl From<InlineQueryResultPhotoKind> for InlineQueryResultPhotoKindType {
    fn from(val: InlineQueryResultPhotoKind) -> Self {
        InlineQueryResultPhotoKindType::from(&val)
    }
}
/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following types:
/// - [`crate::types::InputTextMessageContent`]
/// - [`crate::types::InputRichMessageContent`]
/// - [`crate::types::InputLocationMessageContent`]
/// - [`crate::types::InputVenueMessageContent`]
/// - [`crate::types::InputContactMessageContent`]
/// - [`crate::types::InputInvoiceMessageContent`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmessagecontent>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InputMessageContentType {
    #[strum(serialize = "input_invoice_message_content")]
    InputInvoiceMessageContent,
    #[strum(serialize = "input_venue_message_content")]
    InputVenueMessageContent,
    #[strum(serialize = "input_location_message_content")]
    InputLocationMessageContent,
    #[strum(serialize = "input_contact_message_content")]
    InputContactMessageContent,
    #[strum(serialize = "input_text_message_content")]
    InputTextMessageContent,
    #[strum(serialize = "input_rich_message_content")]
    InputRichMessageContent,
}
impl InputMessageContentType {
    #[must_use]
    pub const fn all() -> [InputMessageContentType; 6usize] {
        [
            InputMessageContentType::InputInvoiceMessageContent,
            InputMessageContentType::InputVenueMessageContent,
            InputMessageContentType::InputLocationMessageContent,
            InputMessageContentType::InputContactMessageContent,
            InputMessageContentType::InputTextMessageContent,
            InputMessageContentType::InputRichMessageContent,
        ]
    }
}
impl From<InputMessageContentType> for Box<str> {
    fn from(val: InputMessageContentType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputMessageContentType> for String {
    fn from(val: InputMessageContentType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputMessageContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputMessageContent> for InputMessageContentType {
    fn from(val: &'a InputMessageContent) -> Self {
        match val {
            InputMessageContent::InputInvoiceMessageContent(_) => {
                InputMessageContentType::InputInvoiceMessageContent
            }
            InputMessageContent::InputVenueMessageContent(_) => {
                InputMessageContentType::InputVenueMessageContent
            }
            InputMessageContent::InputLocationMessageContent(_) => {
                InputMessageContentType::InputLocationMessageContent
            }
            InputMessageContent::InputContactMessageContent(_) => {
                InputMessageContentType::InputContactMessageContent
            }
            InputMessageContent::InputTextMessageContent(_) => {
                InputMessageContentType::InputTextMessageContent
            }
            InputMessageContent::InputRichMessageContent(_) => {
                InputMessageContentType::InputRichMessageContent
            }
        }
    }
}
impl From<InputMessageContent> for InputMessageContentType {
    fn from(val: InputMessageContent) -> Self {
        InputMessageContentType::from(&val)
    }
}
/// At most one of the optional fields can be present in any given object.
/// Currently, it can be one of
/// - [`crate::types::PollMediaAnimation`]
/// - [`crate::types::PollMediaAudio`]
/// - [`crate::types::PollMediaDocument`]
/// - [`crate::types::PollMediaLink`]
/// - [`crate::types::PollMediaLivePhoto`]
/// - [`crate::types::PollMediaLocation`]
/// - [`crate::types::PollMediaPhoto`]
/// - [`crate::types::PollMediaSticker`]
/// - [`crate::types::PollMediaVenue`]
/// - [`crate::types::PollMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum PollMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "link")]
    Link,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl PollMediaType {
    #[must_use]
    pub const fn all() -> [PollMediaType; 11usize] {
        [
            PollMediaType::Animation,
            PollMediaType::LivePhoto,
            PollMediaType::Venue,
            PollMediaType::Audio,
            PollMediaType::Document,
            PollMediaType::Link,
            PollMediaType::Location,
            PollMediaType::Photo,
            PollMediaType::Sticker,
            PollMediaType::Video,
            PollMediaType::Unknown,
        ]
    }
}
impl From<PollMediaType> for Box<str> {
    fn from(val: PollMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PollMediaType> for String {
    fn from(val: PollMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PollMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a PollMedia> for PollMediaType {
    fn from(val: &'a PollMedia) -> Self {
        match val {
            PollMedia::Animation(_) => PollMediaType::Animation,
            PollMedia::LivePhoto(_) => PollMediaType::LivePhoto,
            PollMedia::Venue(_) => PollMediaType::Venue,
            PollMedia::Audio(_) => PollMediaType::Audio,
            PollMedia::Document(_) => PollMediaType::Document,
            PollMedia::Link(_) => PollMediaType::Link,
            PollMedia::Location(_) => PollMediaType::Location,
            PollMedia::Photo(_) => PollMediaType::Photo,
            PollMedia::Sticker(_) => PollMediaType::Sticker,
            PollMedia::Video(_) => PollMediaType::Video,
            PollMedia::Unknown(_) => PollMediaType::Unknown,
        }
    }
}
impl From<PollMedia> for PollMediaType {
    fn from(val: PollMedia) -> Self {
        PollMediaType::from(&val)
    }
}
/// This object represents a message about the completion of a giveaway with public winners.
/// Currently, it can be one of
/// - [`crate::types::GiveawayWinnersPremium`]
/// - [`crate::types::GiveawayWinnersStar`]
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaywinners>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum GiveawayWinnersType {
    #[strum(serialize = "premium")]
    Premium,
    #[strum(serialize = "star")]
    Star,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl GiveawayWinnersType {
    #[must_use]
    pub const fn all() -> [GiveawayWinnersType; 3usize] {
        [
            GiveawayWinnersType::Premium,
            GiveawayWinnersType::Star,
            GiveawayWinnersType::Unknown,
        ]
    }
}
impl From<GiveawayWinnersType> for Box<str> {
    fn from(val: GiveawayWinnersType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<GiveawayWinnersType> for String {
    fn from(val: GiveawayWinnersType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for GiveawayWinnersType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a GiveawayWinners> for GiveawayWinnersType {
    fn from(val: &'a GiveawayWinners) -> Self {
        match val {
            GiveawayWinners::Premium(_) => GiveawayWinnersType::Premium,
            GiveawayWinners::Star(_) => GiveawayWinnersType::Star,
            GiveawayWinners::Unknown(_) => GiveawayWinnersType::Unknown,
        }
    }
}
impl From<GiveawayWinners> for GiveawayWinnersType {
    fn from(val: GiveawayWinners) -> Self {
        GiveawayWinnersType::from(&val)
    }
}
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedDocument`] and [`crate::types::InlineQueryResultDocument`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultDocumentKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultDocumentKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultDocumentKindType; 2usize] {
        [
            InlineQueryResultDocumentKindType::Uncached,
            InlineQueryResultDocumentKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultDocumentKindType> for Box<str> {
    fn from(val: InlineQueryResultDocumentKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultDocumentKindType> for String {
    fn from(val: InlineQueryResultDocumentKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultDocumentKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultDocumentKind> for InlineQueryResultDocumentKindType {
    fn from(val: &'a InlineQueryResultDocumentKind) -> Self {
        match val {
            InlineQueryResultDocumentKind::Uncached(_) => {
                InlineQueryResultDocumentKindType::Uncached
            }
            InlineQueryResultDocumentKind::Cached(_) => InlineQueryResultDocumentKindType::Cached,
        }
    }
}
impl From<InlineQueryResultDocumentKind> for InlineQueryResultDocumentKindType {
    fn from(val: InlineQueryResultDocumentKind) -> Self {
        InlineQueryResultDocumentKindType::from(&val)
    }
}
/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
/// - [`crate::types::TransactionPartnerUser`]
/// - [`crate::types::TransactionPartnerChat`]
/// - [`crate::types::TransactionPartnerAffiliateProgram`]
/// - [`crate::types::TransactionPartnerFragment`]
/// - [`crate::types::TransactionPartnerTelegramAds`]
/// - [`crate::types::TransactionPartnerTelegramApi`]
/// - [`crate::types::TransactionPartnerOther`]
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartner>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum TransactionPartnerType {
    #[strum(serialize = "user")]
    User,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "affiliate_program")]
    AffiliateProgram,
    #[strum(serialize = "fragment")]
    Fragment,
    #[strum(serialize = "telegram_ads")]
    TelegramAds,
    #[strum(serialize = "telegram_api")]
    TelegramApi,
    #[strum(serialize = "other")]
    Other,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl TransactionPartnerType {
    #[must_use]
    pub const fn all() -> [TransactionPartnerType; 8usize] {
        [
            TransactionPartnerType::User,
            TransactionPartnerType::Chat,
            TransactionPartnerType::AffiliateProgram,
            TransactionPartnerType::Fragment,
            TransactionPartnerType::TelegramAds,
            TransactionPartnerType::TelegramApi,
            TransactionPartnerType::Other,
            TransactionPartnerType::Unknown,
        ]
    }
}
impl From<TransactionPartnerType> for Box<str> {
    fn from(val: TransactionPartnerType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<TransactionPartnerType> for String {
    fn from(val: TransactionPartnerType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for TransactionPartnerType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a TransactionPartner> for TransactionPartnerType {
    fn from(val: &'a TransactionPartner) -> Self {
        match val {
            TransactionPartner::User(_) => TransactionPartnerType::User,
            TransactionPartner::Chat(_) => TransactionPartnerType::Chat,
            TransactionPartner::AffiliateProgram(_) => TransactionPartnerType::AffiliateProgram,
            TransactionPartner::Fragment(_) => TransactionPartnerType::Fragment,
            TransactionPartner::TelegramAds(_) => TransactionPartnerType::TelegramAds,
            TransactionPartner::TelegramApi(_) => TransactionPartnerType::TelegramApi,
            TransactionPartner::Other(_) => TransactionPartnerType::Other,
            TransactionPartner::Unknown(_) => TransactionPartnerType::Unknown,
        }
    }
}
impl From<TransactionPartner> for TransactionPartnerType {
    fn from(val: TransactionPartner) -> Self {
        TransactionPartnerType::from(&val)
    }
}
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedVideo`] and [`crate::types::InlineQueryResultVideo`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultVideoKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultVideoKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultVideoKindType; 2usize] {
        [
            InlineQueryResultVideoKindType::Uncached,
            InlineQueryResultVideoKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultVideoKindType> for Box<str> {
    fn from(val: InlineQueryResultVideoKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultVideoKindType> for String {
    fn from(val: InlineQueryResultVideoKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultVideoKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultVideoKind> for InlineQueryResultVideoKindType {
    fn from(val: &'a InlineQueryResultVideoKind) -> Self {
        match val {
            InlineQueryResultVideoKind::Uncached(_) => InlineQueryResultVideoKindType::Uncached,
            InlineQueryResultVideoKind::Cached(_) => InlineQueryResultVideoKindType::Cached,
        }
    }
}
impl From<InlineQueryResultVideoKind> for InlineQueryResultVideoKindType {
    fn from(val: InlineQueryResultVideoKind) -> Self {
        InlineQueryResultVideoKindType::from(&val)
    }
}
/// This object describes the type of a background. Currently, it can be one of
/// - [`crate::types::BackgroundTypeFill`]
/// - [`crate::types::BackgroundTypeWallpaper`]
/// - [`crate::types::BackgroundTypePattern`]
/// - [`crate::types::BackgroundTypeChatTheme`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtype>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum BackgroundTypeType {
    #[strum(serialize = "fill")]
    Fill,
    #[strum(serialize = "wallpaper")]
    Wallpaper,
    #[strum(serialize = "pattern")]
    Pattern,
    #[strum(serialize = "chat_theme")]
    ChatTheme,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl BackgroundTypeType {
    #[must_use]
    pub const fn all() -> [BackgroundTypeType; 5usize] {
        [
            BackgroundTypeType::Fill,
            BackgroundTypeType::Wallpaper,
            BackgroundTypeType::Pattern,
            BackgroundTypeType::ChatTheme,
            BackgroundTypeType::Unknown,
        ]
    }
}
impl From<BackgroundTypeType> for Box<str> {
    fn from(val: BackgroundTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<BackgroundTypeType> for String {
    fn from(val: BackgroundTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for BackgroundTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a BackgroundType> for BackgroundTypeType {
    fn from(val: &'a BackgroundType) -> Self {
        match val {
            BackgroundType::Fill(_) => BackgroundTypeType::Fill,
            BackgroundType::Wallpaper(_) => BackgroundTypeType::Wallpaper,
            BackgroundType::Pattern(_) => BackgroundTypeType::Pattern,
            BackgroundType::ChatTheme(_) => BackgroundTypeType::ChatTheme,
            BackgroundType::Unknown(_) => BackgroundTypeType::Unknown,
        }
    }
}
impl From<BackgroundType> for BackgroundTypeType {
    fn from(val: BackgroundType) -> Self {
        BackgroundTypeType::from(&val)
    }
}
/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
/// - [`crate::types::RevenueWithdrawalStatePending`]
/// - [`crate::types::RevenueWithdrawalStateSucceeded`]
/// - [`crate::types::RevenueWithdrawalStateFailed`]
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstate>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum RevenueWithdrawalStateType {
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "succeeded")]
    Succeeded,
    #[strum(serialize = "failed")]
    Failed,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl RevenueWithdrawalStateType {
    #[must_use]
    pub const fn all() -> [RevenueWithdrawalStateType; 4usize] {
        [
            RevenueWithdrawalStateType::Pending,
            RevenueWithdrawalStateType::Succeeded,
            RevenueWithdrawalStateType::Failed,
            RevenueWithdrawalStateType::Unknown,
        ]
    }
}
impl From<RevenueWithdrawalStateType> for Box<str> {
    fn from(val: RevenueWithdrawalStateType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<RevenueWithdrawalStateType> for String {
    fn from(val: RevenueWithdrawalStateType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for RevenueWithdrawalStateType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a RevenueWithdrawalState> for RevenueWithdrawalStateType {
    fn from(val: &'a RevenueWithdrawalState) -> Self {
        match val {
            RevenueWithdrawalState::Pending(_) => RevenueWithdrawalStateType::Pending,
            RevenueWithdrawalState::Succeeded(_) => RevenueWithdrawalStateType::Succeeded,
            RevenueWithdrawalState::Failed(_) => RevenueWithdrawalStateType::Failed,
            RevenueWithdrawalState::Unknown(_) => RevenueWithdrawalStateType::Unknown,
        }
    }
}
impl From<RevenueWithdrawalState> for RevenueWithdrawalStateType {
    fn from(val: RevenueWithdrawalState) -> Self {
        RevenueWithdrawalStateType::from(&val)
    }
}
/// This object represents a rich formatted text. Currently, it can be either a String for plain text, an Array of [`crate::types::RichText`], or any of the following types:
/// - [`crate::types::RichTextBold`]
/// - [`crate::types::RichTextItalic`]
/// - [`crate::types::RichTextUnderline`]
/// - [`crate::types::RichTextStrikethrough`]
/// - [`crate::types::RichTextSpoiler`]
/// - [`crate::types::RichTextDateTime`]
/// - [`crate::types::RichTextTextMention`]
/// - [`crate::types::RichTextSubscript`]
/// - [`crate::types::RichTextSuperscript`]
/// - [`crate::types::RichTextMarked`]
/// - [`crate::types::RichTextCode`]
/// - [`crate::types::RichTextCustomEmoji`]
/// - [`crate::types::RichTextMathematicalExpression`]
/// - [`crate::types::RichTextUrl`]
/// - [`crate::types::RichTextEmailAddress`]
/// - [`crate::types::RichTextPhoneNumber`]
/// - [`crate::types::RichTextBankCardNumber`]
/// - [`crate::types::RichTextMention`]
/// - [`crate::types::RichTextHashtag`]
/// - [`crate::types::RichTextCashtag`]
/// - [`crate::types::RichTextBotCommand`]
/// - [`crate::types::RichTextButton`]
/// - [`crate::types::RichTextAnchor`]
/// - [`crate::types::RichTextAnchorLink`]
/// - [`crate::types::RichTextReference`]
/// - [`crate::types::RichTextReferenceLink`]
/// # Documentation
/// <https://core.telegram.org/bots/api#richtext>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum RichTextType {
    #[strum(serialize = "bold")]
    Bold,
    #[strum(serialize = "italic")]
    Italic,
    #[strum(serialize = "underline")]
    Underline,
    #[strum(serialize = "strikethrough")]
    Strikethrough,
    #[strum(serialize = "spoiler")]
    Spoiler,
    #[strum(serialize = "date_time")]
    DateTime,
    #[strum(serialize = "text_mention")]
    TextMention,
    #[strum(serialize = "subscript")]
    Subscript,
    #[strum(serialize = "superscript")]
    Superscript,
    #[strum(serialize = "marked")]
    Marked,
    #[strum(serialize = "code")]
    Code,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "mathematical_expression")]
    MathematicalExpression,
    #[strum(serialize = "url")]
    Url,
    #[strum(serialize = "email_address")]
    EmailAddress,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "bank_card_number")]
    BankCardNumber,
    #[strum(serialize = "mention")]
    Mention,
    #[strum(serialize = "hashtag")]
    Hashtag,
    #[strum(serialize = "cashtag")]
    Cashtag,
    #[strum(serialize = "bot_command")]
    BotCommand,
    #[strum(serialize = "button")]
    Button,
    #[strum(serialize = "anchor")]
    Anchor,
    #[strum(serialize = "anchor_link")]
    AnchorLink,
    #[strum(serialize = "reference")]
    Reference,
    #[strum(serialize = "reference_link")]
    ReferenceLink,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl RichTextType {
    #[must_use]
    pub const fn all() -> [RichTextType; 27usize] {
        [
            RichTextType::Bold,
            RichTextType::Italic,
            RichTextType::Underline,
            RichTextType::Strikethrough,
            RichTextType::Spoiler,
            RichTextType::DateTime,
            RichTextType::TextMention,
            RichTextType::Subscript,
            RichTextType::Superscript,
            RichTextType::Marked,
            RichTextType::Code,
            RichTextType::CustomEmoji,
            RichTextType::MathematicalExpression,
            RichTextType::Url,
            RichTextType::EmailAddress,
            RichTextType::PhoneNumber,
            RichTextType::BankCardNumber,
            RichTextType::Mention,
            RichTextType::Hashtag,
            RichTextType::Cashtag,
            RichTextType::BotCommand,
            RichTextType::Button,
            RichTextType::Anchor,
            RichTextType::AnchorLink,
            RichTextType::Reference,
            RichTextType::ReferenceLink,
            RichTextType::Unknown,
        ]
    }
}
impl From<RichTextType> for Box<str> {
    fn from(val: RichTextType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<RichTextType> for String {
    fn from(val: RichTextType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for RichTextType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> TryFrom<&'a RichText> for RichTextType {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: &'a RichText) -> Result<Self, Self::Error> {
        match val {
            RichText::Bold(_) => Ok(RichTextType::Bold),
            RichText::Italic(_) => Ok(RichTextType::Italic),
            RichText::Underline(_) => Ok(RichTextType::Underline),
            RichText::Strikethrough(_) => Ok(RichTextType::Strikethrough),
            RichText::Spoiler(_) => Ok(RichTextType::Spoiler),
            RichText::DateTime(_) => Ok(RichTextType::DateTime),
            RichText::TextMention(_) => Ok(RichTextType::TextMention),
            RichText::Subscript(_) => Ok(RichTextType::Subscript),
            RichText::Superscript(_) => Ok(RichTextType::Superscript),
            RichText::Marked(_) => Ok(RichTextType::Marked),
            RichText::Code(_) => Ok(RichTextType::Code),
            RichText::CustomEmoji(_) => Ok(RichTextType::CustomEmoji),
            RichText::MathematicalExpression(_) => Ok(RichTextType::MathematicalExpression),
            RichText::Url(_) => Ok(RichTextType::Url),
            RichText::EmailAddress(_) => Ok(RichTextType::EmailAddress),
            RichText::PhoneNumber(_) => Ok(RichTextType::PhoneNumber),
            RichText::BankCardNumber(_) => Ok(RichTextType::BankCardNumber),
            RichText::Mention(_) => Ok(RichTextType::Mention),
            RichText::Hashtag(_) => Ok(RichTextType::Hashtag),
            RichText::Cashtag(_) => Ok(RichTextType::Cashtag),
            RichText::BotCommand(_) => Ok(RichTextType::BotCommand),
            RichText::Button(_) => Ok(RichTextType::Button),
            RichText::Anchor(_) => Ok(RichTextType::Anchor),
            RichText::AnchorLink(_) => Ok(RichTextType::AnchorLink),
            RichText::Reference(_) => Ok(RichTextType::Reference),
            RichText::ReferenceLink(_) => Ok(RichTextType::ReferenceLink),
            RichText::Unknown(_) => Ok(RichTextType::Unknown),
            _ => Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextType),
            )),
        }
    }
}
impl TryFrom<RichText> for RichTextType {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        RichTextType::try_from(&val)
    }
}
/// This object represents a chat.
/// Currently, it can be one of
/// - [`crate::types::ChatChannel`]
/// - [`crate::types::ChatGroup`]
/// - [`crate::types::ChatPrivate`]
/// - [`crate::types::ChatSupergroup`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ChatType {
    #[strum(serialize = "private")]
    Private,
    #[strum(serialize = "group")]
    Group,
    #[strum(serialize = "supergroup")]
    Supergroup,
    #[strum(serialize = "channel")]
    Channel,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ChatType {
    #[must_use]
    pub const fn all() -> [ChatType; 5usize] {
        [
            ChatType::Private,
            ChatType::Group,
            ChatType::Supergroup,
            ChatType::Channel,
            ChatType::Unknown,
        ]
    }
}
impl From<ChatType> for Box<str> {
    fn from(val: ChatType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatType> for String {
    fn from(val: ChatType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Chat> for ChatType {
    fn from(val: &'a Chat) -> Self {
        match val {
            Chat::Private(_) => ChatType::Private,
            Chat::Group(_) => ChatType::Group,
            Chat::Supergroup(_) => ChatType::Supergroup,
            Chat::Channel(_) => ChatType::Channel,
            Chat::Unknown(_) => ChatType::Unknown,
        }
    }
}
impl From<Chat> for ChatType {
    fn from(val: Chat) -> Self {
        ChatType::from(&val)
    }
}
/// This object contains full information about a chat.
/// Currently, it can be one of
/// - [`crate::types::ChatFullInfoChannel`]
/// - [`crate::types::ChatFullInfoGroup`]
/// - [`crate::types::ChatFullInfoPrivate`]
/// - [`crate::types::ChatFullInfoSupergroup`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ChatFullInfoType {
    #[strum(serialize = "private")]
    Private,
    #[strum(serialize = "group")]
    Group,
    #[strum(serialize = "supergroup")]
    Supergroup,
    #[strum(serialize = "channel")]
    Channel,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ChatFullInfoType {
    #[must_use]
    pub const fn all() -> [ChatFullInfoType; 5usize] {
        [
            ChatFullInfoType::Private,
            ChatFullInfoType::Group,
            ChatFullInfoType::Supergroup,
            ChatFullInfoType::Channel,
            ChatFullInfoType::Unknown,
        ]
    }
}
impl From<ChatFullInfoType> for Box<str> {
    fn from(val: ChatFullInfoType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatFullInfoType> for String {
    fn from(val: ChatFullInfoType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatFullInfoType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ChatFullInfo> for ChatFullInfoType {
    fn from(val: &'a ChatFullInfo) -> Self {
        match val {
            ChatFullInfo::Private(_) => ChatFullInfoType::Private,
            ChatFullInfo::Group(_) => ChatFullInfoType::Group,
            ChatFullInfo::Supergroup(_) => ChatFullInfoType::Supergroup,
            ChatFullInfo::Channel(_) => ChatFullInfoType::Channel,
            ChatFullInfo::Unknown(_) => ChatFullInfoType::Unknown,
        }
    }
}
impl From<ChatFullInfo> for ChatFullInfoType {
    fn from(val: ChatFullInfo) -> Self {
        ChatFullInfoType::from(&val)
    }
}
/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// - [`crate::types::ChatMemberOwner`]
/// - [`crate::types::ChatMemberAdministrator`]
/// - [`crate::types::ChatMemberMember`]
/// - [`crate::types::ChatMemberRestricted`]
/// - [`crate::types::ChatMemberLeft`]
/// - [`crate::types::ChatMemberBanned`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ChatMemberType {
    #[strum(serialize = "creator")]
    Creator,
    #[strum(serialize = "administrator")]
    Administrator,
    #[strum(serialize = "member")]
    Member,
    #[strum(serialize = "restricted")]
    Restricted,
    #[strum(serialize = "left")]
    Left,
    #[strum(serialize = "kicked")]
    Kicked,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ChatMemberType {
    #[must_use]
    pub const fn all() -> [ChatMemberType; 7usize] {
        [
            ChatMemberType::Creator,
            ChatMemberType::Administrator,
            ChatMemberType::Member,
            ChatMemberType::Restricted,
            ChatMemberType::Left,
            ChatMemberType::Kicked,
            ChatMemberType::Unknown,
        ]
    }
}
impl From<ChatMemberType> for Box<str> {
    fn from(val: ChatMemberType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatMemberType> for String {
    fn from(val: ChatMemberType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatMemberType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ChatMember> for ChatMemberType {
    fn from(val: &'a ChatMember) -> Self {
        match val {
            ChatMember::Creator(_) => ChatMemberType::Creator,
            ChatMember::Administrator(_) => ChatMemberType::Administrator,
            ChatMember::Member(_) => ChatMemberType::Member,
            ChatMember::Restricted(_) => ChatMemberType::Restricted,
            ChatMember::Left(_) => ChatMemberType::Left,
            ChatMember::Kicked(_) => ChatMemberType::Kicked,
            ChatMember::Unknown(_) => ChatMemberType::Unknown,
        }
    }
}
impl From<ChatMember> for ChatMemberType {
    fn from(val: ChatMember) -> Self {
        ChatMemberType::from(&val)
    }
}
/// This object represents a message about a scheduled giveaway.
/// Currently, it can be one of
/// - [`crate::types::GiveawayPremium`]
/// - [`crate::types::GiveawayStar`]
/// # Documentation
/// <https://core.telegram.org/bots/api#giveaway>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum GiveawayType {
    #[strum(serialize = "premium")]
    Premium,
    #[strum(serialize = "star")]
    Star,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl GiveawayType {
    #[must_use]
    pub const fn all() -> [GiveawayType; 3usize] {
        [
            GiveawayType::Premium,
            GiveawayType::Star,
            GiveawayType::Unknown,
        ]
    }
}
impl From<GiveawayType> for Box<str> {
    fn from(val: GiveawayType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<GiveawayType> for String {
    fn from(val: GiveawayType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for GiveawayType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Giveaway> for GiveawayType {
    fn from(val: &'a Giveaway) -> Self {
        match val {
            Giveaway::Premium(_) => GiveawayType::Premium,
            Giveaway::Star(_) => GiveawayType::Star,
            Giveaway::Unknown(_) => GiveawayType::Unknown,
        }
    }
}
impl From<Giveaway> for GiveawayType {
    fn from(val: Giveaway) -> Self {
        GiveawayType::from(&val)
    }
}
/// This object describes a gift received and owned by a user or a chat. Currently, it can be one of
/// - [`crate::types::OwnedGiftRegular`]
/// - [`crate::types::OwnedGiftUnique`]
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgift>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum OwnedGiftType {
    #[strum(serialize = "regular")]
    Regular,
    #[strum(serialize = "unique")]
    Unique,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl OwnedGiftType {
    #[must_use]
    pub const fn all() -> [OwnedGiftType; 3usize] {
        [
            OwnedGiftType::Regular,
            OwnedGiftType::Unique,
            OwnedGiftType::Unknown,
        ]
    }
}
impl From<OwnedGiftType> for Box<str> {
    fn from(val: OwnedGiftType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<OwnedGiftType> for String {
    fn from(val: OwnedGiftType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for OwnedGiftType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a OwnedGift> for OwnedGiftType {
    fn from(val: &'a OwnedGift) -> Self {
        match val {
            OwnedGift::Regular(_) => OwnedGiftType::Regular,
            OwnedGift::Unique(_) => OwnedGiftType::Unique,
            OwnedGift::Unknown(_) => OwnedGiftType::Unknown,
        }
    }
}
impl From<OwnedGift> for OwnedGiftType {
    fn from(val: OwnedGift) -> Self {
        OwnedGiftType::from(&val)
    }
}
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
/// Currently, it can be one of
/// - [`crate::types::ExternalReplyInfoAnimation`]
/// - [`crate::types::ExternalReplyInfoAudio`]
/// - [`crate::types::ExternalReplyInfoChecklist`]
/// - [`crate::types::ExternalReplyInfoContact`]
/// - [`crate::types::ExternalReplyInfoDice`]
/// - [`crate::types::ExternalReplyInfoDocument`]
/// - [`crate::types::ExternalReplyInfoGame`]
/// - [`crate::types::ExternalReplyInfoGiveaway`]
/// - [`crate::types::ExternalReplyInfoGiveawayWinners`]
/// - [`crate::types::ExternalReplyInfoInvoice`]
/// - [`crate::types::ExternalReplyInfoLivePhoto`]
/// - [`crate::types::ExternalReplyInfoLocation`]
/// - [`crate::types::ExternalReplyInfoPaidMedia`]
/// - [`crate::types::ExternalReplyInfoPhoto`]
/// - [`crate::types::ExternalReplyInfoPoll`]
/// - [`crate::types::ExternalReplyInfoSticker`]
/// - [`crate::types::ExternalReplyInfoStory`]
/// - [`crate::types::ExternalReplyInfoUnknown`]
/// - [`crate::types::ExternalReplyInfoVenue`]
/// - [`crate::types::ExternalReplyInfoVideo`]
/// - [`crate::types::ExternalReplyInfoVideoNote`]
/// - [`crate::types::ExternalReplyInfoVoice`]
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ExternalReplyInfoType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "checklist")]
    Checklist,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "dice")]
    Dice,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "giveaway")]
    Giveaway,
    #[strum(serialize = "giveaway_winners")]
    GiveawayWinners,
    #[strum(serialize = "invoice")]
    Invoice,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "paid_media")]
    PaidMedia,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "story")]
    Story,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "video_note")]
    VideoNote,
    #[strum(serialize = "voice")]
    Voice,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ExternalReplyInfoType {
    #[must_use]
    pub const fn all() -> [ExternalReplyInfoType; 22usize] {
        [
            ExternalReplyInfoType::Animation,
            ExternalReplyInfoType::LivePhoto,
            ExternalReplyInfoType::Venue,
            ExternalReplyInfoType::Audio,
            ExternalReplyInfoType::Checklist,
            ExternalReplyInfoType::Contact,
            ExternalReplyInfoType::Dice,
            ExternalReplyInfoType::Document,
            ExternalReplyInfoType::Game,
            ExternalReplyInfoType::Giveaway,
            ExternalReplyInfoType::GiveawayWinners,
            ExternalReplyInfoType::Invoice,
            ExternalReplyInfoType::Location,
            ExternalReplyInfoType::PaidMedia,
            ExternalReplyInfoType::Photo,
            ExternalReplyInfoType::Poll,
            ExternalReplyInfoType::Sticker,
            ExternalReplyInfoType::Story,
            ExternalReplyInfoType::Video,
            ExternalReplyInfoType::VideoNote,
            ExternalReplyInfoType::Voice,
            ExternalReplyInfoType::Unknown,
        ]
    }
}
impl From<ExternalReplyInfoType> for Box<str> {
    fn from(val: ExternalReplyInfoType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ExternalReplyInfoType> for String {
    fn from(val: ExternalReplyInfoType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ExternalReplyInfoType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ExternalReplyInfo> for ExternalReplyInfoType {
    fn from(val: &'a ExternalReplyInfo) -> Self {
        match val {
            ExternalReplyInfo::Animation(_) => ExternalReplyInfoType::Animation,
            ExternalReplyInfo::LivePhoto(_) => ExternalReplyInfoType::LivePhoto,
            ExternalReplyInfo::Venue(_) => ExternalReplyInfoType::Venue,
            ExternalReplyInfo::Audio(_) => ExternalReplyInfoType::Audio,
            ExternalReplyInfo::Checklist(_) => ExternalReplyInfoType::Checklist,
            ExternalReplyInfo::Contact(_) => ExternalReplyInfoType::Contact,
            ExternalReplyInfo::Dice(_) => ExternalReplyInfoType::Dice,
            ExternalReplyInfo::Document(_) => ExternalReplyInfoType::Document,
            ExternalReplyInfo::Game(_) => ExternalReplyInfoType::Game,
            ExternalReplyInfo::Giveaway(_) => ExternalReplyInfoType::Giveaway,
            ExternalReplyInfo::GiveawayWinners(_) => ExternalReplyInfoType::GiveawayWinners,
            ExternalReplyInfo::Invoice(_) => ExternalReplyInfoType::Invoice,
            ExternalReplyInfo::Location(_) => ExternalReplyInfoType::Location,
            ExternalReplyInfo::PaidMedia(_) => ExternalReplyInfoType::PaidMedia,
            ExternalReplyInfo::Photo(_) => ExternalReplyInfoType::Photo,
            ExternalReplyInfo::Poll(_) => ExternalReplyInfoType::Poll,
            ExternalReplyInfo::Sticker(_) => ExternalReplyInfoType::Sticker,
            ExternalReplyInfo::Story(_) => ExternalReplyInfoType::Story,
            ExternalReplyInfo::Video(_) => ExternalReplyInfoType::Video,
            ExternalReplyInfo::VideoNote(_) => ExternalReplyInfoType::VideoNote,
            ExternalReplyInfo::Voice(_) => ExternalReplyInfoType::Voice,
            ExternalReplyInfo::Unknown(_) => ExternalReplyInfoType::Unknown,
        }
    }
}
impl From<ExternalReplyInfo> for ExternalReplyInfoType {
    fn from(val: ExternalReplyInfo) -> Self {
        ExternalReplyInfoType::from(&val)
    }
}
/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
/// - [`crate::types::BackgroundFillSolid`]
/// - [`crate::types::BackgroundFillGradient`]
/// - [`crate::types::BackgroundFillFreeformGradient`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfill>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum BackgroundFillType {
    #[strum(serialize = "solid")]
    Solid,
    #[strum(serialize = "gradient")]
    Gradient,
    #[strum(serialize = "freeform_gradient")]
    FreeformGradient,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl BackgroundFillType {
    #[must_use]
    pub const fn all() -> [BackgroundFillType; 4usize] {
        [
            BackgroundFillType::Solid,
            BackgroundFillType::Gradient,
            BackgroundFillType::FreeformGradient,
            BackgroundFillType::Unknown,
        ]
    }
}
impl From<BackgroundFillType> for Box<str> {
    fn from(val: BackgroundFillType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<BackgroundFillType> for String {
    fn from(val: BackgroundFillType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for BackgroundFillType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a BackgroundFill> for BackgroundFillType {
    fn from(val: &'a BackgroundFill) -> Self {
        match val {
            BackgroundFill::Solid(_) => BackgroundFillType::Solid,
            BackgroundFill::Gradient(_) => BackgroundFillType::Gradient,
            BackgroundFill::FreeformGradient(_) => BackgroundFillType::FreeformGradient,
            BackgroundFill::Unknown(_) => BackgroundFillType::Unknown,
        }
    }
}
impl From<BackgroundFill> for BackgroundFillType {
    fn from(val: BackgroundFill) -> Self {
        BackgroundFillType::from(&val)
    }
}
/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// - [`crate::types::InlineQueryResultCachedAudio`]
/// - [`crate::types::InlineQueryResultCachedDocument`]
/// - [`crate::types::InlineQueryResultCachedGif`]
/// - [`crate::types::InlineQueryResultCachedMpeg4Gif`]
/// - [`crate::types::InlineQueryResultCachedPhoto`]
/// - [`crate::types::InlineQueryResultCachedSticker`]
/// - [`crate::types::InlineQueryResultCachedVideo`]
/// - [`crate::types::InlineQueryResultCachedVoice`]
/// - [`crate::types::InlineQueryResultArticle`]
/// - [`crate::types::InlineQueryResultAudio`]
/// - [`crate::types::InlineQueryResultContact`]
/// - [`crate::types::InlineQueryResultGame`]
/// - [`crate::types::InlineQueryResultDocument`]
/// - [`crate::types::InlineQueryResultGif`]
/// - [`crate::types::InlineQueryResultLocation`]
/// - [`crate::types::InlineQueryResultMpeg4Gif`]
/// - [`crate::types::InlineQueryResultPhoto`]
/// - [`crate::types::InlineQueryResultVenue`]
/// - [`crate::types::InlineQueryResultVideo`]
/// - [`crate::types::InlineQueryResultVoice`]
///
/// Note: All URLs passed in inline query results will be available to end users and therefore must be assumed to be public.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum InlineQueryResultType {
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "gif")]
    Gif,
    #[strum(serialize = "mpeg4_gif")]
    Mpeg4Gif,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice")]
    Voice,
    #[strum(serialize = "article")]
    Article,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "venue")]
    Venue,
}
impl InlineQueryResultType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultType; 13usize] {
        [
            InlineQueryResultType::Audio,
            InlineQueryResultType::Document,
            InlineQueryResultType::Gif,
            InlineQueryResultType::Mpeg4Gif,
            InlineQueryResultType::Photo,
            InlineQueryResultType::Sticker,
            InlineQueryResultType::Video,
            InlineQueryResultType::Voice,
            InlineQueryResultType::Article,
            InlineQueryResultType::Contact,
            InlineQueryResultType::Game,
            InlineQueryResultType::Location,
            InlineQueryResultType::Venue,
        ]
    }
}
impl From<InlineQueryResultType> for Box<str> {
    fn from(val: InlineQueryResultType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultType> for String {
    fn from(val: InlineQueryResultType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResult> for InlineQueryResultType {
    fn from(val: &'a InlineQueryResult) -> Self {
        match val {
            InlineQueryResult::Audio(_) => InlineQueryResultType::Audio,
            InlineQueryResult::Document(_) => InlineQueryResultType::Document,
            InlineQueryResult::Gif(_) => InlineQueryResultType::Gif,
            InlineQueryResult::Mpeg4Gif(_) => InlineQueryResultType::Mpeg4Gif,
            InlineQueryResult::Photo(_) => InlineQueryResultType::Photo,
            InlineQueryResult::Sticker(_) => InlineQueryResultType::Sticker,
            InlineQueryResult::Video(_) => InlineQueryResultType::Video,
            InlineQueryResult::Voice(_) => InlineQueryResultType::Voice,
            InlineQueryResult::Article(_) => InlineQueryResultType::Article,
            InlineQueryResult::Contact(_) => InlineQueryResultType::Contact,
            InlineQueryResult::Game(_) => InlineQueryResultType::Game,
            InlineQueryResult::Location(_) => InlineQueryResultType::Location,
            InlineQueryResult::Venue(_) => InlineQueryResultType::Venue,
        }
    }
}
impl From<InlineQueryResult> for InlineQueryResultType {
    fn from(val: InlineQueryResult) -> Self {
        InlineQueryResultType::from(&val)
    }
}
/// This object describes paid media. Currently, it can be one of
/// - [`crate::types::PaidMediaLivePhoto`]
/// - [`crate::types::PaidMediaPhoto`]
/// - [`crate::types::PaidMediaPreview`]
/// - [`crate::types::PaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum PaidMediaType {
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "preview")]
    Preview,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl PaidMediaType {
    #[must_use]
    pub const fn all() -> [PaidMediaType; 5usize] {
        [
            PaidMediaType::LivePhoto,
            PaidMediaType::Photo,
            PaidMediaType::Preview,
            PaidMediaType::Video,
            PaidMediaType::Unknown,
        ]
    }
}
impl From<PaidMediaType> for Box<str> {
    fn from(val: PaidMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PaidMediaType> for String {
    fn from(val: PaidMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PaidMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a PaidMedia> for PaidMediaType {
    fn from(val: &'a PaidMedia) -> Self {
        match val {
            PaidMedia::LivePhoto(_) => PaidMediaType::LivePhoto,
            PaidMedia::Photo(_) => PaidMediaType::Photo,
            PaidMedia::Preview(_) => PaidMediaType::Preview,
            PaidMedia::Video(_) => PaidMediaType::Video,
            PaidMedia::Unknown(_) => PaidMediaType::Unknown,
        }
    }
}
impl From<PaidMedia> for PaidMediaType {
    fn from(val: PaidMedia) -> Self {
        PaidMediaType::from(&val)
    }
}
/// This enum represents all possible types of the parse mode
/// # Documentation
/// <https://core.telegram.org/bots/api#formatting-options>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ParseMode {
    #[strum(serialize = "Markdown")]
    Markdown,
    #[strum(serialize = "MarkdownV2")]
    MarkdownV2,
    #[strum(serialize = "HTML")]
    HTML,
}
impl ParseMode {
    #[must_use]
    pub const fn all() -> [ParseMode; 3usize] {
        [ParseMode::Markdown, ParseMode::MarkdownV2, ParseMode::HTML]
    }
}
impl From<ParseMode> for Box<str> {
    fn from(parse_mode: ParseMode) -> Self {
        Into::<&'static str>::into(parse_mode).into()
    }
}
impl From<ParseMode> for String {
    fn from(parse_mode: ParseMode) -> Self {
        parse_mode.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ParseMode {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref().to_lowercase() == other.to_lowercase()
    }
}
/// This enum represents all possible telegram observer types.
/// It contains all [`UpdateType`] variants plus `Update` and `Error`.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum TelegramObserverType {
    #[strum(serialize = "business_connection")]
    BusinessConnection,
    #[strum(serialize = "business_message")]
    BusinessMessage,
    #[strum(serialize = "callback_query")]
    CallbackQuery,
    #[strum(serialize = "channel_post")]
    ChannelPost,
    #[strum(serialize = "chat_boost")]
    ChatBoost,
    #[strum(serialize = "chat_join_request")]
    ChatJoinRequest,
    #[strum(serialize = "chat_member")]
    ChatMember,
    #[strum(serialize = "chosen_inline_result")]
    ChosenInlineResult,
    #[strum(serialize = "deleted_business_messages")]
    DeletedBusinessMessages,
    #[strum(serialize = "edited_business_message")]
    EditedBusinessMessage,
    #[strum(serialize = "edited_channel_post")]
    EditedChannelPost,
    #[strum(serialize = "edited_message")]
    EditedMessage,
    #[strum(serialize = "guest_message")]
    GuestMessage,
    #[strum(serialize = "inline_query")]
    InlineQuery,
    #[strum(serialize = "managed_bot")]
    ManagedBot,
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "message_reaction")]
    MessageReaction,
    #[strum(serialize = "message_reaction_count")]
    MessageReactionCount,
    #[strum(serialize = "my_chat_member")]
    MyChatMember,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "poll_answer")]
    PollAnswer,
    #[strum(serialize = "pre_checkout_query")]
    PreCheckoutQuery,
    #[strum(serialize = "purchased_paid_media")]
    PurchasedPaidMedia,
    #[strum(serialize = "removed_chat_boost")]
    RemovedChatBoost,
    #[strum(serialize = "shipping_query")]
    ShippingQuery,
    #[strum(serialize = "stopped_message_generation")]
    StoppedMessageGeneration,
    #[strum(serialize = "subscription")]
    Subscription,
    #[strum(serialize = "update")]
    Update,
    #[strum(serialize = "error")]
    Error,
}
macro_rules! with_telegram_observer_variants {
    ($callback:ident $(, $args:tt)*) => {
        $callback ! { $($args,)* (BusinessConnection, business_connection),
        (BusinessMessage, business_message), (CallbackQuery, callback_query),
        (ChannelPost, channel_post), (ChatBoost, chat_boost), (ChatJoinRequest,
        chat_join_request), (ChatMember, chat_member), (ChosenInlineResult,
        chosen_inline_result), (DeletedBusinessMessages, deleted_business_messages),
        (EditedBusinessMessage, edited_business_message), (EditedChannelPost,
        edited_channel_post), (EditedMessage, edited_message), (GuestMessage,
        guest_message), (InlineQuery, inline_query), (ManagedBot, managed_bot), (Message,
        message), (MessageReaction, message_reaction), (MessageReactionCount,
        message_reaction_count), (MyChatMember, my_chat_member), (Poll, poll),
        (PollAnswer, poll_answer), (PreCheckoutQuery, pre_checkout_query),
        (PurchasedPaidMedia, purchased_paid_media), (RemovedChatBoost,
        removed_chat_boost), (ShippingQuery, shipping_query), (StoppedMessageGeneration,
        stopped_message_generation), (Subscription, subscription), (Update, update),
        (Error, error), }
    };
}
pub(crate) use with_telegram_observer_variants;
impl TelegramObserverType {
    #[must_use]
    pub const fn all() -> [TelegramObserverType; 29usize] {
        [
            TelegramObserverType::BusinessConnection,
            TelegramObserverType::BusinessMessage,
            TelegramObserverType::CallbackQuery,
            TelegramObserverType::ChannelPost,
            TelegramObserverType::ChatBoost,
            TelegramObserverType::ChatJoinRequest,
            TelegramObserverType::ChatMember,
            TelegramObserverType::ChosenInlineResult,
            TelegramObserverType::DeletedBusinessMessages,
            TelegramObserverType::EditedBusinessMessage,
            TelegramObserverType::EditedChannelPost,
            TelegramObserverType::EditedMessage,
            TelegramObserverType::GuestMessage,
            TelegramObserverType::InlineQuery,
            TelegramObserverType::ManagedBot,
            TelegramObserverType::Message,
            TelegramObserverType::MessageReaction,
            TelegramObserverType::MessageReactionCount,
            TelegramObserverType::MyChatMember,
            TelegramObserverType::Poll,
            TelegramObserverType::PollAnswer,
            TelegramObserverType::PreCheckoutQuery,
            TelegramObserverType::PurchasedPaidMedia,
            TelegramObserverType::RemovedChatBoost,
            TelegramObserverType::ShippingQuery,
            TelegramObserverType::StoppedMessageGeneration,
            TelegramObserverType::Subscription,
            TelegramObserverType::Update,
            TelegramObserverType::Error,
        ]
    }
}
impl From<TelegramObserverType> for Box<str> {
    fn from(val: TelegramObserverType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<TelegramObserverType> for String {
    fn from(val: TelegramObserverType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for TelegramObserverType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl From<UpdateType> for TelegramObserverType {
    fn from(val: UpdateType) -> Self {
        match val {
            UpdateType::BusinessConnection => TelegramObserverType::BusinessConnection,
            UpdateType::BusinessMessage => TelegramObserverType::BusinessMessage,
            UpdateType::CallbackQuery => TelegramObserverType::CallbackQuery,
            UpdateType::ChannelPost => TelegramObserverType::ChannelPost,
            UpdateType::ChatBoost => TelegramObserverType::ChatBoost,
            UpdateType::ChatJoinRequest => TelegramObserverType::ChatJoinRequest,
            UpdateType::ChatMember => TelegramObserverType::ChatMember,
            UpdateType::ChosenInlineResult => TelegramObserverType::ChosenInlineResult,
            UpdateType::DeletedBusinessMessages => TelegramObserverType::DeletedBusinessMessages,
            UpdateType::EditedBusinessMessage => TelegramObserverType::EditedBusinessMessage,
            UpdateType::EditedChannelPost => TelegramObserverType::EditedChannelPost,
            UpdateType::EditedMessage => TelegramObserverType::EditedMessage,
            UpdateType::GuestMessage => TelegramObserverType::GuestMessage,
            UpdateType::InlineQuery => TelegramObserverType::InlineQuery,
            UpdateType::ManagedBot => TelegramObserverType::ManagedBot,
            UpdateType::Message => TelegramObserverType::Message,
            UpdateType::MessageReaction => TelegramObserverType::MessageReaction,
            UpdateType::MessageReactionCount => TelegramObserverType::MessageReactionCount,
            UpdateType::MyChatMember => TelegramObserverType::MyChatMember,
            UpdateType::Poll => TelegramObserverType::Poll,
            UpdateType::PollAnswer => TelegramObserverType::PollAnswer,
            UpdateType::PreCheckoutQuery => TelegramObserverType::PreCheckoutQuery,
            UpdateType::PurchasedPaidMedia => TelegramObserverType::PurchasedPaidMedia,
            UpdateType::RemovedChatBoost => TelegramObserverType::RemovedChatBoost,
            UpdateType::ShippingQuery => TelegramObserverType::ShippingQuery,
            UpdateType::StoppedMessageGeneration => TelegramObserverType::StoppedMessageGeneration,
            UpdateType::Subscription => TelegramObserverType::Subscription,
        }
    }
}
impl<'a> From<&'a UpdateType> for TelegramObserverType {
    fn from(val: &'a UpdateType) -> Self {
        TelegramObserverType::from(*val)
    }
}
impl<'a> From<&'a Update> for TelegramObserverType {
    fn from(val: &'a Update) -> Self {
        TelegramObserverType::from(UpdateType::from(val))
    }
}
impl From<Update> for TelegramObserverType {
    fn from(val: Update) -> Self {
        TelegramObserverType::from(&val)
    }
}
