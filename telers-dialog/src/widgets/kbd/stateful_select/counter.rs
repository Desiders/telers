use bon::bon;
use std::{borrow::Cow, fmt::Display};

use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{
    format_callback_data, parse_callback_data, render_button_row, when::is_allowed, Button,
    ButtonAction, ClickContext, Keyboard, WhenCondition,
};
use crate::entities::{Context, DataMap, RenderContext};

/// Numeric counter widget stored in `widget_data`.
pub struct Counter<WidgetId> {
    id: WidgetId,
    plus_text: Cow<'static, str>,
    minus_text: Cow<'static, str>,
    plus_hidden: bool,
    minus_hidden: bool,
    min: f64,
    max: f64,
    increment: f64,
    default: f64,
    cycle: bool,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    when: Option<WhenCondition>,
}

#[bon]
impl<WidgetId> Counter<WidgetId> {
    /// Create a counter widget.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        #[builder(default = "+".into())] plus_text: Cow<'static, str>,
        #[builder(default = "-".into())] minus_text: Cow<'static, str>,
        #[builder(default = false)] plus_hidden: bool,
        #[builder(default = false)] minus_hidden: bool,
        #[builder(default = 0.0)] min: f64,
        #[builder(default = 999_999.0)] max: f64,
        #[builder(default = 1.0)] increment: f64,
        #[builder(default = 0.0)] default: f64,
        #[builder(default = false)] cycle: bool,
        when: Option<WhenCondition>,
    ) -> Self
    where
        WidgetId: Display,
    {
        Self {
            id,
            plus_text,
            minus_text,
            plus_hidden,
            minus_hidden,
            min,
            max,
            increment,
            default,
            cycle,
            header_rows,
            footer_rows,
            when,
        }
    }
}

impl<S, WidgetId> CounterBuilder<WidgetId, S>
where
    S: counter_builder::State,
    WidgetId: Display,
{
    /// Append a full header row before the counter row.
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

    /// Append a full footer row after the counter row.
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

impl<WidgetId> Keyboard for Counter<WidgetId>
where
    WidgetId: Display + Send + Sync + 'static,
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
        let value = ctx
            .widget_value_as::<f64>(&widget_id)
            .unwrap_or(self.default)
            .clamp(self.min, self.max);

        let mut rows: Vec<_> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, render_ctx))
            .collect();

        let mut counter_row = Vec::new();
        if !self.minus_hidden {
            counter_row.push(
                InlineKeyboardButton::new(self.minus_text.as_ref())
                    .callback_data(format_callback_data(ctx, &self.id, Some("-"))),
            );
        }
        counter_row.push(
            InlineKeyboardButton::new(render_value(value)).callback_data(format_callback_data(
                ctx,
                &self.id,
                Some(""),
            )),
        );
        if !self.plus_hidden {
            counter_row.push(
                InlineKeyboardButton::new(self.plus_text.as_ref())
                    .callback_data(format_callback_data(ctx, &self.id, Some("+"))),
            );
        }
        rows.push(counter_row.into_boxed_slice());

        rows.extend(
            self.footer_rows
                .iter()
                .map(|row| render_button_row(row, render_ctx)),
        );

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        let callback_data = click.callback_data;
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        if let Some(action) = self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(click))
        {
            return Some(action);
        }

        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.to_string() {
            return None;
        }

        let current = ctx
            .widget_value_as::<f64>(&self.id.to_string())
            .unwrap_or(self.default)
            .clamp(self.min, self.max);

        match parsed.payload? {
            "+" => {
                let mut next = current + self.increment;
                if next > self.max {
                    next = if self.cycle { self.min } else { self.max };
                }
                debug!(
                    context_id = %ctx.id,
                    widget_id = %self.id,
                    current,
                    next,
                    "Resolved counter increment callback"
                );
                Some(ButtonAction::set_widget_value(
                    Cow::Owned(self.id.to_string()),
                    next,
                ))
            }
            "-" => {
                let mut next = current - self.increment;
                if next < self.min {
                    next = if self.cycle { self.max } else { self.min };
                }
                debug!(
                    context_id = %ctx.id,
                    widget_id = %self.id,
                    current,
                    next,
                    "Resolved counter decrement callback"
                );
                Some(ButtonAction::set_widget_value(
                    Cow::Owned(self.id.to_string()),
                    next,
                ))
            }
            "" => Some(ButtonAction::noop()),
            _ => None,
        }
    }
}

fn render_value(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}
