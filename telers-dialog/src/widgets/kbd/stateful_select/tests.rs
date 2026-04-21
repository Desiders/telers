use serde_json::{json, Value};

use super::{Checkbox, Counter, Multiselect, Radio, TimeSelect, Toggle};
use crate::{
    entities::{Context, DataMap},
    widgets::{Button, ButtonAction, ClickContext, Keyboard},
};

async fn store_selected_hour(_click: ClickContext, hour: u8) -> ButtonAction {
    ButtonAction::set_dialog_value("selected_hour", hour)
}

async fn store_selected_minute(_click: ClickContext, minute: u8) -> ButtonAction {
    ButtonAction::set_dialog_value("selected_minute", minute)
}

#[tokio::test]
async fn radio_renders_checked_and_unchecked_items() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red", "blue", "green"])
        .checked_renderer(|&item, _data| format!("* {item}"))
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let markup = radio
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "red");
    assert_eq!(&*rows[1][0].text, "blue");
}

#[tokio::test]
async fn radio_renders_selected_item_as_checked() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("color".into(), json!("blue"));

    let radio = Radio::builder("color")
        .items_getter(|_data| ["red", "blue", "green"])
        .checked_renderer(|&item, _data| format!("* {item}"))
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let markup = radio
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "red");
    assert_eq!(&*rows[1][0].text, "* blue");
    assert_eq!(&*rows[2][0].text, "green");
}

#[tokio::test]
async fn radio_callback_produces_set_widget_value() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red", "blue"])
        .checked_renderer(|&item, _data| format!("* {item}"))
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let action = radio
        .handle_callback_for_test(&ctx, &format!("td:{}:color:blue", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "color" && value == "blue"
    ));
}

#[tokio::test]
async fn radio_ignores_foreign_intent_callbacks() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red"])
        .checked_renderer(|&item, _data| item.to_owned())
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    assert!(radio
        .handle_callback_for_test(&ctx, "td:other:color:red")
        .await
        .is_none());
}

#[tokio::test]
async fn radio_allows_header_and_footer_buttons() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red"])
        .checked_renderer(|&item, _data| item.to_owned())
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .footer_push(Button::done("done", "Done"))
        .build();

    let action = radio
        .handle_callback_for_test(&ctx, &format!("td:{}:done", ctx.id))
        .await
        .unwrap();

    assert!(matches!(action, ButtonAction::Done));
}

#[tokio::test]
async fn checkbox_renders_unchecked_and_toggles_to_true() {
    let ctx = Context::new("", "state", Value::Null);
    let checkbox = Checkbox::builder("notify")
        .checked_text("[x] Notify me")
        .unchecked_text("[ ] Notify me")
        .build();

    let markup = checkbox
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let expected = format!("td:{}:notify:true", ctx.id);

    assert_eq!(&*rows[0][0].text, "[ ] Notify me");
    assert_eq!(rows[0][0].callback_data.as_deref(), Some(expected.as_str()));
}

#[tokio::test]
async fn checkbox_renders_checked_and_toggles_to_false() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("notify".into(), json!(true));
    let checkbox = Checkbox::builder("notify")
        .checked_text("[x] Notify me")
        .unchecked_text("[ ] Notify me")
        .build();

    let markup = checkbox
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let expected = format!("td:{}:notify:false", ctx.id);

    assert_eq!(&*rows[0][0].text, "[x] Notify me");
    assert_eq!(rows[0][0].callback_data.as_deref(), Some(expected.as_str()));
}

#[tokio::test]
async fn checkbox_callback_updates_widget_value() {
    let ctx = Context::new("", "state", Value::Null);
    let checkbox = Checkbox::builder("notify")
        .checked_text("[x] Notify me")
        .unchecked_text("[ ] Notify me")
        .build();

    let action = checkbox
        .handle_callback_for_test(&ctx, &format!("td:{}:notify:true", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "notify" && value == &json!(true)
    ));
}

#[tokio::test]
async fn counter_renders_default_value() {
    let ctx = Context::new("", "state", Value::Null);
    let counter = Counter::builder("qty").default(2.0).build();

    let markup = counter
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "-");
    assert_eq!(&*rows[0][1].text, "2");
    assert_eq!(&*rows[0][2].text, "+");
}

#[tokio::test]
async fn counter_can_hide_plus_and_minus_buttons() {
    let ctx = Context::new("", "state", Value::Null);
    let counter = Counter::builder("qty")
        .default(2.0)
        .minus_hidden(true)
        .plus_hidden(true)
        .build();

    let markup = counter
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows[0].len(), 1);
    assert_eq!(&*rows[0][0].text, "2");
}

