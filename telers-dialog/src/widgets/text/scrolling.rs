use bon::bon;
use std::{borrow::Cow, sync::Arc};

use super::Text;
use crate::{
    entities::{Context, DataMap},
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
