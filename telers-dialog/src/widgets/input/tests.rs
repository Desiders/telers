use super::{
    CombinedInput, Input, InputFilter, InputFilterContext, MessageInput, MessageInputContext, TextInput,
    TextInputContext,
};
use crate::{entities::Context, widgets::ButtonAction};
use telers::{
    serialization::{json, Value},
    types::{ChatPrivate, Message, MessageText, User},
};

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
    let ctx = Context::new("", "state", Value::default());

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
    let ctx = Context::new("", "state", Value::default());

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

    let ctx = Context::new("", "state", Value::default());

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
    let mut ctx = Context::new("", "state", Value::default());
    ctx.widget_data.insert("age_input".into(), json!("42"));

    assert_eq!(input.value(&ctx), Some(42));
}

#[tokio::test]
async fn text_input_can_map_parse_errors_to_action() {
    let input = TextInput::builder("age_input")
        .on_success(store_age)
        .on_error(store_parse_error)
        .build();
    let ctx = Context::new("", "state", Value::default());

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

#[tokio::test]
async fn text_input_success_context_contains_raw_value_and_original_message() {
    let input = TextInput::builder("number")
        .on_success(|ctx: TextInputContext, value: u32| async move {
            assert_eq!(value, 42);
            assert_eq!(
                ctx.context.widget_value_as::<String>("number").as_deref(),
                Some("042")
            );
            assert_eq!(ctx.message.text(), Some("042"));
            ButtonAction::noop()
        })
        .build();
    let ctx = Context::new("", "state", json!(null));

    assert!(input
        .handle_message(&ctx, text_message("042"))
        .await
        .is_some());
    assert!(!ctx.widget_data.contains_key("number"));
}

#[tokio::test]
async fn text_input_error_context_keeps_previous_raw_value() {
    let input = TextInput::builder("number")
        .on_success(|_: TextInputContext, _: u32| async move { ButtonAction::noop() })
        .on_error(|ctx: TextInputContext, _: std::num::ParseIntError| async move {
            assert_eq!(
                ctx.context.widget_value_as::<String>("number").as_deref(),
                Some("12")
            );
            assert_eq!(ctx.message.text(), Some("oops"));
            ButtonAction::noop()
        })
        .build();
    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("number".into(), json!("12"));

    assert!(input
        .handle_message(&ctx, text_message("oops"))
        .await
        .is_some());
    assert_eq!(input.value(&ctx), Some(12));
}

#[tokio::test]
async fn message_input_context_keeps_original_message() {
    let input = MessageInput::new(|ctx: MessageInputContext, message: MessageText| async move {
        assert_eq!(ctx.message.text(), Some(message.text.as_ref()));
        assert_eq!(ctx.context.state.as_str(), "original");
        ButtonAction::noop()
    });
    let ctx = Context::new("", "original", json!(null));

    assert!(input
        .handle_message(&ctx, text_message("hello"))
        .await
        .is_some());
}

#[tokio::test]
async fn input_filter_rejects_before_handler_and_allows_matching_messages() {
    let input = MessageInput::new(|_: MessageInputContext, message: MessageText| async move {
        assert_eq!(message.text.as_ref(), "accepted");
        ButtonAction::noop()
    })
    .filter(InputFilter::new(|ctx: InputFilterContext| async move {
        ctx.message.text() == Some("accepted")
    }));
    let ctx = Context::new("", "state", json!(null));

    assert!(input
        .handle_message(&ctx, text_message("rejected"))
        .await
        .is_none());
    assert!(input
        .handle_message(&ctx, text_message("accepted"))
        .await
        .is_some());
}

#[tokio::test]
async fn input_filter_can_read_persisted_data_before_parsing() {
    let input = TextInput::builder("age")
        .on_success(store_age)
        .parser(|_: &str| -> Result<u8, std::num::ParseIntError> {
            panic!("filtered parser called")
        })
        .build()
        .filter(InputFilter::new(|ctx: InputFilterContext| async move {
            ctx.context
                .dialog_value_as::<bool>("enabled")
                .unwrap_or(false)
        }));
    let ctx = Context::new("", "state", json!(null));

    assert!(input
        .handle_message(&ctx, text_message("18"))
        .await
        .is_none());
}

#[tokio::test]
async fn combined_input_falls_through_filtered_children_and_stops_after_first_match() {
    let input = CombinedInput::builder()
        .input(
            MessageInput::new(store_message_name)
                .filter(InputFilter::new(|_: InputFilterContext| async { false })),
        )
        .input(
            TextInput::builder("age")
                .on_success(store_age)
                .build(),
        )
        .input(MessageInput::new(
            |_: MessageInputContext, _: MessageText| async move { panic!("later handler called") },
        ))
        .build();

    let action = input
        .handle_message(&Context::new("", "state", json!(null)), text_message("42"))
        .await
        .unwrap();
    let ButtonAction::Chain(actions) = action else {
        panic!("expected parsed input")
    };
    assert!(matches!(
        actions[1],
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "age" && value == &json!(42)
    ));
}

#[tokio::test]
async fn combined_input_can_filter_the_entire_group() {
    let input = CombinedInput::builder()
        .input(MessageInput::new(
            |_: MessageInputContext, _: MessageText| async move { panic!("group handler called") },
        ))
        .filter(InputFilter::new(|_: InputFilterContext| async { false }))
        .build();

    assert!(input
        .handle_message(
            &Context::new("", "state", json!(null)),
            text_message("hello"),
        )
        .await
        .is_none());
}

#[tokio::test]
async fn combined_input_accepts_fallback_after_unhandled_parse_error() {
    let input = CombinedInput::builder()
        .input(
            TextInput::builder("age")
                .on_success(store_age)
                .build(),
        )
        .input(MessageInput::new(store_message_name))
        .build();

    let action = input
        .handle_message(
            &Context::new("", "state", json!(null)),
            text_message("alice"),
        )
        .await
        .unwrap();
    assert!(matches!(
        action,
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "name" && value == "alice"
    ));
}

#[tokio::test]
async fn combined_input_empty_group_does_not_claim_messages() {
    assert!(CombinedInput::builder()
        .build()
        .handle_message(
            &Context::new("", "state", json!(null)),
            text_message("hello"),
        )
        .await
        .is_none());
}

#[tokio::test]
async fn text_input_value_returns_none_for_missing_or_invalid_saved_data() {
    let input = TextInput::builder("age")
        .on_success(store_age)
        .build();
    let mut ctx = Context::new("", "state", json!(null));

    assert_eq!(input.value(&ctx), None);
    ctx.widget_data.insert("age".into(), json!("oops"));
    assert_eq!(input.value(&ctx), None);
    ctx.widget_data.insert("age".into(), json!(42));
    assert_eq!(input.value(&ctx), None);
}

#[tokio::test]
async fn custom_parser_accepts_types_without_from_str_and_preserves_raw_input() {
    #[derive(Debug, PartialEq, Eq)]
    struct Name(String);
    let input = TextInput::builder_with_parser("name", |text: &str| {
        let name = text.trim();
        if name.is_empty() {
            Err("empty")
        } else {
            Ok(Name(name.to_owned()))
        }
    })
    .on_success(|ctx: TextInputContext, name: Name| async move {
        assert_eq!(
            ctx.context.widget_value_as::<String>("name").as_deref(),
            Some(" Alice ")
        );
        ButtonAction::set_dialog_value("name", name.0)
    })
    .build();
    let mut ctx = Context::new("", "state", json!(null));

    let action = input
        .handle_message(&ctx, text_message(" Alice "))
        .await
        .unwrap();
    let ButtonAction::Chain(actions) = action else {
        panic!("expected parsed input")
    };
    assert!(matches!(
        actions[0],
        ButtonAction::SetWidgetValue { ref value, .. } if value == " Alice "
    ));
    assert!(matches!(
        actions[1],
        ButtonAction::SetDialogValue { ref value, .. } if value == "Alice"
    ));
    ctx.widget_data.insert("name".into(), json!(" Alice "));
    assert_eq!(input.value(&ctx), Some(Name("Alice".into())));
}

#[tokio::test]
async fn custom_parser_error_handler_receives_original_message() {
    struct Name;
    let input = TextInput::builder_with_parser("name", |_: &str| -> Result<Name, &'static str> {
        Err("invalid")
    })
    .on_success(|_: TextInputContext, _: Name| async { panic!("success called") })
    .on_error(|ctx: TextInputContext, error: &'static str| async move {
        assert_eq!(ctx.message.text(), Some("bad"));
        assert_eq!(error, "invalid");
        ButtonAction::noop()
    })
    .build();

    assert!(input
        .handle_message(&Context::new("", "state", json!(null)), text_message("bad"))
        .await
        .is_some());
}
