use bon::bon;
use std::{borrow::Cow, fmt::Display};

use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{
    format_callback_data, parse_callback_data, render_button_row, when::is_allowed, Button,
    ButtonAction, Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    widgets::Text,
};

/// Single-button boolean selector stored in `widget_data`.
pub struct Checkbox<WidgetId, CheckedText, UncheckedText> {
    id: WidgetId,
    checked_text: CheckedText,
    unchecked_text: UncheckedText,
    default: bool,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    when: Option<WhenCondition>,
}

#[bon]
impl<WidgetId, CheckedText, UncheckedText> Checkbox<WidgetId, CheckedText, UncheckedText> {
    /// Create a checkbox widget.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        checked_text: CheckedText,
        unchecked_text: UncheckedText,
        #[builder(default = false)] default: bool,
        when: Option<WhenCondition>,
    ) -> Self
    where
        WidgetId: Display,
        CheckedText: Text,
        UncheckedText: Text,
    {
        Self {
            id,
            checked_text,
            unchecked_text,
            default,
            header_rows,
            footer_rows,
            when,
        }
    }
}

impl<S, WidgetId, CheckedText, UncheckedText>
    CheckboxBuilder<WidgetId, CheckedText, UncheckedText, S>
where
    S: checkbox_builder::State,
    WidgetId: Display,
    CheckedText: Text,
    UncheckedText: Text,
{
    /// Append a full header row before the checkbox button.
    pub fn header_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.header_rows.push(buttons.into_iter().collect());
        self
    }

    /// Append a button to the last header row, or create one if absent.
    pub fn header_push(mut self, button: Button) -> Self {
        match self.header_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.header_rows.push(vec![button]),
        }
        self
    }

    /// Append a full footer row after the checkbox button.
    pub fn footer_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.footer_rows.push(buttons.into_iter().collect());
        self
    }

    /// Append a button to the last footer row, or create one if absent.
    pub fn footer_push(mut self, button: Button) -> Self {
        match self.footer_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.footer_rows.push(vec![button]),
        }
        self
    }
}

impl<WidgetId, CheckedText, UncheckedText> Keyboard
    for Checkbox<WidgetId, CheckedText, UncheckedText>
where
    WidgetId: Display + Send + Sync + 'static,
    CheckedText: Text,
    UncheckedText: Text,
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
        let widget_id = self.id.to_string();
        let is_checked = ctx
            .widget_value_as::<bool>(&widget_id)
            .unwrap_or(self.default);
        let next_value = (!is_checked).to_string();
        let text = if is_checked {
            self.checked_text.render_text_in_context(render_ctx)
        } else {
            self.unchecked_text.render_text_in_context(render_ctx)
        };

        let mut rows: Vec<_> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, render_ctx))
            .collect();

        rows.push(
            [
                InlineKeyboardButton::new(text).callback_data(format_callback_data(
                    ctx,
                    &self.id,
                    Some(&next_value),
                )),
            ]
            .into(),
        );

        rows.extend(
            self.footer_rows
                .iter()
                .map(|row| render_button_row(row, render_ctx)),
        );

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        if let Some(action) = self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(ctx, callback_data))
        {
            return Some(action);
        }

        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.to_string() {
            return None;
        }

        let checked = match parsed.payload? {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => return None,
        };
        debug!(
            context_id = %ctx.id,
            widget_id = %self.id,
            checked,
            "Resolved checkbox toggle callback"
        );
        Some(ButtonAction::set_widget_value(
            Cow::Owned(self.id.to_string()),
            checked,
        ))
    }
}
