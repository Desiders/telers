use bon::Builder;
use std::borrow::Cow;

use crate::entities::{Data, DataMap};

pub trait Text: Send + Sync + 'static {
    #[must_use]
    fn render_text(&self, data: &DataMap) -> Box<str>;
}

impl<T> Text for T
where
    T: ToString + Send + Sync + 'static,
{
    fn render_text(&self, _data: &DataMap) -> Box<str> {
        self.to_string().into_boxed_str()
    }
}

pub(crate) struct FnText<Renderer> {
    renderer: Renderer,
}

impl<Renderer> FnText<Renderer> {
    #[inline]
    #[must_use]
    pub(crate) const fn new(renderer: Renderer) -> Self {
        Self {
            renderer,
        }
    }
}

impl<Renderer, Item> Text for FnText<Renderer>
where
    Renderer: Fn(&DataMap) -> Item + Send + Sync + 'static,
    Item: Into<Box<str>>,
{
    fn render_text(&self, data: &DataMap) -> Box<str> {
        (self.renderer)(data).into()
    }
}

pub(crate) struct FormatText {
    template: Cow<'static, str>,
}

impl FormatText {
    #[must_use]
    pub(crate) fn new(template: impl Into<Cow<'static, str>>) -> Self {
        Self {
            template: template.into(),
        }
    }
}

impl Text for FormatText {
    #[inline]
    fn render_text(&self, data: &DataMap) -> Box<str> {
        render_template(&self.template, data).into_boxed_str()
    }
}

#[derive(Builder)]
pub struct MultiText {
    #[builder(field)]
    items: Vec<Box<dyn Text>>,
    #[builder(default = "\n", into)]
    separator: Cow<'static, str>,
}

impl<S> MultiTextBuilder<S>
where
    S: multi_text_builder::State,
{
    pub fn text(mut self, text: impl Text) -> Self {
        self.items.push(Box::new(text));
        self
    }

    pub(crate) fn text_boxed(mut self, text: Box<dyn Text>) -> Self {
        self.items.push(text);
        self
    }
}

impl Text for MultiText {
    fn render_text(&self, data: &DataMap) -> Box<str> {
        self.items
            .iter()
            .map(|item| item.render_text(data).into_string())
            .collect::<Vec<_>>()
            .join(&self.separator)
            .into_boxed_str()
    }
}

fn render_template(template: &str, data: &DataMap) -> String {
    let mut output = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(start) = rest.find('{') {
        output.push_str(&rest[..start]);
        let after_start = &rest[start + 1..];

        let Some(end) = after_start.find('}') else {
            output.push_str(&rest[start..]);
            return output;
        };

        let key = &after_start[..end];
        if key.is_empty() || key.contains('{') {
            output.push_str(&rest[start..start + end + 2]);
        } else if let Some(value) = data.get(key) {
            output.push_str(&render_data_value(value));
        } else {
            output.push('{');
            output.push_str(key);
            output.push('}');
        }

        rest = &after_start[end + 1..];
    }

    output.push_str(rest);
    output
}

fn render_data_value(value: &Data) -> String {
    match value {
        Data::String(value) => value.clone(),
        Data::Null => String::new(),
        _ => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{FormatText, MultiText, Text};
    use crate::entities::DataMap;

    #[test]
    fn format_text_replaces_known_keys() {
        let mut data = DataMap::new();
        data.insert("name".into(), "telers".into());

        let text = FormatText::new("hello {name}");

        assert_eq!(&*text.render_text(&data), "hello telers");
    }

    #[test]
    fn format_text_keeps_unknown_keys_visible() {
        let text = FormatText::new("hello {name}");

        assert_eq!(&*text.render_text(&DataMap::new()), "hello {name}");
    }

    #[test]
    fn multi_text_joins_items() {
        let text = MultiText::builder()
            .text("one")
            .text("two")
            .separator(" | ")
            .build();

        assert_eq!(&*text.render_text(&DataMap::new()), "one | two");
    }
}
