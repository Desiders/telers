//! Static media widget for fixed URL or file ID media.

use std::{borrow::Cow, sync::Arc};

use async_trait::async_trait;
use bon::Builder;

use super::{Media, MediaAttachment, MediaContentType, MediaId};
use crate::{entities::RenderContext, widgets::Text};

/// A static media widget with fixed URL or file ID.
///
/// The media source can be a URL, file ID, or local path.
/// The caption can be static or rendered from a `Text` widget.
#[derive(Builder)]
pub struct StaticMedia {
    /// The type of media content.
    #[builder(default = MediaContentType::Photo)]
    content_type: MediaContentType,
    /// URL to fetch the media from.
    #[builder(into)]
    url: Option<Cow<'static, str>>,
    /// Existing Telegram file ID.
    file_id: Option<MediaId>,
    /// Local file path.
    #[builder(into)]
    path: Option<Cow<'static, str>>,
    /// Caption text widget.
    caption: Option<Arc<dyn Text>>,
    /// Parse mode for the caption.
    #[builder(into)]
    parse_mode: Option<Cow<'static, str>>,
    /// Whether to show caption above media.
    show_caption_above_media: Option<bool>,
    /// Whether to mark media as spoiler.
    has_spoiler: Option<bool>,
    /// Width in pixels (for video/animation).
    width: Option<i64>,
    /// Height in pixels (for video/animation).
    height: Option<i64>,
    /// Duration in seconds (for audio/video/animation).
    duration: Option<i64>,
    /// Performer name (for audio).
    #[builder(into)]
    performer: Option<Cow<'static, str>>,
    /// Title (for audio).
    #[builder(into)]
    title: Option<Cow<'static, str>>,
    /// Whether video supports streaming.
    supports_streaming: Option<bool>,
}

impl StaticMedia {
    /// Create a static photo from a URL.
    #[must_use]
    pub fn photo_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Photo,
            url: Some(url.into()),
            file_id: None,
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static photo from a file ID.
    #[must_use]
    pub fn photo_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Photo,
            url: None,
            file_id: Some(MediaId::new(file_id)),
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static video from a URL.
    #[must_use]
    pub fn video_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Video,
            url: Some(url.into()),
            file_id: None,
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static video from a file ID.
    #[must_use]
    pub fn video_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Video,
            url: None,
            file_id: Some(MediaId::new(file_id)),
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static document from a URL.
    #[must_use]
    pub fn document_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Document,
            url: Some(url.into()),
            file_id: None,
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static document from a file ID.
    #[must_use]
    pub fn document_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Document,
            url: None,
            file_id: Some(MediaId::new(file_id)),
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static audio from a URL.
    #[must_use]
    pub fn audio_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Audio,
            url: Some(url.into()),
            file_id: None,
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static audio from a file ID.
    #[must_use]
    pub fn audio_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Audio,
            url: None,
            file_id: Some(MediaId::new(file_id)),
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static animation from a URL.
    #[must_use]
    pub fn animation_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Animation,
            url: Some(url.into()),
            file_id: None,
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Create a static animation from a file ID.
    #[must_use]
    pub fn animation_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Animation,
            url: None,
            file_id: Some(MediaId::new(file_id)),
            path: None,
            caption: None,
            parse_mode: None,
            show_caption_above_media: None,
            has_spoiler: None,
            width: None,
            height: None,
            duration: None,
            performer: None,
            title: None,
            supports_streaming: None,
        }
    }

    /// Set the caption text.
    #[must_use]
    pub fn with_caption(mut self, caption: impl Text) -> Self {
        self.caption = Some(Arc::new(caption));
        self
    }

    /// Set the parse mode.
    #[must_use]
    pub fn with_parse_mode(mut self, parse_mode: impl Into<Cow<'static, str>>) -> Self {
        self.parse_mode = Some(parse_mode.into());
        self
    }

    /// Set whether to show caption above media.
    #[must_use]
    pub fn with_show_caption_above_media(mut self, show: bool) -> Self {
        self.show_caption_above_media = Some(show);
        self
    }

    /// Set whether media has spoiler.
    #[must_use]
    pub fn with_has_spoiler(mut self, spoiler: bool) -> Self {
        self.has_spoiler = Some(spoiler);
        self
    }

    /// Set width in pixels.
    #[must_use]
    pub fn with_width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height in pixels.
    #[must_use]
    pub fn with_height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set duration in seconds.
    #[must_use]
    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set performer (for audio).
    #[must_use]
    pub fn with_performer(mut self, performer: impl Into<Cow<'static, str>>) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Set title (for audio).
    #[must_use]
    pub fn with_title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set whether video supports streaming.
    #[must_use]
    pub fn with_supports_streaming(mut self, supports: bool) -> Self {
        self.supports_streaming = Some(supports);
        self
    }
}

#[async_trait]
impl Media for StaticMedia {
    async fn render_media(&self, render_ctx: &RenderContext) -> Option<MediaAttachment> {
        // Ensure we have at least one source
        if self.url.is_none() && self.file_id.is_none() && self.path.is_none() {
            return None;
        }

        let caption = match &self.caption {
            Some(caption_text) => Some(Cow::Owned(
                caption_text
                    .render_text_in_context(render_ctx)
                    .await
                    .to_string(),
            )),
            None => None,
        };

        Some(MediaAttachment {
            content_type: self.content_type,
            url: self.url.clone(),
            path: self.path.clone(),
            file_id: self.file_id.clone(),
            caption,
            parse_mode: self.parse_mode.clone(),
            show_caption_above_media: self.show_caption_above_media,
            has_spoiler: self.has_spoiler,
            width: self.width,
            height: self.height,
            duration: self.duration,
            performer: self.performer.clone(),
            title: self.title.clone(),
            supports_streaming: self.supports_streaming,
        })
    }
}
