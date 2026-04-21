use bon::bon;
use telers::types::{InlineKeyboardMarkup, ReplyMarkup};

use super::{when::is_allowed, Button, ButtonAction, ClickContext, Keyboard, WhenCondition};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

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
    pub fn row(mut self, row: impl IntoIterator<Item = Button>) -> Self {
        self.rows.push(row.into_iter().collect());
        self
    }

    /// Add a button to the last row or create a new row if the last row not found
    pub fn push(mut self, button: Button) -> Self {
        match self.rows.last_mut() {
            Some(row) => row.push(button),
            None => self.rows.push(vec![button]),
        }
        self
    }
}

impl Keyboard for InlineKeyboard {
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
            if self.rows.is_empty() {
                return None;
            }

            let mut rows = Vec::with_capacity(self.rows.len());
            for row in &self.rows {
                let mut rendered_row = Vec::with_capacity(row.len());
                for button in row {
                    rendered_row.push(button.render(render_ctx).await);
                }
                rows.push(rendered_row.into_boxed_slice());
            }

            Some(ReplyMarkup::InlineKeyboardMarkup(
                InlineKeyboardMarkup::new(rows),
            ))
        })
    }

    fn handle_callback<'a>(
        &'a self,
        click: &'a ClickContext,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
            let ctx = click.context.as_ref();
            let data = &ctx.dialog_data;
            if !self.is_visible(ctx, data).await {
                return None;
            }
            for button in self.rows.iter().flat_map(|row| row.iter()) {
                if let Some(action) = button.resolve_callback(click).await {
                    return Some(action);
                }
            }
            None
        })
    }
}
