use serde::{Deserialize, Serialize};
/// Message is a sticker, information about the sticker
/// # Notes
/// This object represents an external reply info from original field `sticker`.
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalReplyInfoSticker {
    /// Origin of the message replied to by the given message
    pub origin: crate::types::MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Box<crate::types::Chat>>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Options used for link preview generation for the original message, if it is a text message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
    /// `true`, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Message is a sticker, information about the sticker
    pub sticker: Box<crate::types::Sticker>,
}
impl ExternalReplyInfoSticker {
    /// Creates a new `ExternalReplyInfoSticker`.
    ///
    /// # Arguments
    /// * `origin` - Origin of the message replied to by the given message
    /// * `sticker` - Message is a sticker, information about the sticker
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::MessageOrigin>, T1: Into<crate::types::Sticker>>(
        origin: T0,
        sticker: T1,
    ) -> Self {
        Self {
            origin: origin.into(),
            chat: None,
            message_id: None,
            link_preview_options: None,
            has_media_spoiler: None,
            sticker: Box::new(sticker.into()),
        }
    }

    /// Origin of the message replied to by the given message
    #[must_use]
    pub fn origin<T: Into<crate::types::MessageOrigin>>(mut self, val: T) -> Self {
        self.origin = val.into();
        self
    }

    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Some(Box::new(val.into()));
        self
    }

    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[must_use]
    pub fn chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = Some(val.into());
        self
    }

    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_id = val.map(Into::into);
        self
    }

    /// Options used for link preview generation for the original message, if it is a text message
    #[must_use]
    pub fn link_preview_options<T: Into<crate::types::LinkPreviewOptions>>(
        mut self,
        val: T,
    ) -> Self {
        self.link_preview_options = Some(val.into());
        self
    }

    /// Options used for link preview generation for the original message, if it is a text message
    #[must_use]
    pub fn link_preview_options_option<T: Into<crate::types::LinkPreviewOptions>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.link_preview_options = val.map(Into::into);
        self
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_media_spoiler = Some(val.into());
        self
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_media_spoiler = val.map(Into::into);
        self
    }

    /// Message is a sticker, information about the sticker
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(mut self, val: T) -> Self {
        self.sticker = Box::new(val.into());
        self
    }
}
