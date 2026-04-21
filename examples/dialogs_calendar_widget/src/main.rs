//! Coffee tasting reservation example for `telers-dialog`.
//!
//! Shows how `Calendar` can drive a real date-selection flow: the calendar
//! callback stores the selected `time::Date` string in dialog data, the same
//! window previews it, and the review screen can save the reservation.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_calendar_widget
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
    widgets::{fn_text, keyboard, text, Button, ButtonAction, Calendar, InlineKeyboard},
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

fn selected_date_text(data: &DataMap) -> String {
    data.get("selected_date")
        .and_then(Value::as_str)
        .map_or_else(
            || "Selected date: choose a day in the calendar.".to_owned(),
            |date| format!("Selected date: {date}"),
        )
}

fn registry() -> DialogRegistry {
    let dialog = dialog([
        window(
            START_STATE,
            [
                text(
                    "Coffee Tasting Reservation\n\nUse this flow when an operator needs to pick \
                     an exact calendar date before confirming a reservation.\n\n[Calendar] The \
                     next screen stores the chosen `time::Date` in dialog data.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::next("choose_date", "Choose date"))
                        .push(Button::done("close", "Close"))
                        .build(),
                ),
            ],
        ),
        window(
            "date",
            [
                text("Coffee Tasting Reservation\n\nPick the tasting day.\n"),
                fn_text(selected_date_text),
                keyboard(
                    Calendar::builder("reservation_date_calendar")
                        .on_click(|_click, selected_date| async move {
                            ButtonAction::set_dialog_value(
                                "selected_date",
                                selected_date.to_string(),
                            )
                        })
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back", "Back"))
                        .push(Button::next("review", "Review reservation"))
                        .push(Button::done("close", "Close"))
                        .build(),
                ),
            ],
        ),
        window(
            "review",
            [
                text("Reservation Review\n\n"),
                fn_text(|data: &DataMap| {
                    let selected_date = data
                        .get("selected_date")
                        .and_then(Value::as_str)
                        .unwrap_or("not selected");
                    format!(
                        "Tasting date: {selected_date}\n\n[Calendar] The selected date is now \
                         ordinary dialog data, so a real handler could store it with the rest of \
                         the reservation."
                    )
                }),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back_to_calendar", "Change date"))
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

    let router = Router::new("dialogs_calendar_widget")
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
