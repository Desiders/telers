use async_trait::async_trait;
use bon::bon;

use super::Text;
use crate::entities::{DataMap, RenderContext};

/// Renders the first nonempty text, allowing conditional fallbacks.
pub struct FirstText {
    items: Vec<Box<dyn Text>>,
}

#[bon]
impl FirstText {
    #[builder]
    #[must_use]
    pub fn new(#[builder(field)] items: Vec<Box<dyn Text>>) -> Self {
        Self {
            items,
        }
    }
}

impl<S> FirstTextBuilder<S>
where
    S: first_text_builder::State,
{
    pub fn text(mut self, text: impl Text) -> Self {
        self.items.push(Box::new(text));
        self
    }
}

#[async_trait]
impl Text for FirstText {
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        for item in &self.items {
            let text = item.render_text(data).await;
            if !text.is_empty() {
                return text;
            }
        }
        Box::default()
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        for item in &self.items {
            let text = item.render_text_in_context(render_ctx).await;
            if !text.is_empty() {
                return text;
            }
        }
        Box::default()
    }
}
