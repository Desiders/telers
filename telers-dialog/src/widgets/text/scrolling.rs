use bon::bon;
use std::{borrow::Cow, sync::Arc};

use super::Text;
#[cfg(test)]
use crate::entities::{ChatEvent, EventContext};
use crate::{
    entities::{DataMap, RenderContext},
    future::BoxFuture,
    widgets::{BaseScroll, OnPageChanged, Scroll},
};

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
    pub async fn page_count(&self, data: &DataMap) -> usize {
        self.page_count_for_text(&self.text.render_text(data).await)
    }

    /// Compute how many pages the current text produces with access to widget state.
    #[must_use]
    pub async fn page_count_in_context(&self, render_ctx: &RenderContext) -> usize {
        self.page_count_for_text(&self.text.render_text_in_context(render_ctx).await)
    }

    #[cfg(test)]
    #[must_use]
    pub fn page_count_in_context_for_test<'a>(
        &'a self,
        ctx: &'a crate::entities::Context,
        data: &'a DataMap,
    ) -> BoxFuture<'a, usize> {
        Box::pin(async move {
            use telers::{
                client::Reqwest,
                types::{ChatPrivate, MessageText, User},
                Bot,
            };

            let event = ChatEvent::Message(
                MessageText::new(1, 1, ChatPrivate::new(10), "/test")
                    .from(User::new(10, false, "tester"))
                    .into(),
            );
            let event_context =
                EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
            let render_ctx = RenderContext::new(ctx, data, &event, &event_context);
            self.page_count_in_context(&render_ctx).await
        })
    }
}

impl Scroll for ScrollingText {
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    fn get_page_count(&self, render_ctx: RenderContext) -> BoxFuture<'_, usize> {
        Box::pin(async move { self.page_count_in_context(&render_ctx).await })
    }
}

impl Text for ScrollingText {
    fn render_text<'a>(&'a self, data: &'a DataMap) -> BoxFuture<'a, Box<str>> {
        Box::pin(async move { self.render_page(self.text.render_text(data).await, 0) })
    }

    fn render_text_in_context<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, Box<str>> {
        Box::pin(async move {
            self.render_page(
                self.text.render_text_in_context(render_ctx).await,
                self.get_page(render_ctx.context.as_ref()),
            )
        })
    }
}
