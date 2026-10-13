use async_fn_traits::AsyncFn2;
use async_trait::async_trait;
use bon::bon;
use std::{borrow::Cow, fmt::Display, sync::Arc};
use telers::{
    serialization::json,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup},
};
use tracing::debug;
use time::Time;

use super::change::with_widget_value;
use super::super::{
    format_callback_data, parse_callback_data, when::is_allowed, ButtonAction, ButtonAppearance, ClickContext,
    Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

type TimeSelectValueRenderer = dyn Fn(u8, &DataMap) -> String + Send + Sync + 'static;
type TimeSelectClickHandler =
    dyn Fn(ClickContext, u8) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;
type TimeSelectChangedHandler =
    dyn Fn(ClickContext, Option<Time>) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;

/// Time picker storing the selected `(hour, minute)` pair in `widget_data`.
///
/// The widget renders a header row, a grid of selectable hours (24-hour),
/// another header row, and a grid of minutes stepped by `minute_precision`.
/// The selected hour or minute is written back as a JSON `[hour, minute]`
/// array under `widget_id`; missing values stay `null`. Actions returned by
/// `on_hour_click` and `on_minute_click` are chained after the state update.
/// `on_value_changed` receives a complete [`Time`] once both fields are selected,
/// or `None` while selection is partial. Selected appearances inherit the common
/// button style and emoji when those fields are omitted.
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
    on_value_changed: Option<Arc<TimeSelectChangedHandler>>,
    when: Option<WhenCondition>,
    header_appearance: Option<ButtonAppearance>,
    button_appearance: Option<ButtonAppearance>,
    selected_button_appearance: Option<ButtonAppearance>,
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
        #[builder(field)] on_value_changed: Option<Arc<TimeSelectChangedHandler>>,
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
        header_appearance: Option<ButtonAppearance>,
        button_appearance: Option<ButtonAppearance>,
        selected_button_appearance: Option<ButtonAppearance>,
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
            on_value_changed,
            when,
            header_appearance,
            button_appearance,
            selected_button_appearance,
        }
    }
}

