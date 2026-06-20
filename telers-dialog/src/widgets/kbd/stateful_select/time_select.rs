use async_fn_traits::AsyncFn2;
use async_trait::async_trait;
use bon::bon;
use serde_json::json;
use std::{borrow::Cow, fmt::Display, sync::Arc};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{
    format_callback_data, parse_callback_data, when::is_allowed, ButtonAction, ClickContext,
    Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

type TimeSelectValueRenderer = dyn Fn(u8, &DataMap) -> String + Send + Sync + 'static;
type TimeSelectClickHandler =
    dyn Fn(ClickContext, u8) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;

/// Time picker storing the selected `(hour, minute)` pair in `widget_data`.
///
/// The widget renders a header row, a grid of selectable hours (24-hour),
/// another header row, and a grid of minutes stepped by `minute_precision`.
/// The selected hour or minute is written back as a JSON `[hour, minute]`
/// array under `widget_id`; missing values stay `null`. Optional
/// [`on_hour_click`](TimeSelectBuilder::on_hour_click) and
/// [`on_minute_click`](TimeSelectBuilder::on_minute_click) handlers run after
/// the state mutation and their action is chained onto the update.
pub struct TimeSelect<WidgetId> {
    id: WidgetId,
    hour_header: Cow<'static, str>,
    minute_header: Cow<'static, str>,
    button_renderer: Arc<TimeSelectValueRenderer>,
    selected_button_renderer: Arc<TimeSelectValueRenderer>,
    hour_width: usize,
    minute_precision: usize,
    minute_width: usize,
    on_hour_click: Option<Arc<TimeSelectClickHandler>>,
    on_minute_click: Option<Arc<TimeSelectClickHandler>>,
    when: Option<WhenCondition>,
}

#[bon]
impl<WidgetId> TimeSelect<WidgetId> {
    /// Create a time selection widget.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] on_hour_click: Option<Arc<TimeSelectClickHandler>>,
        #[builder(field)] on_minute_click: Option<Arc<TimeSelectClickHandler>>,
        #[builder(default = "Hour".into())] hour_header: Cow<'static, str>,
        #[builder(default = "Minute".into())] minute_header: Cow<'static, str>,
        #[builder(
            default = Arc::new(default_button_renderer),
            with = |button_renderer: impl Fn(u8, &DataMap) -> String + Send + Sync + 'static| {
                Arc::new(button_renderer)
            }
        )]
        button_renderer: Arc<TimeSelectValueRenderer>,
        #[builder(
            default = Arc::new(default_selected_button_renderer),
            with = |selected_button_renderer: impl Fn(u8, &DataMap) -> String + Send + Sync + 'static| {
                Arc::new(selected_button_renderer)
            }
        )]
        selected_button_renderer: Arc<TimeSelectValueRenderer>,
        #[builder(default = 6)] hour_width: usize,
        #[builder(default = 5)] minute_precision: usize,
        #[builder(default = 6)] minute_width: usize,
        when: Option<WhenCondition>,
    ) -> Self
    where
        WidgetId: Display,
    {
        Self {
            id,
            hour_header,
            minute_header,
            button_renderer,
            selected_button_renderer,
            hour_width,
            minute_precision,
            minute_width,
            on_hour_click,
            on_minute_click,
            when,
        }
    }
}

impl<WidgetId, S> TimeSelectBuilder<WidgetId, S>
where
    S: time_select_builder::State,
    WidgetId: Display,
{
    /// Register an async handler invoked after an hour has been chosen.
    pub fn on_hour_click<F>(mut self, on_hour_click: F) -> Self
    where
        F: AsyncFn(ClickContext, u8) -> ButtonAction
            + AsyncFn2<ClickContext, u8, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn2<ClickContext, u8>>::OutputFuture: Send + 'static,
    {
        let on_hour_click = Arc::new(on_hour_click);
        self.on_hour_click = Some(Arc::new(move |click, value| {
            let on_hour_click = on_hour_click.clone();
            Box::pin(async move { on_hour_click(click, value).await })
        }));
        self
    }

    /// Register an async handler invoked after a minute has been chosen.
    pub fn on_minute_click<F>(mut self, on_minute_click: F) -> Self
    where
        F: AsyncFn(ClickContext, u8) -> ButtonAction
            + AsyncFn2<ClickContext, u8, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn2<ClickContext, u8>>::OutputFuture: Send + 'static,
    {
        let on_minute_click = Arc::new(on_minute_click);
        self.on_minute_click = Some(Arc::new(move |click, value| {
            let on_minute_click = on_minute_click.clone();
            Box::pin(async move { on_minute_click(click, value).await })
        }));
        self
    }
}