#[tokio::test]
async fn counter_plus_callback_increments_value() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("qty".into(), json!(2.0));
    let counter = Counter::builder("qty").increment(0.5).build();

    let action = counter
        .handle_callback_for_test(&ctx, &format!("td:{}:qty:+", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "qty" && value == &json!(2.5)
    ));
}

#[tokio::test]
async fn counter_minus_callback_respects_minimum() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("qty".into(), json!(1.0));
    let counter = Counter::builder("qty").min(1.0).default(1.0).build();

    let action = counter
        .handle_callback_for_test(&ctx, &format!("td:{}:qty:-", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "qty" && value == &json!(1.0)
    ));
}

#[tokio::test]
async fn counter_cycles_when_enabled() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("qty".into(), json!(3.0));
    let counter = Counter::builder("qty")
        .min(1.0)
        .max(3.0)
        .cycle(true)
        .build();

    let action = counter
        .handle_callback_for_test(&ctx, &format!("td:{}:qty:+", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "qty" && value == &json!(1.0)
    ));
}

#[tokio::test]
async fn time_select_renders_headers_and_selected_values() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data
        .insert("pickup_time".into(), json!([13, 30]));
    let picker = TimeSelect::builder("pickup_time")
        .minute_precision(15)
        .build();

    let markup = picker
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "Hour");
    assert!(rows.iter().flatten().any(|button| &*button.text == "[13]"));
    assert_eq!(&*rows[5][0].text, "Minute");
    assert!(rows.iter().flatten().any(|button| &*button.text == "[30]"));
}

#[tokio::test]
async fn time_select_zero_pads_default_labels() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pickup_time".into(), json!([0, 5]));
    let picker = TimeSelect::builder("pickup_time").build();

    let markup = picker
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert!(rows.iter().flatten().any(|button| &*button.text == "[00]"));
    assert!(rows.iter().flatten().any(|button| &*button.text == "[05]"));
}

#[tokio::test]
async fn time_select_allows_custom_value_renderers() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pickup_time".into(), json!([0, 5]));
    let picker = TimeSelect::builder("pickup_time")
        .button_renderer(|value, _data| format!("{value}"))
        .selected_button_renderer(|value, _data| format!("*{value}*"))
        .build();

    let markup = picker
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert!(rows.iter().flatten().any(|button| &*button.text == "*0*"));
    assert!(rows.iter().flatten().any(|button| &*button.text == "*5*"));
}

#[tokio::test]
async fn time_select_hour_callback_updates_partial_value() {
    let ctx = Context::new("", "state", Value::Null);
    let picker = TimeSelect::builder("pickup_time").build();

    let action = picker
        .handle_callback_for_test(&ctx, &format!("td:{}:pickup_time:h13", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pickup_time" && value == &json!([13, null])
    ));
}

#[tokio::test]
async fn time_select_hour_callback_runs_click_handler() {
    let ctx = Context::new("", "state", Value::Null);
    let picker = TimeSelect::builder("pickup_time")
        .on_hour_click(store_selected_hour)
        .build();

    let action = picker
        .handle_callback_for_test(&ctx, &format!("td:{}:pickup_time:h8", ctx.id))
        .await
        .unwrap();

    let ButtonAction::Chain(actions) = action else {
        panic!("expected chain action");
    };
    assert!(matches!(
        actions[0],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pickup_time" && value == &json!([8, null])
    ));
    assert!(matches!(
        actions[1],
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "selected_hour" && value == &json!(8)
    ));
}

#[tokio::test]
async fn time_select_minute_callback_preserves_selected_hour() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data
        .insert("pickup_time".into(), json!([13, null]));
    let picker = TimeSelect::builder("pickup_time")
        .minute_precision(15)
        .build();

    let action = picker
        .handle_callback_for_test(&ctx, &format!("td:{}:pickup_time:m30", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pickup_time" && value == &json!([13, 30])
    ));
}

#[tokio::test]
async fn time_select_minute_callback_runs_click_handler() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data
        .insert("pickup_time".into(), json!([13, null]));
    let picker = TimeSelect::builder("pickup_time")
        .on_minute_click(store_selected_minute)
        .minute_precision(15)
        .build();

    let action = picker
        .handle_callback_for_test(&ctx, &format!("td:{}:pickup_time:m45", ctx.id))
        .await
        .unwrap();

    let ButtonAction::Chain(actions) = action else {
        panic!("expected chain action");
    };
    assert!(matches!(
        actions[0],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pickup_time" && value == &json!([13, 45])
    ));
    assert!(matches!(
        actions[1],
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "selected_minute" && value == &json!(45)
    ));
}

