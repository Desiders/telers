use bon::bon;
use std::{borrow::Cow, marker::PhantomData};

use super::Text;
use crate::{
    entities::{DataMap, RenderContext},
    widgets::{BaseScroll, OnPageChanged, Scroll},
};
use async_trait::async_trait;

/// Render a list of items pulled from render data and joined by a separator.
///
/// `items_getter` is invoked with the current [`DataMap`] and returns any
/// `IntoIterator`. `item_renderer` formats each item and the rendered strings
/// are joined by the configured separator (default `"\n"`).
///
/// # Pagination
///
/// Set [`page_size`](ListTextBuilder::page_size) together with an
/// [`id`](ListTextBuilder::id) to paginate the list: only the current page's
/// items are rendered, the page is stored in `widget_data[id]`, and the widget
/// implements [`Scroll`] so it can drive a
/// [`NumberedPager`](crate::widgets::NumberedPager). Without `page_size` every
/// item is rendered (a single page).
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::ListText;
///
/// let text = ListText::builder()
///     .items_getter(|_data| ["espresso", "filter"])
///     .item_renderer(|item, _data| format!("- {item}"))
///     .build();
///
/// // Paginated: shows `page_size` items per page and drives a pager via `id`.
/// let paged = ListText::builder()
///     .id("catalog")
///     .page_size(10)
///     .items_getter(|_data| ["espresso", "filter"])
///     .item_renderer(|item, _data| format!("- {item}"))
///     .build();
/// ```
pub struct ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr> {
    items_getter: ItemsGetter,
    item_renderer: ItemRenderer,
    separator: Cow<'static, str>,
    base_scroll: BaseScroll,
    page_size: Option<usize>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr)>,
}

#[bon]
impl<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
    ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
{
    /// Build a list-text widget.
    ///
    /// Provide [`id`](ListTextBuilder::id) and
    /// [`page_size`](ListTextBuilder::page_size) to paginate the list; omit them
    /// to render every item.
    #[builder]
    #[must_use]
    pub fn new(
        items_getter: ItemsGetter,
        item_renderer: ItemRenderer,
        #[builder(default = "\n", into)] separator: Cow<'static, str>,
        #[builder(into)] id: Option<Cow<'static, str>>,
        page_size: Option<usize>,
        on_page_changed: Option<OnPageChanged>,
    ) -> Self
    where
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        ItemRenderer: Fn(&Item, &DataMap) -> ItemStr,
        ItemStr: Into<Box<str>>,
    {
        Self {
            items_getter,
            item_renderer,
            separator,
            base_scroll: BaseScroll::new(id.unwrap_or(Cow::Borrowed("")), on_page_changed),
            page_size,
            marker: PhantomData,
        }
    }
}

impl<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
    ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
where
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
{
    /// Number of pages the current items produce (always at least `1`).
    fn page_count_for(&self, data: &DataMap) -> usize {
        match self.page_size {
            Some(page_size) if page_size > 0 => (self.items_getter)(data)
                .into_iter()
                .count()
                .div_ceil(page_size)
                .max(1),
            _ => 1,
        }
    }

    /// Render the items for `page`, slicing by `page_size` when paginated.
    fn render_items(&self, data: &DataMap, page: usize) -> Box<str> {
        let rendered: Vec<Box<str>> = (self.items_getter)(data)
            .into_iter()
            .map(|item| (self.item_renderer)(&item, data).into())
            .collect();
        let slice: &[Box<str>] = match self.page_size {
            Some(page_size) if page_size > 0 => {
                let pages = rendered.len().div_ceil(page_size).max(1);
                let current = page.min(pages - 1);
                let start = current * page_size;
                let end = (start + page_size).min(rendered.len());
                &rendered[start..end]
            }
            _ => &rendered,
        };
        slice.join(&self.separator).into_boxed_str()
    }
}

#[async_trait]
impl<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr> Text
    for ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
where
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
{
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        self.render_items(data, 0)
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        let page = self.base_scroll.get_page(render_ctx.context.as_ref());
        self.render_items(render_ctx.data.as_ref(), page)
    }
}

#[async_trait]
impl<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr> Scroll
    for ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
where
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
{
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    async fn get_page_count(&self, render_ctx: RenderContext) -> usize {
        self.page_count_for(render_ctx.data.as_ref())
    }
}
