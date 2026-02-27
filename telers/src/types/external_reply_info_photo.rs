use serde::{Deserialize, Serialize};
/// Message is a photo, available sizes of the photo
/// # Notes
/// This object represents an external reply info from original field `photo`.
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalReplyInfoPhoto {
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
    /// Message contains paid media; information about the paid media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<crate::types::PaidMediaInfo>,
    /// `true`, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Message is a photo, available sizes of the photo
    pub photo: Box<[crate::types::PhotoSize]>,
}
impl ExternalReplyInfoPhoto {
    /// Creates a new `ExternalReplyInfoPhoto`.
    ///
    /// # Arguments
    /// * `origin` - Origin of the message replied to by the given message
    /// * `photo` - Message is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::MessageOrigin>,
        T1Item: Into<crate::types::PhotoSize>,
        T1: IntoIterator<Item = T1Item>,
    >(
        origin: T0,
        photo: T1,
    ) -> Self {
        Self {
            origin: origin.into(),
            chat: None,
            message_id: None,
            link_preview_options: None,
            paid_media: None,
            has_media_spoiler: None,
            photo: photo.into_iter().map(Into::into).collect(),
        }
    }

    /// Origin of the message replied to by the given message
    #[must_use]
    pub fn origin<T: Into<crate::types::MessageOrigin>>(self, val: T) -> Self {
        let mut this = self;
        this.origin = val.into();
        this
    }

    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Some(Box::new(val.into()));
        this
    }

    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[must_use]
    pub fn chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.chat = val.map(|val| Box::new(val.into()));
        this
    }

    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = Some(val.into());
        this
    }

    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_id = val.map(Into::into);
        this
    }

    /// Options used for link preview generation for the original message, if it is a text message
    #[must_use]
    pub fn link_preview_options<T: Into<crate::types::LinkPreviewOptions>>(self, val: T) -> Self {
        let mut this = self;
        this.link_preview_options = Some(val.into());
        this
    }

    /// Options used for link preview generation for the original message, if it is a text message
    #[must_use]
    pub fn link_preview_options_option<T: Into<crate::types::LinkPreviewOptions>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.link_preview_options = val.map(Into::into);
        this
    }

    /// Message contains paid media; information about the paid media
    #[must_use]
    pub fn paid_media<T: Into<crate::types::PaidMediaInfo>>(self, val: T) -> Self {
        let mut this = self;
        this.paid_media = Some(val.into());
        this
    }

    /// Message contains paid media; information about the paid media
    #[must_use]
    pub fn paid_media_option<T: Into<crate::types::PaidMediaInfo>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.paid_media = val.map(Into::into);
        this
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_media_spoiler = Some(val.into());
        this
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_media_spoiler = val.map(Into::into);
        this
    }

    /// Message is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = this
            .photo
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// Message is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = this
            .photo
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
