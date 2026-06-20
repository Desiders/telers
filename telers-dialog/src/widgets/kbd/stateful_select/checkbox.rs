use async_trait::async_trait;
use bon::bon;
use std::{borrow::Cow, fmt::Display};

use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{
    format_callback_data, macros::impl_button_row_helpers, parse_callback_data, render_button_row,
    when::is_allowed, Button, ButtonAction, ClickContext, Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    widgets::Text,
};

/// Single-button boolean toggle stored in `widget_data`.
///
/// The button label switches between `checked_text` and `unchecked_text` based
/// on the boolean stored under `widget_id`. The widget writes back the toggled
/// value as a [`ButtonAction::SetWidgetValue`] when clicked.
///
/// [`ButtonAction::SetWidgetValue`]: crate::widgets::ButtonAction::SetWidgetValue
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
    impl_button_row_helpers!();
}

#[async_trait]
impl<WidgetId, CheckedText, UncheckedText> Keyboard
    for Checkbox<WidgetId, CheckedText, UncheckedText>
where
    WidgetId: Display + Send + Sync + 'static,
    CheckedText: Text,
    UncheckedText: Text,
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
        let widget_id = self.id.to_string();
        let is_checked = ctx
            .widget_value_as::<bool>(&widget_id)
            .unwrap_or(self.default);
        let next_value = (!is_checked).to_string();
        let text = if is_checked {
            self.checked_text.render_text_in_context(render_ctx).await
        } else {
            self.unchecked_text.render_text_in_context(render_ctx).await
        };

        let mut rows = Vec::new();
        for row in &self.header_rows {
            rows.push(render_button_row(row, render_ctx).await);
        }

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

        for row in &self.footer_rows {
            rows.push(render_button_row(row, render_ctx).await);
        }

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        let callback_data = click.callback_data.as_str();
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data).await {
            return None;
        }
        for button in self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
        {
            if let Some(action) = button.resolve_callback(click).await {
                return Some(action);
            }
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
