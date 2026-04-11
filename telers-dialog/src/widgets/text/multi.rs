use bon::Builder;
use std::borrow::Cow;

use super::Text;
use crate::entities::{Context, DataMap};

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
