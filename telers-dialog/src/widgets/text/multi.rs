use bon::bon;
use std::borrow::Cow;

use super::Text;
use crate::entities::{DataMap, RenderContext};
use async_trait::async_trait;

/// Composite text widget that joins several [`Text`] sources with a separator.
///
/// This is the same composition used internally by the window builder when a
/// caller registers more than one `text(...)` widget in a single window. The
/// default separator is a newline.
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::{FormatText, MultiText};
///
/// let text = MultiText::builder()
///     .text("Order summary")
///     .text(FormatText::new("Total: {total}"))
///     .separator("\n\n")
///     .build();
/// ```
pub struct MultiText {
    items: Vec<Box<dyn Text>>,
    separator: Cow<'static, str>,
}

#[bon]
impl MultiText {
    /// Create a new multi-text widget; use the builder helpers to populate items.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field)] items: Vec<Box<dyn Text>>,
        #[builder(default = "\n", into)] separator: Cow<'static, str>,
    ) -> Self {
        Self {
            items,
            separator,
        }
    }
}

impl<S> MultiTextBuilder<S>
where
    S: multi_text_builder::State,
{
    /// Append a [`Text`] source to the composition.
    pub fn text(mut self, text: impl Text) -> Self {
        self.items.push(Box::new(text));
        self
    }

    pub(crate) fn text_boxed(mut self, text: Box<dyn Text>) -> Self {
        self.items.push(text);
        self
    }
}

#[async_trait]
impl Text for MultiText {
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        let mut rendered = Vec::with_capacity(self.items.len());
        for item in &self.items {
            rendered.push(item.render_text(data).await.into_string());
        }
        rendered.join(&self.separator).into_boxed_str()
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        let mut rendered = Vec::with_capacity(self.items.len());
        for item in &self.items {
            rendered.push(item.render_text_in_context(render_ctx).await.into_string());
        }
        rendered.join(&self.separator).into_boxed_str()
    }
}
