use telers::{
    enums::{MessageType, ReplyMarkupType},
    types::{Chat, LinkPreviewOptions, ReplyMarkup},
};

use super::ShowMode;
use crate::widgets::media::MediaAttachment;

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
    pub media_content_type: Option<MessageType>,
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
        content_type: Option<MessageType>,
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

#[cfg(test)]
mod tests {
    use super::{NewMessage, OldMessage};
    use crate::entities::ShowMode;
    use serde_json::json;
    use telers::{
        enums::{MessageType, ReplyMarkupType},
        types::ChatPrivate,
    };

    #[test]
    fn old_message_new_stores_every_field() {
        let message = OldMessage::new(
            ChatPrivate::new(1),
            10,
            Some("hi"),
            Some(true),
            Some(ReplyMarkupType::InlineKeyboardMarkup),
            Some(json!({"k": "v"})),
            Some("bc"),
            Some(MessageType::Text),
            Some(json!({"u": "x"})),
        );

        assert_eq!(message.chat.id(), 1);
        assert_eq!(message.message_id, 10);
        assert_eq!(message.text.as_deref(), Some("hi"));
        assert_eq!(message.has_protected_content, Some(true));
        assert_eq!(
            message.reply_markup_type,
            Some(ReplyMarkupType::InlineKeyboardMarkup)
        );
        assert_eq!(message.reply_markup_value, Some(json!({"k": "v"})));
        assert_eq!(message.business_connection_id.as_deref(), Some("bc"));
        assert_eq!(message.message_type, Some(MessageType::Text));
        assert_eq!(message.link_preview_options_value, Some(json!({"u": "x"})));
        assert!(!message.has_media());
        assert!(message.media_file_id.is_none());
        assert!(message.media_unique_id.is_none());
        assert!(message.media_content_type.is_none());
    }

    #[test]
    fn old_message_new_none_optionals() {
        let message = OldMessage::new(
            ChatPrivate::new(1),
            5,
            None::<&str>,
            None,
            None,
            None,
            None::<Box<str>>,
            None,
            None,
        );

        assert_eq!(message.message_id, 5);
        assert!(message.text.is_none());
        assert!(message.has_protected_content.is_none());
        assert!(message.message_type.is_none());
        assert!(message.business_connection_id.is_none());
        assert!(!message.has_media());
    }

    #[test]
    fn old_message_with_media_sets_fields() {
        let message = OldMessage::new(
            ChatPrivate::new(1),
            10,
            Some("hi"),
            Some(true),
            None,
            None,
            None::<Box<str>>,
            Some(MessageType::Text),
            None,
        )
        .with_media(Some("fileid"), Some("uniq"), Some(MessageType::Photo));

        assert!(message.has_media());
        assert_eq!(message.media_file_id.as_deref(), Some("fileid"));
        assert_eq!(message.media_unique_id.as_deref(), Some("uniq"));
        assert_eq!(message.media_content_type, Some(MessageType::Photo));
        assert_eq!(message.message_id, 10);
        assert_eq!(message.text.as_deref(), Some("hi"));
    }

    #[test]
    fn old_message_with_media_none_keeps_no_media() {
        let message = OldMessage::new(
            ChatPrivate::new(1),
            10,
            Some("hi"),
            None,
            None,
            None,
            None::<Box<str>>,
            Some(MessageType::Text),
            None,
        )
        .with_media(None::<Box<str>>, None::<Box<str>>, None);

        assert!(!message.has_media());
        assert!(message.media_file_id.is_none());
        assert!(message.media_unique_id.is_none());
        assert!(message.media_content_type.is_none());
    }

    #[test]
    fn new_message_new_basic_fields() {
        let message = NewMessage::new(
            ChatPrivate::new(1),
            None,
            None::<Box<str>>,
            "text",
            None::<telers::types::ReplyMarkup>,
            None::<Box<str>>,
            None,
            ShowMode::Auto,
            None,
        );

        assert_eq!(message.chat.id(), 1);
        assert_eq!(&*message.text, "text");
        assert_eq!(message.show_mode, ShowMode::Auto);
        assert!(!message.has_media());
        assert!(message.media.is_none());
        assert!(message.message_thread_id.is_none());
        assert!(message.business_connection_id.is_none());
        assert!(message.reply_markup.is_none());
        assert!(message.parse_mode.is_none());
        assert!(message.protect_content.is_none());
        assert!(message.link_preview_options.is_none());
    }

    #[test]
    fn new_message_default_media_is_none() {
        let message = NewMessage::new(
            ChatPrivate::new(1),
            Some(7),
            None::<Box<str>>,
            "hello",
            None::<telers::types::ReplyMarkup>,
            None::<Box<str>>,
            Some(false),
            ShowMode::Edit,
            None,
        );

        assert!(!message.has_media());
        assert_eq!(message.message_thread_id, Some(7));
        assert_eq!(message.protect_content, Some(false));
        assert_eq!(message.show_mode, ShowMode::Edit);
    }

    #[test]
    fn new_message_with_media_none_keeps_no_media() {
        let message = NewMessage::new(
            ChatPrivate::new(1),
            None,
            None::<Box<str>>,
            "text",
            None::<telers::types::ReplyMarkup>,
            None::<Box<str>>,
            None,
            ShowMode::Auto,
            None,
        )
        .with_media(None);

        assert!(!message.has_media());
        assert!(message.media.is_none());
        assert_eq!(&*message.text, "text");
    }
}
