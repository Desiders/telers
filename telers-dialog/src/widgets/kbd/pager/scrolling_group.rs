use bon::bon;
use std::borrow::Cow;
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use super::{
    super::{when::is_allowed, ButtonAction, ClickContext, Keyboard, WhenCondition},
    build_pager_row, page_count_from_rows, render_fixed_width_page, BaseScroll, OnPageChanged,
    Scroll,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

type PageRows = Option<(Vec<Box<[InlineKeyboardButton]>>, usize)>;

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
    fn page_rows<'a>(&'a self, render_ctx: &'a RenderContext) -> BoxFuture<'a, PageRows> {
        Box::pin(async move {
            let ctx = render_ctx.context.as_ref();
            let inner_markup = self.kbd.render_keyboard(render_ctx).await?;
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
        })
    }
}

impl<Kbd> Scroll for ScrollingGroup<Kbd>
where
    Kbd: Keyboard,
{
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    fn get_page_count(&self, render_ctx: RenderContext) -> BoxFuture<'_, usize> {
        Box::pin(async move {
            self.page_rows(&render_ctx)
                .await
                .map_or(0, |(_, pages_count)| pages_count)
        })
    }
}

impl<Kbd> Keyboard for ScrollingGroup<Kbd>
where
    Kbd: Keyboard,
{
    fn is_visible<'a>(&'a self, ctx: &'a Context, data: &'a DataMap) -> BoxFuture<'a, bool> {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, Option<ReplyMarkup>> {
        Box::pin(async move {
            let ctx = render_ctx.context.as_ref();
            let data = render_ctx.data.as_ref();
            if !self.is_visible(ctx, data).await {
                return None;
            }
            let (mut rows, pages_count) = self.page_rows(render_ctx).await?;

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
        })
    }

    fn handle_callback<'a>(
        &'a self,
        click: &'a ClickContext,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
            let ctx = click.context.as_ref();
            let callback_data = click.callback_data.as_str();
            let data = &ctx.dialog_data;
            if !self.is_visible(ctx, data).await {
                return None;
            }
            if let Some(action) = self.base_scroll.handle_callback(ctx, callback_data).await {
                return Some(action);
            }

            if !self.kbd.is_visible(ctx, data).await {
                return None;
            }
            self.kbd.handle_callback(click).await
        })
    }
}