impl<WidgetId, S> TimeSelectBuilder<WidgetId, S>
where
    S: time_select_builder::State,
    WidgetId: Display,
{
    /// Handle the complete selected time, or `None` while either field is unset.
    pub fn on_value_changed<F>(mut self, on_value_changed: F) -> Self
    where
        F: AsyncFn(ClickContext, Option<Time>) -> ButtonAction
            + AsyncFn2<ClickContext, Option<Time>, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn2<ClickContext, Option<Time>>>::OutputFuture: Send + 'static,
    {
        let on_value_changed = Arc::new(on_value_changed);
        self.on_value_changed = Some(Arc::new(move |click, value| {
            let on_value_changed = on_value_changed.clone();
            Box::pin(async move { on_value_changed(click, value).await })
        }));
        self
    }

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

    async fn value_button(
        &self,
        render_ctx: &RenderContext,
        prefix: &str,
        value: u8,
        is_selected: bool,
    ) -> InlineKeyboardButton {
        let ctx = render_ctx.context.as_ref();
        let data = render_ctx.data.as_ref();
        let text = if is_selected {
            (self.selected_button_renderer)(value, data)
        } else {
            (self.button_renderer)(value, data)
        };
        let mut button = InlineKeyboardButton::new(text).callback_data(format_callback_data(
            ctx,
            &self.id,
            Some(&format!("{prefix}{value}")),
        ));
        let appearance = match (&self.selected_button_appearance, &self.button_appearance, is_selected) {
            (Some(selected), Some(base), true) => Some(selected.clone().or(base.clone())),
            (Some(selected), _, true) => Some(selected.clone()),
            (_, base, _) => base.clone(),
        };
        if let Some(appearance) = appearance {
            let mut data = data.clone();
            data.insert("value".into(), value.into());
            let render_ctx = render_ctx.clone().with_data(data);
            button = appearance.apply(button, &render_ctx).await;
        }
        button
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

        let mut header = self.header_button(ctx, "hh", &self.hour_header);
        if let Some(appearance) = &self.header_appearance {
            header = appearance.apply(header, render_ctx).await;
        }
        rows.push([header].into());
        for row in Self::rows(0, 24, 1, self.hour_width) {
            let mut buttons = Vec::with_capacity(row.len());
            for hour in row {
                buttons.push(self.value_button(render_ctx, "h", hour, selected_hour == Some(hour)).await);
            }
            rows.push(buttons.into_boxed_slice());
        }

        let mut header = self.header_button(ctx, "mm", &self.minute_header);
        if let Some(appearance) = &self.header_appearance {
            header = appearance.apply(header, render_ctx).await;
        }
        rows.push([header].into());
        for row in Self::rows(0, 60, self.minute_precision, self.minute_width) {
            let mut buttons = Vec::with_capacity(row.len());
            for minute in row {
                buttons.push(self.value_button(render_ctx, "m", minute, selected_minute == Some(minute)).await);
            }
            rows.push(buttons.into_boxed_slice());
        }

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        let callback_data = click.callback_data.as_str();
        let data = click.data.as_ref();
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
            if value >= 24 {
                return None;
            }
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
        } else {
            let value = payload.strip_prefix('m')?;
            let value = value.parse::<u8>().ok()?;
            if value >= 60 || !usize::from(value).is_multiple_of(self.minute_precision.max(1)) {
                return None;
            }
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
        }

        let update_action = ButtonAction::set_widget_value(self.widget_id(), json!([hour, minute]));
        let changed_action = match &self.on_value_changed {
            Some(handler) => {
                let value = hour.zip(minute).and_then(|(hour, minute)| Time::from_hms(hour, minute, 0).ok());
                let updated_click = with_widget_value(click, self.widget_id(), json!([hour, minute]));
                Some(handler(updated_click, value).await)
            }
            None => None,
        };
        if click_action.is_none() && changed_action.is_none() {
            return Some(update_action);
        }
        Some(ButtonAction::chain(
            std::iter::once(update_action).chain(click_action).chain(changed_action),
        ))
    }
}

fn default_button_renderer(value: u8, _data: &DataMap) -> String {
    format!("{value:02}")
}

fn default_selected_button_renderer(value: u8, _data: &DataMap) -> String {
    format!("[{value:02}]")
}

#[cfg(test)]
mod tests {
    use telers::serialization::{json, Value};
    use time::Time;

    use super::TimeSelect;
    use crate::{
        entities::{Context, DataMap},
        widgets::{ButtonAction, ButtonAppearance, ButtonStyle, ClickContext, Keyboard},
    };

    async fn store_time(_click: ClickContext, value: Option<Time>) -> ButtonAction {
        ButtonAction::set_dialog_value(
            "selected",
            value.map(|value| format!("{:02}:{:02}", value.hour(), value.minute())),
        )
    }

    #[tokio::test]
    async fn time_select_changed_hook_observes_updated_context() {
        let mut ctx = Context::new("", "state", Value::default());
        ctx.widget_data.insert("time".into(), json!([13, null]));
        let picker = TimeSelect::builder("time")
            .on_value_changed(|click: ClickContext, value: Option<Time>| async move {
                assert_eq!(value, Time::from_hms(13, 30, 0).ok());
                assert_eq!(click.context.widget_value("time"), Some(&json!([13, 30])));
                assert_eq!(
                    click.data.get("widget_data").unwrap()["time"],
                    json!([13, 30])
                );
                ButtonAction::noop()
            })
            .build();
        assert!(picker
            .handle_callback_for_test(&ctx, &format!("td:{}:time:m30", ctx.id))
            .await
            .is_some());
        assert_eq!(ctx.widget_value("time"), Some(&json!([13, null])));
    }

