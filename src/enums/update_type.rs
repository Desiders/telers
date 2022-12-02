use crate::{error::app::UpdateTypeError, types::Update};

/// Known update types
#[derive(Debug)]
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

impl From<UpdateType> for &str {
    fn from(update_type: UpdateType) -> Self {
        match update_type {
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
}

impl<'a> From<&'a UpdateType> for &str {
    fn from(update_type: &UpdateType) -> Self {
        match update_type {
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
}

impl TryFrom<Update> for UpdateType {
    type Error = UpdateTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
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
            Err(UpdateTypeError::UnknownUpdateType(
                format!("Couldn't convert `Update` to `UpdateType`. Please, open an issue on GitHub. Update: {update:?}"),
            ))
        }
    }
}

impl<'a> TryFrom<&'a Update> for UpdateType {
    type Error = UpdateTypeError;

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
            Err(UpdateTypeError::UnknownUpdateType(
                format!("Couldn't convert `Update` to `UpdateType`. Please, open an issue on GitHub. Update: {update:?}"),
            ))
        }
    }
}
