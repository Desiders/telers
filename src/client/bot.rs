use super::session::base::Session;

use crate::{
    error::SessionErrorKind,
    methods::{
        AddStickerToSet, AnswerCallbackQuery, AnswerInlineQuery, AnswerWebAppQuery,
        ApproveChatJoinRequest, BanChatMember, BanChatSenderChat, CloseForumTopic,
        CloseGeneralForumTopic, CopyMessage, CreateChatInviteLink, CreateForumTopic,
        DeclineChatJoinRequest, DeleteChatPhoto, DeleteChatStickerSet, DeleteForumTopic,
        DeleteMessage, DeleteMyCommands, EditChatInviteLink, EditForumTopic, EditGeneralForumTopic,
        EditMessageCaption, EditMessageLiveLocation, EditMessageMedia, EditMessageReplyMarkup,
        EditMessageText, ExportChatInviteLink, ForwardMessage, GetChat, GetChatAdministrators,
        GetChatMember, GetChatMemberCount, GetChatMenuButton, GetFile, GetForumTopicIconStickers,
        GetMe, GetMyCommands, GetMyDefaultAdministratorRights, GetUpdates, GetUserProfilePhotos,
        HideGeneralForumTopic, LeaveChat, LogOut, PinChatMessage, PromoteChatMember,
        ReopenForumTopic, ReopenGeneralForumTopic, RestrictChatMember, RevokeChatInviteLink,
        SendAnimation, SendAudio, SendChatAction, SendContact, SendDice, SendDocument,
        SendLocation, SendMediaGroup, SendMessage, SendPhoto, SendPoll, SendVenue, SendVideo,
        SendVideoNote, SendVoice, SetChatAdministratorCustomTitle, SetChatDescription,
        SetChatMenuButton, SetChatPermissions, SetChatStickerSet, SetChatTitle, SetMyCommands,
        SetMyDefaultAdministratorRights, StopMessageLiveLocation, StopPoll, TelegramMethod,
        UnbanChatMember, UnbanChatSenderChat, UnhideGeneralForumTopic, UnpinAllChatMessages,
        UnpinAllForumTopicMessages, UnpinChatMessage,
    },
    types::{
        BotCommand, BotCommandScope, Chat, ChatAdministratorRights, ChatIdKind, ChatInviteLink,
        ChatMember, ChatPermissions, File, ForumTopic, InlineKeyboardMarkup, InlineQueryResult,
        InputFile, InputMedia, MaskPosition, MenuButton, Message, MessageEntity, MessageId,
        MessageOrTrue, Poll, ReplyMarkup, SentWebAppMessage, Sticker, Update, User,
        UserProfilePhotos,
    },
};

use std::fmt::{self, Debug, Formatter};

/// Hide token for privacy. \
/// If token length is less than 4, then it will be hidden as `*`. \
/// For example,
/// `1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11` will be hidden as `12********11`
fn hide_token(token: &str) -> String {
    let token_len = token.len();

    if token_len < 4 {
        return "*".repeat(token_len);
    }

    let mut hidden = String::with_capacity(token_len);
    hidden.push_str(&token[..2]);
    hidden.push_str(&"*".repeat(8));
    hidden.push_str(&token[token_len - 2..]);
    hidden
}

/// Represents a bot with a token for getting updates and sending requests to Telegram API
#[derive(Clone, Default)]
pub struct Bot<Client> {
    /// Bot token, which is used to receive updates and send requests to the Telegram API
    token: String,
    /// Bot token, which is used in `Debug` implementation for privacy
    hidden_token: String,
    /// Client for sending requests to Telegram API
    client: Client,
}

impl<Client> Bot<Client> {
    #[must_use]
    pub fn new<T>(token: T, client: Client) -> Self
    where
        T: Into<String>,
    {
        let token = token.into();
        let hidden_token = hide_token(&token);

        Self {
            token,
            hidden_token,
            client,
        }
    }

    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[must_use]
    pub fn hidden_token(&self) -> &str {
        &self.hidden_token
    }
}

impl<Client> Debug for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bot")
            .field("token", &self.hidden_token)
            .finish()
    }
}

