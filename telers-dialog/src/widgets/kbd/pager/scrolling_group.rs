use bon::bon;
use std::fmt::Display;
use telers::types::{InlineKeyboardMarkup, ReplyMarkup};

use super::{
    super::{ButtonAction, Keyboard},
    build_pager_row, handle_pager_callback, page_count_from_rows, read_page,
    render_fixed_width_page, OnPageChanged,
};
use crate::entities::{Context, DataMap};

pub struct ScrollingGroup<WidgetId, Kbd> {
    id: WidgetId,
    kbd: Kbd,
    width: usize,
    height: usize,
    filler_text: Box<str>,
    on_page_changed: Option<OnPageChanged>,
    hide_on_single_page: bool,
    hide_pager: bool,
}

#[bon]
impl<WidgetId, Kbd> ScrollingGroup<WidgetId, Kbd> {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        kbd: Kbd,
        #[builder(default = 0)] width: usize,
        height: usize,
        #[builder(default = " ".into())] filler_text: Box<str>,
        on_page_changed: Option<OnPageChanged>,
        #[builder(default = false)] hide_on_single_page: bool,
        #[builder(default = false)] hide_pager: bool,
    ) -> Self
    where
        WidgetId: Display,
        Kbd: Keyboard,
    {
        Self {
            id,
            kbd,
            width,
            height,
            filler_text,
            on_page_changed,
            hide_on_single_page,
            hide_pager,
        }
    }
}

impl<WidgetId, Kbd> Keyboard for ScrollingGroup<WidgetId, Kbd>
where
    WidgetId: Display + Send + Sync + 'static,
    Kbd: Keyboard,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let inner_markup = self.kbd.render_keyboard(ctx, data)?;
        let rows = inner_markup.inline_keyboard()?;

        if rows.is_empty() || self.height == 0 {
            return None;
        }

        let widget_id = self.id.to_string();
        let (mut rows, pages_count) = if self.width > 0 {
            render_fixed_width_page(
                ctx,
                &widget_id,
                rows,
                self.width,
                self.height,
                &self.filler_text,
            )?
        } else {
            let total_rows = rows.len();
            let pages_count = page_count_from_rows(total_rows, self.height);
            let current_page = read_page(ctx, &widget_id).min(pages_count.saturating_sub(1));
            let start = current_page * self.height;
            let end = (start + self.height).min(total_rows);
            (
                rows[start..end]
                    .iter()
                    .map(|val| val.to_vec().into_boxed_slice())
                    .collect::<Vec<_>>(),
                pages_count,
            )
        };

        if !(self.hide_pager || self.hide_on_single_page && pages_count <= 1) {
            let current_page = read_page(ctx, &widget_id).min(pages_count.saturating_sub(1));
            rows.push(build_pager_row(ctx, &widget_id, current_page, pages_count));
        }

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        if let Some(action) = handle_pager_callback(
            ctx,
            &self.id.to_string(),
            callback_data,
            self.on_page_changed.as_ref(),
        ) {
            return Some(action);
        }

        self.kbd.handle_callback(ctx, callback_data)
    }
}
