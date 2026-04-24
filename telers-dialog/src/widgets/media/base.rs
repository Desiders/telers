//! Base media trait and types.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use telers::types::InputFile;

use crate::entities::RenderContext;

/// Type of media content.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaContentType {
    /// Photo image.
    Photo,
    /// Video file.
    Video,
    /// Audio file.
    Audio,
    /// Document/file.
    Document,
    /// Animation (GIF or H.264 without sound).
    Animation,
    /// Voice message.
    Voice,
    /// Video note (round video).
    VideoNote,
}

impl MediaContentType {
    /// Returns true if this media type can be edited in place via `editMessageMedia`.
    #[must_use]
    pub const fn can_edit_in_place(&self) -> bool {
        matches!(
            self,
            Self::Photo | Self::Video | Self::Audio | Self::Document | Self::Animation
        )
    }
}

/// Identifier for a Telegram media file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaId {
    /// The file ID for reusing the file.
    pub file_id: Cow<'static, str>,
    /// The unique file ID for deduplication.
    pub file_unique_id: Option<Cow<'static, str>>,
}

impl MediaId {
    /// Create a new media ID from a file ID.
    #[must_use]
    pub fn new(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: None,
        }
    }

    /// Create a new media ID with both file ID and unique ID.
    #[must_use]
    pub fn with_unique_id(
        file_id: impl Into<Cow<'static, str>>,
        file_unique_id: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: Some(file_unique_id.into()),
        }
    }
}

/// Rendered media attachment ready to be sent.
#[derive(Clone, Debug)]
pub struct MediaAttachment {
    /// The type of media content.
    pub content_type: MediaContentType,
    /// URL to fetch the media from.
    pub url: Option<Cow<'static, str>>,
    /// Local file path.
    pub path: Option<Cow<'static, str>>,
    /// Existing Telegram file ID.
    pub file_id: Option<MediaId>,
    /// Caption for the media.
    pub caption: Option<Cow<'static, str>>,
    /// Parse mode for the caption.
    pub parse_mode: Option<Cow<'static, str>>,
    /// Whether to show the caption above the media.
    pub show_caption_above_media: Option<bool>,
    /// Whether to mark media as spoiler.
    pub has_spoiler: Option<bool>,
    /// Width in pixels (for video/animation).
    pub width: Option<i64>,
    /// Height in pixels (for video/animation).
    pub height: Option<i64>,
    /// Duration in seconds (for audio/video/animation).
    pub duration: Option<i64>,
    /// Performer name (for audio).
    pub performer: Option<Cow<'static, str>>,
    /// Title (for audio).
    pub title: Option<Cow<'static, str>>,
    /// Whether video supports streaming.
    pub supports_streaming: Option<bool>,
}

impl MediaAttachment {
    /// Create a new photo attachment from a URL.
    #[must_use]
    pub fn photo_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Photo,
            url: Some(url.into()),
            path: None,
            file_id: None,
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

    /// Create a new photo attachment from a file ID.
    #[must_use]
    pub fn photo_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Photo,
            url: None,
            path: None,
            file_id: Some(MediaId::new(file_id)),
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

    /// Create a new video attachment from a URL.
    #[must_use]
    pub fn video_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Video,
            url: Some(url.into()),
            path: None,
            file_id: None,
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

    /// Create a new video attachment from a file ID.
    #[must_use]
    pub fn video_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Video,
            url: None,
            path: None,
            file_id: Some(MediaId::new(file_id)),
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

    /// Create a new document attachment from a URL.
    #[must_use]
    pub fn document_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Document,
            url: Some(url.into()),
            path: None,
            file_id: None,
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

    /// Create a new document attachment from a file ID.
    #[must_use]
    pub fn document_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Document,
            url: None,
            path: None,
            file_id: Some(MediaId::new(file_id)),
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

    /// Create a new audio attachment from a URL.
    #[must_use]
    pub fn audio_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Audio,
            url: Some(url.into()),
            path: None,
            file_id: None,
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

    /// Create a new audio attachment from a file ID.
    #[must_use]
    pub fn audio_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Audio,
            url: None,
            path: None,
            file_id: Some(MediaId::new(file_id)),
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

