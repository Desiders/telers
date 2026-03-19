use telers::{
    enums::{MessageType, ReplyMarkupType},
    types::{Chat, LinkPreviewOptions, ReplyMarkup},
};

use super::ShowMode;

#[derive(Clone, Debug)]
pub struct OldMessage {
    pub chat: Chat,
    pub message_id: i64,
    pub text: Option<Box<str>>,
    pub has_protected_content: Option<bool>,
    pub reply_markup_type: Option<ReplyMarkupType>,
    pub reply_markup_value: Option<serde_json::Value>,
    pub business_connection_id: Option<Box<str>>,
    pub message_type: Option<MessageType>,
    pub link_preview_options_value: Option<serde_json::Value>,
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
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewMessage {
    pub chat: Chat,
    pub message_thread_id: Option<i64>,
    pub business_connection_id: Option<Box<str>>,
    pub text: Box<str>,
    pub reply_markup: Option<ReplyMarkup>,
    pub parse_mode: Option<Box<str>>,
    pub protect_content: Option<bool>,
    pub show_mode: ShowMode,
    pub link_preview_options: Option<LinkPreviewOptions>,
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
        }
    }
}
