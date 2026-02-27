use crate::client::Bot;
use serde::Serialize;
/// Use this method to create a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator right. Returns information about the created topic as a [`ForumTopic`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#createforumtopic>
/// # Returns
/// - `crate::types::ForumTopic`
#[derive(Clone, Debug, Serialize)]
pub struct CreateForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Topic name, 1-128 characters
    pub name: Box<str>,
    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,
    /// Unique identifier of the custom emoji shown as the topic icon. Use get[`ForumTopicIconStickers`] to get all allowed custom emoji identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<Box<str>>,
}
impl CreateForumTopic {
    /// Creates a new `CreateForumTopic`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    /// * `name` - Topic name, 1-128 characters
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<Box<str>>>(
        chat_id: T0,
        name: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
            icon_color: None,
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

    /// Topic name, 1-128 characters
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }

    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    #[must_use]
    pub fn icon_color<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.icon_color = Some(val.into());
        this
    }

    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    #[must_use]
    pub fn icon_color_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.icon_color = val.map(Into::into);
        this
    }

    /// Unique identifier of the custom emoji shown as the topic icon. Use get[`ForumTopicIconStickers`] to get all allowed custom emoji identifiers.
    #[must_use]
    pub fn icon_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.icon_custom_emoji_id = Some(val.into());
        this
    }

    /// Unique identifier of the custom emoji shown as the topic icon. Use get[`ForumTopicIconStickers`] to get all allowed custom emoji identifiers.
    #[must_use]
    pub fn icon_custom_emoji_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.icon_custom_emoji_id = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for CreateForumTopic {
    type Method = Self;
    type Return = crate::types::ForumTopic;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("createForumTopic", self, None)
    }
}
