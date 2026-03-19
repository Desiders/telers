use crate::entities::DataMap;
use std::marker::PhantomData;

use super::Text;

pub struct ListText<T, I, R> {
    items: I,
    render_item: R,
    separator: Box<str>,
    marker: PhantomData<fn() -> T>,
}

impl<T, I, R> ListText<T, I, R> {
    #[must_use]
    pub fn new(items: I, render_item: R) -> Self {
        Self {
            items,
            render_item,
            separator: "\n".into(),
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn with_separator(mut self, separator: impl Into<Box<str>>) -> Self {
        self.separator = separator.into();
        self
    }
}

impl<T, I, R, S> Text for ListText<T, I, R>
where
    T: Send + Sync + 'static,
    I: Fn(&DataMap) -> Vec<T> + Send + Sync + 'static,
    R: Fn(&T, &DataMap) -> S + Send + Sync + 'static,
    S: Into<Box<str>>,
{
    fn render_text(&self, data: &DataMap) -> Box<str> {
        let items = (self.items)(data);
        items
            .iter()
            .map(|item| (self.render_item)(item, data).into().into_string())
            .collect::<Vec<_>>()
            .join(&self.separator)
            .into_boxed_str()
    }
}

#[cfg(test)]
mod tests {
    use super::ListText;
    use crate::widgets::Text;

    #[test]
    fn list_text_renders_items_with_separator() {
        let text = ListText::<&'static str, _, _>::new(
            |_data: &crate::entities::DataMap| vec!["one", "two", "three"],
            |item: &&'static str, _data: &crate::entities::DataMap| format!("- {item}"),
        )
        .with_separator(" | ");

        assert_eq!(
            &*text.render_text(&crate::entities::DataMap::new()),
            "- one | - two | - three"
        );
    }
}
