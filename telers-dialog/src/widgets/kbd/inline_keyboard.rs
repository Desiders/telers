use bon::bon;
use telers::types::{InlineKeyboardMarkup, ReplyMarkup};

use super::{when::is_allowed, Button, ButtonAction, Keyboard, WhenCondition};
use crate::entities::{Context, DataMap, RenderContext};

#[derive(Clone, Default)]
pub struct InlineKeyboard {
    rows: Vec<Vec<Button>>,
    when: Option<WhenCondition>,
}

#[bon]
impl InlineKeyboard {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field = Vec::new())] rows: Vec<Vec<Button>>,
        when: Option<WhenCondition>,
    ) -> Self {
        Self {
            rows,
            when,
        }
    }

    #[must_use]
    pub fn row(mut self, row: impl IntoIterator<Item = Button>) -> Self {
        self.rows.push(row.into_iter().collect());
        self
    }

    /// Add a button to the last row or create a new row if the last row not found
    #[must_use]
    pub fn push(mut self, button: Button) -> Self {
        match self.rows.last_mut() {
            Some(row) => row.push(button),
            None => self.rows.push(vec![button]),
        }
        self
    }

    #[must_use]
    pub fn when(mut self, when: WhenCondition) -> Self {
        self.when = Some(when);
        self
    }
}

impl<S> InlineKeyboardBuilder<S>
where
    S: inline_keyboard_builder::State,
{
    #[must_use]
    pub fn row(mut self, row: impl IntoIterator<Item = Button>) -> Self {
        self.rows.push(row.into_iter().collect());
        self
    }

    /// Add a button to the last row or create a new row if the last row not found
    #[must_use]
    pub fn push(mut self, button: Button) -> Self {
        match self.rows.last_mut() {
            Some(row) => row.push(button),
            None => self.rows.push(vec![button]),
        }
        self
    }
}

impl Keyboard for InlineKeyboard {
    fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context;
        let data = render_ctx.data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        if self.rows.is_empty() {
            return None;
        }

        let rows = self.rows.iter().map(|row| {
            row.iter()
                .map(|button| button.render(render_ctx))
                .collect::<Box<[_]>>()
        });

        Some(ReplyMarkup::InlineKeyboardMarkup(
            InlineKeyboardMarkup::new(rows),
        ))
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(ctx, callback_data))
    }
}
