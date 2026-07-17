use serde::{Deserialize, Serialize};
/// Message is a scheduled giveaway, information about the giveaway
/// # Notes
/// This object represents an external reply info from original field `giveaway`.
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalReplyInfoGiveaway {
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
    /// Message is a scheduled giveaway, information about the giveaway
    pub giveaway: crate::types::Giveaway,
}
impl ExternalReplyInfoGiveaway {
    /// Creates a new `ExternalReplyInfoGiveaway`.
    ///
    /// # Arguments
    /// * `origin` - Origin of the message replied to by the given message
    /// * `giveaway` - Message is a scheduled giveaway, information about the giveaway
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::MessageOrigin>, T1: Into<crate::types::Giveaway>>(
        origin: T0,
        giveaway: T1,
    ) -> Self {
        Self {
            origin: origin.into(),
            chat: None,
            message_id: None,
            link_preview_options: None,
            has_media_spoiler: None,
            giveaway: giveaway.into(),
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

    /// Message is a scheduled giveaway, information about the giveaway
    #[must_use]
    pub fn giveaway<T: Into<crate::types::Giveaway>>(mut self, val: T) -> Self {
        self.giveaway = val.into();
        self
    }
}
