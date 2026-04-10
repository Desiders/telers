use bon::{bon, Builder};
use std::{borrow::Cow, sync::Arc};

use super::kbd::{BaseScroll, OnPageChanged, Scroll};
use crate::entities::{Context, Data, DataMap};

pub trait Text: Send + Sync + 'static {
    #[must_use]
    fn render_text(&self, data: &DataMap) -> Box<str>;

    #[must_use]
    fn render_text_in_context(&self, _ctx: &Context, data: &DataMap) -> Box<str> {
        self.render_text(data)
    }
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

    fn render_text_in_context(&self, ctx: &Context, data: &DataMap) -> Box<str> {
        let selected = (self.selector)(data);
        self.variants
            .iter()
            .find(|(key, _)| *key == selected)
            .map(|(_, text)| text.render_text_in_context(ctx, data))
            .or_else(|| {
                self.default
                    .as_ref()
                    .map(|text| text.render_text_in_context(ctx, data))
            })
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

    fn render_text_in_context(&self, ctx: &Context, data: &DataMap) -> Box<str> {
        self.items
            .iter()
            .map(|item| item.render_text_in_context(ctx, data).into_string())
            .collect::<Vec<_>>()
            .join(&self.separator)
            .into_boxed_str()
    }
}

/// Paged text widget driven by shared page state in `widget_data`.
#[derive(Clone)]
pub struct ScrollingText {
    base_scroll: BaseScroll,
    text: Arc<dyn Text>,
    page_size: usize,
}

#[bon]
impl ScrollingText {
    /// Create a scrolling text widget bound to `widget_data[id]`.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] id: Cow<'static, str>,
        #[builder(with = |text: impl Text| Arc::new(text) as Arc<dyn Text>)] text: Arc<dyn Text>,
        page_size: usize,
        on_page_changed: Option<OnPageChanged>,
    ) -> Self {
        Self {
            base_scroll: BaseScroll::new(id, on_page_changed),
            text,
            page_size,
        }
    }
}

impl ScrollingText {
    #[inline]
    fn effective_page_size(&self) -> usize {
        self.page_size.max(1)
    }

    fn page_count_for_text(&self, full_text: &str) -> usize {
        let page_size = self.effective_page_size();
        let chars_count = full_text.chars().count();
        chars_count / page_size + usize::from(!chars_count.is_multiple_of(page_size))
    }

    fn char_to_byte_index(text: &str, char_index: usize) -> usize {
        if char_index == 0 {
            return 0;
        }

        text.char_indices()
            .nth(char_index)
            .map_or(text.len(), |(byte_index, _)| byte_index)
    }

    fn render_page(&self, full_text: Box<str>, page: usize) -> Box<str> {
        if full_text.is_empty() {
            return full_text;
        }

        let page_size = self.effective_page_size();
        let pages_count = self.page_count_for_text(&full_text);
        let current_page = page.min(pages_count.saturating_sub(1));
        let start_char = current_page * page_size;
        let end_char = start_char + page_size;
        let start = Self::char_to_byte_index(&full_text, start_char);
        let end = Self::char_to_byte_index(&full_text, end_char);
        full_text[start..end].to_owned().into_boxed_str()
    }

    /// Compute how many pages the current text produces.
    #[must_use]
    pub fn page_count(&self, data: &DataMap) -> usize {
        self.page_count_for_text(&self.text.render_text(data))
    }

    /// Compute how many pages the current text produces with access to widget state.
    #[must_use]
    pub fn page_count_in_context(&self, ctx: &Context, data: &DataMap) -> usize {
        self.page_count_for_text(&self.text.render_text_in_context(ctx, data))
    }
}

impl Scroll for ScrollingText {
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    fn get_page_count(&self, ctx: &Context, data: &DataMap) -> usize {
        self.page_count_in_context(ctx, data)
    }
}

impl Text for ScrollingText {
    fn render_text(&self, data: &DataMap) -> Box<str> {
        self.render_page(self.text.render_text(data), 0)
    }

    fn render_text_in_context(&self, ctx: &Context, data: &DataMap) -> Box<str> {
        self.render_page(
            self.text.render_text_in_context(ctx, data),
            self.get_page(ctx),
        )
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
    use super::{Case, FormatText, MultiText, Progress, ScrollingText, Text};
    use crate::{
        entities::{Context, DataMap},
        widgets::{Keyboard, NumberedPager},
    };
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

    #[test]
    fn scrolling_text_defaults_to_first_page_without_context() {
        let text = ScrollingText::builder("article")
            .text("abcdefghij")
            .page_size(4)
            .build();

        assert_eq!(&*text.render_text(&DataMap::new()), "abcd");
        assert_eq!(text.page_count(&DataMap::new()), 3);
    }

    #[test]
    fn scrolling_text_uses_widget_page_from_context() {
        let mut ctx = Context::new("", "state", json!(null));
        ctx.widget_data.insert("article".into(), json!(2));

        let text = ScrollingText::builder("article")
            .text("abcdefghij")
            .page_size(4)
            .build();

        assert_eq!(&*text.render_text_in_context(&ctx, &DataMap::new()), "ij");
        assert_eq!(text.page_count_in_context(&ctx, &DataMap::new()), 3);
    }

    #[test]
    fn scrolling_text_slices_by_char_boundaries() {
        let mut ctx = Context::new("", "state", json!(null));
        ctx.widget_data.insert("article".into(), json!(1));

        let text = ScrollingText::builder("article")
            .text("ab😀cd")
            .page_size(3)
            .build();

        assert_eq!(&*text.render_text_in_context(&ctx, &DataMap::new()), "cd");
        assert_eq!(text.page_count(&DataMap::new()), 2);
    }

    #[test]
    fn scrolling_text_can_drive_numbered_pager_via_scroll_trait() {
        let text = ScrollingText::builder("article")
            .text("abcdefghij")
            .page_size(4)
            .build();
        let markup = NumberedPager::builder(text)
            .page_renderer(|page, _data| format!("{}", page + 1))
            .current_page_renderer(|page, _data| format!("[{}]", page + 1))
            .length(5)
            .build()
            .render_keyboard(&Context::new("", "state", json!(null)), &DataMap::new())
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), 3);
        assert_eq!(&*rows[0][0].text, "[1]");
        assert_eq!(&*rows[0][2].text, "3");
    }
}
