use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit name and icon of a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights, unless it is the creator of the topic. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#editforumtopic>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// New unique identifier of the custom emoji shown as the topic icon. Use [`crate::methods::GetForumTopicIconStickers`] to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<Box<str>>,
}
impl EditForumTopic {
    /// Creates a new `EditForumTopic`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    /// * `message_thread_id` - Unique identifier for the target message thread of the forum topic
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        message_thread_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: message_thread_id.into(),
            name: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier for the target message thread of the forum topic
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = val.into();
        this
    }

    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = Some(val.into());
        this
    }

    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.name = val.map(Into::into);
        this
    }

    /// New unique identifier of the custom emoji shown as the topic icon. Use [`crate::methods::GetForumTopicIconStickers`] to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    #[must_use]
    pub fn icon_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.icon_custom_emoji_id = Some(val.into());
        this
    }

    /// New unique identifier of the custom emoji shown as the topic icon. Use [`crate::methods::GetForumTopicIconStickers`] to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    #[must_use]
    pub fn icon_custom_emoji_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.icon_custom_emoji_id = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for EditForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editForumTopic", self, None)
    }
}
