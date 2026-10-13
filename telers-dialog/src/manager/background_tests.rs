use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use telers::{
    serialization::{json, Value},
    types::{CallbackQuery, ChatPrivate, Message, MessageText},
};

use super::{
    DialogControl,
    tests::{
        manager_for_event, message_event, prime_last_message, registry_with, test_bot, test_fsm,
        test_user,
    },
};
use crate::{
    dialog,
    entities::{
        ChatEvent, Context, DataGetter, DataMap, LaunchMode, RenderContext, ShowMode, StartMode,
        DEFAULT_STACK_ID,
    },
    message_manager::tests::{mock_bot, requests},
    widgets::{
        fn_text, keyboard, media, text, Button, ButtonAction, InlineKeyboard, MediaContentType, MediaId,
        MediaIdStorage, StaticMedia,
    },
    window, DialogError,
};

const TRUE_RESPONSE: &str = r#"{"ok":true,"result":true}"#;
const TEXT_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":42,"date":1,"chat":{"id":10,"type":"private"},"text":"Refreshed"}}"#;

#[tokio::test]
async fn named_background_updates_preserve_default_stack_and_selector() {
    let bot = test_bot();
    let registry = registry_with([dialog([
        window("default", [text("Default")]),
        window("worker", [text("Worker")]),
    ])]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let original = manager
        .start(&bot, "default", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    manager
        .set_dialog_value("location", json!("default"))
        .await
        .unwrap();
    let mut background = manager.background_for_stack(&bot, "worker-stack");
    let worker = background
        .start("worker", Value::default(), StartMode::Normal)
        .await
        .unwrap();

    background
        .update(
            DataMap::from([("progress".into(), json!(75))]),
            Some(ShowMode::NoUpdate),
        )
        .await
        .unwrap();

    assert_eq!(worker.stack_id, "worker-stack");
    assert_eq!(
        background
            .current_context()
            .await
            .unwrap()
            .dialog_data
            .get("progress"),
        Some(&json!(75)),
    );
    let default = manager.current_context().await.unwrap();
    assert_eq!(default.id, original.id);
    assert_eq!(default.dialog_data.get("location"), Some(&json!("default")));
    assert!(!default.dialog_data.contains_key("progress"));
    assert_eq!(
        manager.load_storage().await.unwrap().current_stack,
        DEFAULT_STACK_ID,
    );
}

#[tokio::test]
async fn pinned_background_rejects_covered_intent_and_resumes_when_uncovered() {
    let bot = test_bot();
    let registry = registry_with([dialog([
        window("parent", [text("Parent")]),
        window("child", [text("Child")]),
    ])]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let original = manager
        .start(&bot, "parent", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    let background = manager.background(&bot).await.unwrap();
    manager
        .start(&bot, "child", Value::default(), StartMode::Normal)
        .await
        .unwrap();

    assert!(matches!(
        background
            .update(DataMap::from([("late".into(), json!(true))]), None)
            .await,
        Err(DialogError::NoContext),
    ));
    assert!(manager
        .current_context()
        .await
        .unwrap()
        .dialog_data
        .is_empty());
    manager.done(&bot, None).await.unwrap();
    background
        .update(
            DataMap::from([("resumed".into(), json!(true))]),
            Some(ShowMode::NoUpdate),
        )
        .await
        .unwrap();
    assert_eq!(background.current_context().await.unwrap().id, original.id);
    assert_eq!(
        manager.dialog_data().await.unwrap().get("resumed"),
        Some(&json!(true)),
    );
}

#[tokio::test]
async fn pinned_background_cannot_mutate_or_restart_a_closed_intent() {
    let bot = test_bot();
    let registry = registry_with([dialog([window("state", [text("State")])])]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    manager
        .start(&bot, "state", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    let mut background = manager.background(&bot).await.unwrap();
    manager.done(&bot, None).await.unwrap();

    assert!(matches!(
        background.current_context().await,
        Err(DialogError::NoContext),
    ));
    assert!(matches!(
        background.update(DataMap::new(), None).await,
        Err(DialogError::NoContext),
    ));
    assert!(matches!(
        background
            .start("state", Value::default(), StartMode::Normal)
            .await,
        Err(DialogError::NoContext),
    ));
    assert!(!manager.has_context().await.unwrap());
}

#[tokio::test]
async fn single_top_replacement_invalidates_pinned_background_without_reusing_intent() {
    let bot = test_bot();
    let registry = registry_with([dialog([
        window("first", [text("First")]),
        window("second", [text("Second")]),
    ])
    .with_launch_mode(LaunchMode::SingleTop)]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let first = manager
        .start(&bot, "first", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    let background = manager.background(&bot).await.unwrap();
    let second = manager
        .start(&bot, "second", Value::default(), StartMode::Normal)
        .await
        .unwrap();

    assert_ne!(first.id, second.id);
    assert!(!manager
        .load_storage()
        .await
        .unwrap()
        .contexts
        .contains_key(&first.id));
    assert!(matches!(
        background
            .update(DataMap::from([("late".into(), json!(true))]), None)
            .await,
        Err(DialogError::NoContext),
    ));
    assert!(manager.dialog_data().await.unwrap().is_empty());
}

#[tokio::test]
async fn background_new_stack_rebinds_and_pins_subsequent_updates() {
    let bot = test_bot();
    let registry = registry_with([dialog([
        window("default", [text("Default")]),
        window("worker", [text("Worker")]),
    ])]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let original = manager
        .start(&bot, "default", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    let mut background = manager.background(&bot).await.unwrap();

    let worker = background
        .start("worker", Value::default(), StartMode::NewStack)
        .await
        .unwrap();
    background
        .update(
            DataMap::from([("progress".into(), json!(100))]),
            Some(ShowMode::NoUpdate),
        )
        .await
        .unwrap();

    assert_ne!(worker.stack_id, original.stack_id);
    assert_eq!(background.current_context().await.unwrap().id, worker.id);
    assert_eq!(
        background
            .current_context()
            .await
            .unwrap()
            .dialog_data
            .get("progress"),
        Some(&json!(100)),
    );
    assert_eq!(manager.current_context().await.unwrap().id, original.id);
    assert!(manager.dialog_data().await.unwrap().is_empty());
}

#[tokio::test]
async fn unbound_new_stack_start_hook_targets_new_context_and_renders_it_once() {
    async fn initialize(
        ctx: Context,
        control: &dyn DialogControl,
    ) -> Result<Option<ButtonAction>, DialogError> {
        assert_eq!(control.current_context().await?.id, ctx.id);
        control
            .set_dialog_value("initialized", ctx.start_data)
            .await?;
        Ok(None)
    }
    let bot = mock_bot(&[(200, TEXT_RESPONSE)]);
    let renders = Arc::new(AtomicUsize::new(0));
    let counter = renders.clone();
    let registry = registry_with([
        dialog([window("default", [text("Default")])]),
        dialog([window(
            "worker",
            [fn_text(move |_: &DataMap| {
                counter.fetch_add(1, Ordering::SeqCst);
                "Refreshed"
            })],
        )])
        .on_start(initialize),
    ]);
    let mut manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let original = manager
        .start(&bot, "default", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    manager
        .set_dialog_value("original", json!(true))
        .await
        .unwrap();
    manager.set_show_mode(ShowMode::Edit);

    let worker = manager
        .start(&bot, "worker", json!("job"), StartMode::NewStack)
        .await
        .unwrap();

    assert_ne!(worker.stack_id, DEFAULT_STACK_ID);
    assert_eq!(worker.dialog_data.get("initialized"), Some(&json!("job")));
    assert_eq!(renders.load(Ordering::SeqCst), 1);
    assert_eq!(
        requests(&bot)
            .iter()
            .map(|(method, _)| *method)
            .collect::<Vec<_>>(),
        ["sendMessage"],
    );
    assert_eq!(manager.current_context().await.unwrap().id, original.id);
    assert_eq!(
        manager.dialog_data().await.unwrap(),
        DataMap::from([("original".into(), json!(true))]),
    );
}

fn callback(intent: &str, target: &str, message_id: i64) -> CallbackQuery {
    let message: Message = MessageText::new(message_id, 1, ChatPrivate::new(10), "Older")
        .from(test_user())
        .into();
    CallbackQuery::new("callback", test_user(), "instance")
        .data(format!("td:{intent}:{target}"))
        .message(message)
}

#[tokio::test]
async fn callbacks_route_to_retained_stack_and_leave_default_message_target_unchanged() {
    let bot = mock_bot(&[(200, TRUE_RESPONSE)]);
    let registry = registry_with([
        dialog([window("default", [text("Default")])]),
        dialog([window(
            "worker",
            [
                text("Worker"),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "click",
                            "Click",
                            ButtonAction::set_dialog_value("clicked", true),
                        )])
                        .build(),
                ),
            ],
        )]),
    ]);
    let fsm = test_fsm(bot.id);
    let manager = manager_for_event(fsm.clone(), registry.clone(), message_event("/start"));
    let original = manager
        .start(&bot, "default", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    let worker = manager
        .start(&bot, "worker", Value::default(), StartMode::NewStack)
        .await
        .unwrap();
    let query = callback(&worker.id, "click", 99);
    let callback_manager = manager_for_event(fsm, registry, ChatEvent::CallbackQuery(query.clone()));

    assert!(callback_manager
        .handle_callback_query(&bot, query)
        .await
        .unwrap());

    let worker_context = manager
        .clone()
        .for_stack(worker.stack_id)
        .current_context()
        .await
        .unwrap();
    assert_eq!(worker_context.dialog_data.get("clicked"), Some(&json!(true)));
    assert_eq!(manager.current_context().await.unwrap().id, original.id);
    assert!(manager.dialog_data().await.unwrap().is_empty());
    assert_eq!(
        requests(&bot)
            .iter()
            .map(|(method, _)| *method)
            .collect::<Vec<_>>(),
        ["answerCallbackQuery"],
    );
}

#[tokio::test]
async fn noop_callback_refreshes_the_canonical_message_instead_of_clicked_older_message() {
    let bot = mock_bot(&[(200, TRUE_RESPONSE), (200, TEXT_RESPONSE)]);
    let registry = registry_with([dialog([window(
        "state",
        [
            text("Refreshed"),
            keyboard(
                InlineKeyboard::builder()
                    .row([Button::action("refresh", "Refresh", ButtonAction::noop())])
                    .build(),
            ),
        ],
    )])]);
    let fsm = test_fsm(bot.id);
    let manager = manager_for_event(fsm.clone(), registry.clone(), message_event("/start"));
    let original = manager
        .start(&bot, "state", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    prime_last_message(&manager, 42).await;
    let query = callback(&original.id, "refresh", 99);
    let mut callback_manager = manager_for_event(fsm, registry, ChatEvent::CallbackQuery(query.clone()));
    callback_manager.set_show_mode(ShowMode::Edit);

    assert!(callback_manager
        .handle_callback_query(&bot, query)
        .await
        .unwrap());

    let actual = requests(&bot);
    assert_eq!(
        actual
            .iter()
            .map(|(method, _)| *method)
            .collect::<Vec<_>>(),
        ["answerCallbackQuery", "editMessageText"],
    );
    assert_eq!(actual[1].1["message_id"], json!(42));
    assert_eq!(
        callback_manager
            .load_storage()
            .await
            .unwrap()
            .current_stack()
            .unwrap()
            .last_text
            .as_deref(),
        Some("Refreshed"),
    );
}

#[tokio::test]
async fn expired_callback_answer_does_not_discard_the_resolved_dialog_action() {
    let bot = mock_bot(&[(
        400,
        r#"{"ok":false,"error_code":400,"description":"Bad Request: query is too old and response timeout expired or query ID is invalid"}"#,
    )]);
    let registry = registry_with([dialog([window(
        "state",
        [
            text("State"),
            keyboard(
                InlineKeyboard::builder()
                    .row([Button::action(
                        "click",
                        "Click",
                        ButtonAction::set_dialog_value("clicked", true),
                    )])
                    .build(),
            ),
        ],
    )])]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let original = manager
        .start(&bot, "state", Value::default(), StartMode::Normal)
        .await
        .unwrap();

    assert!(manager
        .handle_callback_query(&bot, callback(&original.id, "click", 99))
        .await
        .unwrap());

    assert_eq!(
        manager.dialog_data().await.unwrap().get("clicked"),
        Some(&json!(true)),
    );
    assert_eq!(requests(&bot).len(), 1);
}

#[tokio::test]
async fn unrelated_callback_answer_errors_propagate_before_mutation() {
    let bot = mock_bot(&[(
        500,
        r#"{"ok":false,"error_code":500,"description":"Internal Server Error"}"#,
    )]);
    let registry = registry_with([dialog([window(
        "state",
        [
            text("State"),
            keyboard(
                InlineKeyboard::builder()
                    .row([Button::action(
                        "click",
                        "Click",
                        ButtonAction::set_dialog_value("clicked", true),
                    )])
                    .build(),
            ),
        ],
    )])]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let original = manager
        .start(&bot, "state", Value::default(), StartMode::Normal)
        .await
        .unwrap();

    assert!(manager
        .handle_callback_query(&bot, callback(&original.id, "click", 99))
        .await
        .is_err());

    assert!(manager.dialog_data().await.unwrap().is_empty());
}

#[tokio::test]
async fn mutation_followed_by_start_does_not_render_the_destination_twice() {
    let bot = mock_bot(&[(200, TRUE_RESPONSE)]);
    let renders = Arc::new(AtomicUsize::new(0));
    let counter = renders.clone();
    let registry = registry_with([
        dialog([window(
            "parent",
            [
                text("Parent"),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "start",
                            "Start",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("prepared", true),
                                ButtonAction::start("child", Value::default(), StartMode::Normal),
                            ]),
                        )])
                        .build(),
                ),
            ],
        )]),
        dialog([window(
            "child",
            [fn_text(move |_: &DataMap| {
                counter.fetch_add(1, Ordering::SeqCst);
                "Child"
            })],
        )
        .show_mode(ShowMode::NoUpdate)]),
    ]);
    let mut manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let parent = manager
        .start(&bot, "parent", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    manager.set_show_mode(ShowMode::Auto);

    assert!(manager
        .handle_callback_query(&bot, callback(&parent.id, "start", 99))
        .await
        .unwrap());

    assert_eq!(manager.current_context().await.unwrap().state, "child");
    assert_eq!(renders.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn stale_physical_callbacks_are_rejected_without_mutation_or_acknowledgement() {
    let bot = mock_bot(&[]);
    let registry = registry_with([dialog([window(
        "state",
        [
            text("State"),
            keyboard(
                InlineKeyboard::builder()
                    .row([Button::action(
                        "click",
                        "Click",
                        ButtonAction::set_dialog_value("clicked", true),
                    )])
                    .build(),
            ),
        ],
    )])]);
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    let original = manager
        .start(&bot, "state", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    let mut storage = manager.load_storage().await.unwrap();
    let stack = storage.current_stack_mut();
    stack.last_message_id = Some(42);
    stack.last_message_chat = Some(ChatPrivate::new(10).into());
    manager.save_storage(storage).await.unwrap();
    let wrong_chat: Message = MessageText::new(42, 1, ChatPrivate::new(20), "Moved")
        .from(test_user())
        .into();
    let wrong_connection: Message = MessageText::new(42, 1, ChatPrivate::new(10), "Moved")
        .from(test_user())
        .business_connection_id("other")
        .into();
    for query in [
        callback(&original.id, "click", 99),
        callback(&original.id, "click", 42).message(wrong_chat),
        callback(&original.id, "click", 42).message(wrong_connection),
    ] {
        assert!(!manager.handle_callback_query(&bot, query).await.unwrap());
    }
    assert!(manager.dialog_data().await.unwrap().is_empty());
    assert!(requests(&bot).is_empty());
}

#[tokio::test]
async fn pinned_background_cannot_move_a_conversation_back_to_its_old_destination() {
    for (chat_id, thread_id, business) in [
        (20, None, None),
        (10, Some(7), None),
        (10, None, Some("connection")),
    ] {
        let bot = mock_bot(&[]);
        let registry = registry_with([dialog([window("state", [text("State")])])]);
        let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
        manager
            .start(&bot, "state", Value::default(), StartMode::Normal)
            .await
            .unwrap();
        let background = manager.background(&bot).await.unwrap();
        let mut storage = manager.load_storage().await.unwrap();
        let stack = storage.current_stack_mut();
        stack.last_message_id = Some(42);
        stack.last_message_chat = Some(ChatPrivate::new(chat_id).into());
        stack.last_message_thread_id = thread_id;
        stack.last_business_connection_id = business.map(Into::into);
        manager.save_storage(storage).await.unwrap();

        assert!(matches!(
            background
                .update(
                    DataMap::from([("late".into(), json!(true))]),
                    Some(ShowMode::Auto),
                )
                .await,
            Err(DialogError::NoContext),
        ));

        assert!(manager.dialog_data().await.unwrap().is_empty());
        assert!(requests(&bot).is_empty());
    }
}

#[derive(Clone, Default)]
struct CountingMediaStorage {
    reads: Arc<AtomicUsize>,
    writes: Arc<AtomicUsize>,
}

impl MediaIdStorage for CountingMediaStorage {
    fn get_media_id(
        &self,
        _path: Option<&str>,
        _url: Option<&str>,
        _content_type: MediaContentType,
    ) -> Option<MediaId> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        None
    }

    fn save_media_id(
        &self,
        _path: Option<&str>,
        _url: Option<&str>,
        _content_type: MediaContentType,
        _media_id: MediaId,
    ) {
        self.writes.fetch_add(1, Ordering::SeqCst);
    }
}

#[tokio::test]
async fn manager_no_update_overrides_window_send_and_skips_getters_and_media_cache() {
    let bot = mock_bot(&[]);
    let getter_calls = Arc::new(AtomicUsize::new(0));
    let calls = getter_calls.clone();
    let cache = CountingMediaStorage::default();
    let registry = registry_with([dialog([window(
        "state",
        [
            text("Caption"),
            media(
                StaticMedia::builder(MediaContentType::Photo)
                    .url("https://example.com/photo.jpg")
                    .build(),
            ),
        ],
    )
    .show_mode(ShowMode::Send)
    .getter(DataGetter::new(move |_render: RenderContext| {
        let calls = calls.clone();
        async move {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(DataMap::new())
        }
    }))])])
    .with_media_id_storage(cache.clone());
    let manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));

    manager
        .start(&bot, "state", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    manager.show(&bot, None).await.unwrap();

    assert_eq!(getter_calls.load(Ordering::SeqCst), 0);
    assert_eq!(cache.reads.load(Ordering::SeqCst), 0);
    assert_eq!(cache.writes.load(Ordering::SeqCst), 0);
    assert!(requests(&bot).is_empty());
}

#[tokio::test]
async fn window_no_update_can_render_getters_but_cannot_touch_media_cache() {
    let bot = mock_bot(&[]);
    let getter_calls = Arc::new(AtomicUsize::new(0));
    let calls = getter_calls.clone();
    let cache = CountingMediaStorage::default();
    let registry = registry_with([dialog([window(
        "state",
        [
            text("Caption"),
            media(
                StaticMedia::builder(MediaContentType::Photo)
                    .url("https://example.com/photo.jpg")
                    .build(),
            ),
        ],
    )
    .show_mode(ShowMode::NoUpdate)
    .getter(DataGetter::new(move |_render: RenderContext| {
        let calls = calls.clone();
        async move {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(DataMap::new())
        }
    }))])])
    .with_media_id_storage(cache.clone());
    let mut manager = manager_for_event(test_fsm(bot.id), registry, message_event("/start"));
    manager
        .start(&bot, "state", Value::default(), StartMode::Normal)
        .await
        .unwrap();
    manager.set_show_mode(ShowMode::Auto);

    manager.show(&bot, None).await.unwrap();

    assert_eq!(getter_calls.load(Ordering::SeqCst), 1);
    assert_eq!(cache.reads.load(Ordering::SeqCst), 0);
    assert_eq!(cache.writes.load(Ordering::SeqCst), 0);
    assert!(requests(&bot).is_empty());
}
