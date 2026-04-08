use telers::types::{InlineKeyboardMarkup, ReplyMarkup};

use super::{Button, ButtonAction, Keyboard};
use crate::entities::{Context, DataMap};

#[derive(Default)]
pub struct InlineKeyboard {
    rows: Vec<Vec<Button>>,
}

impl InlineKeyboard {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
}

impl Keyboard for InlineKeyboard {
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        if self.rows.is_empty() {
            return None;
        }

        let rows = self.rows.iter().map(|row| {
            row.iter()
                .map(|button| button.render(ctx, data))
                .collect::<Box<[_]>>()
        });

        Some(ReplyMarkup::InlineKeyboardMarkup(
            InlineKeyboardMarkup::new(rows),
        ))
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(ctx, callback_data))
    }
}
