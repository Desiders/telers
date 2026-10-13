use std::sync::Arc;

use async_fn_traits::AsyncFn1;

use super::super::{ButtonAction, ClickContext};
use crate::{entities::Data, future::BoxFuture};

type StateChangedHandler =
    dyn Fn(StateChange) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;

/// A widget value transition. The click context includes the newly selected value.
#[derive(Clone, Debug)]
pub struct StateChange {
    pub click: ClickContext,
    pub widget_id: Box<str>,
    pub old_value: Option<Data>,
    pub new_value: Data,
}

/// Async hook that can return a dialog action after a widget's value changes.
#[derive(Clone)]
pub struct OnStateChanged(Arc<StateChangedHandler>);

impl OnStateChanged {
    #[must_use]
    pub fn new<F>(handler: F) -> Self
    where
        F: AsyncFn(StateChange) -> ButtonAction
            + AsyncFn1<StateChange, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<StateChange>>::OutputFuture: Send + 'static,
    {
        let handler = Arc::new(handler);
        Self(Arc::new(move |change| {
            let handler = handler.clone();
            Box::pin(async move { handler(change).await })
        }))
    }
}

pub(super) async fn state_changed_action(
    click: &ClickContext,
    widget_id: String,
    old_value: Option<Data>,
    new_value: Data,
    on_state_changed: Option<&OnStateChanged>,
) -> ButtonAction {
    let mutation = ButtonAction::set_widget_value(widget_id.clone(), new_value.clone());
    let Some(handler) = on_state_changed else {
        return mutation;
    };
    if old_value.as_ref() == Some(&new_value) {
        return mutation;
    }

    let updated_click = with_widget_value(click, widget_id.clone(), new_value.clone());
    let action = (handler.0)(StateChange {
        click: updated_click,
        widget_id: widget_id.into_boxed_str(),
        old_value,
        new_value,
    })
    .await;
    ButtonAction::chain([mutation, action])
}

