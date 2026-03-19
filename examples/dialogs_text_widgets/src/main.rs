//! Text widget examples for `telers-dialog`.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_text_widgets
//! ```

use serde_json::Value;
use telers::{
    enums::UpdateType,
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    fsm::{MemoryStorage, Strategy::UserInChat},
    middlewares::outer::FSMContext as FSMContextMiddleware,
    Bot, Dispatcher, Router,
};
use telers_dialog::{
    dialog,
    entities::DataMap,
    widgets::{keyboard, text, Button, ButtonAction, FnText, FormatText, InlineKeyboard, ListText},
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "intro";

type Manager = DialogManager<MemoryStorage>;

async fn handle_start(bot: Bot, manager: Manager) -> HandlerResult<()> {
    let _ = manager
        .start(
            &bot,
            START_STATE.to_owned(),
            Value::Null,
            StartMode::ResetStack,
        )
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

fn registry() -> DialogRegistry {
    let dialog = dialog([
        window(
            "intro",
            [
                text("This bot demonstrates the current text widgets."),
                text(
                    "Press the button below. The next window combines several text widgets into \
                     one message.",
                ),
                keyboard(InlineKeyboard::new([[Button::action(
                    "render",
                    "Show text widgets",
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value("title", "telers-dialog"),
                        ButtonAction::next(),
                    ]),
                )]])),
            ],
        ),
        window(
            "rendered",
            [
                text("Static text via `&str`."),
                text(FnText::new(|data: &DataMap| {
                    format!("FnText sees {} dialog-data keys.", data.len())
                })),
                text(FormatText::new("FormatText title: {title}")),
                text(ListText::new(
                    |_data: &DataMap| vec!["one".to_owned(), "two".to_owned(), "three".to_owned()],
                    |item: &String, _data: &DataMap| format!("- {item}"),
                )),
                keyboard(InlineKeyboard::new([[
                    Button::back("back", "Back"),
                    Button::done("done", "Close"),
                ]])),
            ],
        ),
    ]);

    DialogRegistry::new().register(dialog).unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("info,telers_dialog=trace"))
        .init();

    let bot = Bot::from_env();
    let storage = MemoryStorage::new();
    let registry = registry();

    let router = Router::new("dialogs_text_widgets")
        .on_update(|observer| {
            observer
                .register_outer_middleware(FSMContextMiddleware::new(storage).strategy(UserInChat))
        })
        .on_message(|observer| {
            observer
                .register(Handler::new(handle_start).filter(Command::one("start")))
                .setup_dialogs::<MemoryStorage>()
        })
        .on_callback_query(|observer| observer.setup_dialogs::<MemoryStorage>());

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .extension(registry)
        .allowed_updates([UpdateType::Message, UpdateType::CallbackQuery])
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
