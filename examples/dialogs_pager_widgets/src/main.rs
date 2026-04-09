//! Pager widget examples for `telers-dialog`.
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
        format_text, keyboard, Button, ButtonAction, InlineKeyboard, NumberedPager, ScrollingGroup,
        Select,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "built_in";
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
    let drink_count = |_data: &_| DRINKS.len().div_ceil(GRID_WIDTH * PAGE_HEIGHT);
    let dialog = dialog([
        window(
            "built_in",
            [
                format_text(
                    "Cafe Menu\n\nBrowse a long drink list without flooding the chat.\nSelected \
                     drink: {selected_drink}\n\nThis version keeps pagination inside the catalog \
                     itself and keeps the catalog grid shape stable on the last page.\n\n[Pager] \
                     Built-in `ScrollingGroup` pager",
                ),
                keyboard(
                    ScrollingGroup::builder("built_in_catalog")
                        .height(PAGE_HEIGHT)
                        .width(GRID_WIDTH)
                        .kbd(
                            Select::builder("built_in_items")
                                .items_getter(|_data| DRINKS)
                                .item_renderer(|item, _data| format!("{} {}", item.0, item.1))
                                .id_getter(|item| item.0)
                                .action(|value| {
                                    ButtonAction::set_dialog_value("selected_drink", value)
                                })
                                .build(),
                        )
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::new()
                        .push(Button::switch_to(
                            "custom_layout",
                            "Open custom layout",
                            "standalone",
                        ))
                        .push(Button::done("close", "Close")),
                ),
            ],
        ),
        window(
            "standalone",
            [
                format_text(
                    "Cafe Menu\n\nSelected drink: {selected_drink}\n\nThis version keeps the same \
                     catalog grid, but moves paging into its own row so the bot can place it \
                     exactly where it wants.\n\n[Pager] `hide_pager(true)` + standalone \
                     `NumberedPager`",
                ),
                keyboard(
                    ScrollingGroup::builder("standalone_catalog")
                        .height(PAGE_HEIGHT)
                        .width(GRID_WIDTH)
                        .hide_pager(true)
                        .kbd(
                            Select::builder("standalone_items")
                                .items_getter(|_data| DRINKS)
                                .item_renderer(|item, _data| format!("{} {}", item.0, item.1))
                                .id_getter(|item| item.0)
                                .action(|value| {
                                    ButtonAction::set_dialog_value("selected_drink", value)
                                })
                                .build(),
                        )
                        .build(),
                ),
                keyboard(
                    NumberedPager::builder("standalone_catalog")
                        .page_count_getter(drink_count)
                        .page_renderer(|page, _data| format!("{}", page + 1))
                        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                        .length(5)
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::new()
                        .push(Button::switch_to(
                            "back_to_builtin",
                            "Back to built-in layout",
                            "built_in",
                        ))
                        .push(Button::done("close", "Close")),
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
