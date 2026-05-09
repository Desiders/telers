use bon::bon;
use std::borrow::Cow;

use super::Text;
use crate::entities::{DataMap, RenderContext};
use async_trait::async_trait;

pub struct MultiText {
    items: Vec<Box<dyn Text>>,
    separator: Cow<'static, str>,
}

#[bon]
impl MultiText {
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
