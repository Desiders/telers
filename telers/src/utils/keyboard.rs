//! Builders for inline and reply keyboards.
//!
//! [`KeyboardBuilder`] accumulates buttons in a flat list and chunks them into
//! rows of at most `adjust` columns when exported, mirroring aiogram's
//! `KeyboardBuilder`.

use crate::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup,
};

/// A button type that a [`KeyboardBuilder`] can build a markup from.
pub trait KeyboardButtonKind: Clone {
    /// The markup produced by [`KeyboardBuilder::as_markup`].
    type Markup;

    /// Default maximum columns per row.
    const DEFAULT_ADJUST: usize;

    /// Wrap a list of rows into the concrete markup type.
    fn from_rows(rows: Box<[Box<[Self]>]>) -> Self::Markup;
}

impl KeyboardButtonKind for InlineKeyboardButton {
    type Markup = InlineKeyboardMarkup;

    const DEFAULT_ADJUST: usize = 8;

    fn from_rows(rows: Box<[Box<[Self]>]>) -> Self::Markup {
        InlineKeyboardMarkup::new(rows)
    }
}

impl KeyboardButtonKind for KeyboardButton {
    type Markup = ReplyKeyboardMarkup;

    const DEFAULT_ADJUST: usize = 10;

    fn from_rows(rows: Box<[Box<[Self]>]>) -> Self::Markup {
        ReplyKeyboardMarkup::new(rows)
    }
}

/// Accumulates buttons and exports them as a keyboard markup.
///
/// Buttons are stored in a flat list and split into rows of at most
/// [`Self::adjust`] columns when exported.
pub struct KeyboardBuilder<B> {
    buttons: Vec<B>,
    adjust: usize,
}

impl<B> KeyboardBuilder<B> {
    /// Creates an empty builder.
    ///
    /// Defaults to [`KeyboardButtonKind::DEFAULT_ADJUST`] columns, matching the
    /// Telegram limits.
    #[must_use]
    pub fn new() -> Self
    where
        B: KeyboardButtonKind,
    {
        Self {
            buttons: Vec::new(),
            adjust: B::DEFAULT_ADJUST,
        }
    }

    /// Appends one button.
    #[must_use]
    pub fn button(mut self, button: B) -> Self {
        self.buttons.push(button);
        self
    }

    /// Sets the maximum number of columns per row used by [`Self::export`].
    #[must_use]
    pub fn adjust(mut self, max_columns: usize) -> Self {
        self.adjust = max_columns;
        self
    }

    /// Splits the accumulated buttons into rows of at most `adjust` columns.
    #[must_use]
    pub fn export(self) -> Box<[Box<[B]>]>
    where
        B: Clone,
    {
        self.buttons
            .chunks(self.adjust.max(1))
            .map(Into::into)
            .collect()
    }

    /// Exports the buttons as a keyboard markup.
    #[must_use]
    pub fn as_markup(self) -> B::Markup
    where
        B: KeyboardButtonKind,
    {
        B::from_rows(self.export())
    }
}

impl<B: KeyboardButtonKind> Default for KeyboardBuilder<B> {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for inline keyboards (up to 8 columns per row, like Telegram).
pub type InlineKeyboardBuilder = KeyboardBuilder<InlineKeyboardButton>;

/// Builder for reply keyboards (up to 10 columns per row, like Telegram).
pub type ReplyKeyboardBuilder = KeyboardBuilder<KeyboardButton>;

#[cfg(test)]
mod tests {
    use super::*;

    fn inline_button(text: &str) -> InlineKeyboardButton {
        InlineKeyboardButton::new(text)
    }

    #[test]
    fn export_chunks_by_adjust() {
        let builder = InlineKeyboardBuilder::new()
            .button(inline_button("a"))
            .button(inline_button("b"))
            .button(inline_button("c"))
            .adjust(2);

        let rows = builder.export();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 1);
    }

    #[test]
    fn as_markup_returns_inline_markup() {
        let markup = InlineKeyboardBuilder::new()
            .button(inline_button("a"))
            .button(inline_button("b"))
            .as_markup();

        assert_eq!(markup.inline_keyboard.len(), 1);
        assert_eq!(markup.inline_keyboard[0].len(), 2);
    }

    #[test]
    fn reply_builder_returns_reply_markup() {
        let markup = ReplyKeyboardBuilder::new()
            .button(KeyboardButton::new("yes"))
            .button(KeyboardButton::new("no"))
            .adjust(1)
            .as_markup();

        assert_eq!(markup.keyboard.len(), 2);
        assert_eq!(markup.keyboard[0][0].text.as_ref(), "yes");
    }

    #[test]
    fn empty_export_produces_no_rows() {
        let rows = InlineKeyboardBuilder::new().export();
        assert!(rows.is_empty());
    }
}
