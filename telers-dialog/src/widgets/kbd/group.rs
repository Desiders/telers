use async_trait::async_trait;
use bon::bon;
use telers::types::{InlineKeyboardMarkup, ReplyMarkup};

use super::{when::is_allowed, ButtonAction, ClickContext, Keyboard, WhenCondition};
use crate::entities::{Context, DataMap, RenderContext};

/// Layout wrapper that regroups inline keyboard buttons into fixed-width rows.
pub struct Group<Kbd> {
    kbd: Kbd,
    items_per_row: usize,
    when: Option<WhenCondition>,
}

#[bon]
impl<Kbd> Group<Kbd> {
    /// Wrap a keyboard and regroup its inline buttons by `items_per_row`.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] kbd: Kbd,
        #[builder(default = 1)] items_per_row: usize,
        when: Option<WhenCondition>,
    ) -> Self {
        Self {
            kbd,
            items_per_row: items_per_row.max(1),
            when,
        }
    }

    #[must_use]
    pub fn when(mut self, when: WhenCondition) -> Self {
        self.when = Some(when);
        self
    }
}

#[async_trait]
impl<Kbd> Keyboard for Group<Kbd>
where
    Kbd: Keyboard,
{
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context.as_ref();
        let data = render_ctx.data.as_ref();
        if !self.is_visible(ctx, data).await {
            return None;
        }
        if !self.kbd.is_visible(ctx, data).await {
            return None;
        }
        let markup = self.kbd.render_keyboard(render_ctx).await?;
        let ReplyMarkup::InlineKeyboardMarkup(markup) = markup else {
            return Some(markup);
        };

        let items_per_row = self.items_per_row.max(1);
        let mut grouped_rows = Vec::new();
        let mut current_row = Vec::with_capacity(items_per_row);
        for button in markup.inline_keyboard.into_vec().into_iter().flatten() {
            current_row.push(button);
            if current_row.len() == items_per_row {
                grouped_rows.push(current_row.into_boxed_slice());
                current_row = Vec::with_capacity(items_per_row);
            }
        }
        if !current_row.is_empty() {
            grouped_rows.push(current_row.into_boxed_slice());
        }

        if grouped_rows.is_empty() {
            None
        } else {
            Some(ReplyMarkup::InlineKeyboardMarkup(
                InlineKeyboardMarkup::new(grouped_rows),
            ))
        }
    }

    #[inline]
    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data).await {
            return None;
        }
        if !self.kbd.is_visible(ctx, data).await {
            return None;
        }
        self.kbd.handle_callback(click).await
    }
}
