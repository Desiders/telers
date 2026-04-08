use bon::bon;
use std::{borrow::Cow, marker::PhantomData};

use super::Text;
use crate::entities::DataMap;

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

impl<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr> Text
    for ListText<ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr>
where
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
{
    fn render_text(&self, data: &DataMap) -> Box<str> {
        let items = (self.items_getter)(data);
        items
            .into_iter()
            .map(|item| (self.item_renderer)(&item, data).into())
            .collect::<Box<[_]>>()
            .join(&self.separator)
            .into_boxed_str()
    }
}

#[cfg(test)]
mod tests {
    use super::ListText;
    use crate::{entities::DataMap, widgets::Text};

    #[test]
    fn list_text_renders_items_with_separator() {
        let text = ListText::builder()
            .items_getter(|_data| ["one", "two", "three"])
            .item_renderer(|&item, _data| format!("- {item}"))
            .separator(" | ")
            .build();

        assert_eq!(
            &*text.render_text(&DataMap::new()),
            "- one | - two | - three"
        );
    }
}
