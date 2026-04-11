use super::Text;
use crate::entities::{DataMap, RenderContext};

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

    fn render_text_in_context(&self, render_ctx: &RenderContext<'_>) -> Box<str> {
        let selected = (self.selector)(render_ctx.data);
        self.variants
            .iter()
            .find(|(key, _)| *key == selected)
            .map(|(_, text)| text.render_text_in_context(render_ctx))
            .or_else(|| {
                self.default
                    .as_ref()
                    .map(|text| text.render_text_in_context(render_ctx))
            })
            .unwrap_or_default()
    }
}
