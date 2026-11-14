use serde_json::{json, Value};
use telers::{
    client::Reqwest,
    event::{bases::EventReturn, telegram::Handler},
    methods::{AnswerWidgetQuery, SendMessage},
    serialization::{self, Serialize},
    types::{EphemeralMessageParameters, Message, MessageShortcuts},
    Router,
};

fn value(method: &impl Serialize) -> Value {
    serde_json::from_str(&serialization::to_string(method).unwrap()).unwrap()
}

fn message() -> Message {
    serialization::from_str(
        r#"{"message_id":42,"date":1,"chat":{"id":123,"type":"private"},"text":"Hi"}"#,
    )
    .unwrap()
}

#[test]
fn widget_shortcuts_fill_message_address_and_reply_parameters() {
    let message = message();
    assert_eq!(
        value(&message.answer_widget("<p>Hello</p>")),
        json!({"chat_id":123,"html":"<p>Hello</p>"})
    );
    assert_eq!(
        value(&message.reply_shared_widget("widget-id")),
        json!({
            "chat_id":123,"widget_id":"widget-id",
            "reply_parameters":{"message_id":42,"chat_id":123}
        })
    );
    assert_eq!(
        value(&message.edit_widget_data().chat_data("updated")),
        json!({"chat_id":123,"message_id":42,"chat_data":"updated"})
    );
}

#[test]
fn optional_chat_id_supports_ephemeral_callback_replies_and_shortcuts_still_address_chat() {
    let method = SendMessage::new("Only for you").ephemeral_message_parameters(
        EphemeralMessageParameters::new().callback_query_id("callback-id"),
    );
    assert_eq!(
        value(&method),
        json!({
            "text":"Only for you",
            "ephemeral_message_parameters":{"callback_query_id":"callback-id"}
        })
    );
    assert_eq!(
        value(&message().answer("Hello")),
        json!({"chat_id":123,"text":"Hello"})
    );
}

#[test]
fn widget_query_answer_uses_the_api_parameter_name() {
    assert_eq!(
        value(&AnswerWidgetQuery::new("query-id").data("result")),
        json!({"widget_query_id":"query-id","data":"result"})
    );
}

#[test]
fn widget_query_handlers_are_included_in_polling_subscriptions() {
    let router = Router::<Reqwest>::new("widgets")
        .on_widget_query(|observer| {
            observer.register(Handler::new(|| async { EventReturn::Finish }))
        })
        .on_shared_widget_query(|observer| {
            observer.register(Handler::new(|| async { EventReturn::Finish }))
        });
    let updates = router.resolve_used_update_types();
    assert_eq!(updates.len(), 2);
    assert!(updates.contains("widget_query"));
    assert!(updates.contains("shared_widget_query"));
}
