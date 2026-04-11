//! `sync_scroll` example for `telers-dialog`.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_sync_scroll
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
        format_text, keyboard, sync_scroll, Button, ButtonAction, InlineKeyboard, NumberedPager,
        ScrollingGroup, Select,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "catalog";
const GRID_WIDTH: usize = 2;
const PAGE_HEIGHT: usize = 2;
const PRODUCTS: &[(&str, &str, &str)] = &[
    ("Espresso", "$2.50", "strong shot"),
    ("Americano", "$3.00", "espresso + water"),
    ("Latte", "$4.50", "milk-forward"),
    ("Cappuccino", "$4.00", "airy foam"),
    ("Flat White", "$4.20", "velvety texture"),
    ("Mocha", "$4.80", "chocolate + espresso"),
    ("Raf", "$4.90", "cream and vanilla"),
    ("Tea", "$2.80", "black tea"),
    ("Matcha", "$4.70", "green tea latte"),
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
    let catalog_list = ScrollingGroup::builder("catalog_list")
        .height(PAGE_HEIGHT)
        .width(GRID_WIDTH)
        .hide_pager(true)
        .on_page_changed(sync_scroll("catalog_notes"))
        .kbd(
            Select::builder("catalog_items")
                .items_getter(|_data| PRODUCTS)
                .item_renderer(|item, _data| format!("{}", item.0))
                .id_getter(|item| item.0)
                .action(|value| ButtonAction::set_dialog_value("selected_product", value))
                .build(),
        )
        .build();
    let dialog = dialog([window(
        "catalog",
        [
            format_text(
                "Product Browser\n\nSelected product: {selected_product}\n\nThe top block is the \
                 compact picker. The bottom block shows matching price and note cards for the \
                 same page.\n\n[Helper] `sync_scroll` keeps both blocks on the same page",
            ),
            keyboard(catalog_list.clone()),
            keyboard(
                InlineKeyboard::builder()
                    .row([Button::action(
                        "details_label",
                        "PRODUCT DETAILS",
                        ButtonAction::noop(),
                    )])
                    .build(),
            ),
            keyboard(
                ScrollingGroup::builder("catalog_notes")
                    .height(PAGE_HEIGHT)
                    .width(GRID_WIDTH)
                    .hide_pager(true)
                    .kbd(
                        Select::builder("catalog_note_items")
                            .items_getter(|_data| PRODUCTS)
                            .item_renderer(|item, _data| format!("{} | {}", item.1, item.2))
                            .id_getter(|item| item.0)
                            .action(|_value| ButtonAction::noop())
                            .build(),
                    )
                    .build(),
            ),
            keyboard(
                NumberedPager::builder(catalog_list)
                    .page_renderer(|page, _data| format!("{}", page + 1))
                    .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                    .length(5)
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

    let router = Router::new("dialogs_sync_scroll")
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
