use bon::bon;
use std::borrow::Cow;
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use super::{
    super::{when::is_allowed, ButtonAction, ClickContext, Keyboard, WhenCondition},
    build_pager_row, page_count_from_rows, render_fixed_width_page, BaseScroll, OnPageChanged,
    Scroll,
};
use crate::entities::{Context, DataMap, RenderContext};

#[derive(Clone)]
pub struct ScrollingGroup<Kbd> {
    base_scroll: BaseScroll,
    kbd: Kbd,
    width: usize,
    height: usize,
    filler_text: Box<str>,
    hide_on_single_page: bool,
    hide_pager: bool,
    when: Option<WhenCondition>,
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
        when: Option<WhenCondition>,
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
            when,
        }
    }
}

impl<Kbd> ScrollingGroup<Kbd>
where
    Kbd: Keyboard,
{
    fn page_rows(
        &self,
        render_ctx: &RenderContext<'_>,
    ) -> Option<(Vec<Box<[InlineKeyboardButton]>>, usize)> {
        let ctx = render_ctx.context;
        let inner_markup = self.kbd.render_keyboard(render_ctx)?;
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

    fn get_page_count(&self, render_ctx: &RenderContext<'_>) -> usize {
        self.page_rows(render_ctx)
            .map(|(_, pages_count)| pages_count)
            .unwrap_or(0)
    }
}

impl<Kbd> Keyboard for ScrollingGroup<Kbd>
where
    Kbd: Keyboard,
{
    fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context;
        let data = render_ctx.data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        let (mut rows, pages_count) = self.page_rows(render_ctx)?;

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

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        let callback_data = click.callback_data;
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        if let Some(action) = self.base_scroll.handle_callback(ctx, callback_data) {
            return Some(action);
        }

        if !self.kbd.is_visible(ctx, data) {
            return None;
        }
        self.kbd.handle_callback(click)
    }
}