impl<Client: Default> Bot<Client> {
    #[must_use]
    pub fn builder() -> BotBuilder<Client> {
        BotBuilder::default()
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct BotBuilder<Client> {
    token: String,
    client: Client,
}

impl<Client> BotBuilder<Client> {
    /// Set bot token, which is used to receive updates and send requests to the Telegram API
    #[must_use]
    pub fn token<T: Into<String>>(mut self, val: T) -> Self {
        self.token = val.into();
        self
    }

    /// Set client for sending requests to Telegram API
    #[must_use]
    pub fn client(mut self, val: Client) -> Self {
        self.client = val;
        self
    }

    #[must_use]
    pub fn build(self) -> Bot<Client> {
        let token = self.token;
        let hidden_token = hide_token(&token);

        Bot {
            token,
            hidden_token,
            client: self.client,
        }
    }
}

/// A block of Telegram methods
impl<Client: Session + Sync> Bot<Client> {
    /// Use this method to send requests to Telegram API
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `request_timeout` - Request timeout
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send<T>(
        &self,
        method: &T,
        request_timeout: Option<f32>,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        self.client
            .make_request_and_get_result(self, method, request_timeout)
            .await
    }

    /// Use this method to add a new sticker to a set created by the bot. \
    /// You **must** use exactly one of the fields `png_sticker`, `tgs_sticker`, or `webm_sticker`. Animated stickers can be added to animated sticker sets and only to them. Animated sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#addstickertoset>
    /// # Returns
    /// `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn add_sticker_to_set<'a>(
        &self,
        user_id: i64,
        name: impl Into<String>,
        png_sticker: Option<InputFile<'a>>,
        tgs_sticker: Option<InputFile<'a>>,
        webm_sticker: Option<InputFile<'a>>,
        emojis: impl Into<String>,
        mask_position: Option<MaskPosition>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &AddStickerToSet {
                user_id,
                name: name.into(),
                png_sticker,
                tgs_sticker,
                webm_sticker,
                emojis: emojis.into(),
                mask_position,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#answercallbackquery>
    /// # Notes
    /// Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via @BotFather and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    /// # Returns
    /// On success, `True` is returned.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn answer_callback_query(
        &self,
        callback_query_id: impl Into<String>,
        text: Option<impl Into<String>>,
        show_alert: Option<bool>,
        url: Option<impl Into<String>>,
        cache_time: Option<i32>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &AnswerCallbackQuery {
                callback_query_id: callback_query_id.into(),
                text: text.map(Into::into),
                show_alert,
                url: url.map(Into::into),
                cache_time,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send answers to an inline query. No more than 50 results per query are allowed.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#answerinlinequery>
    /// # Returns
    /// On success, `True` is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn answer_inline_query(
        &self,
        inline_query_id: impl Into<String>,
        results: Vec<impl Into<InlineQueryResult>>,
        cache_time: Option<i32>,
        is_personal: Option<bool>,
        next_offset: Option<impl Into<String>>,
        switch_pm_text: Option<impl Into<String>>,
        switch_pm_parameter: Option<impl Into<String>>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &AnswerInlineQuery {
                inline_query_id: inline_query_id.into(),
                results: results.into_iter().map(Into::into).collect(),
                cache_time,
                is_personal,
                next_offset: next_offset.map(Into::into),
                switch_pm_text: switch_pm_text.map(Into::into),
                switch_pm_parameter: switch_pm_parameter.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#answerwebappquery>
    /// # Returns
    /// On success, a [`SentWebAppMessage`] object is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn answer_web_app_query(
        &self,
        web_app_query_id: impl Into<String>,
        result: impl Into<InlineQueryResult>,
        request_timeout: Option<f32>,
    ) -> Result<SentWebAppMessage, SessionErrorKind> {
        self.send(
            &AnswerWebAppQuery {
                web_app_query_id: web_app_query_id.into(),
                result: result.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the `can_invite_users` administrator right.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#approvechatjoinrequest>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn approve_chat_join_request(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &ApproveChatJoinRequest {
                chat_id: chat_id.into(),
                user_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [`unbanned`](crate::methods::UnbanChatMember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#banchatmember>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn ban_chat_member(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        until_date: Option<i64>,
        revoke_messages: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &BanChatMember {
                chat_id: chat_id.into(),
                user_id,
                until_date,
                revoke_messages,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [`unbanned`](crate::methods::UnbanChatSenderChat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#banchatsenderchat>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn ban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatIdKind>,
        sender_chat_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &BanChatSenderChat {
                chat_id: chat_id.into(),
                sender_chat_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights, unless it is the creator of the topic.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#closeforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn close_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &CloseForumTopic {
                chat_id: chat_id.into(),
                message_thread_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to close an open `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#closegeneralforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn close_general_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &CloseGeneralForumTopic {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to copy messages of any kind. Service messages and invoice messages can't be copied. A quiz [`poll`](crate::types::Poll) can be copied only if the value of the field `correct_option_id` is known to the bot. The method is analogous to the method [`ForwardMessage`](crate::methods::ForwardMessage), but the copied message doesn't have a link to the original message.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#copymessage>
    /// # Returns
    /// Returns the [`MessageId`] of the sent message on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn copy_message(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        from_chat_id: impl Into<ChatIdKind>,
        message_id: i64,
        caption: Option<impl Into<String>>,
        parse_mode: Option<impl Into<String>>,
        caption_entities: Option<Vec<MessageEntity>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<MessageId, SessionErrorKind> {
        self.send(
            &CopyMessage {
                chat_id: chat_id.into(),
                message_thread_id,
                from_chat_id: from_chat_id.into(),
                message_id,
                caption: caption.map(Into::into),
                parse_mode: parse_mode.map(Into::into),
                caption_entities,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [`RevokeChatInviteLink`](crate::methods::RevokeChatInviteLink).
    /// # Documentation
    /// <https://core.telegram.org/bots/api#createchatinvitelink>
    /// # Returns
    /// Returns the new invite link as [`ChatInviteLink`] object.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn create_chat_invite_link(
        &self,
        chat_id: impl Into<ChatIdKind>,
        invite_link: Option<impl Into<String>>,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        creates_join_request: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<ChatInviteLink, SessionErrorKind> {
        self.send(
            &CreateChatInviteLink {
                chat_id: chat_id.into(),
                invite_link: invite_link.map(Into::into),
                expire_date,
                member_limit,
                creates_join_request,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#createforumtopic>
    /// # Returns
    /// Returns information about the created topic as a [`ForumTopic`] object.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn create_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        title: impl Into<String>,
        icon_color: Option<impl Into<String>>,
        icon_custom_emoji_id: Option<impl Into<String>>,
        request_timeout: Option<f32>,
    ) -> Result<ForumTopic, SessionErrorKind> {
        self.send(
            &CreateForumTopic {
                chat_id: chat_id.into(),
                title: title.into(),
                icon_color: icon_color.map(Into::into),
                icon_custom_emoji_id: icon_custom_emoji_id.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the `can_invite_users` administrator right.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#declinechatjoinrequest>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn decline_chat_join_request(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &DeclineChatJoinRequest {
                chat_id: chat_id.into(),
                user_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#deletechatphoto>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn delete_chat_photo(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &DeleteChatPhoto {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field `can_set_sticker_set` optionally returned in [`GetChat`](crate::methods::GetChat) requests to check if the bot can use this method.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#deletechatstickerset>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn delete_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &DeleteChatStickerSet {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_delete_messages` administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#deleteforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn delete_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &DeleteForumTopic {
                chat_id: chat_id.into(),
                message_thread_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to delete a message, including service messages, with the following limitations:
    /// - A message can only be deleted if it was sent less than 48 hours ago.
    /// - Service messages about a supergroup, channel, or forum topic creation can't be deleted.
    /// - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
    /// - Bots can delete outgoing messages in private chats, groups, and supergroups.
    /// - Bots can delete incoming messages in private chats.
    /// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
    /// - If the bot is an administrator of a group, it can delete any message there.
    /// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#deletemessage>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn delete_message(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &DeleteMessage {
                chat_id: chat_id.into(),
                message_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api#determining-list-of-commands) will be shown to affected users.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#deletemycommands>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn delete_my_commands(
        &self,
        scope: Option<impl Into<BotCommandScope>>,
        language_code: Option<impl Into<String>>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &DeleteMyCommands {
                scope: scope.map(Into::into),
                language_code: language_code.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editchatinvitelink>
    /// # Returns
    /// Returns the edited invite link as a [`ChatInviteLink`] object.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_chat_invite_link(
        &self,
        chat_id: impl Into<ChatIdKind>,
        invite_link: impl Into<String>,
        name: Option<impl Into<String>>,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        creates_join_request: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<ChatInviteLink, SessionErrorKind> {
        self.send(
            &EditChatInviteLink {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
                name: name.map(Into::into),
                expire_date,
                creates_join_request,
                member_limit,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights, unless it is the creator of the topic.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: i64,
        name: Option<impl Into<String>>,
        icon_custom_emoji_id: Option<impl Into<String>>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &EditForumTopic {
                chat_id: chat_id.into(),
                message_thread_id,
                name: name.map(Into::into),
                icon_custom_emoji_id: icon_custom_emoji_id.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to edit captions of messages.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editmessagecaption>
    /// # Returns
    /// On success, if the edited message is not an inline message, the edited [`Message`] is returned, otherwise `True` is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_message_caption(
        &self,
        chat_id: Option<impl Into<ChatIdKind>>,
        message_thread_id: Option<i64>,
        inline_message_id: Option<impl Into<String>>,
        caption: impl Into<String>,
        parse_mode: Option<impl Into<String>>,
        caption_entities: Option<Vec<MessageEntity>>,
        reply_markup: Option<impl Into<InlineKeyboardMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<MessageOrTrue, SessionErrorKind> {
        self.send(
            &EditMessageCaption {
                chat_id: chat_id.map(Into::into),
                message_thread_id,
                inline_message_id: inline_message_id.map(Into::into),
                caption: caption.into(),
                parse_mode: parse_mode.map(Into::into),
                caption_entities,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to close an open `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editgeneralforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_general_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        name: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &EditGeneralForumTopic {
                chat_id: chat_id.into(),
                name: name.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to edit live location messages. A location can be edited until its `live_period` expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](crate::methods::StopMessageLiveLocation).
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editmessagelivelocation>
    /// # Returns
    /// On success, if the edited message is not an inline message, the edited [`Message`] is returned,
    /// otherwise `True` is returned.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_message_live_location(
        &self,
        chat_id: Option<impl Into<ChatIdKind>>,
        message_id: Option<i64>,
        inline_message_id: Option<impl Into<String>>,
        latitude: f64,
        longitude: f64,
        horizontal_accuracy: Option<f64>,
        heading: Option<i64>,
        proximity_alert_radius: Option<i64>,
        reply_markup: Option<impl Into<InlineKeyboardMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<MessageOrTrue, SessionErrorKind> {
        self.send(
            &EditMessageLiveLocation {
                chat_id: chat_id.map(Into::into),
                message_id,
                inline_message_id: inline_message_id.map(Into::into),
                latitude,
                longitude,
                horizontal_accuracy,
                heading,
                proximity_alert_radius,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its `file_id` or specify a URL.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editmessagemedia>
    /// # Returns
    /// On success, if the edited message is not an inline message, the edited [`crate::types::Message`] is returned, otherwise `True` is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_message_media<'a>(
        &self,
        chat_id: Option<impl Into<ChatIdKind>>,
        message_id: Option<i64>,
        inline_message_id: Option<impl Into<String>>,
        media: impl Into<InputMedia<'a>>,
        reply_markup: Option<impl Into<InlineKeyboardMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<MessageOrTrue, SessionErrorKind> {
        self.send(
            &EditMessageMedia {
                chat_id: chat_id.map(Into::into),
                message_id,
                inline_message_id: inline_message_id.map(Into::into),
                media: media.into(),
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to edit only the reply markup of messages.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editmessagereplymarkup>
    /// # Returns
    /// On success, if the edited message is not an inline message, the edited [`crate::types::Message`] is returned, otherwise `True` is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_message_reply_markup(
        &self,
        chat_id: Option<impl Into<ChatIdKind>>,
        message_id: Option<i64>,
        inline_message_id: Option<impl Into<String>>,
        reply_markup: Option<impl Into<InlineKeyboardMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<MessageOrTrue, SessionErrorKind> {
        self.send(
            &EditMessageReplyMarkup {
                chat_id: chat_id.map(Into::into),
                message_id,
                inline_message_id: inline_message_id.map(Into::into),
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to edit text and [game](https://core.telegram.org/bots/api#games) messages.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#editmessagetext>
    /// # Returns
    /// On success, if the edited message is not an inline message, the edited [`Message`] is returned, otherwise `True` is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn edit_message_text(
        &self,
        chat_id: Option<impl Into<ChatIdKind>>,
        message_thread_id: Option<i64>,
        inline_message_id: Option<impl Into<String>>,
        text: impl Into<String>,
        parse_mode: Option<impl Into<String>>,
        entities: Option<Vec<MessageEntity>>,
        disable_web_page_preview: Option<bool>,
        reply_markup: Option<impl Into<InlineKeyboardMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<MessageOrTrue, SessionErrorKind> {
        self.send(
            &EditMessageText {
                chat_id: chat_id.map(Into::into),
                message_thread_id,
                inline_message_id: inline_message_id.map(Into::into),
                text: text.into(),
                parse_mode: parse_mode.map(Into::into),
                entities,
                disable_web_page_preview,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#exportchatinvitelink>
    /// # Notes
    /// Each administrator in a chat generates their own invite links. Bots can't use invite links generated by other administrators. If you want your bot to work with invite links, it will need to generate its own link using [`crate::methods::ExportChatInviteLink`] or by calling the [`crate::methods::GetChat`] method. If your bot needs to generate a new primary invite link replacing its previous one, use [`crate::methods::ExportChatInviteLink`] again.
    /// # Returns
    /// Returns the new invite link as `String` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn export_chat_invite_link(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<String, SessionErrorKind> {
        self.send(
            &ExportChatInviteLink {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to forward messages of any kind. Service messages can't be forwarded.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#forwardmessage>
    /// # Returns
    /// On success, the sent [`Message`] is returned.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn forward_message(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        from_chat_id: impl Into<ChatIdKind>,
        message_id: i64,
        disable_notification: bool,
        protect_content: bool,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &ForwardMessage {
                chat_id: chat_id.into(),
                message_thread_id,
                from_chat_id: from_chat_id.into(),
                message_id,
                disable_notification,
                protect_content,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.).
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getchat>
    /// # Returns
    /// Returns a [`Chat`] object on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_chat(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<Chat, SessionErrorKind> {
        self.send(
            &GetChat {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get a list of administrators in a chat, which aren't bots.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getchatadministrators>
    /// # Returns
    /// Returns an Array of [`ChatMember`] objects.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_chat_administrators(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<Vec<ChatMember>, SessionErrorKind> {
        self.send(
            &GetChatAdministrators {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get information about a member of a chat.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getchatmember>
    /// # Returns
    /// Returns a [`ChatMember`] object on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_chat_member(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<ChatMember, SessionErrorKind> {
        self.send(
            &GetChatMember {
                chat_id: chat_id.into(),
                user_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get the number of members in a chat.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getchatmembercount>
    /// # Returns
    /// Returns `Int` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_chat_member_count(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<i64, SessionErrorKind> {
        self.send(
            &GetChatMemberCount {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getchatmenubutton>
    /// # Returns
    /// Returns [`MenuButton`] on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_chat_menu_button(
        &self,
        chat_id: Option<i64>,
        request_timeout: Option<f32>,
    ) -> Result<MenuButton, SessionErrorKind> {
        self.send(&GetChatMenuButton { chat_id }, request_timeout)
            .await
    }

    /// Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [`GetFile`](crate::methods::GetFile) again.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getfile>
    /// # Notes
    /// This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received.
    /// # Returns
    /// On success, a [`File`] object is returned.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_file(
        &self,
        file_id: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<File, SessionErrorKind> {
        self.send(
            &GetFile {
                file_id: file_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getforumtopiciconstickers>
    /// # Returns
    /// Returns an Array of [`Sticker`] objects.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_forum_topic_icon_stickers(
        &self,
        request_timeout: Option<f32>,
    ) -> Result<Vec<Sticker>, SessionErrorKind> {
        self.send(&GetForumTopicIconStickers {}, request_timeout)
            .await
    }

    /// A simple method for testing your bot's authentication token. Requires no parameters.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getme>
    /// # Returns
    /// Returns basic information about the bot in form of a [`User`] object
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_me(&self, request_timeout: Option<f32>) -> Result<User, SessionErrorKind> {
        self.send(&GetMe {}, request_timeout).await
    }

    /// Use this method to get the current list of the bot's commands for the given scope and user language.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getmycommands>
    /// # Returns
    /// Returns an Array of [`BotCommand`] objects. If commands aren't set, an empty list is returned.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_my_commands(
        &self,
        scope: Option<impl Into<BotCommandScope>>,
        language_code: Option<impl Into<String>>,
        request_timeout: Option<f32>,
    ) -> Result<Vec<BotCommand>, SessionErrorKind> {
        self.send(
            &GetMyCommands {
                scope: scope.map(Into::into),
                language_code: language_code.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get the current default administrator rights of the bot.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getmydefaultadministratorrights>
    /// # Returns
    /// Returns [`ChatAdministratorRights`] on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_my_default_administrator_rights(
        &self,
        for_channels: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<ChatAdministratorRights, SessionErrorKind> {
        self.send(
            &GetMyDefaultAdministratorRights { for_channels },
            request_timeout,
        )
        .await
    }

    /// Use this method to receive incoming updates using long polling (`wiki <https://en.wikipedia.org/wiki/Push_technology#Long_polling>`).
    /// # Documentation
    /// <https://core.telegram.org/bots/apigetupdates>
    /// # Arguments
    /// * `offset` - Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [`crate::methods::get_updates::GetUpdates`] is called with an `offset` higher than its `update_id`. The negative offset can be specified to retrieve updates starting from `-offset` update from the end of the updates queue. All previous updates will forgotten.
    /// * `limit` - Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    /// * `timeout` - Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` - A JSON-serialized list of the update types you want your bot to receive. For example, specify [`message`, `edited_channel_post`, `callback_query`] to only receive updates of these types. See [`crate::types::Update`] for a complete list of available update types. Specify an empty list to receive all update types except `chat_member` (default). If not specified, the previous setting will be used.
    /// * `request_timeout` - Request timeout
    /// # Notes
    /// - This method will not work if an outgoing webhook is set up. \
    /// - In order to avoid getting duplicate updates, recalculate `offset` after each server response. \
    /// # Returns
    /// Array of [`Update`] objects
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_updates(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
        timeout: Option<i64>,
        allowed_updates: Option<Vec<String>>,
        request_timeout: Option<f32>,
    ) -> Result<Vec<Update>, SessionErrorKind> {
        self.send(
            &GetUpdates {
                offset,
                limit,
                timeout,
                allowed_updates,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to get a list of profile pictures for a user.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#getuserprofilephotos>
    /// # Returns
    /// Returns a [`UserProfilePhotos`] object.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn get_user_profile_photos(
        &self,
        user_id: i64,
        offset: Option<i64>,
        limit: Option<i64>,
        request_timeout: Option<f32>,
    ) -> Result<UserProfilePhotos, SessionErrorKind> {
        self.send(
            &GetUserProfilePhotos {
                user_id,
                offset,
                limit,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to hide the `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights. The topic will be automatically closed if it was open.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#hidegeneralforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn hide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &HideGeneralForumTopic {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method for your bot to leave a group, supergroup or channel.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#leavechat>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn leave_chat(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &LeaveChat {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Requires no parameters.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#logout>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn log_out(&self, request_timeout: Option<f32>) -> Result<bool, SessionErrorKind> {
        self.send(&LogOut {}, request_timeout).await
    }

    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in a supergroup or `can_edit_messages` administrator right in a channel.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#pinchatmessage>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn pin_chat_message(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_id: i64,
        disable_notification: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &PinChatMessage {
                chat_id: chat_id.into(),
                message_id,
                disable_notification,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass `False` for all boolean parameters to demote a user.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#promotechatmember>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn promote_chat_member(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        is_anonymous: Option<bool>,
        can_manage_chat: Option<bool>,
        can_post_messages: Option<bool>,
        can_edit_messages: Option<bool>,
        can_delete_messages: Option<bool>,
        can_manage_voice_chats: Option<bool>,
        can_restrict_members: Option<bool>,
        can_promote_members: Option<bool>,
        can_change_info: Option<bool>,
        can_invite_users: Option<bool>,
        can_pin_messages: Option<bool>,
        can_manage_topics: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &PromoteChatMember {
                chat_id: chat_id.into(),
                user_id,
                is_anonymous,
                can_manage_chat,
                can_post_messages,
                can_edit_messages,
                can_delete_messages,
                can_manage_voice_chats,
                can_restrict_members,
                can_promote_members,
                can_change_info,
                can_invite_users,
                can_pin_messages,
                can_manage_topics,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights, unless it is the creator of the topic.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#reopenforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn reopen_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &ReopenForumTopic {
                chat_id: chat_id.into(),
                message_thread_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to reopen a closed `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights. The topic will be automatically unhidden if it was hidden.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#reopengeneralforumtopic>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn reopen_general_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &ReopenGeneralForumTopic {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass `True` for all permissions to lift restrictions from a user.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#restrictchatmember>
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn restrict_chat_member(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        permissions: ChatPermissions,
        until_date: Option<i64>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &RestrictChatMember {
                chat_id: chat_id.into(),
                user_id,
                permissions,
                until_date,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#revokechatinvitelink>
    /// # Returns
    /// Returns the revoked invite link as [`ChatInviteLink`] object.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn revoke_chat_invite_link(
        &self,
        chat_id: impl Into<ChatIdKind>,
        invite_link: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<ChatInviteLink, SessionErrorKind> {
        self.send(
            &RevokeChatInviteLink {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendanimation>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_animation<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        animation: impl Into<InputFile<'a>>,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        thumb: Option<impl Into<InputFile<'a>>>,
        caption: Option<impl Into<String>>,
        parse_mode: Option<impl Into<String>>,
        caption_entities: Option<Vec<MessageEntity>>,
        has_spoiler: Option<bool>,
        supports_streaming: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendAnimation {
                chat_id: chat_id.into(),
                message_thread_id,
                animation: animation.into(),
                duration,
                width,
                height,
                thumb: thumb.map(Into::into),
                caption: caption.map(Into::into),
                parse_mode: parse_mode.map(Into::into),
                caption_entities,
                has_spoiler,
                supports_streaming,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendaudio>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_audio<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        audio: impl Into<InputFile<'a>>,
        caption: Option<impl Into<String>>,
        parse_mode: Option<impl Into<String>>,
        caption_entities: Option<Vec<MessageEntity>>,
        duration: Option<i64>,
        performer: Option<impl Into<String>>,
        title: Option<impl Into<String>>,
        thumb: Option<impl Into<InputFile<'a>>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendAudio {
                chat_id: chat_id.into(),
                message_thread_id,
                audio: audio.into(),
                caption: caption.map(Into::into),
                parse_mode: parse_mode.map(Into::into),
                caption_entities,
                duration,
                performer: performer.map(Into::into),
                title: title.map(Into::into),
                thumb: thumb.map(Into::into),
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status).
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendchataction>
    /// # Notes
    /// We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    /// # Example
    /// The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [`SendChatAction`](crate::methods::SendChatAction) with `action = upload_photo`. The user will see a “sending photo” status for the bot.
    /// # Returns
    /// Returns `True` on success.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_chat_action(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        action: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SendChatAction {
                chat_id: chat_id.into(),
                message_thread_id,
                action: action.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send phone contacts.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendcontact>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_contact(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
        last_name: Option<impl Into<String>>,
        vcard: Option<impl Into<String>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendContact {
                chat_id: chat_id.into(),
                message_thread_id,
                phone_number: phone_number.into(),
                first_name: first_name.into(),
                last_name: last_name.map(Into::into),
                vcard: vcard.map(Into::into),
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send an animated emoji that will display a random value.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#senddice>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_dice(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        emoji: Option<impl Into<String>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendDice {
                chat_id: chat_id.into(),
                message_thread_id,
                emoji: emoji.map(Into::into),
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send general files. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#senddocument>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_document<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        document: impl Into<InputFile<'a>>,
        thumb: Option<impl Into<InputFile<'a>>>,
        caption: Option<impl Into<String>>,
        parse_mode: Option<impl Into<String>>,
        caption_entities: Option<Vec<MessageEntity>>,
        disable_content_type_detection: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendDocument {
                chat_id: chat_id.into(),
                message_thread_id,
                document: document.into(),
                thumb: thumb.map(Into::into),
                caption: caption.map(Into::into),
                parse_mode: parse_mode.map(Into::into),
                caption_entities,
                disable_content_type_detection,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send point on the map.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendlocation>
    /// # Returns
    /// On success, the sent [`Message`] is return
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_location(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        latitude: f64,
        longitude: f64,
        horizontal_accuracy: Option<f64>,
        live_period: Option<i64>,
        heading: Option<i64>,
        proximity_alert_radius: Option<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendLocation {
                chat_id: chat_id.into(),
                message_thread_id,
                latitude,
                longitude,
                horizontal_accuracy,
                live_period,
                heading,
                proximity_alert_radius,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendmediagroup>
    /// # Returns
    /// On success, an array of [`Message`]s that were sent is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_media_group<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        media: Vec<impl Into<InputMedia<'a>>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<Vec<Message>, SessionErrorKind> {
        self.send(
            &SendMediaGroup {
                chat_id: chat_id.into(),
                message_thread_id,
                media: media.into_iter().map(Into::into).collect(),
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send text messages.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendmessage>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_message(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        text: impl Into<String>,
        parse_mode: Option<impl Into<String>>,
        entities: Option<Vec<MessageEntity>>,
        disable_web_page_preview: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendMessage {
                chat_id: chat_id.into(),
                message_thread_id,
                text: text.into(),
                parse_mode: parse_mode.map(Into::into),
                entities,
                disable_web_page_preview,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send photos.
    /// # Documentation
    /// <https://core.telegram.org/bots/apisendphoto>
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    /// * `message_thread_id` - Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    /// * `photo` - Photo to send. Pass a `file_id` as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using `multipart/form-data`. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. See `more information on Sending Files <https://core.telegram.org/bots/api#sending-files>`.
    /// * `caption` - Photo caption (may also be used when resending photos by `file_id`), 0-1024 characters after entities parsing
    /// * `parse_mode` - Mode for parsing entities in the photo caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    /// * `caption_entities` - List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    /// * `disable_notification` - Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    /// * `reply_to_message_id` - If the message is a reply, ID of the original message
    /// * `allow_sending_without_reply` - Pass `True` if the message should be sent even if the specified replied-to message is not found
    /// * `reply_markup` - Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    /// * `request_timeout` - Request timeout
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    #[allow(clippy::too_many_arguments)]
    pub async fn send_photo<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        photo: impl Into<InputFile<'a>>,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        has_spoiler: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendPhoto {
                chat_id: chat_id.into(),
                message_thread_id,
                photo: photo.into(),
                caption,
                parse_mode,
                caption_entities,
                has_spoiler,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send a native poll.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendpoll>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_poll(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        question: impl Into<String>,
        options: Vec<impl Into<String>>,
        is_anonymous: Option<bool>,
        poll_type: Option<impl Into<String>>,
        allows_multiple_answers: Option<bool>,
        correct_option_id: Option<i64>,
        explanation: Option<impl Into<String>>,
        explanation_parse_mode: Option<impl Into<String>>,
        explanation_entities: Option<Vec<MessageEntity>>,
        open_period: Option<i64>,
        close_date: Option<i64>,
        is_closed: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendPoll {
                chat_id: chat_id.into(),
                message_thread_id,
                question: question.into(),
                options: options.into_iter().map(Into::into).collect(),
                is_anonymous,
                poll_type: poll_type.map(Into::into),
                allows_multiple_answers,
                correct_option_id,
                explanation: explanation.map(Into::into),
                explanation_parse_mode: explanation_parse_mode.map(Into::into),
                explanation_entities,
                open_period,
                close_date,
                is_closed,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send information about a venue.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendvenue>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_venue(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
        foursquare_id: Option<impl Into<String>>,
        foursquare_type: Option<impl Into<String>>,
        google_place_id: Option<impl Into<String>>,
        google_place_type: Option<impl Into<String>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendVenue {
                chat_id: chat_id.into(),
                message_thread_id,
                latitude,
                longitude,
                title: title.into(),
                address: address.into(),
                foursquare_id: foursquare_id.map(Into::into),
                foursquare_type: foursquare_type.map(Into::into),
                google_place_id: google_place_id.map(Into::into),
                google_place_type: google_place_type.map(Into::into),
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [`crate::types::Document`]). Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendvideo>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_video<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        video: impl Into<InputFile<'a>>,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        thumb: Option<impl Into<InputFile<'a>>>,
        caption: Option<impl Into<String>>,
        parse_mode: Option<impl Into<String>>,
        caption_entities: Option<Vec<MessageEntity>>,
        has_spoiler: Option<bool>,
        supports_streaming: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendVideo {
                chat_id: chat_id.into(),
                message_thread_id,
                video: video.into(),
                duration,
                width,
                height,
                thumb: thumb.map(Into::into),
                caption: caption.map(Into::into),
                parse_mode: parse_mode.map(Into::into),
                caption_entities,
                has_spoiler,
                supports_streaming,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendvideonote>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_video_note<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        video_note: impl Into<InputFile<'a>>,
        duration: Option<i64>,
        length: Option<i64>,
        thumb: Option<impl Into<InputFile<'a>>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendVideoNote {
                chat_id: chat_id.into(),
                message_thread_id,
                video_note: video_note.into(),
                duration,
                length,
                thumb: thumb.map(Into::into),
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as [Audio](crate::types::Audio) or [Document](crate::types::Document)). Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#sendvoice>
    /// # Returns
    /// On success, the sent [`Message`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn send_voice<'a>(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: Option<i64>,
        voice: impl Into<InputFile<'a>>,
        caption: Option<impl Into<String>>,
        parse_mode: Option<impl Into<String>>,
        caption_entities: Option<Vec<MessageEntity>>,
        duration: Option<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Message, SessionErrorKind> {
        self.send(
            &SendVoice {
                chat_id: chat_id.into(),
                message_thread_id,
                voice: voice.into(),
                caption: caption.map(Into::into),
                parse_mode: parse_mode.map(Into::into),
                caption_entities,
                duration,
                disable_notification,
                protect_content,
                reply_to_message_id,
                allow_sending_without_reply,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setchatadministratorcustomtitle>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_chat_administrator_custom_title(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        custom_title: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetChatAdministratorCustomTitle {
                chat_id: chat_id.into(),
                user_id,
                custom_title: custom_title.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to change the description of a group, a supergroup or a channel. descriptions can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setchatdescription>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_chat_description(
        &self,
        chat_id: impl Into<ChatIdKind>,
        description: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetChatDescription {
                chat_id: chat_id.into(),
                description: description.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to change the bot's menu button in a private chat, or the default menu button.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setchatmenubutton>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_chat_menu_button(
        &self,
        chat_id: i64,
        menu_button: Option<impl Into<MenuButton>>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetChatMenuButton {
                chat_id,
                menu_button: menu_button.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the `can_restrict_members` administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setchatpermissions>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_chat_permissions(
        &self,
        chat_id: impl Into<ChatIdKind>,
        permissions: ChatPermissions,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetChatPermissions {
                chat_id: chat_id.into(),
                permissions,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field `can_set_sticker_set` optionally returned in [`GetChat`](crate::methods::GetChat) requests to check if the bot can use this method.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setchatstickerset>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatIdKind>,
        sticker_set_name: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetChatStickerSet {
                chat_id: chat_id.into(),
                sticker_set_name: sticker_set_name.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setchattitle>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_chat_title(
        &self,
        chat_id: impl Into<ChatIdKind>,
        title: impl Into<String>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetChatTitle {
                chat_id: chat_id.into(),
                title: title.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setmycommands>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_my_commands(
        &self,
        commands: Vec<BotCommand>,
        scope: Option<impl Into<BotCommandScope>>,
        language_code: Option<impl Into<String>>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetMyCommands {
                commands,
                scope: scope.map(Into::into),
                language_code: language_code.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are are free to modify the list before adding the bot.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#setmydefaultadministratorrights>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn set_my_default_administrator_rights(
        &self,
        rights: Option<ChatAdministratorRights>,
        for_channels: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &SetMyDefaultAdministratorRights {
                rights,
                for_channels,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to stop updating a live location message before `live_period` expires.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#stopmessagelivelocation>
    /// # Returns
    /// On success, if the message is not an inline message, the edited [`Message`] is returned,
    /// otherwise `True` is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn stop_message_live_location(
        &self,
        chat_id: Option<impl Into<ChatIdKind>>,
        message_id: Option<i64>,
        inline_message_id: Option<impl Into<String>>,
        reply_markup: Option<impl Into<InlineKeyboardMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<MessageOrTrue, SessionErrorKind> {
        self.send(
            &StopMessageLiveLocation {
                chat_id: chat_id.map(Into::into),
                message_id,
                inline_message_id: inline_message_id.map(Into::into),
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to stop a poll which was sent by the bot.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#stoppoll>
    /// # Returns
    /// On success, the stopped [`Poll`] is returned
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn stop_poll(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_id: i64,
        reply_markup: Option<impl Into<InlineKeyboardMarkup>>,
        request_timeout: Option<f32>,
    ) -> Result<Poll, SessionErrorKind> {
        self.send(
            &StopPoll {
                chat_id: chat_id.into(),
                message_id,
                reply_markup: reply_markup.map(Into::into),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter `only_if_banned`.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#unbanchatmember>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn unban_chat_member(
        &self,
        chat_id: impl Into<ChatIdKind>,
        user_id: i64,
        only_if_banned: Option<bool>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &UnbanChatMember {
                chat_id: chat_id.into(),
                user_id,
                only_if_banned,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#unbanchatsenderchat>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn unban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatIdKind>,
        sender_chat_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &UnbanChatSenderChat {
                chat_id: chat_id.into(),
                sender_chat_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to unhide the `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#unhidegeneralforumtopic>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn unhide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &UnhideGeneralForumTopic {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in a supergroup or `can_edit_messages` administrator right in a channel.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#unpinallchatmessages>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn unpin_all_chat_messages(
        &self,
        chat_id: impl Into<ChatIdKind>,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &UnpinAllChatMessages {
                chat_id: chat_id.into(),
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in the supergroup.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#unpinallforumtopicmessages>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn unpin_all_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_thread_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &UnpinAllForumTopicMessages {
                chat_id: chat_id.into(),
                message_thread_id,
            },
            request_timeout,
        )
        .await
    }

    /// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in a supergroup or `can_edit_messages` administrator right in a channel.
    /// # Documentation
    /// <https://core.telegram.org/bots/api#unpinchatmessage>
    /// # Returns
    /// Returns `True` on success
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an telegram api error
    pub async fn unpin_chat_message(
        &self,
        chat_id: impl Into<ChatIdKind>,
        message_id: i64,
        request_timeout: Option<f32>,
    ) -> Result<bool, SessionErrorKind> {
        self.send(
            &UnpinChatMessage {
                chat_id: chat_id.into(),
                message_id,
            },
            request_timeout,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_hide_token() {
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"), "12********11");
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew1"), "12********w1");
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew"), "12********ew");
        assert_eq!(hide_token("123"), "***");
        assert_eq!(hide_token("1234"), "12********34");
    }
}
