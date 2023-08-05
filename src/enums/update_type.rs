use crate::{errors::UnknownUpdateTypeError, types::Update};

use std::fmt::{self, Debug, Display};

/// This enum represents all possible types of the update
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UpdateType {
    Message,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ChannelPost,
    EditedMessage,
    EditedChannelPost,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
}

impl UpdateType {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            UpdateType::Message => "message",
            UpdateType::InlineQuery => "inline_query",
            UpdateType::ChosenInlineResult => "chosen_inline_result",
            UpdateType::CallbackQuery => "callback_query",
            UpdateType::ChannelPost => "channel_post",
            UpdateType::EditedMessage => "edited_message",
            UpdateType::EditedChannelPost => "edited_channel_post",
            UpdateType::ShippingQuery => "shipping_query",
            UpdateType::PreCheckoutQuery => "pre_checkout_query",
            UpdateType::Poll => "poll",
            UpdateType::PollAnswer => "poll_answer",
            UpdateType::MyChatMember => "my_chat_member",
            UpdateType::ChatMember => "chat_member",
            UpdateType::ChatJoinRequest => "chat_join_request",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [UpdateType; 14] {
        &[
            UpdateType::Message,
            UpdateType::InlineQuery,
            UpdateType::ChosenInlineResult,
            UpdateType::CallbackQuery,
            UpdateType::ChannelPost,
            UpdateType::EditedMessage,
            UpdateType::EditedChannelPost,
            UpdateType::ShippingQuery,
            UpdateType::PreCheckoutQuery,
            UpdateType::Poll,
            UpdateType::PollAnswer,
            UpdateType::MyChatMember,
            UpdateType::ChatMember,
            UpdateType::ChatJoinRequest,
        ]
    }
}

impl Display for UpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a UpdateType> for String {
    fn from(update_type: &'a UpdateType) -> Self {
        update_type.to_string()
    }
}

impl From<UpdateType> for String {
    fn from(update_type: UpdateType) -> Self {
        update_type.to_string()
    }
}

impl<'a> PartialEq<&'a str> for UpdateType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> TryFrom<&'a Update> for UpdateType {
    type Error = UnknownUpdateTypeError;

    fn try_from(update: &Update) -> Result<Self, Self::Error> {
        if update.message.is_some() {
            Ok(UpdateType::Message)
        } else if update.inline_query.is_some() {
            Ok(UpdateType::InlineQuery)
        } else if update.chosen_inline_result.is_some() {
            Ok(UpdateType::ChosenInlineResult)
        } else if update.callback_query.is_some() {
            Ok(UpdateType::CallbackQuery)
        } else if update.channel_post.is_some() {
            Ok(UpdateType::ChannelPost)
        } else if update.edited_message.is_some() {
            Ok(UpdateType::EditedMessage)
        } else if update.edited_channel_post.is_some() {
            Ok(UpdateType::EditedChannelPost)
        } else if update.shipping_query.is_some() {
            Ok(UpdateType::ShippingQuery)
        } else if update.pre_checkout_query.is_some() {
            Ok(UpdateType::PreCheckoutQuery)
        } else if update.poll.is_some() {
            Ok(UpdateType::Poll)
        } else if update.poll_answer.is_some() {
            Ok(UpdateType::PollAnswer)
        } else if update.my_chat_member.is_some() {
            Ok(UpdateType::MyChatMember)
        } else if update.chat_member.is_some() {
            Ok(UpdateType::ChatMember)
        } else if update.chat_join_request.is_some() {
            Ok(UpdateType::ChatJoinRequest)
        } else {
            Err(UnknownUpdateTypeError::new(format!("{update:?}")))
        }
    }
}

impl TryFrom<Update> for UpdateType {
    type Error = UnknownUpdateTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        Self::try_from(&update)
    }
}

impl<'a> TryFrom<&'a str> for UpdateType {
    type Error = UnknownUpdateTypeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "message" => Ok(UpdateType::Message),
            "inline_query" => Ok(UpdateType::InlineQuery),
            "chosen_inline_result" => Ok(UpdateType::ChosenInlineResult),
            "callback_query" => Ok(UpdateType::CallbackQuery),
            "channel_post" => Ok(UpdateType::ChannelPost),
            "edited_message" => Ok(UpdateType::EditedMessage),
            "edited_channel_post" => Ok(UpdateType::EditedChannelPost),
            "shipping_query" => Ok(UpdateType::ShippingQuery),
            "pre_checkout_query" => Ok(UpdateType::PreCheckoutQuery),
            "poll" => Ok(UpdateType::Poll),
            "poll_answer" => Ok(UpdateType::PollAnswer),
            "my_chat_member" => Ok(UpdateType::MyChatMember),
            "chat_member" => Ok(UpdateType::ChatMember),
            "chat_join_request" => Ok(UpdateType::ChatJoinRequest),
            _ => Err(UnknownUpdateTypeError::new(value.to_owned())),
        }
    }
}
