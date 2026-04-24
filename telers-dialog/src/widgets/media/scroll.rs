//! Media scroll widget for paginated media content.

use std::sync::Arc;

use async_trait::async_trait;

use super::{Media, MediaAttachment};
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

impl<T> MediaScroll<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Create a new media scroll widget.
    ///
    /// The `id` should match the scroll widget ID used for paging.
    #[must_use]
    pub fn new<G, R>(id: impl Into<String>, items_getter: G, item_renderer: R) -> Self
    where
        G: Fn(&DataMap) -> Vec<T> + Send + Sync + 'static,
        R: Fn(&T, &DataMap) -> MediaAttachment + Send + Sync + 'static,
    {
        let id = id.into();
        Self {
            scroll: BaseScroll::new(id, None),
            items_getter: Arc::new(items_getter),
            item_renderer: Arc::new(item_renderer),
        }
    }

    /// Set a callback to be invoked when the page changes.
    #[must_use]
    pub fn on_page_changed(mut self, callback: OnPageChanged) -> Self {
        let id = self.scroll.widget_id().to_owned();
        self.scroll = BaseScroll::new(id, Some(callback));
        self
    }
}

impl MediaScroll<String> {
    /// Create a media scroll that reads URLs from a data field array.
    ///
    /// Each URL becomes a photo attachment.
    #[must_use]
    pub fn photo_urls_from_field(id: impl Into<String>, field: impl Into<String>) -> Self {
        let field = field.into();
        MediaScroll::new(
            id,
            move |data| {
                data.get(&field)
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default()
            },
            |url, _data| MediaAttachment::photo_url(url.clone()),
        )
    }

    /// Create a media scroll that reads file IDs from a data field array.
    ///
    /// Each file ID becomes a photo attachment.
    #[must_use]
    pub fn photo_ids_from_field(id: impl Into<String>, field: impl Into<String>) -> Self {
        let field = field.into();
        MediaScroll::new(
            id,
            move |data| {
                data.get(&field)
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default()
            },
            |file_id, _data| MediaAttachment::photo_id(file_id.clone()),
        )
    }
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
