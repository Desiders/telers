use std::sync::Arc;

use telers::{
    types::{
        CallbackQuery, CallbackQueryShortcuts as _, Message, MessageRenderers as _,
        MessageShortcuts as _, MessageText, Update, UpdateMessage,
    },
    Bot, Extractor as _, Request,
};
use telers_types::types::{ChatIdKind, ChatPrivate, User};

fn text_request() -> Request<()> {
    let message: telers_types::types::Message =
        MessageText::new(11, 22, ChatPrivate::new(33), "<greeting>").into();
    let update: Update = telers_types::types::UpdateMessage::new(44, message).into();

    Request {
        bot: Bot::with_client("123:token", ()),
        update: Arc::new(update),
        context: Default::default(),
        extensions: Default::default(),
    }
}

#[tokio::test]
async fn standalone_types_keep_framework_extractors() {
    let request = text_request();

    let owned: telers_types::types::Update = Update::extract(&request).await.unwrap();
    assert_eq!(owned.update_id(), 44);

    let shared: Arc<telers_types::types::Update> = Arc::<Update>::extract(&request).await.unwrap();
    assert!(Arc::ptr_eq(&shared, &request.update));

    let envelope: telers_types::types::UpdateMessage =
        UpdateMessage::extract(&request).await.unwrap();
    assert_eq!(envelope.update_id, 44);

    let message: telers_types::types::Message = Message::extract(&request).await.unwrap();
    assert_eq!(message.message_id(), 11);

    let text: telers_types::types::MessageText = MessageText::extract(&request).await.unwrap();
    assert_eq!(&*text.text, "<greeting>");
}

#[tokio::test]
async fn extraction_failures_use_the_shared_conversion_error() {
    let request = text_request();

    let error: telers_types::errors::ConvertToTypeError =
        CallbackQuery::extract(&request).await.unwrap_err();
    assert_eq!(
        error.to_string(),
        "Can't convert from `Update` to `CallbackQuery`"
    );

    let error: telers::errors::ExtractionError = error.into();
    assert_eq!(
        error.to_string(),
        "Extraction error: Can't convert from `Update` to `CallbackQuery`"
    );
}

#[test]
fn framework_shortcuts_accept_standalone_types() {
    let message = telers_types::types::MessageText::new(11, 22, ChatPrivate::new(33), "<greeting>");
    let parse_mode: telers_types::enums::ParseMode = telers::enums::ParseMode::HTML;

    let reply = message.reply("answer").parse_mode(parse_mode);
    assert_eq!(reply.chat_id, ChatIdKind::Id(33));
    assert_eq!(reply.reply_parameters.unwrap().message_id, Some(11));
    assert_eq!(reply.parse_mode.as_deref(), Some("HTML"));
    assert_eq!(message.html_text().as_deref(), Some("&lt;greeting&gt;"));

    let query = telers_types::types::CallbackQuery::new(
        "query-id",
        User::new(1, false, "Alice"),
        "chat-instance",
    );
    assert_eq!(&*query.answer().callback_query_id, "query-id");
}
