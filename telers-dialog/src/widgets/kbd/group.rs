use telers::types::{InlineKeyboardMarkup, ReplyMarkup};

use super::{ButtonAction, Keyboard};
use crate::entities::{Context, DataMap};

/// Layout wrapper that regroups inline keyboard buttons into fixed-width rows.
pub struct Group<Kbd> {
    kbd: Kbd,
    items_per_row: usize,
}

impl<Kbd> Group<Kbd> {
    /// Wrap a keyboard and regroup its inline buttons by `items_per_row`.
    #[inline]
    #[must_use]
    pub fn new(kbd: Kbd, items_per_row: usize) -> Self {
        Self {
            kbd,
            items_per_row: items_per_row.max(1),
        }
    }
}

impl<Kbd> Keyboard for Group<Kbd>
where
    Kbd: Keyboard,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let markup = self.kbd.render_keyboard(ctx, data)?;
        let ReplyMarkup::InlineKeyboardMarkup(markup) = markup else {
            return Some(markup);
        };

        let mut grouped_rows = Vec::new();
        let mut current_row = Vec::with_capacity(self.items_per_row);
        for button in markup.inline_keyboard.into_vec().into_iter().flatten() {
            current_row.push(button);
            if current_row.len() == self.items_per_row {
                grouped_rows.push(current_row.into_boxed_slice());
                current_row = Vec::with_capacity(self.items_per_row);
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
    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        self.kbd.handle_callback(ctx, callback_data)
    }
}
