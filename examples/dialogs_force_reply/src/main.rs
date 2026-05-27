//! `ForceReply` prompt-flow example for `telers-dialog`.
//!
//! Shows how a `ForceReply` reply-markup widget combines with a `MessageInput`
//! to drive a chat that feels like an inline form: the client auto-opens the
//! reply UI, the message handler captures the text, and the dialog moves on.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_force_reply
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
        format_text, input, keyboard, text, Button, ButtonAction, ForceReply, InlineKeyboard,
        MessageInput,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "ask_name";

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
            START_STATE,
            [
                text(
                    "Reservation Setup\n\nStep 1 of 2. Telegram will auto-open the reply UI with \
                     the placeholder visible. Send the guest's full name.\n\n[Reply] \
                     `ForceReply::builder().input_field_placeholder(..)`",
                ),
                keyboard(
                    ForceReply::builder()
                        .input_field_placeholder("Full name")
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: MessageText| async move {
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value("guest_name", message.text.to_string()),
                        ButtonAction::next(),
                    ])
                })),
            ],
        ),
        window(
            "ask_party_size",
            [
                format_text(
                    "Reservation Setup\n\nStep 2 of 2. Guest: {guest_name}\n\nSend how many \
                     people the table is for. The reply UI is forced again with a different \
                     placeholder.",
                ),
                keyboard(
                    ForceReply::builder()
                        .input_field_placeholder("Number of guests")
                        .selective(true)
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: MessageText| async move {
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value("party_size", message.text.to_string()),
                        ButtonAction::next(),
                    ])
                })),
            ],
        ),
        window(
            "done",
            [
                format_text(
                    "Reservation Recorded\n\nGuest: {guest_name}\nParty size: \
                     {party_size}\n\n[Reply] `ForceReply` only owns the reply markup. The prompt \
                     text and persistence still come from the window's text and the \
                     `MessageInput` that consumes the response.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
                        .row([Button::done("close", "Close")])
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

    let router = Router::new("dialogs_force_reply")
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
