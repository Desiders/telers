//! Builders for inline and reply keyboards.
//!
//! [`KeyboardBuilder`] mirrors aiogram's `KeyboardBuilder`: [`Self::add`]
//! flows buttons into rows of at most `adjust` columns, and [`Self::row`]
//! appends explicit rows.

use crate::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup,
};
use std::mem;

/// A button type that a [`KeyboardBuilder`] can build a markup from.
pub trait KeyboardButtonKind: Sized {
    /// The markup produced by [`KeyboardBuilder::as_markup`].
    type Markup;

    /// Default maximum columns per row.
    const DEFAULT_ADJUST: usize;

    /// Maximum number of buttons in a markup.
    const MAX_BUTTONS: usize;

    /// Wrap a list of rows into the concrete markup type.
    fn from_rows(rows: Box<[Box<[Self]>]>) -> Self::Markup;
}

impl KeyboardButtonKind for InlineKeyboardButton {
    type Markup = InlineKeyboardMarkup;

    const DEFAULT_ADJUST: usize = 8;
    const MAX_BUTTONS: usize = 100;

    fn from_rows(rows: Box<[Box<[Self]>]>) -> Self::Markup {
        InlineKeyboardMarkup::new(rows)
    }
}

impl KeyboardButtonKind for KeyboardButton {
    type Markup = ReplyKeyboardMarkup;

    const DEFAULT_ADJUST: usize = 10;
    const MAX_BUTTONS: usize = 300;

    fn from_rows(rows: Box<[Box<[Self]>]>) -> Self::Markup {
        ReplyKeyboardMarkup::new(rows)
    }
}

/// Accumulates buttons and exports them as a keyboard markup.
///
/// [`Self::add`] appends buttons flowing into rows of at most `adjust`
/// columns, [`Self::row`] appends explicit rows.
pub struct KeyboardBuilder<B> {
    rows: Vec<Vec<B>>,
    adjust: usize,
    buttons: usize,
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
            rows: Vec::new(),
            adjust: B::DEFAULT_ADJUST,
            buttons: 0,
        }
    }

    /// Appends one button, starting a new row when the last one is full.
    ///
    /// # Panics
    ///
    /// Panics if the markup already has [`KeyboardButtonKind::MAX_BUTTONS`]
    /// buttons.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, button: B) -> Self
    where
        B: KeyboardButtonKind,
    {
        assert!(
            self.buttons < B::MAX_BUTTONS,
            "cannot add more than {} buttons",
            B::MAX_BUTTONS
        );
        match self.rows.last_mut() {
            Some(row) if row.len() < self.adjust => row.push(button),
            _ => self.rows.push(vec![button]),
        }
        self.buttons += 1;
        self
    }

    /// Appends one or more explicit rows.
    ///
    /// When too many buttons are passed, they are split into rows of at most
    /// `adjust` columns.
    ///
    /// # Panics
    ///
    /// Panics if the markup would exceed [`KeyboardButtonKind::MAX_BUTTONS`]
    /// buttons.
    #[must_use]
    pub fn row(mut self, buttons: impl IntoIterator<Item = B>) -> Self
    where
        B: KeyboardButtonKind,
    {
        let mut row: Vec<B> = Vec::new();
        for button in buttons {
            assert!(
                self.buttons < B::MAX_BUTTONS,
                "cannot add more than {} buttons",
                B::MAX_BUTTONS
            );
            if row.len() == self.adjust {
                self.rows.push(mem::take(&mut row));
            }
            row.push(button);
            self.buttons += 1;
        }
        if !row.is_empty() {
            self.rows.push(row);
        }
        self
    }

    /// Re-flows all buttons into rows of at most `max_columns` columns.
    #[must_use]
    pub fn adjust(mut self, max_columns: usize) -> Self {
        let adjust = max_columns.max(1);
        let buttons: Vec<B> = self.rows.into_iter().flatten().collect();
        let mut rows = Vec::with_capacity(buttons.len().div_ceil(adjust));
        let mut iter = buttons.into_iter();
        loop {
            let row: Vec<B> = iter.by_ref().take(adjust).collect();
            if row.is_empty() {
                break;
            }
            rows.push(row);
        }
        self.rows = rows;
        self.adjust = adjust;
        self
    }

    /// Exports the buttons as rows.
    #[must_use]
    pub fn export(self) -> Box<[Box<[B]>]> {
        self.rows.into_iter().map(Into::into).collect()
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
    fn add_flows_into_rows() {
        let rows = InlineKeyboardBuilder::new()
            .adjust(2)
            .add(inline_button("a"))
            .add(inline_button("b"))
            .add(inline_button("c"))
            .export();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 1);
    }

    #[test]
    fn row_appends_explicit_rows() {
        let rows = InlineKeyboardBuilder::new()
            .add(inline_button("a"))
            .row([inline_button("b"), inline_button("c")])
            .row([inline_button("d")])
            .export();

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[1].len(), 2);
        assert_eq!(rows[2].len(), 1);
    }

    #[test]
    fn row_splits_too_long_rows() {
        let rows = InlineKeyboardBuilder::new()
            .adjust(2)
            .row([inline_button("a"), inline_button("b"), inline_button("c")])
            .export();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 1);
    }

    #[test]
    #[should_panic]
    fn add_panics_beyond_max() {
        let mut builder = InlineKeyboardBuilder::new();
        for _ in 0..InlineKeyboardButton::MAX_BUTTONS {
            builder = builder.add(inline_button("x"));
        }
        let _ = builder.add(inline_button("x"));
    }

    #[test]
    fn as_markup_returns_inline_markup() {
        let markup = InlineKeyboardBuilder::new()
            .add(inline_button("a"))
            .add(inline_button("b"))
            .as_markup();

        assert_eq!(markup.inline_keyboard.len(), 1);
        assert_eq!(markup.inline_keyboard[0].len(), 2);
    }

    #[test]
    fn reply_builder_returns_reply_markup() {
        let markup = ReplyKeyboardBuilder::new()
            .add(KeyboardButton::new("yes"))
            .add(KeyboardButton::new("no"))
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
