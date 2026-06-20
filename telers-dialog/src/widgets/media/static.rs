//! Static media widget for fixed URL or file ID media.

use std::{borrow::Cow, sync::Arc};

use async_trait::async_trait;
use bon::bon;

use super::{Media, MediaAttachment, MediaContentType, MediaId};
use crate::{entities::RenderContext, widgets::Text};

/// A static media widget with fixed URL or file ID.
///
/// The media source can be a URL, file ID, or local path.
/// The caption can be static or rendered from a [`Text`] widget.
///
/// Construct via [`StaticMedia::builder`].
pub struct StaticMedia {
    content_type: MediaContentType,
    url: Option<Cow<'static, str>>,
    file_id: Option<MediaId>,
    path: Option<Cow<'static, str>>,
    caption: Option<Arc<dyn Text>>,
    parse_mode: Option<Cow<'static, str>>,
    show_caption_above_media: Option<bool>,
    has_spoiler: Option<bool>,
    width: Option<i64>,
    height: Option<i64>,
    duration: Option<i64>,
    performer: Option<Cow<'static, str>>,
    title: Option<Cow<'static, str>>,
    supports_streaming: Option<bool>,
}

#[bon]
impl StaticMedia {
    /// Create a static media widget for the given content type.
    ///
    /// At least one of [`url`], [`file_id`], or [`path`] must be set, otherwise
    /// rendering returns `None`.
    ///
    /// [`url`]: StaticMediaBuilder::url
    /// [`file_id`]: StaticMediaBuilder::file_id
    /// [`path`]: StaticMediaBuilder::path
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] content_type: MediaContentType,
        #[builder(into)] url: Option<Cow<'static, str>>,
        file_id: Option<MediaId>,
        #[builder(into)] path: Option<Cow<'static, str>>,
        #[builder(with = |caption: impl Text| Arc::new(caption))] caption: Option<Arc<dyn Text>>,
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
            file_id,
            path,
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
}

#[async_trait]
impl Media for StaticMedia {
    async fn render_media(&self, render_ctx: &RenderContext) -> Option<MediaAttachment> {
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
