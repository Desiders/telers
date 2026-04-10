use bon::bon;
use std::borrow::Cow;
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use super::{
    super::{ButtonAction, Keyboard},
    build_pager_row, page_count_from_rows, render_fixed_width_page, BaseScroll, OnPageChanged,
    Scroll,
};
use crate::entities::{Context, DataMap};

#[derive(Clone)]
pub struct ScrollingGroup<Kbd> {
    base_scroll: BaseScroll,
    kbd: Kbd,
    width: usize,
    height: usize,
    filler_text: Box<str>,
    hide_on_single_page: bool,
    hide_pager: bool,
}

#[bon]
impl<Kbd> ScrollingGroup<Kbd> {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] id: Cow<'static, str>,
        kbd: Kbd,
        #[builder(default = 0)] width: usize,
        height: usize,
        #[builder(default = " ".into())] filler_text: Box<str>,
        on_page_changed: Option<OnPageChanged>,
        #[builder(default = false)] hide_on_single_page: bool,
        #[builder(default = false)] hide_pager: bool,
    ) -> Self
    where
        Kbd: Keyboard,
    {
        Self {
            base_scroll: BaseScroll::new(id, on_page_changed),
            kbd,
            width,
            height,
            filler_text,
            hide_on_single_page,
            hide_pager,
        }
    }
}

impl<Kbd> ScrollingGroup<Kbd>
where
    Kbd: Keyboard,
{
    fn page_rows(
        &self,
        ctx: &Context,
        data: &DataMap,
    ) -> Option<(Vec<Box<[InlineKeyboardButton]>>, usize)> {
        let inner_markup = self.kbd.render_keyboard(ctx, data)?;
        let rows = inner_markup.inline_keyboard()?;

        if rows.is_empty() || self.height == 0 {
            return None;
        }

        if self.width > 0 {
            render_fixed_width_page(
                ctx,
                self.widget_id(),
                rows,
                self.width,
                self.height,
                &self.filler_text,
            )
        } else {
            let total_rows = rows.len();
            let pages_count = page_count_from_rows(total_rows, self.height);
            let current_page = self.get_page(ctx).min(pages_count.saturating_sub(1));
            let start = current_page * self.height;
            let end = (start + self.height).min(total_rows);
            Some((
                rows[start..end]
                    .iter()
                    .map(|val| val.to_vec().into_boxed_slice())
                    .collect::<Vec<_>>(),
                pages_count,
            ))
        }
    }
}

impl<Kbd> Scroll for ScrollingGroup<Kbd>
where
    Kbd: Keyboard,
{
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    fn get_page_count(&self, ctx: &Context, data: &DataMap) -> usize {
        self.page_rows(ctx, data)
            .map(|(_, pages_count)| pages_count)
            .unwrap_or(0)
    }
}

impl<Kbd> Keyboard for ScrollingGroup<Kbd>
where
    Kbd: Keyboard,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let (mut rows, pages_count) = self.page_rows(ctx, data)?;

        if !(self.hide_pager || self.hide_on_single_page && pages_count <= 1) {
            let current_page = self.get_page(ctx).min(pages_count.saturating_sub(1));
            rows.push(build_pager_row(
                ctx,
                self.widget_id(),
                current_page,
                pages_count,
            ));
        }

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        if let Some(action) = self.base_scroll.handle_callback(ctx, callback_data) {
            return Some(action);
        }

        self.kbd.handle_callback(ctx, callback_data)
    }
}
