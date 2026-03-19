//! Button helper examples for `telers-dialog`.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_button_actions
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
    widgets::{keyboard, text, Button, ButtonAction, FormatText, InlineKeyboard},
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "home";

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
    let main_dialog = dialog([
        window(
            "home",
            [
                text("Button helpers: `next`, `switch_to`, and `start`."),
                keyboard(InlineKeyboard::new([
                    [Button::next("next", "Next helper")],
                    [Button::switch_to("details", "SwitchTo details", "details")],
                    [Button::start(
                        "modal",
                        "Start subdialog",
                        "modal",
                        Value::Null,
                        StartMode::Normal,
                    )],
                ])),
            ],
        ),
        window(
            "step",
            [
                text("This state demonstrates `back` and a custom `action` chain."),
                keyboard(InlineKeyboard::new([
                    [Button::back("back", "Back helper")],
                    [Button::action(
                        "remember",
                        "Remember source and open details",
                        ButtonAction::chain([
                            ButtonAction::set_dialog_value("source", "custom action"),
                            ButtonAction::switch_to("details"),
                        ]),
                    )],
                ])),
            ],
        ),
        window(
            "details",
            [
                text(FormatText::new(
                    "Details window.\nsource = {source}\nnote = {note}",
                )),
                keyboard(InlineKeyboard::new([
                    [Button::set_dialog_value(
                        "note",
                        "Set dialog value",
                        "note",
                        "saved from helper",
                    )],
                    [Button::done("done", "Done helper")],
                    [Button::url("Open docs", "https://docs.rs/telers-dialog")],
                ])),
            ],
        ),
    ]);

    let modal_dialog = dialog([window(
        "modal",
        [
            text("This dialog was opened with `Button::start`."),
            keyboard(InlineKeyboard::new([[Button::done(
                "close",
                "Close modal",
            )]])),
        ],
    )]);

    DialogRegistry::new()
        .register(main_dialog)
        .unwrap()
        .register(modal_dialog)
        .unwrap()
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

    let router = Router::new("dialogs_button_actions")
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
