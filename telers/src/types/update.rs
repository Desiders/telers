use super::{
    BusinessConnection, BusinessMessagesDeleted, CallbackQuery, Chat, ChatBoostRemoved,
    ChatBoostSource, ChatBoostSourcePremium, ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated,
    ChosenInlineResult, InaccessibleMessage, InlineQuery, MaybeInaccessibleMessage, Message,
    MessageReactionCountUpdated, MessageReactionUpdated, PaidMediaPurchased, Poll, PollAnswer,
    PreCheckoutQuery, ShippingQuery, User,
};

use crate::{enums::UpdateType, extractors::FromEvent};

use serde::{de::MapAccess, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Formatter},
    str::FromStr as _,
};

/// This [`object`](https://core.telegram.org/bots/api#available-types) represents an incoming update.
/// At most **one** of the optional parameters can be present in any given update.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Update {
    #[serde(rename = "update_id")]
    pub id: i64,

    #[serde(flatten)]
    pub kind: Kind,
}

#[derive(Debug, Clone, PartialEq, FromEvent)]
#[event(from = Update)]
pub enum Kind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(Message),
    /// The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    BusinessConnection(BusinessConnection),
    /// New non-service message from a connected business account
    BusinessMessage(Message),
    /// New version of a message from a connected business account
    EditedBusinessMessage(Message),
    /// Messages were deleted from a connected business account
    DeletedBusinessMessages(BusinessMessagesDeleted),
    /// A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `message_reaction` in the list of `allowed_updates` to receive these updates. The update isn't received for reactions set by bots.
    MessageReaction(MessageReactionUpdated),
    /// Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `message_reaction_count` in the list of `allowed_updates` to receive these updates.
    MessageReactionCount(MessageReactionCountUpdated),
    /// New incoming inline query
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the [`feedback collecting`](https://core.telegram.org/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query. Only for invoices with flexible price
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query. Contains full information about checkout
    PreCheckoutQuery(PreCheckoutQuery),
    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    PurchasedPaidMedia(PaidMediaPurchased),
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    PollAnswer(PollAnswer),
    /// New incoming my chat member update.
    MyChatMember(ChatMemberUpdated),
    /// New incoming chat member update.
    ChatMember(ChatMemberUpdated),
    /// A request to join the chat has been sent. The bot must have the `can_invite_users` administrator right in the chat to receive these updates.
    ChatJoinRequest(ChatJoinRequest),
    /// A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    ChatBoost(ChatBoostUpdated),
    /// A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    RemovedChatBoost(ChatBoostRemoved),
}

