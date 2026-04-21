use bon::Builder;
use std::borrow::Cow;

use super::Text;
use crate::{
    entities::{DataMap, RenderContext},
    future::BoxFuture,
};

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
    fn render_text<'a>(&'a self, data: &'a DataMap) -> BoxFuture<'a, Box<str>> {
        Box::pin(async move {
            let mut rendered = Vec::with_capacity(self.items.len());
            for item in &self.items {
                rendered.push(item.render_text(data).await.into_string());
            }
            rendered.join(&self.separator).into_boxed_str()
        })
    }

    fn render_text_in_context<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, Box<str>> {
        Box::pin(async move {
            let mut rendered = Vec::with_capacity(self.items.len());
            for item in &self.items {
                rendered.push(item.render_text_in_context(render_ctx).await.into_string());
            }
            rendered.join(&self.separator).into_boxed_str()
        })
    }
}
