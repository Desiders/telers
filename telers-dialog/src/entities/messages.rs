use telers::{
    enums::{MessageType, ReplyMarkupType},
    types::{Chat, LinkPreviewOptions, ReplyMarkup},
};

use super::ShowMode;
use crate::widgets::media::{MediaAttachment, MediaContentType};

/// Snapshot of the last rendered dialog message.
#[derive(Clone, Debug)]
pub struct OldMessage {
    /// Target chat.
    pub chat: Chat,
    /// Telegram message id.
    pub message_id: i64,
    /// Rendered text snapshot.
    pub text: Option<Box<str>>,
    /// Protected-content flag recorded for the sent message.
    pub has_protected_content: Option<bool>,
    /// Reply markup kind stored for edit/delete decisions.
    pub reply_markup_type: Option<ReplyMarkupType>,
    /// Serialized reply markup snapshot.
    pub reply_markup_value: Option<serde_json::Value>,
    /// Business connection id used to send the message.
    pub business_connection_id: Option<Box<str>>,
    /// Telegram message type snapshot when available.
    pub message_type: Option<MessageType>,
    /// Serialized link preview options snapshot.
    pub link_preview_options_value: Option<serde_json::Value>,
    /// Media file ID from the last message.
    pub media_file_id: Option<Box<str>>,
    /// Media unique ID from the last message.
    pub media_unique_id: Option<Box<str>>,
    /// Media content type from the last message.
    pub media_content_type: Option<MediaContentType>,
}

impl OldMessage {
    #[expect(
        clippy::too_many_arguments,
        reason = "This constructor intentionally mirrors the persisted old-message snapshot \
                  fields."
    )]
    #[must_use]
    pub fn new(
        chat: impl Into<Chat>,
        message_id: i64,
        text: Option<impl Into<Box<str>>>,
        has_protected_content: Option<bool>,
        reply_markup_type: Option<ReplyMarkupType>,
        reply_markup_value: Option<serde_json::Value>,
        business_connection_id: Option<impl Into<Box<str>>>,
        message_type: Option<MessageType>,
        link_preview_options_value: Option<serde_json::Value>,
    ) -> Self {
        let chat = chat.into();
        Self {
            chat,
            message_id,
            text: text.map(Into::into),
            has_protected_content,
            reply_markup_type,
            reply_markup_value,
            business_connection_id: business_connection_id.map(Into::into),
            message_type,
            link_preview_options_value,
            media_file_id: None,
            media_unique_id: None,
            media_content_type: None,
        }
    }

    /// Set media information from a sent message.
    #[must_use]
    pub fn with_media(
        mut self,
        file_id: Option<impl Into<Box<str>>>,
        unique_id: Option<impl Into<Box<str>>>,
        content_type: Option<MediaContentType>,
    ) -> Self {
        self.media_file_id = file_id.map(Into::into);
        self.media_unique_id = unique_id.map(Into::into);
        self.media_content_type = content_type;
        self
    }

    /// Returns true if the old message contained media.
    #[must_use]
    pub fn has_media(&self) -> bool {
        self.media_file_id.is_some()
    }
}

/// Rendered dialog message ready to be shown via Telegram.
#[derive(Clone, Debug)]
pub struct NewMessage {
    /// Target chat.
    pub chat: Chat,
    /// Optional message thread id.
    pub message_thread_id: Option<i64>,
    /// Optional business connection id.
    pub business_connection_id: Option<Box<str>>,
    /// Message text (used as caption when media is present).
    pub text: Box<str>,
    /// Reply markup to attach.
    pub reply_markup: Option<ReplyMarkup>,
    /// Parse mode used for text rendering.
    pub parse_mode: Option<Box<str>>,
    /// Protected-content flag.
    pub protect_content: Option<bool>,
    /// Requested show mode.
    pub show_mode: ShowMode,
    /// Link preview options.
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Media attachment to send.
    pub media: Option<MediaAttachment>,
}

impl NewMessage {
    #[expect(
        clippy::too_many_arguments,
        reason = "This constructor intentionally mirrors the rendered telegram message fields."
    )]
    #[must_use]
    pub fn new(
        chat: impl Into<Chat>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<impl Into<Box<str>>>,
        text: impl Into<Box<str>>,
        reply_markup: Option<impl Into<ReplyMarkup>>,
        parse_mode: Option<impl Into<Box<str>>>,
        protect_content: Option<bool>,
        show_mode: ShowMode,
        link_preview_options: Option<LinkPreviewOptions>,
    ) -> Self {
        let chat = chat.into();
        let text = text.into();
        Self {
            chat,
            message_thread_id,
            business_connection_id: business_connection_id.map(Into::into),
            text,
            reply_markup: reply_markup.map(Into::into),
            parse_mode: parse_mode.map(Into::into),
            protect_content,
            show_mode,
            link_preview_options,
            media: None,
        }
    }

    /// Set media attachment.
    #[must_use]
    pub fn with_media(mut self, media: Option<MediaAttachment>) -> Self {
        self.media = media;
        self
    }

    /// Returns true if this message contains media.
    #[must_use]
    pub fn has_media(&self) -> bool {
        self.media.is_some()
    }
}
