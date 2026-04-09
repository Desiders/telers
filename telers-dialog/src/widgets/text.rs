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

/// Select one of several text variants based on a computed key.
pub struct Case<Selector, Key> {
    selector: Selector,
    variants: Vec<(Key, Box<dyn Text>)>,
    default: Option<Box<dyn Text>>,
}

impl<Selector, Key> Case<Selector, Key> {
    /// Create a new conditional text widget.
    #[must_use]
    pub const fn new(selector: Selector) -> Self {
        Self {
            selector,
            variants: Vec::new(),
            default: None,
        }
    }
}

impl<Selector, Key> Case<Selector, Key>
where
    Selector: Fn(&DataMap) -> Key,
    Key: PartialEq,
{
    /// Add a keyed text variant.
    #[must_use]
    pub fn when(mut self, key: Key, text: impl Text) -> Self {
        self.variants.push((key, Box::new(text)));
        self
    }

    /// Add a default text variant used when no keyed variant matches.
    #[must_use]
    pub fn default(mut self, text: impl Text) -> Self {
        self.default = Some(Box::new(text));
        self
    }
}

impl<Selector, Key> Text for Case<Selector, Key>
where
    Selector: Fn(&DataMap) -> Key + Send + Sync + 'static,
    Key: PartialEq + Send + Sync + 'static,
{
    fn render_text(&self, data: &DataMap) -> Box<str> {
        let selected = (self.selector)(data);
        self.variants
            .iter()
            .find(|(key, _)| *key == selected)
            .map(|(_, text)| text.render_text(data))
            .or_else(|| self.default.as_ref().map(|text| text.render_text(data)))
            .unwrap_or_default()
    }
}

/// Render a textual progress bar from a percentage field in `DataMap`.
pub struct Progress {
    field: Cow<'static, str>,
    width: usize,
    filled: Cow<'static, str>,
    empty: Cow<'static, str>,
}

impl Progress {
    /// Create a new progress widget reading percent from `field`.
    #[must_use]
    pub fn new(field: impl Into<Cow<'static, str>>) -> Self {
        Self {
            field: field.into(),
            width: 10,
            filled: "#".into(),
            empty: "-".into(),
        }
    }

    /// Set the progress bar width in cells.
    #[must_use]
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set the filled-cell glyph.
    #[must_use]
    pub fn filled(mut self, filled: impl Into<Cow<'static, str>>) -> Self {
        self.filled = filled.into();
        self
    }

    /// Set the empty-cell glyph.
    #[must_use]
    pub fn empty(mut self, empty: impl Into<Cow<'static, str>>) -> Self {
        self.empty = empty.into();
        self
    }
}

impl Text for Progress {
    fn render_text(&self, data: &DataMap) -> Box<str> {
        let percent = data
            .get(self.field.as_ref())
            .and_then(|value| match value {
                Data::Number(value) => value.as_f64(),
                Data::String(value) => value.parse::<f64>().ok(),
                _ => None,
            })
            .unwrap_or_default()
            .clamp(0.0, 100.0);
        let done = ((self.width as f64 * percent) / 100.0).round() as usize;
        let rest = self.width.saturating_sub(done);

        format!(
            "{}{} {:>3.0}%",
            self.filled.repeat(done),
            self.empty.repeat(rest),
            percent
        )
        .into_boxed_str()
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
    use super::{Case, FormatText, MultiText, Progress, Text};
    use crate::entities::DataMap;
    use serde_json::json;

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

    #[test]
    fn case_selects_matching_variant() {
        let mut data = DataMap::new();
        data.insert("status".into(), json!("paid"));

        let text = Case::new(|data: &DataMap| data.get("status").cloned())
            .when(Some(json!("draft")), "Draft order")
            .when(Some(json!("paid")), "Paid order")
            .default("Unknown");

        assert_eq!(&*text.render_text(&data), "Paid order");
    }

    #[test]
    fn case_uses_default_when_key_missing() {
        let text = Case::new(|data: &DataMap| data.get("status").cloned())
            .when(Some(json!("draft")), "Draft order")
            .default("Unknown");

        assert_eq!(&*text.render_text(&DataMap::new()), "Unknown");
    }

    #[test]
    fn progress_renders_bar_from_percentage_field() {
        let mut data = DataMap::new();
        data.insert("percent".into(), json!(35));

        let text = Progress::new("percent").width(10);

        assert_eq!(&*text.render_text(&data), "####------  35%");
    }

    #[test]
    fn progress_clamps_and_supports_custom_symbols() {
        let mut data = DataMap::new();
        data.insert("percent".into(), json!(120));

        let text = Progress::new("percent").width(4).filled("=").empty(".");

        assert_eq!(&*text.render_text(&data), "==== 100%");
    }
}
