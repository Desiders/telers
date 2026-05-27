use bon::bon;
use std::{borrow::Cow, marker::PhantomData};

use super::Text;
use crate::entities::DataMap;
use async_trait::async_trait;

/// Render a list of items pulled from render data and joined by a separator.
///
/// `items_getter` is invoked with the current [`DataMap`] and returns any
/// `IntoIterator`. `item_renderer` formats each item and the rendered strings
/// are joined by the configured separator (default `"\n"`).
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
/// ```
pub struct ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr> {
    items_getter: ItemsGetter,
    item_renderer: ItemRenderer,
    separator: Cow<'static, str>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr)>,
}

#[bon]
impl<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
    ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
{
    /// Build a list-text widget.
    #[builder]
    #[must_use]
    pub fn new(
        items_getter: ItemsGetter,
        item_renderer: ItemRenderer,
        #[builder(default = "\n", into)] separator: Cow<'static, str>,
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
            marker: PhantomData,
        }
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
        let items = (self.items_getter)(data);
        items
            .into_iter()
            .map(|item| (self.item_renderer)(&item, data).into())
            .collect::<Box<[_]>>()
            .join(&self.separator)
            .into_boxed_str()
    }
}
