//! Dynamic media widget that reads media from render data.

use std::sync::Arc;

use async_trait::async_trait;

use super::{Media, MediaAttachment};
use crate::entities::{DataMap, RenderContext};

/// A function that selects media from render data.
pub type MediaSelector = Arc<dyn Fn(&DataMap) -> Option<MediaAttachment> + Send + Sync + 'static>;

/// A dynamic media widget that reads media attachment from render data.
///
/// This allows the media to be determined at runtime based on dialog state
/// or other dynamic data.
pub struct DynamicMedia {
    selector: MediaSelector,
}

impl DynamicMedia {
    /// Create a dynamic media widget with a selector function.
    ///
    /// The selector receives render data and returns an optional `MediaAttachment`.
    #[must_use]
    pub fn new<F>(selector: F) -> Self
    where
        F: Fn(&DataMap) -> Option<MediaAttachment> + Send + Sync + 'static,
    {
        Self {
            selector: Arc::new(selector),
        }
    }

    /// Create a dynamic media widget that reads from a specific data field.
    ///
    /// The field should contain a `MediaAttachment` serialized as JSON.
    #[must_use]
    pub fn from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::new(move |data| {
            data.get(&field)
                .and_then(|v| serde_json::from_value::<MediaAttachmentData>(v.clone()).ok())
                .map(|d| d.into_attachment())
        })
    }

    /// Create a dynamic media widget that reads URL from a data field.
    ///
    /// The field should contain a string URL.
    #[must_use]
    pub fn photo_url_from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::new(move |data| {
            data.get(&field)
                .and_then(serde_json::Value::as_str)
                .map(|url| MediaAttachment::photo_url(url.to_string()))
        })
    }

    /// Create a dynamic media widget that reads file ID from a data field.
    ///
    /// The field should contain a string file ID.
    #[must_use]
    pub fn photo_id_from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::new(move |data| {
            data.get(&field)
                .and_then(serde_json::Value::as_str)
                .map(|id| MediaAttachment::photo_id(id.to_string()))
        })
    }

    /// Create a dynamic media widget that reads video URL from a data field.
    #[must_use]
    pub fn video_url_from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::new(move |data| {
            data.get(&field)
                .and_then(serde_json::Value::as_str)
                .map(|url| MediaAttachment::video_url(url.to_string()))
        })
    }

    /// Create a dynamic media widget that reads video file ID from a data field.
    #[must_use]
    pub fn video_id_from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::new(move |data| {
            data.get(&field)
                .and_then(serde_json::Value::as_str)
                .map(|id| MediaAttachment::video_id(id.to_string()))
        })
    }

    /// Create a dynamic media widget that reads document URL from a data field.
    #[must_use]
    pub fn document_url_from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::new(move |data| {
            data.get(&field)
                .and_then(serde_json::Value::as_str)
                .map(|url| MediaAttachment::document_url(url.to_string()))
        })
    }

    /// Create a dynamic media widget that reads document file ID from a data field.
    #[must_use]
    pub fn document_id_from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::new(move |data| {
            data.get(&field)
                .and_then(serde_json::Value::as_str)
                .map(|id| MediaAttachment::document_id(id.to_string()))
        })
    }
}

#[async_trait]
impl Media for DynamicMedia {
    async fn render_media(&self, render_ctx: &RenderContext) -> Option<MediaAttachment> {
        (self.selector)(render_ctx.data.as_ref())
    }
}

/// Helper struct for deserializing media attachment from JSON.
#[derive(serde::Deserialize)]
struct MediaAttachmentData {
    content_type: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    file_id: Option<String>,
    #[serde(default)]
    file_unique_id: Option<String>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    caption: Option<String>,
    #[serde(default)]
    parse_mode: Option<String>,
    #[serde(default)]
    show_caption_above_media: Option<bool>,
    #[serde(default)]
    has_spoiler: Option<bool>,
    #[serde(default)]
    width: Option<i64>,
    #[serde(default)]
    height: Option<i64>,
    #[serde(default)]
    duration: Option<i64>,
    #[serde(default)]
    performer: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    supports_streaming: Option<bool>,
}

impl MediaAttachmentData {
    fn into_attachment(self) -> MediaAttachment {
        use super::{MediaContentType, MediaId};
        use std::borrow::Cow;

        let content_type = match self.content_type.to_lowercase().as_str() {
            "photo" => MediaContentType::Photo,
            "video" => MediaContentType::Video,
            "audio" => MediaContentType::Audio,
            "document" => MediaContentType::Document,
            "animation" => MediaContentType::Animation,
            "voice" => MediaContentType::Voice,
            "video_note" | "videonote" => MediaContentType::VideoNote,
            _ => MediaContentType::Photo,
        };

        let file_id = self.file_id.map(|id| {
            if let Some(unique_id) = self.file_unique_id {
                MediaId::with_unique_id(id, unique_id)
            } else {
                MediaId::new(id)
            }
        });

        MediaAttachment {
            content_type,
            url: self.url.map(Cow::Owned),
            path: self.path.map(Cow::Owned),
            file_id,
            caption: self.caption.map(Cow::Owned),
            parse_mode: self.parse_mode.map(Cow::Owned),
            show_caption_above_media: self.show_caption_above_media,
            has_spoiler: self.has_spoiler,
            width: self.width,
            height: self.height,
            duration: self.duration,
            performer: self.performer.map(Cow::Owned),
            title: self.title.map(Cow::Owned),
            supports_streaming: self.supports_streaming,
        }
    }
}