    #[tokio::test]
    async fn time_select_rejects_invalid_or_unrendered_values() {
        let ctx = Context::new("", "state", Value::default());
        let picker = TimeSelect::builder("time")
            .minute_precision(15)
            .build();
        for payload in ["h24", "h255", "m60", "m255", "m7", "h-1", "x12"] {
            assert!(picker
                .handle_callback_for_test(&ctx, &format!("td:{}:time:{payload}", ctx.id))
                .await
                .is_none());
        }
        assert!(picker
            .handle_callback_for_test(&ctx, &format!("td:{}:time:h23", ctx.id))
            .await
            .is_some());
        assert!(picker
            .handle_callback_for_test(&ctx, &format!("td:{}:time:m45", ctx.id))
            .await
            .is_some());
    }

    #[tokio::test]
    async fn time_select_selected_appearance_inherits_common_emoji() {
        let mut ctx = Context::new("", "state", Value::default());
        ctx.widget_data.insert("time".into(), json!([1, 15]));
        let picker = TimeSelect::builder("time")
            .header_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Primary)
                    .build(),
            )
            .button_appearance(
                ButtonAppearance::builder()
                    .icon_custom_emoji_id("123")
                    .build(),
            )
            .selected_button_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Success)
                    .build(),
            )
            .build();
        let markup = picker
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows[0][0].style.as_deref(), Some("primary"));
        assert_eq!(rows[1][1].style.as_deref(), Some("success"));
        assert_eq!(rows[1][1].icon_custom_emoji_id.as_deref(), Some("123"));
        assert_eq!(rows[1][0].style, None);
        assert_eq!(rows[1][0].icon_custom_emoji_id.as_deref(), Some("123"));
    }

    #[tokio::test]
    async fn time_select_value_changed_receives_partial_and_complete_time() {
        let mut ctx = Context::new("", "state", Value::default());
        let picker = TimeSelect::builder("time")
            .on_value_changed(store_time)
            .build();
        let action = picker
            .handle_callback_for_test(&ctx, &format!("td:{}:time:h13", ctx.id))
            .await
            .unwrap();
        let ButtonAction::Chain(actions) = action else {
            panic!("expected state update and notification");
        };
        assert!(matches!(
            &actions[0],
            ButtonAction::SetWidgetValue { value, .. } if value == &json!([13, null])
        ));
        assert!(matches!(
            &actions[1],
            ButtonAction::SetDialogValue { value, .. } if value == &json!(null)
        ));

        ctx.widget_data.insert("time".into(), json!([13, null]));
        let action = picker
            .handle_callback_for_test(&ctx, &format!("td:{}:time:m30", ctx.id))
            .await
            .unwrap();
        let ButtonAction::Chain(actions) = action else {
            panic!("expected state update and notification");
        };
        assert!(matches!(
            &actions[0],
            ButtonAction::SetWidgetValue { value, .. } if value == &json!([13, 30])
        ));
        assert!(matches!(
            &actions[1],
            ButtonAction::SetDialogValue { value, .. } if value == &json!("13:30")
        ));
    }

    #[tokio::test]
    async fn time_select_runs_click_hook_before_value_changed_hook() {
        let ctx = Context::new("", "state", Value::default());
        let picker = TimeSelect::builder("time")
            .on_hour_click(|_click, hour| async move {
                ButtonAction::set_dialog_value("hour", hour)
            })
            .on_value_changed(store_time)
            .build();
        let action = picker
            .handle_callback_for_test(&ctx, &format!("td:{}:time:h13", ctx.id))
            .await
            .unwrap();
        let ButtonAction::Chain(actions) = action else {
            panic!("expected state update and notifications");
        };
        assert_eq!(actions.len(), 3);
        assert!(matches!(
            &actions[1],
            ButtonAction::SetDialogValue { key, value }
                if key.as_ref() == "hour" && value == &json!(13)
        ));
        assert!(matches!(
            &actions[2],
            ButtonAction::SetDialogValue { key, value }
                if key.as_ref() == "selected" && value == &json!(null)
        ));
    }
}