impl Kind {
    #[must_use]
    pub const fn text(&self) -> Option<&str> {
        match self {
            Kind::Message(message)
            | Kind::EditedMessage(message)
            | Kind::BusinessMessage(message)
            | Kind::EditedBusinessMessage(message)
            | Kind::ChannelPost(message)
            | Kind::EditedChannelPost(message) => message.text(),
            Kind::InlineQuery(InlineQuery { query, .. })
            | Kind::ChosenInlineResult(ChosenInlineResult { query, .. }) => Some(query),
            Kind::CallbackQuery(CallbackQuery {
                data: Some(data), ..
            }) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub const fn caption(&self) -> Option<&str> {
        match self {
            Kind::Message(message)
            | Kind::EditedMessage(message)
            | Kind::BusinessMessage(message)
            | Kind::EditedBusinessMessage(message)
            | Kind::ChannelPost(message)
            | Kind::EditedChannelPost(message)
            | Kind::CallbackQuery(CallbackQuery {
                message: Some(MaybeInaccessibleMessage::Message(message)),
                ..
            }) => message.caption(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn text_or_caption(&self) -> Option<&str> {
        match self.text() {
            Some(text) => Some(text),
            None => self.caption(),
        }
    }

    #[must_use]
    pub const fn from(&self) -> Option<&User> {
        match self {
            Kind::Message(message)
            | Kind::EditedMessage(message)
            | Kind::BusinessMessage(message)
            | Kind::EditedBusinessMessage(message) => message.from(),
            Kind::InlineQuery(InlineQuery { from, .. })
            | Kind::ChosenInlineResult(ChosenInlineResult { from, .. })
            | Kind::CallbackQuery(CallbackQuery { from, .. })
            | Kind::ShippingQuery(ShippingQuery { from, .. })
            | Kind::PreCheckoutQuery(PreCheckoutQuery { from, .. })
            | Kind::PurchasedPaidMedia(PaidMediaPurchased { from, .. })
            | Kind::MyChatMember(ChatMemberUpdated { from, .. })
            | Kind::ChatMember(ChatMemberUpdated { from, .. })
            | Kind::ChatJoinRequest(ChatJoinRequest { from, .. }) => Some(from),
            Kind::BusinessConnection(BusinessConnection { user, .. })
            | Kind::ChatBoost(ChatBoostUpdated {
                boost: ChatBoostSource::Premium(ChatBoostSourcePremium { user }),
                ..
            }) => Some(user),
            Kind::PollAnswer(PollAnswer { user, .. })
            | Kind::MessageReaction(MessageReactionUpdated { user, .. }) => user.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn from_id(&self) -> Option<i64> {
        match self.from() {
            Some(user) => Some(user.id),
            None => None,
        }
    }

    #[must_use]
    pub const fn chat(&self) -> Option<&Chat> {
        match self {
            Kind::Message(message)
            | Kind::EditedMessage(message)
            | Kind::BusinessMessage(message)
            | Kind::EditedBusinessMessage(message)
            | Kind::ChannelPost(message)
            | Kind::EditedChannelPost(message) => Some(message.chat()),
            Kind::CallbackQuery(CallbackQuery {
                message: Some(message),
                ..
            }) => match message {
                MaybeInaccessibleMessage::Message(message) => Some(message.chat()),
                MaybeInaccessibleMessage::InaccessibleMessage(InaccessibleMessage {
                    chat, ..
                }) => Some(chat),
            },
            Kind::MyChatMember(ChatMemberUpdated { chat, .. })
            | Kind::ChatMember(ChatMemberUpdated { chat, .. })
            | Kind::ChatJoinRequest(ChatJoinRequest { chat, .. })
            | Kind::MessageReactionCount(MessageReactionCountUpdated { chat, .. })
            | Kind::ChatBoost(ChatBoostUpdated { chat, .. })
            | Kind::RemovedChatBoost(ChatBoostRemoved { chat, .. })
            | Kind::DeletedBusinessMessages(BusinessMessagesDeleted { chat, .. }) => Some(chat),
            Kind::PollAnswer(PollAnswer { voter_chat, .. }) => voter_chat.as_ref(),
            Kind::MessageReaction(MessageReactionUpdated { actor_chat, .. }) => actor_chat.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn chat_id(&self) -> Option<i64> {
        if let Kind::BusinessConnection(BusinessConnection { user_chat_id, .. }) = self {
            return Some(*user_chat_id);
        }

        if let Some(chat) = self.chat() {
            Some(chat.id())
        } else {
            None
        }
    }

    #[must_use]
    pub const fn sender_chat(&self) -> Option<&Chat> {
        match self {
            Kind::Message(message)
            | Kind::EditedMessage(message)
            | Kind::BusinessMessage(message)
            | Kind::EditedBusinessMessage(message)
            | Kind::ChannelPost(message)
            | Kind::EditedChannelPost(message)
            | Kind::CallbackQuery(CallbackQuery {
                message: Some(MaybeInaccessibleMessage::Message(message)),
                ..
            }) => message.sender_chat(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn sender_chat_id(&self) -> Option<i64> {
        if let Some(chat) = self.sender_chat() {
            Some(chat.id())
        } else {
            None
        }
    }

    #[must_use]
    pub const fn message_thread_id(&self) -> Option<i64> {
        match self {
            Kind::Message(message)
            | Kind::EditedMessage(message)
            | Kind::BusinessMessage(message)
            | Kind::EditedBusinessMessage(message)
            | Kind::ChannelPost(message)
            | Kind::EditedChannelPost(message)
            | Kind::CallbackQuery(CallbackQuery {
                message: Some(MaybeInaccessibleMessage::Message(message)),
                ..
            }) => message.thread_id(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn business_connection_id(&self) -> Option<&str> {
        match self {
            Kind::DeletedBusinessMessages(BusinessMessagesDeleted {
                business_connection_id,
                ..
            })
            | Kind::BusinessConnection(BusinessConnection {
                id: business_connection_id,
                ..
            }) => Some(business_connection_id),
            Kind::BusinessMessage(message) => message.business_connection_id(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn message(&self) -> Option<&Message> {
        match self {
            Kind::Message(message)
            | Kind::EditedMessage(message)
            | Kind::BusinessMessage(message)
            | Kind::EditedBusinessMessage(message)
            | Kind::ChannelPost(message)
            | Kind::EditedChannelPost(message)
            | Kind::CallbackQuery(CallbackQuery {
                message: Some(MaybeInaccessibleMessage::Message(message)),
                ..
            }) => Some(message),
            _ => None,
        }
    }
}

impl Default for Kind {
    #[must_use]
    fn default() -> Self {
        Self::Message(Message::default())
    }
}

impl From<Update> for Kind {
    #[must_use]
    fn from(update: Update) -> Self {
        update.kind
    }
}

impl<'de> Deserialize<'de> for Kind {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Kind;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut tmp = None;

                // Try to deserialize a borrowed-str key, or else try deserializing an owned string key
                let key = map.next_key::<&str>().or_else(|_| {
                    map.next_key::<String>().map(|k| {
                        tmp = k;
                        tmp.as_deref()
                    })
                });

                let update_type = match key {
                    Ok(Some(key)) => match UpdateType::from_str(key) {
                        Ok(update_type) => update_type,
                        Err(err) => {
                            return Err(serde::de::Error::custom(format!(
                                "Unknown update type: {err}"
                            )))
                        }
                    },
                    Ok(None) => return Err(serde::de::Error::custom("No update type key found")),
                    Err(err) => {
                        return Err(serde::de::Error::custom(format!(
                            "No update type key with type &str or String found: {err}"
                        )))
                    }
                };

                let update_kind = match update_type {
                    UpdateType::Message => map.next_value::<Message>().map(Kind::Message),
                    UpdateType::InlineQuery => {
                        map.next_value::<InlineQuery>().map(Kind::InlineQuery)
                    }
                    UpdateType::ChosenInlineResult => map
                        .next_value::<ChosenInlineResult>()
                        .map(Kind::ChosenInlineResult),
                    UpdateType::CallbackQuery => {
                        map.next_value::<CallbackQuery>().map(Kind::CallbackQuery)
                    }
                    UpdateType::ChannelPost => map.next_value::<Message>().map(Kind::ChannelPost),
                    UpdateType::EditedMessage => {
                        map.next_value::<Message>().map(Kind::EditedMessage)
                    }
                    UpdateType::EditedChannelPost => {
                        map.next_value::<Message>().map(Kind::EditedChannelPost)
                    }
                    UpdateType::BusinessConnection => map
                        .next_value::<BusinessConnection>()
                        .map(Kind::BusinessConnection),
                    UpdateType::BusinessMessage => {
                        map.next_value::<Message>().map(Kind::BusinessMessage)
                    }
                    UpdateType::EditedBusinessMessage => {
                        map.next_value::<Message>().map(Kind::EditedBusinessMessage)
                    }
                    UpdateType::DeletedBusinessMessages => map
                        .next_value::<BusinessMessagesDeleted>()
                        .map(Kind::DeletedBusinessMessages),
                    UpdateType::ShippingQuery => {
                        map.next_value::<ShippingQuery>().map(Kind::ShippingQuery)
                    }
                    UpdateType::PreCheckoutQuery => map
                        .next_value::<PreCheckoutQuery>()
                        .map(Kind::PreCheckoutQuery),
                    UpdateType::PurchasedPaidMedia => map
                        .next_value::<PaidMediaPurchased>()
                        .map(Kind::PurchasedPaidMedia),
                    UpdateType::Poll => map.next_value::<Poll>().map(Kind::Poll),
                    UpdateType::PollAnswer => map.next_value::<PollAnswer>().map(Kind::PollAnswer),
                    UpdateType::MyChatMember => map
                        .next_value::<ChatMemberUpdated>()
                        .map(Kind::MyChatMember),
                    UpdateType::ChatMember => {
                        map.next_value::<ChatMemberUpdated>().map(Kind::ChatMember)
                    }
                    UpdateType::ChatJoinRequest => map
                        .next_value::<ChatJoinRequest>()
                        .map(Kind::ChatJoinRequest),
                    UpdateType::MessageReaction => map
                        .next_value::<MessageReactionUpdated>()
                        .map(Kind::MessageReaction),
                    UpdateType::MessageReactionCount => map
                        .next_value::<MessageReactionCountUpdated>()
                        .map(Kind::MessageReactionCount),
                    UpdateType::ChatBoost => {
                        map.next_value::<ChatBoostUpdated>().map(Kind::ChatBoost)
                    }
                    UpdateType::RemovedChatBoost => map
                        .next_value::<ChatBoostRemoved>()
                        .map(Kind::RemovedChatBoost),
                };

                match update_kind {
                    Ok(update_kind) => Ok(update_kind),
                    Err(err) => Err(serde::de::Error::custom(format!(
                        "Error deserializing update kind: {err}"
                    ))),
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name = "UpdateKind";

        match self {
            Kind::Message(value) => serializer.serialize_newtype_variant(name, 0, "message", value),
            Kind::EditedMessage(value) => {
                serializer.serialize_newtype_variant(name, 1, "edited_message", value)
            }
            Kind::ChannelPost(value) => {
                serializer.serialize_newtype_variant(name, 2, "channel_post", value)
            }
            Kind::EditedChannelPost(value) => {
                serializer.serialize_newtype_variant(name, 3, "edited_channel_post", value)
            }
            Kind::BusinessConnection(value) => {
                serializer.serialize_newtype_variant(name, 4, "business_connection", value)
            }
            Kind::BusinessMessage(value) => {
                serializer.serialize_newtype_variant(name, 5, "business_message", value)
            }
            Kind::EditedBusinessMessage(value) => {
                serializer.serialize_newtype_variant(name, 6, "edited_business_message", value)
            }
            Kind::DeletedBusinessMessages(value) => {
                serializer.serialize_newtype_variant(name, 7, "deleted_business_messages", value)
            }
            Kind::MessageReaction(value) => {
                serializer.serialize_newtype_variant(name, 8, "message_reaction", value)
            }
            Kind::MessageReactionCount(value) => {
                serializer.serialize_newtype_variant(name, 9, "message_reaction_count", value)
            }
            Kind::InlineQuery(value) => {
                serializer.serialize_newtype_variant(name, 10, "inline_query", value)
            }
            Kind::ChosenInlineResult(value) => {
                serializer.serialize_newtype_variant(name, 11, "chosen_inline_result", value)
            }
            Kind::CallbackQuery(value) => {
                serializer.serialize_newtype_variant(name, 12, "callback_query", value)
            }
            Kind::ShippingQuery(value) => {
                serializer.serialize_newtype_variant(name, 13, "shipping_query", value)
            }
            Kind::PreCheckoutQuery(value) => {
                serializer.serialize_newtype_variant(name, 14, "pre_checkout_query", value)
            }
            Kind::PurchasedPaidMedia(value) => {
                serializer.serialize_newtype_variant(name, 15, "purchased_paid_media", value)
            }
            Kind::Poll(value) => serializer.serialize_newtype_variant(name, 16, "poll", value),
            Kind::PollAnswer(value) => {
                serializer.serialize_newtype_variant(name, 17, "poll_answer", value)
            }
            Kind::MyChatMember(value) => {
                serializer.serialize_newtype_variant(name, 18, "my_chat_member", value)
            }
            Kind::ChatMember(value) => {
                serializer.serialize_newtype_variant(name, 19, "chat_member", value)
            }
            Kind::ChatJoinRequest(value) => {
                serializer.serialize_newtype_variant(name, 20, "chat_join_request", value)
            }
            Kind::ChatBoost(value) => {
                serializer.serialize_newtype_variant(name, 21, "chat_boost", value)
            }
            Kind::RemovedChatBoost(value) => {
                serializer.serialize_newtype_variant(name, 22, "removed_chat_boost", value)
            }
        }
    }
}

impl Update {
    #[must_use]
    pub const fn text(&self) -> Option<&str> {
        self.kind().text()
    }

    #[must_use]
    pub const fn caption(&self) -> Option<&str> {
        self.kind().caption()
    }

    #[must_use]
    pub const fn text_or_caption(&self) -> Option<&str> {
        self.kind().text_or_caption()
    }

    #[must_use]
    pub const fn kind(&self) -> &Kind {
        &self.kind
    }

    #[must_use]
    pub const fn from(&self) -> Option<&User> {
        self.kind().from()
    }

    #[must_use]
    pub const fn from_id(&self) -> Option<i64> {
        self.kind().from_id()
    }

    #[must_use]
    pub const fn chat(&self) -> Option<&Chat> {
        self.kind().chat()
    }

    #[must_use]
    pub const fn chat_id(&self) -> Option<i64> {
        self.kind().chat_id()
    }

    #[must_use]
    pub const fn sender_chat(&self) -> Option<&Chat> {
        self.kind().sender_chat()
    }

    #[must_use]
    pub const fn sender_chat_id(&self) -> Option<i64> {
        self.kind().sender_chat_id()
    }

    #[must_use]
    pub const fn message_thread_id(&self) -> Option<i64> {
        self.kind().message_thread_id()
    }

    #[must_use]
    pub const fn business_connection_id(&self) -> Option<&str> {
        self.kind().business_connection_id()
    }

    #[must_use]
    pub const fn message(&self) -> Option<&Message> {
        self.kind().message()
    }
}
