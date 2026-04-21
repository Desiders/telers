use super::{Input, MessageInput, MessageInputContext, TextInput, TextInputContext};
use crate::{entities::Context, widgets::ButtonAction};
use serde_json::{json, Value};
use telers::types::{ChatPrivate, Message, MessageText, User};

fn text_message(text: &str) -> Message {
    MessageText::new(1, 1, ChatPrivate::new(10), text)
        .from(User::new(10, false, "tester"))
        .into()
}

async fn store_message_name(_ctx: MessageInputContext, message: MessageText) -> ButtonAction {
    ButtonAction::set_dialog_value("name", message.text.to_string())
}

async fn store_text_name(_ctx: TextInputContext, value: String) -> ButtonAction {
    ButtonAction::set_dialog_value("name", value)
}

async fn store_age(_ctx: TextInputContext, age: u8) -> ButtonAction {
    ButtonAction::set_dialog_value("age", age)
}

async fn store_parse_error(_ctx: TextInputContext, err: std::num::ParseIntError) -> ButtonAction {
    ButtonAction::set_dialog_value("error", err.to_string())
}

#[tokio::test]
async fn message_input_text_handles_text_messages() {
    let input = MessageInput::new(store_message_name);
    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("alice"))
        .await
        .expect("text action");

    assert!(matches!(
        action,
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "name" && value == "alice"
    ));
}

#[tokio::test]
async fn message_input_text_can_store_dialog_value() {
    let input = MessageInput::new(store_message_name);
    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("bob"))
        .await
        .expect("text action");

    assert!(matches!(
        action,
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "name" && value == "bob"
    ));
}

#[tokio::test]
async fn text_input_stores_raw_text_in_widget_data_and_runs_success_action() {
    let input = TextInput::builder("name_input")
        .on_success(store_text_name)
        .build();

    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("alice"))
        .await
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

#[tokio::test]
async fn text_input_reads_typed_value_from_widget_data() {
    let input = TextInput::builder("age_input")
        .on_success(store_age)
        .build();
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("age_input".into(), json!("42"));

    assert_eq!(input.value(&ctx), Some(42));
}

#[tokio::test]
async fn text_input_can_map_parse_errors_to_action() {
    let input = TextInput::builder("age_input")
        .on_success(store_age)
        .on_error(store_parse_error)
        .build();
    let ctx = Context::new("", "state", Value::Null);

    let action = input
        .handle_message(&ctx, text_message("oops"))
        .await
        .expect("error action");

    assert!(matches!(
        action,
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "error" && value == "invalid digit found in string"
    ));
}
