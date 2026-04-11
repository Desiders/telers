//! `MessageInput` widget examples for `telers-dialog`.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_message_input
//! ```

use serde_json::Value;
use telers::{
    enums::UpdateType,
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    fsm::{MemoryStorage, Strategy::UserInChat},
    middlewares::outer::FSMContext as FSMContextMiddleware,
    types::MessageText,
    Bot, Dispatcher, Router,
};
use telers_dialog::{
    dialog,
    widgets::{
        format_text, input, keyboard, text, Button, ButtonAction, InlineKeyboard, MessageInput,
        TextInput,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "name";

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
            "name",
            [
                text("Send your name. This window uses `TextInput` and continues on success."),
                format_text("Stored name: {name}"),
                input(
                    TextInput::builder("name_input")
                        .on_success(|_ctx, name: String| {
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("name", name),
                                ButtonAction::next(),
                            ])
                        })
                        .build(),
                ),
            ],
        ),
        window(
            "city",
            [
                text("Send your city. This window uses `MessageInput::text`."),
                input(MessageInput::new(|_ctx, message: MessageText| {
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value("city", message.text.to_string()),
                        ButtonAction::next(),
                    ])
                })),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
                        .build(),
                ),
            ],
        ),
        window(
            "note",
            [
                text("Send any final note. This window uses `MessageInput::new`."),
                input(MessageInput::new(|_ctx, message: MessageText| {
                    let text = message.text.to_string();
                    let text_len = text.len() as u64;
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value("note", text),
                        ButtonAction::set_dialog_value("note_len", text_len),
                        ButtonAction::next(),
                    ])
                })),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back", "Back"))
                        .build(),
                ),
            ],
        ),
        window(
            "summary",
            [
                format_text("name = {name}\ncity = {city}\nnote = {note}\nnote_len = {note_len}"),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back", "Back"))
                        .push(Button::done("done", "Close"))
                        .build(),
                ),
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

    let router = Router::new("dialogs_message_input")
        .on_update(|observer| {
            observer
                .register_outer_middleware(FSMContextMiddleware::new(storage).strategy(UserInChat))
        })
        .on_message(|observer| {
            observer
                .register(Handler::new(handle_start).filter(Command::one("start")))
                .setup_dialogs::<MemoryStorage>()
        })
        .on_callback_query(DialogObserverExt::setup_dialogs::<MemoryStorage>);

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
