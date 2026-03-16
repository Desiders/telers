//! This example shows the minimal `telers-dialog` integration path with `telers`.
//!
//! It registers dialogs once in dispatcher extensions, wires dialog middlewares on
//! message and callback query observers, and extracts `DialogManager<MemoryStorage>`
//! directly in handlers.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs
//! ```

use telers::{
    enums::UpdateType,
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    fsm::{MemoryStorage, Strategy::UserInChat},
    middlewares::outer::FSMContext as FSMContextMiddleware,
    types::CallbackQuery,
    Bot, Dispatcher, Router,
};
use telers_dialog::{
    widgets::WidgetKind, Button, ButtonAction, DialogImpl, DialogManager, DialogObserverExt,
    Dialogs, InlineKeyboard, StartMode, WindowImpl,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

type Manager = DialogManager<MemoryStorage>;

fn dialogs() -> Dialogs {
    let dialog = DialogImpl::new([
        WindowImpl::new(
            "main",
            [
                WidgetKind::text("Welcome to telers-dialog."),
                WidgetKind::keyboard(InlineKeyboard::new([[Button::action(
                    "next",
                    "Open second window",
                    ButtonAction::Next,
                )]])),
            ],
        ),
        WindowImpl::new(
            "second",
            [
                WidgetKind::text("Second window. Use buttons below to navigate."),
                WidgetKind::keyboard(InlineKeyboard::new([[
                    Button::action("back", "Back", ButtonAction::Back),
                    Button::action("done", "Close", ButtonAction::Done),
                ]])),
            ],
        ),
    ]);

    Dialogs::new().register(dialog).unwrap()
}

async fn start_dialog(bot: Bot, manager: Manager) -> HandlerResult<()> {
    let _ = manager
        .start(&bot, "main", serde_json::Value::Null, StartMode::ResetStack)
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

async fn handle_dialog_callback(
    bot: Bot,
    callback_query: CallbackQuery,
    manager: Manager,
) -> HandlerResult<()> {
    let _ = manager
        .handle_callback_query(&bot, &callback_query)
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("info,telers_dialog=trace"))
        .init();

    let bot = Bot::from_env();
    let storage = MemoryStorage::new();

    let router = Router::new("dialogs")
        .on_update(|observer| {
            observer
                .register_outer_middleware(FSMContextMiddleware::new(storage).strategy(UserInChat))
        })
        .on_message(|observer| {
            observer
                .setup_dialogs::<MemoryStorage>()
                .register(Handler::new(start_dialog).filter(Command::one("start")))
        })
        .on_callback_query(|observer| {
            observer
                .setup_dialogs::<MemoryStorage>()
                .register(Handler::new(handle_dialog_callback))
        });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        // Don't forget to register dialogs in extensions
        .extension(dialogs())
        .allowed_updates([UpdateType::Message, UpdateType::CallbackQuery])
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
