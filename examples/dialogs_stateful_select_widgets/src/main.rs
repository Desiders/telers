//! Stateful select widget examples for `telers-dialog`.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_stateful_select_widgets
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
    widgets::{
        format_text, keyboard, Button, Checkbox, Counter, Group, InlineKeyboard, Multiselect,
        Radio, Toggle,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "channel";
const CHANNELS: &[(&str, &str)] = &[
    ("telegram", "Telegram"),
    ("email", "Email"),
    ("sms", "SMS"),
    ("mute", "Pause alerts"),
];
const DIGEST_MODES: &[(&str, &str)] = &[
    ("instant", "Instant alerts"),
    ("morning", "Morning digest"),
    ("evening", "Evening roundup"),
];
const TOPICS: &[(&str, &str)] = &[
    ("beans", "New beans"),
    ("gear", "Gear drops"),
    ("recipes", "Brew guides"),
    ("events", "Cafe events"),
    ("rewards", "Rewards"),
    ("wholesale", "Wholesale news"),
];

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
            "channel",
            [
                format_text(
                    "Roastery Club Alerts\n\nChoose the main channel for new-bean drops and \
                     weekend offers. The active option stays highlighted in the \
                     keyboard.\n\n[Stateful] `Radio` keeps exactly one item selected",
                ),
                keyboard(
                    Group::builder(
                        Radio::builder("alert_channel")
                            .items_getter(|_data| CHANNELS)
                            .checked_renderer(|item, _data| format!("● {}", item.1))
                            .unchecked_renderer(|item, _data| format!("○ {}", item.1))
                            .id_getter(|item| item.0)
                            .build(),
                    )
                    .items_per_row(2)
                    .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::next("open_digest", "Alert pace"))
                        .push(Button::done("close", "Close"))
                        .build(),
                ),
            ],
        ),
        window(
            "digest",
            [
                format_text(
                    "Roastery Club Alerts\n\nChoose how often campaign messages should arrive. \
                     Tap the single button until the pace matches the audience you \
                     want.\n\n[Stateful] `Toggle` stores one value and cycles to the next one on \
                     every tap",
                ),
                keyboard(
                    Toggle::builder("digest_mode")
                        .items_getter(|_data| DIGEST_MODES)
                        .item_renderer(|item, _data| format!("Alert pace: {}", item.1))
                        .id_getter(|item| item.0)
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back", "Back"))
                        .push(Button::next("open_limits", "Limits"))
                        .push(Button::done("close", "Close"))
                        .build(),
                ),
            ],
        ),
        window(
            "limits",
            [
                format_text(
                    "Roastery Club Alerts\n\nTune how aggressive the campaign should feel. Keep \
                     flash-sale pins on for urgent launches, and set how many promo pushes can go \
                     out each week.\n\n[Stateful] `Checkbox` toggles one boolean value and \
                     `Counter` stores a numeric setting in `widget_data`",
                ),
                keyboard(
                    Checkbox::builder("pin_flash_sales")
                        .checked_text("✓ Pin flash-sale alerts")
                        .unchecked_text("□ Pin flash-sale alerts")
                        .build(),
                ),
                keyboard(
                    Counter::builder("weekly_promo_cap")
                        .default(3.0)
                        .min(0.0)
                        .max(7.0)
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back", "Back"))
                        .push(Button::next("open_topics", "Topics"))
                        .push(Button::done("close", "Close"))
                        .build(),
                ),
            ],
        ),
        window(
            "topics",
            [
                format_text(
                    "Roastery Club Alerts\n\nPick up to three topics subscribers want to follow. \
                     The checked choices stay visible directly in the keyboard.\n\n[Stateful] \
                     `Multiselect` enforces `min_selected` and `max_selected`",
                ),
                keyboard(
                    Group::builder(
                        Multiselect::builder("topics")
                            .items_getter(|_data| TOPICS)
                            .checked_renderer(|item, _data| format!("✓ {}", item.1))
                            .unchecked_renderer(|item, _data| format!("□ {}", item.1))
                            .id_getter(|item| item.0)
                            .min_selected(1)
                            .max_selected(3)
                            .build(),
                    )
                    .items_per_row(2)
                    .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back", "Back"))
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

    let router = Router::new("dialogs_stateful_select_widgets")
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