    /// Create a new animation attachment from a URL.
    #[must_use]
    pub fn animation_url(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Animation,
            url: Some(url.into()),
            path: None,
            file_id: None,
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

    /// Create a new animation attachment from a file ID.
    #[must_use]
    pub fn animation_id(file_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content_type: MediaContentType::Animation,
            url: None,
            path: None,
            file_id: Some(MediaId::new(file_id)),
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

    /// Create a media attachment with custom content type.
    #[must_use]
    pub fn new(content_type: MediaContentType) -> Self {
        Self {
            content_type,
            url: None,
            path: None,
            file_id: None,
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

    /// Set the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<Cow<'static, str>>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the local file path.
    #[must_use]
    pub fn path(mut self, path: impl Into<Cow<'static, str>>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Set the file ID.
    #[must_use]
    pub fn file_id(mut self, file_id: MediaId) -> Self {
        self.file_id = Some(file_id);
        self
    }

    /// Set the caption.
    #[must_use]
    pub fn caption(mut self, caption: impl Into<Cow<'static, str>>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set the parse mode.
    #[must_use]
    pub fn parse_mode(mut self, parse_mode: impl Into<Cow<'static, str>>) -> Self {
        self.parse_mode = Some(parse_mode.into());
        self
    }

    /// Set whether to show caption above media.
    #[must_use]
    pub fn show_caption_above_media(mut self, show: bool) -> Self {
        self.show_caption_above_media = Some(show);
        self
    }

    /// Set whether media has spoiler.
    #[must_use]
    pub fn has_spoiler(mut self, spoiler: bool) -> Self {
        self.has_spoiler = Some(spoiler);
        self
    }

    /// Set width in pixels.
    #[must_use]
    pub fn width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height in pixels.
    #[must_use]
    pub fn height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set duration in seconds.
    #[must_use]
    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set performer (for audio).
    #[must_use]
    pub fn performer(mut self, performer: impl Into<Cow<'static, str>>) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Set title (for audio).
    #[must_use]
    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set whether video supports streaming.
    #[must_use]
    pub fn supports_streaming(mut self, supports: bool) -> Self {
        self.supports_streaming = Some(supports);
        self
    }

    /// Convert to an `InputFile` for sending.
    ///
    /// Returns `None` if no valid source is available.
    #[must_use]
    pub fn to_input_file(&self) -> Option<InputFile> {
        if let Some(ref id) = self.file_id {
            return Some(InputFile::id(id.file_id.as_ref()));
        }
        if let Some(ref url) = self.url {
            return Some(InputFile::url(url.as_ref()));
        }
        if let Some(ref path) = self.path {
            return Some(InputFile::fs(path.as_ref()));
        }
        None
    }

    /// Get the file ID if available.
    #[must_use]
    pub fn get_file_id(&self) -> Option<&str> {
        self.file_id.as_ref().map(|id| id.file_id.as_ref())
    }

    /// Get the unique file ID if available.
    #[must_use]
    pub fn get_file_unique_id(&self) -> Option<&str> {
        self.file_id
            .as_ref()
            .and_then(|id| id.file_unique_id.as_deref())
    }
}

/// Trait for widgets that render media content.
#[async_trait]
pub trait Media: Send + Sync + 'static {
    /// Render media content.
    ///
    /// Returns `None` if no media should be shown.
    async fn render_media(&self, render_ctx: &RenderContext) -> Option<MediaAttachment>;

    #[cfg(test)]
    async fn render_media_for_test(
        &self,
        ctx: &crate::entities::Context,
        data: &crate::entities::DataMap,
    ) -> Option<MediaAttachment> {
        use crate::entities::{ChatEvent, EventContext};
        use telers::{
            client::Reqwest,
            types::{ChatPrivate, MessageText, User},
            Bot,
        };

        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatPrivate::new(10), "/test")
                .from(User::new(10, false, "tester"))
                .into(),
        );
        let event_context = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = crate::entities::RenderContext::new(ctx, data, &event, &event_context);
        self.render_media(&render_ctx).await
    }
}

/// A multi-media container that tries each media in order.
///
/// Returns the first non-None result.
pub struct MultiMedia {
    widgets: Vec<Box<dyn Media>>,
}

impl MultiMedia {
    /// Create a new empty multi-media container.
    #[must_use]
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    /// Add a boxed media widget.
    #[must_use]
    pub fn media_boxed(mut self, media: Box<dyn Media>) -> Self {
        self.widgets.push(media);
        self
    }

    /// Add a media widget.
    #[must_use]
    pub fn media(self, media: impl Media) -> Self {
        self.media_boxed(Box::new(media))
    }
}

impl Default for MultiMedia {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Media for MultiMedia {
    async fn render_media(&self, render_ctx: &RenderContext) -> Option<MediaAttachment> {
        for widget in &self.widgets {
            if let Some(attachment) = widget.render_media(render_ctx).await {
                return Some(attachment);
            }
        }
        None
    }
}
