//! Pop-up launch request-widget example for `telers-dialog`.
//!
//! Shows contact, location, and poll request keyboards. `one_time_keyboard`
//! only asks Telegram clients to hide the keyboard.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_request_widgets
//! ```

use serde_json::Value;
use telers::{
    enums::{PollType, UpdateType},
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    fsm::{MemoryStorage, Strategy::UserInChat},
    middlewares::outer::FSMContext as FSMContextMiddleware,
    types::{MessageContact, MessageLocation, MessagePoll},
    Bot, Dispatcher, Router,
};
use telers_dialog::{
    dialog,
    widgets::{
        format_text, input, keyboard, Button, ButtonAction, InlineKeyboard, MessageInput,
        RequestContact, RequestLocation, RequestPoll,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "contact";

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
            "contact",
            [
                format_text(
                    "Pop-up Launch Setup\n\nStep 1 of 3. Ask the event lead to share the public \
                     phone number attendees can use on launch day.\n\n[Request] `RequestContact` \
                     shows a reply-keyboard button; `MessageInput` receives the shared contact \
                     and advances the flow.",
                ),
                keyboard(
                    RequestContact::builder("Share launch contact")
                        .resize_keyboard(true)
                        .one_time_keyboard(true)
                        .input_field_placeholder("Share organizer phone")
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: MessageContact| {
                    let full_name = match message.contact.last_name.as_deref() {
                        Some(last_name) => format!("{} {}", message.contact.first_name, last_name),
                        None => format!("{}", message.contact.first_name),
                    };
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value("lead_name", full_name),
                        ButtonAction::set_dialog_value(
                            "lead_phone",
                            format!("{}", message.contact.phone_number),
                        ),
                        ButtonAction::next(),
                    ])
                })),
            ],
        ),
        window(
            "location",
            [
                format_text(
                    "Pop-up Launch Setup\n\nStep 2 of 3. Drop the exact map pin for the pickup \
                     point so the launch card can show where guests should arrive.\n\n[Request] \
                     `RequestLocation` gives a reply-keyboard location button; `MessageInput` \
                     receives the map pin and advances the flow.",
                ),
                keyboard(
                    RequestLocation::builder("Share pickup pin")
                        .resize_keyboard(true)
                        .one_time_keyboard(true)
                        .input_field_placeholder("Send venue pin")
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: MessageLocation| {
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value(
                            "pickup_pin",
                            format!(
                                "{:.4}, {:.4}",
                                message.location.latitude, message.location.longitude
                            ),
                        ),
                        ButtonAction::next(),
                    ])
                })),
            ],
        ),
        window(
            "poll",
            [
                format_text(
                    "Pop-up Launch Setup\n\nStep 3 of 3. Create the quick guest poll that will \
                     run in the launch chat after the event opens.\n\n[Request] `RequestPoll` \
                     asks Telegram to create a native poll; `MessageInput` receives the poll and \
                     advances the flow.",
                ),
                keyboard(
                    RequestPoll::builder("Create guest poll")
                        .poll_type(PollType::Regular)
                        .resize_keyboard(true)
                        .one_time_keyboard(true)
                        .input_field_placeholder("Create poll")
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: MessagePoll| {
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value(
                            "poll_question",
                            format!("{}", message.poll.question()),
                        ),
                        ButtonAction::set_dialog_value(
                            "poll_options",
                            format!("{}", message.poll.options().len()),
                        ),
                        ButtonAction::next(),
                    ])
                })),
            ],
        ),
        window(
            "done",
            [
                format_text(
                    "Pop-up Launch Setup\n\nLead: {lead_name}\nPhone: {lead_phone}\nPickup pin: \
                     {pickup_pin}\nGuest poll: {poll_question}\nOptions: \
                     {poll_options}\n\n[Request] Reply-keyboard request widgets collect \
                     Telegram-native payloads, then a matching `MessageInput` stores the data and \
                     moves the dialog forward",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::done("close", "Close"))
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

    let router = Router::new("dialogs_request_widgets")
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