#[tokio::test]
async fn toggle_renders_first_item_when_unset() {
    let ctx = Context::new("", "state", Value::Null);
    let toggle = Toggle::builder("theme")
        .items_getter(|_data| ["light", "dark", "sepia"])
        .item_renderer(|&item, _data| format!("Theme: {item}"))
        .id_getter(|&item| item)
        .build();

    let markup = toggle
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let expected = format!("td:{}:theme:dark", ctx.id);

    assert_eq!(&*rows[0][0].text, "Theme: light");
    assert_eq!(rows[0][0].callback_data.as_deref(), Some(expected.as_str()));
}

#[tokio::test]
async fn toggle_renders_selected_item_and_cycles_to_next() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("theme".into(), json!("dark"));

    let toggle = Toggle::builder("theme")
        .items_getter(|_data| ["light", "dark", "sepia"])
        .item_renderer(|&item, _data| format!("Theme: {item}"))
        .id_getter(|&item| item)
        .build();

    let markup = toggle
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let expected = format!("td:{}:theme:sepia", ctx.id);

    assert_eq!(&*rows[0][0].text, "Theme: dark");
    assert_eq!(rows[0][0].callback_data.as_deref(), Some(expected.as_str()));
}

#[tokio::test]
async fn toggle_callback_updates_widget_value() {
    let ctx = Context::new("", "state", Value::Null);
    let toggle = Toggle::builder("theme")
        .items_getter(|_data| ["light", "dark"])
        .item_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let action = toggle
        .handle_callback_for_test(&ctx, &format!("td:{}:theme:dark", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "theme" && value == "dark"
    ));
}

#[tokio::test]
async fn multiselect_renders_checked_and_unchecked_items() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("fruits".into(), json!(["apple"]));

    let ms = Multiselect::builder("fruits")
        .items_getter(|_data| ["apple", "pear", "grape"])
        .checked_renderer(|&item, _data| format!("[x] {item}"))
        .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
        .id_getter(|&item| item)
        .build();

    let markup = ms
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "[x] apple");
    assert_eq!(&*rows[1][0].text, "[ ] pear");
    assert_eq!(&*rows[2][0].text, "[ ] grape");
}

#[tokio::test]
async fn multiselect_toggle_checks_unchecked_item() {
    let ctx = Context::new("", "state", Value::Null);
    let ms = Multiselect::builder("fruits")
        .items_getter(|_data| ["apple", "pear"])
        .checked_renderer(|&item, _data| format!("[x] {item}"))
        .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
        .id_getter(|&item| item)
        .build();

    let action = ms
        .handle_callback_for_test(&ctx, &format!("td:{}:fruits:apple", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "fruits" && value == &json!(["apple"])
    ));
}

#[tokio::test]
async fn multiselect_toggle_unchecks_checked_item() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data
        .insert("fruits".into(), json!(["apple", "pear"]));

    let ms = Multiselect::builder("fruits")
        .items_getter(|_data| ["apple", "pear"])
        .checked_renderer(|&item, _data| format!("[x] {item}"))
        .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
        .id_getter(|&item| item)
        .build();

    let action = ms
        .handle_callback_for_test(&ctx, &format!("td:{}:fruits:apple", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "fruits" && value == &json!(["pear"])
    ));
}

#[tokio::test]
async fn multiselect_respects_max_selected() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data
        .insert("fruits".into(), json!(["apple", "pear"]));

    let ms = Multiselect::builder("fruits")
        .items_getter(|_data| ["apple", "pear", "grape"])
        .checked_renderer(|&item, _data| format!("[x] {item}"))
        .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
        .id_getter(|&item| item)
        .max_selected(2)
        .build();

    let action = ms
        .handle_callback_for_test(&ctx, &format!("td:{}:fruits:grape", ctx.id))
        .await
        .unwrap();

    assert!(matches!(action, ButtonAction::Noop));
}

#[tokio::test]
async fn multiselect_respects_min_selected() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("fruits".into(), json!(["apple"]));

    let ms = Multiselect::builder("fruits")
        .items_getter(|_data| ["apple", "pear"])
        .checked_renderer(|&item, _data| format!("[x] {item}"))
        .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
        .id_getter(|&item| item)
        .min_selected(1)
        .build();

    let action = ms
        .handle_callback_for_test(&ctx, &format!("td:{}:fruits:apple", ctx.id))
        .await
        .unwrap();

    assert!(matches!(action, ButtonAction::Noop));
}
