//! Dynamic media widget that reads media from render data.

use async_trait::async_trait;
use bon::bon;
use serde::Deserialize;
use serde_json::Value;
use std::{borrow::Cow, sync::Arc};

use super::{Media, MediaAttachment, MediaContentType, MediaId};
use crate::entities::{DataMap, RenderContext};

/// A function that selects media from render data.
pub type MediaSelector = Arc<dyn Fn(&DataMap) -> Option<MediaAttachment> + Send + Sync + 'static>;

/// A dynamic media widget that reads a media attachment from render data.
///
/// The selector receives render data and returns an optional [`MediaAttachment`],
/// so the media source can be chosen at runtime from dialog state.
pub struct DynamicMedia {
    selector: MediaSelector,
}

#[bon]
impl DynamicMedia {
    /// Build a dynamic media widget around an arbitrary selector closure.
    #[builder]
    #[must_use]
    pub fn new<F>(#[builder(start_fn)] selector: F) -> Self
    where
        F: Fn(&DataMap) -> Option<MediaAttachment> + Send + Sync + 'static,
    {
        Self {
            selector: Arc::new(selector),
        }
    }

    /// Read a full [`MediaAttachment`] JSON object from `field`.
    #[must_use]
    pub fn from_field(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::builder(move |data| {
            data.get(&field)
                .and_then(|val| serde_json::from_value::<MediaAttachmentData>(val.clone()).ok())
                .map(MediaAttachmentData::into_attachment)
        })
        .build()
    }

    /// Read a URL string from `field` and wrap it as `content_type` media.
    #[must_use]
    pub fn from_url_field(content_type: MediaContentType, field: impl Into<String>) -> Self {
        let field = field.into();
        Self::builder(move |data| {
            data.get(&field).and_then(Value::as_str).map(|url| {
                MediaAttachment::builder(content_type)
                    .url(url.to_owned())
                    .build()
            })
        })
        .build()
    }

    /// Read a Telegram file id string from `field` and wrap it as `content_type` media.
    #[must_use]
    pub fn from_file_id_field(content_type: MediaContentType, field: impl Into<String>) -> Self {
        let field = field.into();
        Self::builder(move |data| {
            data.get(&field).and_then(Value::as_str).map(|id| {
                MediaAttachment::builder(content_type)
                    .file_id(MediaId::new(id.to_owned()))
                    .build()
            })
        })
        .build()
    }
}

#[async_trait]
impl Media for DynamicMedia {
    async fn render_media(&self, render_ctx: &RenderContext) -> Option<MediaAttachment> {
        (self.selector)(render_ctx.data.as_ref())
    }
}

#[derive(Deserialize)]
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