pub(super) fn with_widget_value(click: &ClickContext, widget_id: String, value: Data) -> ClickContext {
    let mut updated_click = click.clone();
    let context = Arc::make_mut(&mut updated_click.context);
    context.widget_data.insert(widget_id, value);
    let widget_data = telers::serialization::to_value(&context.widget_data).unwrap_or_default();
    Arc::make_mut(&mut updated_click.data).insert("widget_data".into(), widget_data);
    updated_click
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use telers::serialization::{json, Value};

    use super::OnStateChanged;
    use crate::{
        entities::{Context, DataMap},
        widgets::{
            ButtonAction, ButtonAppearance, ButtonStyle, Checkbox, Counter, FormatText, Keyboard,
            Multiselect, Radio, Toggle,
        },
    };

    fn context() -> Context {
        Context::new("", "state", Value::default())
    }

    fn callback(ctx: &Context, id: &str, payload: &str) -> String {
        format!("td:{}:{id}:{payload}", ctx.id)
    }

    fn observer(expected: Value, old: Option<Value>) -> OnStateChanged {
        OnStateChanged::new(move |change| {
            let expected = expected.clone();
            let old = old.clone();
            async move {
                assert_eq!(change.new_value, expected);
                assert_eq!(change.old_value, old);
                assert_eq!(
                    change.click.widget_data().get(change.widget_id.as_ref()),
                    Some(&expected)
                );
                assert_eq!(
                    change.click.data["widget_data"][change.widget_id.as_ref()],
                    expected
                );
                ButtonAction::set_dialog_value("changed", change.widget_id.into_string())
            }
        })
    }

    async fn check_change(
        keyboard: &impl Keyboard,
        ctx: &Context,
        id: &str,
        payload: &str,
        expected: &Value,
    ) {
        let action = keyboard
            .handle_callback_for_test(ctx, &callback(ctx, id, payload))
            .await
            .unwrap();
        let ButtonAction::Chain(actions) = action else {
            panic!("state mutation followed by callback action");
        };
        assert!(matches!(
            &actions[0],
            ButtonAction::SetWidgetValue { key, value }
                if key.as_ref() == id && value == expected
        ));
        assert!(matches!(
            &actions[1],
            ButtonAction::SetDialogValue { key, value }
                if key.as_ref() == "changed" && value == id
        ));
        assert!(!ctx.widget_data.contains_key(id));
    }

    #[tokio::test]
    async fn checkbox_callback_observes_new_state_without_mutating_original_context() {
        let expected = json!(true);
        let keyboard = Checkbox::builder("check")
            .checked_text("Yes")
            .unchecked_text("No")
            .on_state_changed(observer(expected.clone(), Some(json!(false))))
            .build();
        check_change(&keyboard, &context(), "check", "true", &expected).await;
    }

    #[tokio::test]
    async fn counter_callback_observes_changed_value_and_default() {
        let expected = json!(3.0);
        let keyboard = Counter::builder("count")
            .default(2.0)
            .on_state_changed(observer(expected.clone(), Some(json!(2.0))))
            .build();
        check_change(&keyboard, &context(), "count", "+", &expected).await;
    }

    #[tokio::test]
    async fn radio_callback_observes_first_selection() {
        let expected = json!("pear");
        let keyboard = Radio::builder("radio")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|item, _data| *item)
            .unchecked_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .on_state_changed(observer(expected.clone(), None))
            .build();
        check_change(&keyboard, &context(), "radio", "pear", &expected).await;
    }

    #[tokio::test]
    async fn multiselect_callback_observes_updated_selection() {
        let expected = json!(["pear"]);
        let keyboard = Multiselect::builder("multi")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|item, _data| *item)
            .unchecked_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .on_state_changed(observer(expected.clone(), Some(json!([]))))
            .build();
        check_change(&keyboard, &context(), "multi", "pear", &expected).await;
    }

    #[tokio::test]
    async fn toggle_callback_observes_effective_default_selection() {
        let expected = json!("pear");
        let keyboard = Toggle::builder("toggle")
            .items_getter(|_data| ["apple", "pear"])
            .item_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .on_state_changed(observer(expected.clone(), Some(json!("apple"))))
            .build();
        check_change(&keyboard, &context(), "toggle", "pear", &expected).await;
    }

    #[tokio::test]
    async fn unchanged_or_constrained_values_do_not_run_state_change_hooks() {
        let calls = Arc::new(AtomicUsize::new(0));
        let count = calls.clone();
        let hook = OnStateChanged::new(move |_change| {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                ButtonAction::noop()
            }
        });
        let mut ctx = context();
        ctx.widget_data.insert("radio".into(), json!("apple"));
        ctx.widget_data.insert("multi".into(), json!(["apple"]));
        let radio = Radio::builder("radio")
            .items_getter(|_data| ["apple"])
            .checked_renderer(|item, _data| *item)
            .unchecked_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .on_state_changed(hook.clone())
            .build();
        let multi = Multiselect::builder("multi")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|item, _data| *item)
            .unchecked_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .max_selected(1)
            .min_selected(1)
            .on_state_changed(hook.clone())
            .build();
        let counter = Counter::builder("count")
            .min(0.0)
            .max(1.0)
            .default(1.0)
            .on_state_changed(hook)
            .build();

        assert!(matches!(
            radio
                .handle_callback_for_test(&ctx, &callback(&ctx, "radio", "apple"))
                .await,
            Some(ButtonAction::SetWidgetValue { .. })
        ));
        assert!(matches!(
            multi
                .handle_callback_for_test(&ctx, &callback(&ctx, "multi", "pear"))
                .await,
            Some(ButtonAction::Noop)
        ));
        assert!(matches!(
            multi
                .handle_callback_for_test(&ctx, &callback(&ctx, "multi", "apple"))
                .await,
            Some(ButtonAction::Noop)
        ));
        assert!(matches!(
            counter
                .handle_callback_for_test(&ctx, &callback(&ctx, "count", "+"))
                .await,
            Some(ButtonAction::SetWidgetValue { .. })
        ));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn invalid_callbacks_and_navigation_buttons_do_not_run_state_change_hooks() {
        let ctx = context();
        let keyboard = Checkbox::builder("check")
            .checked_text("Yes")
            .unchecked_text("No")
            .footer_push(crate::widgets::Button::done("done", "Done"))
            .on_state_changed(OnStateChanged::new(|_change| async {
                panic!("hook must not run")
            }))
            .build();
        assert!(keyboard
            .handle_callback_for_test(&ctx, "td:foreign:check:true")
            .await
            .is_none());
        assert!(keyboard
            .handle_callback_for_test(&ctx, &callback(&ctx, "check", "invalid"))
            .await
            .is_none());
        assert!(matches!(
            keyboard
                .handle_callback_for_test(&ctx, &format!("td:{}:done", ctx.id))
                .await,
            Some(ButtonAction::Done)
        ));
    }

    #[tokio::test]
    async fn checkbox_renders_appearance_for_current_state() {
        let mut ctx = context();
        let keyboard = Checkbox::builder("check")
            .checked_text("Yes")
            .unchecked_text("No")
            .checked_appearance(
                ButtonAppearance::builder()
                    .style_dynamic(|render: crate::entities::RenderContext| async move {
                        render
                            .context
                            .widget_value_as::<bool>("check")
                            .filter(|checked| *checked)
                            .map(|_| ButtonStyle::Success)
                    })
                    .build(),
            )
            .unchecked_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Primary)
                    .build(),
            )
            .build();
        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        assert_eq!(
            markup.inline_keyboard().unwrap()[0][0].style.as_deref(),
            Some("primary")
        );
        ctx.widget_data.insert("check".into(), json!(true));
        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        assert_eq!(
            markup.inline_keyboard().unwrap()[0][0].style.as_deref(),
            Some("success")
        );
    }

    #[tokio::test]
    async fn radio_and_multiselect_render_checked_and_unchecked_appearances() {
        let mut ctx = context();
        ctx.widget_data.insert("radio".into(), json!("pear"));
        ctx.widget_data.insert("multi".into(), json!(["pear"]));
        let radio = Radio::builder("radio")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|item, _data| *item)
            .unchecked_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .checked_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Success)
                    .build(),
            )
            .unchecked_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Primary)
                    .build(),
            )
            .build();
        let multi = Multiselect::builder("multi")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|item, _data| *item)
            .unchecked_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .checked_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Success)
                    .build(),
            )
            .unchecked_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Primary)
                    .build(),
            )
            .build();

        for keyboard in [&radio as &dyn Keyboard, &multi] {
            let markup = keyboard
                .render_keyboard_for_test(&ctx, &DataMap::new())
                .await
                .unwrap();
            let rows = markup.inline_keyboard().unwrap();
            assert_eq!(rows[0][0].style.as_deref(), Some("primary"));
            assert_eq!(rows[1][0].style.as_deref(), Some("success"));
        }
    }

    #[tokio::test]
    async fn counter_and_toggle_render_individual_appearances_and_dynamic_emoji() {
        let ctx = context();
        let counter = Counter::builder("count")
            .minus_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Danger)
                    .build(),
            )
            .value_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Primary)
                    .build(),
            )
            .plus_appearance(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Success)
                    .build(),
            )
            .build();
        let markup = counter
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows[0][0].style.as_deref(), Some("danger"));
        assert_eq!(rows[0][1].style.as_deref(), Some("primary"));
        assert_eq!(rows[0][2].style.as_deref(), Some("success"));

        let data = [("emoji".into(), json!("selected_emoji"))]
            .into_iter()
            .collect();
        let toggle = Toggle::builder("toggle")
            .items_getter(|_data| ["apple", "pear"])
            .item_renderer(|item, _data| *item)
            .id_getter(|item| *item)
            .appearance(
                ButtonAppearance::builder()
                    .icon_custom_emoji_id(FormatText::new("{emoji}"))
                    .build(),
            )
            .build();
        let markup = toggle
            .render_keyboard_for_test(&ctx, &data)
            .await
            .unwrap();
        assert_eq!(
            markup.inline_keyboard().unwrap()[0][0]
                .icon_custom_emoji_id
                .as_deref(),
            Some("selected_emoji")
        );
    }

    #[tokio::test]
    async fn counter_never_persists_nonfinite_values() {
        let ctx = context();
        for counter in [
            Counter::builder("count")
                .default(f64::NAN)
                .increment(f64::NAN)
                .build(),
            Counter::builder("count")
                .default(f64::INFINITY)
                .increment(f64::INFINITY)
                .build(),
            Counter::builder("count")
                .min(f64::INFINITY)
                .max(f64::INFINITY)
                .build(),
            Counter::builder("count")
                .min(f64::NEG_INFINITY)
                .max(f64::INFINITY)
                .default(f64::MAX)
                .increment(f64::MAX)
                .build(),
        ] {
            for direction in ["+", "-"] {
                let action = counter
                    .handle_callback_for_test(&ctx, &callback(&ctx, "count", direction))
                    .await
                    .unwrap();
                let ButtonAction::SetWidgetValue { value, .. } = action else {
                    panic!("numeric mutation");
                };
                assert!(value.as_f64().unwrap().is_finite());
            }
        }
    }

    #[tokio::test]
    async fn counter_respects_both_bounds_with_negative_increment() {
        let ctx = context();
        let counter = Counter::builder("count")
            .min(1.0)
            .max(3.0)
            .default(2.0)
            .increment(-5.0)
            .build();
        for (direction, expected) in [("+", json!(1.0)), ("-", json!(3.0))] {
            assert!(matches!(
                counter
                    .handle_callback_for_test(&ctx, &callback(&ctx, "count", direction))
                    .await,
                Some(ButtonAction::SetWidgetValue { value, .. }) if value == expected
            ));
        }
    }

    #[tokio::test]
    async fn counter_displays_large_integer_values_without_saturating_to_i64() {
        let ctx = context();
        let value = 1e30;
        let counter = Counter::builder("count")
            .max(f64::MAX)
            .default(value)
            .build();
        let markup = counter
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        assert_eq!(
            markup.inline_keyboard().unwrap()[0][1].text.as_ref(),
            value.to_string()
        );
    }
}
