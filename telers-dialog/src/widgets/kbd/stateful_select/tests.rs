use serde_json::{json, Value};

use super::{Multiselect, Radio, Toggle};
use crate::{
    entities::{Context, DataMap},
    widgets::{Button, ButtonAction, Keyboard},
};

#[test]
fn radio_renders_checked_and_unchecked_items() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red", "blue", "green"])
        .checked_renderer(|&item, _data| format!("* {item}"))
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let markup = radio.render_keyboard(&ctx, &DataMap::new()).unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "red");
    assert_eq!(&*rows[1][0].text, "blue");
}

#[test]
fn radio_renders_selected_item_as_checked() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("color".into(), json!("blue"));

    let radio = Radio::builder("color")
        .items_getter(|_data| ["red", "blue", "green"])
        .checked_renderer(|&item, _data| format!("* {item}"))
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let markup = radio.render_keyboard(&ctx, &DataMap::new()).unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "red");
    assert_eq!(&*rows[1][0].text, "* blue");
    assert_eq!(&*rows[2][0].text, "green");
}

#[test]
fn radio_callback_produces_set_widget_value() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red", "blue"])
        .checked_renderer(|&item, _data| format!("* {item}"))
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let action = radio
        .handle_callback(&ctx, &format!("td:{}:color:blue", ctx.id))
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "color" && value == "blue"
    ));
}

#[test]
fn radio_ignores_foreign_intent_callbacks() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red"])
        .checked_renderer(|&item, _data| item.to_owned())
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    assert!(radio.handle_callback(&ctx, "td:other:color:red").is_none());
}

#[test]
fn radio_allows_header_and_footer_buttons() {
    let ctx = Context::new("", "state", Value::Null);
    let radio = Radio::builder("color")
        .items_getter(|_data| ["red"])
        .checked_renderer(|&item, _data| item.to_owned())
        .unchecked_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .footer_push(Button::done("done", "Done"))
        .build();

    let action = radio
        .handle_callback(&ctx, &format!("td:{}:done", ctx.id))
        .unwrap();

    assert!(matches!(action, ButtonAction::Done));
}

#[test]
fn toggle_renders_first_item_when_unset() {
    let ctx = Context::new("", "state", Value::Null);
    let toggle = Toggle::builder("theme")
        .items_getter(|_data| ["light", "dark", "sepia"])
        .item_renderer(|&item, _data| format!("Theme: {item}"))
        .id_getter(|&item| item)
        .build();

    let markup = toggle.render_keyboard(&ctx, &DataMap::new()).unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let expected = format!("td:{}:theme:dark", ctx.id);

    assert_eq!(&*rows[0][0].text, "Theme: light");
    assert_eq!(rows[0][0].callback_data.as_deref(), Some(expected.as_str()));
}

#[test]
fn toggle_renders_selected_item_and_cycles_to_next() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("theme".into(), json!("dark"));

    let toggle = Toggle::builder("theme")
        .items_getter(|_data| ["light", "dark", "sepia"])
        .item_renderer(|&item, _data| format!("Theme: {item}"))
        .id_getter(|&item| item)
        .build();

    let markup = toggle.render_keyboard(&ctx, &DataMap::new()).unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let expected = format!("td:{}:theme:sepia", ctx.id);

    assert_eq!(&*rows[0][0].text, "Theme: dark");
    assert_eq!(rows[0][0].callback_data.as_deref(), Some(expected.as_str()));
}

#[test]
fn toggle_callback_updates_widget_value() {
    let ctx = Context::new("", "state", Value::Null);
    let toggle = Toggle::builder("theme")
        .items_getter(|_data| ["light", "dark"])
        .item_renderer(|&item, _data| item.to_owned())
        .id_getter(|&item| item)
        .build();

    let action = toggle
        .handle_callback(&ctx, &format!("td:{}:theme:dark", ctx.id))
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "theme" && value == "dark"
    ));
}

#[test]
fn multiselect_renders_checked_and_unchecked_items() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("fruits".into(), json!(["apple"]));

    let ms = Multiselect::builder("fruits")
        .items_getter(|_data| ["apple", "pear", "grape"])
        .checked_renderer(|&item, _data| format!("[x] {item}"))
        .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
        .id_getter(|&item| item)
        .build();

    let markup = ms.render_keyboard(&ctx, &DataMap::new()).unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "[x] apple");
    assert_eq!(&*rows[1][0].text, "[ ] pear");
    assert_eq!(&*rows[2][0].text, "[ ] grape");
}

#[test]
fn multiselect_toggle_checks_unchecked_item() {
    let ctx = Context::new("", "state", Value::Null);
    let ms = Multiselect::builder("fruits")
        .items_getter(|_data| ["apple", "pear"])
        .checked_renderer(|&item, _data| format!("[x] {item}"))
        .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
        .id_getter(|&item| item)
        .build();

    let action = ms
        .handle_callback(&ctx, &format!("td:{}:fruits:apple", ctx.id))
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "fruits" && value == &json!(["apple"])
    ));
}

#[test]
fn multiselect_toggle_unchecks_checked_item() {
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
        .handle_callback(&ctx, &format!("td:{}:fruits:apple", ctx.id))
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "fruits" && value == &json!(["pear"])
    ));
}

#[test]
fn multiselect_respects_max_selected() {
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
        .handle_callback(&ctx, &format!("td:{}:fruits:grape", ctx.id))
        .unwrap();

    assert!(matches!(action, ButtonAction::Noop));
}

#[test]
fn multiselect_respects_min_selected() {
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
        .handle_callback(&ctx, &format!("td:{}:fruits:apple", ctx.id))
        .unwrap();

    assert!(matches!(action, ButtonAction::Noop));
}
