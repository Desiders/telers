use super::Text;
use crate::entities::{DataMap, RenderContext};
use async_trait::async_trait;

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

#[async_trait]
impl<Selector, Key> Text for Case<Selector, Key>
where
    Selector: Fn(&DataMap) -> Key + Send + Sync + 'static,
    Key: PartialEq + Send + Sync + 'static,
{
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        let selected = (self.selector)(data);
        if let Some((_, text)) = self.variants.iter().find(|(key, _)| *key == selected) {
            return text.render_text(data).await;
        }
        match &self.default {
            Some(text) => text.render_text(data).await,
            None => Box::<str>::default(),
        }
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        let selected = (self.selector)(render_ctx.data.as_ref());
        if let Some((_, text)) = self.variants.iter().find(|(key, _)| *key == selected) {
            return text.render_text_in_context(render_ctx).await;
        }
        match &self.default {
            Some(text) => text.render_text_in_context(render_ctx).await,
            None => Box::<str>::default(),
        }
    }
}
