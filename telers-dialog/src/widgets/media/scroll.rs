//! Media scroll widget for paginated media content.

use async_trait::async_trait;
use bon::bon;
use serde_json::Value;
use std::sync::Arc;

use super::{Media, MediaAttachment, MediaContentType, MediaId};
use crate::{
    entities::{DataMap, RenderContext},
    widgets::kbd::{BaseScroll, OnPageChanged, Scroll},
};

/// A function that gets media items from render data.
pub type MediaItemsGetter<T> = Arc<dyn Fn(&DataMap) -> Vec<T> + Send + Sync + 'static>;

/// A function that converts an item to a media attachment.
pub type MediaItemRenderer<T> =
    Arc<dyn Fn(&T, &DataMap) -> MediaAttachment + Send + Sync + 'static>;

/// A scrollable media widget that displays one media item at a time.
///
/// Works with [`ScrollingGroup`] and pager widgets to allow navigation
/// through a collection of media items.
///
/// [`ScrollingGroup`]: crate::widgets::ScrollingGroup
pub struct MediaScroll<T>
where
    T: Clone + Send + Sync + 'static,
{
    scroll: BaseScroll,
    items_getter: MediaItemsGetter<T>,
    item_renderer: MediaItemRenderer<T>,
}

#[bon]
impl<T> MediaScroll<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Create a new media scroll widget.
    ///
    /// The `id` should match the scroll widget ID used for paging.
    #[builder]
    #[must_use]
    pub fn new<G, R>(
        #[builder(start_fn)] id: impl Into<String>,
        items_getter: G,
        item_renderer: R,
        on_page_changed: Option<OnPageChanged>,
    ) -> Self
    where
        G: Fn(&DataMap) -> Vec<T> + Send + Sync + 'static,
        R: Fn(&T, &DataMap) -> MediaAttachment + Send + Sync + 'static,
    {
        Self {
            scroll: BaseScroll::new(id.into(), on_page_changed),
            items_getter: Arc::new(items_getter),
            item_renderer: Arc::new(item_renderer),
        }
    }
}

impl MediaScroll<String> {
    /// Read a string array from `field` and render each entry as a URL-sourced
    /// attachment with `content_type`.
    #[must_use]
    pub fn from_url_array_field(
        id: impl Into<String>,
        content_type: MediaContentType,
        field: impl Into<String>,
    ) -> Self {
        let field = field.into();
        MediaScroll::builder(id)
            .items_getter(move |data: &DataMap| string_array_from_field(data, &field))
            .item_renderer(move |url, _data| {
                MediaAttachment::builder(content_type)
                    .url(url.clone())
                    .build()
            })
            .build()
    }

    /// Read a string array from `field` and render each entry as a file-id-sourced
    /// attachment with `content_type`.
    #[must_use]
    pub fn from_file_id_array_field(
        id: impl Into<String>,
        content_type: MediaContentType,
        field: impl Into<String>,
    ) -> Self {
        let field = field.into();
        MediaScroll::builder(id)
            .items_getter(move |data: &DataMap| string_array_from_field(data, &field))
            .item_renderer(move |file_id, _data| {
                MediaAttachment::builder(content_type)
                    .file_id(MediaId::new(file_id.clone()))
                    .build()
            })
            .build()
    }
}

fn string_array_from_field(data: &DataMap, field: &str) -> Vec<String> {
    data.get(field)
        .and_then(Value::as_array)
        .map(|val| {
            val.iter()
                .filter_map(|item| item.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

#[async_trait]
impl<T> Media for MediaScroll<T>
where
    T: Clone + Send + Sync + 'static,
{
    async fn render_media(&self, render_ctx: &RenderContext) -> Option<MediaAttachment> {
        let items = (self.items_getter)(render_ctx.data.as_ref());
        if items.is_empty() {
            return None;
        }

        let page = self.scroll.get_page(render_ctx.context.as_ref());
        let index = page.min(items.len().saturating_sub(1));

        items
            .get(index)
            .map(|item| (self.item_renderer)(item, render_ctx.data.as_ref()))
    }
}

#[async_trait]
impl<T> Scroll for MediaScroll<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn base_scroll(&self) -> &BaseScroll {
        &self.scroll
    }

    async fn get_page_count(&self, render_ctx: RenderContext) -> usize {
        (self.items_getter)(render_ctx.data.as_ref()).len()
    }
}
