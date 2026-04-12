//! Drink catalog pagination example for `telers-dialog`.
//!
//! Shows a real menu grid where `ScrollingGroup` keeps a long list compact and
//! preserves keyboard width on the last page.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_pager_widgets
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
        format_text, keyboard, Button, ButtonAction, InlineKeyboard, ScrollingGroup, Select,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "menu";
const GRID_WIDTH: usize = 2;
const PAGE_HEIGHT: usize = 2;
const DRINKS: &[(&str, &str)] = &[
    ("Espresso", "$2.50"),
    ("Americano", "$3.00"),
    ("Latte", "$4.50"),
    ("Cappuccino", "$4.00"),
    ("Flat White", "$4.20"),
    ("Mocha", "$4.80"),
    ("Raf", "$4.90"),
    ("Tea", "$2.80"),
    ("Matcha", "$4.70"),
    ("Cocoa", "$4.30"),
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
    let dialog = dialog([window(
        START_STATE,
        [
            format_text(
                "Cafe Menu\n\nBrowse a long drink list without flooding the chat.\nSelected \
                 drink: {selected_drink}\n\nThe catalog keeps pagination inside the drink grid \
                 and keeps the keyboard shape stable on the last page.\n\n[Pager] Built-in \
                 `ScrollingGroup` pager",
            ),
            keyboard(
                ScrollingGroup::builder("drink_catalog")
                    .height(PAGE_HEIGHT)
                    .width(GRID_WIDTH)
                    .kbd(
                        Select::builder("drink_items")
                            .items_getter(|_data| DRINKS)
                            .item_renderer(|item, _data| format!("{} {}", item.0, item.1))
                            .id_getter(|item| item.0)
                            .action(|value| ButtonAction::set_dialog_value("selected_drink", value))
                            .build(),
                    )
                    .build(),
            ),
            keyboard(
                InlineKeyboard::builder()
                    .push(Button::done("close", "Close"))
                    .build(),
            ),
        ],
    )]);

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

    let router = Router::new("dialogs_pager_widgets")
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
