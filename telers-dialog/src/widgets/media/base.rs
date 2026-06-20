//! Base media trait and types.

use async_trait::async_trait;
use bon::bon;
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
#[derive(Clone, Debug, Serialize)]
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

#[bon]
impl MediaAttachment {
    /// Create a media attachment with custom content type.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] content_type: MediaContentType,
        #[builder(into)] url: Option<Cow<'static, str>>,
        #[builder(into)] path: Option<Cow<'static, str>>,
        file_id: Option<MediaId>,
        #[builder(into)] caption: Option<Cow<'static, str>>,
        #[builder(into)] parse_mode: Option<Cow<'static, str>>,
        show_caption_above_media: Option<bool>,
        has_spoiler: Option<bool>,
        width: Option<i64>,
        height: Option<i64>,
        duration: Option<i64>,
        #[builder(into)] performer: Option<Cow<'static, str>>,
        #[builder(into)] title: Option<Cow<'static, str>>,
        supports_streaming: Option<bool>,
    ) -> Self {
        Self {
            content_type,
            url,
            path,
            file_id,
            caption,
            parse_mode,
            show_caption_above_media,
            has_spoiler,
            width,
            height,
            duration,
            performer,
            title,
            supports_streaming,
        }
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

#[bon]
impl MultiMedia {
    /// Create a new empty multi-media container.
    #[builder]
    #[must_use]
    pub fn new(#[builder(field = Vec::new())] widgets: Vec<Box<dyn Media>>) -> Self {
        Self {
            widgets,
        }
    }
}

impl<S> MultiMediaBuilder<S>
where
    S: multi_media_builder::State,
{
    pub fn media(mut self, media: impl Media) -> Self {
        self.widgets.push(Box::new(media));
        self
    }

    pub(crate) fn media_boxed(mut self, media: Box<dyn Media>) -> Self {
        self.widgets.push(media);
        self
    }
}

impl Default for MultiMedia {
    fn default() -> Self {
        Self::builder().build()
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