impl<WidgetId> TimeSelect<WidgetId>
where
    WidgetId: Display,
{
    fn widget_id(&self) -> String {
        self.id.to_string()
    }

    fn read_value(&self, ctx: &Context) -> (Option<u8>, Option<u8>) {
        ctx.widget_value_as::<(Option<u8>, Option<u8>)>(&self.widget_id())
            .unwrap_or((None, None))
    }

    fn rows(start: u8, stop: u8, interval: usize, width: usize) -> Vec<Vec<u8>> {
        let interval = interval.max(1);
        let width = width.max(1);
        let mut rows = vec![Vec::new()];
        for value in (usize::from(start)..usize::from(stop)).step_by(interval) {
            if rows.last().is_some_and(|row| row.len() >= width) {
                rows.push(Vec::new());
            }
            rows.last_mut()
                .unwrap()
                .push(u8::try_from(value).unwrap_or(u8::MAX));
        }
        rows
    }

    fn header_button(&self, ctx: &Context, payload: &str, text: &str) -> InlineKeyboardButton {
        InlineKeyboardButton::new(text).callback_data(format_callback_data(
            ctx,
            &self.id,
            Some(payload),
        ))
    }

    fn value_button(
        &self,
        ctx: &Context,
        data: &DataMap,
        prefix: &str,
        value: u8,
        is_selected: bool,
    ) -> InlineKeyboardButton {
        let text = if is_selected {
            (self.selected_button_renderer)(value, data)
        } else {
            (self.button_renderer)(value, data)
        };
        InlineKeyboardButton::new(text).callback_data(format_callback_data(
            ctx,
            &self.id,
            Some(&format!("{prefix}{value}")),
        ))
    }
}

#[async_trait]
impl<WidgetId> Keyboard for TimeSelect<WidgetId>
where
    WidgetId: Display + Send + Sync + 'static,
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
        let (selected_hour, selected_minute) = self.read_value(ctx);
        let mut rows: Vec<Box<[InlineKeyboardButton]>> = Vec::new();

        rows.push([self.header_button(ctx, "hh", &self.hour_header)].into());
        for row in Self::rows(0, 24, 1, self.hour_width) {
            rows.push(
                row.into_iter()
                    .map(|hour| {
                        self.value_button(ctx, data, "h", hour, selected_hour == Some(hour))
                    })
                    .collect(),
            );
        }

        rows.push([self.header_button(ctx, "mm", &self.minute_header)].into());
        for row in Self::rows(0, 60, self.minute_precision, self.minute_width) {
            rows.push(
                row.into_iter()
                    .map(|minute| {
                        self.value_button(ctx, data, "m", minute, selected_minute == Some(minute))
                    })
                    .collect(),
            );
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
        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.widget_id() {
            return None;
        }

        let payload = parsed.payload?;
        if payload == "hh" || payload == "mm" {
            return Some(ButtonAction::noop());
        }

        let (mut hour, mut minute) = self.read_value(ctx);
        let click_action;
        if let Some(value) = payload.strip_prefix('h') {
            let value = value.parse::<u8>().ok()?;
            hour = Some(value);
            click_action = match &self.on_hour_click {
                Some(handler) => Some(handler(click.clone(), value).await),
                None => None,
            };
            debug!(
                context_id = %ctx.id,
                widget_id = %self.id,
                selected_hour = ?hour,
                "Resolved time-select hour callback"
            );
        } else if let Some(value) = payload.strip_prefix('m') {
            let value = value.parse::<u8>().ok()?;
            minute = Some(value);
            click_action = match &self.on_minute_click {
                Some(handler) => Some(handler(click.clone(), value).await),
                None => None,
            };
            debug!(
                context_id = %ctx.id,
                widget_id = %self.id,
                selected_minute = ?minute,
                "Resolved time-select minute callback"
            );
        } else {
            return None;
        }

        let update_action = ButtonAction::set_widget_value(self.widget_id(), json!([hour, minute]));
        Some(match click_action {
            Some(click_action) => ButtonAction::chain([update_action, click_action]),
            None => update_action,
        })
    }
}

fn default_button_renderer(value: u8, _data: &DataMap) -> String {
    format!("{value:02}")
}

fn default_selected_button_renderer(value: u8, _data: &DataMap) -> String {
    format!("[{value:02}]")
}
