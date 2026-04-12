use super::{Input, MessageInput, TextInput};
use crate::{entities::Context, widgets::ButtonAction};
use serde_json::{json, Value};
use telers::types::{ChatPrivate, Message, MessageText, User};

fn text_message(text: &str) -> Message {
    MessageText::new(1, 1, ChatPrivate::new(10), text)
        .from(User::new(10, false, "tester"))
        .into()
}

#[test]
fn message_input_text_handles_text_messages() {
    let input = MessageInput::new(|_ctx, message: MessageText| {
        ButtonAction::set_dialog_value("name", message.text.to_string())
    });
    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("alice"))
        .expect("text action");

    assert!(matches!(
        action,
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "name" && value == "alice"
    ));
}

#[test]
fn message_input_text_can_store_dialog_value() {
    let input = MessageInput::new(|_ctx, message: MessageText| {
        ButtonAction::set_dialog_value("name", message.text.to_string())
    });
    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("bob"))
        .expect("text action");

    assert!(matches!(
        action,
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "name" && value == "bob"
    ));
}

#[test]
fn text_input_stores_raw_text_in_widget_data_and_runs_success_action() {
    let input = TextInput::builder("name_input")
        .on_success(|_ctx, value: String| ButtonAction::set_dialog_value("name", value))
        .build();

    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("alice"))
        .expect("text action");

    let ButtonAction::Chain(actions) = action else {
        panic!("expected chained action");
    };
    assert_eq!(actions.len(), 2);
    assert!(matches!(
        actions[0],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "name_input" && value == "alice"
    ));
    assert!(matches!(
        actions[1],
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "name" && value == "alice"
    ));
}

#[test]
fn text_input_reads_typed_value_from_widget_data() {
    let input = TextInput::builder("age_input")
        .on_success(|_ctx, age: u8| ButtonAction::set_dialog_value("age", age))
        .build();
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("age_input".into(), json!("42"));

    assert_eq!(input.value(&ctx), Some(42));
}

#[test]
fn text_input_can_map_parse_errors_to_action() {
    let input = TextInput::builder("age_input")
        .on_success(|_ctx, age: u8| ButtonAction::set_dialog_value("age", age))
        .on_error(|_ctx, err| ButtonAction::set_dialog_value("error", err.to_string()))
        .build();
    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("oops"))
        .expect("error action");

    assert!(matches!(
        action,
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "error" && value == "invalid digit found in string"
    ));
}
