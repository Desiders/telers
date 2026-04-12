//! Drink catalog pagination examples for `telers-dialog`.
//!
//! Shows a real menu grid where `ScrollingGroup` keeps a long list compact and
//! a receipt preview where `StubScroll` lets standalone pager buttons control
//! custom-rendered text.
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
    entities::{DataMap, RenderContext},
    widgets::{
        format_text, keyboard, text, Button, ButtonAction, InlineKeyboard, NumberedPager,
        ScrollingGroup, Select, StubScroll, Text,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "menu";
const GRID_WIDTH: usize = 2;
const PAGE_HEIGHT: usize = 2;
const RECEIPT_PAGE_SIZE: usize = 4;
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
const RECEIPT_LINES: &[&str] = &[
    "Table 4 ordered two cappuccinos and one cocoa.",
    "Oat milk substitute was requested for the first cappuccino.",
    "Kitchen added one almond croissant to the same check.",
    "Guest asked to split payment after the drinks are ready.",
    "Barista marked one drink for takeaway.",
    "Loyalty discount should be applied before card payment.",
    "Manager comped the croissant because of the wait.",
    "Receipt note: invite the guest to Saturday cupping.",
    "Final pickup name is Dana.",
];

type Manager = DialogManager<MemoryStorage>;

struct ReceiptPreviewText {
    scroll_id: &'static str,
}

impl ReceiptPreviewText {
    const fn new(scroll_id: &'static str) -> Self {
        Self {
            scroll_id,
        }
    }
}

impl Text for ReceiptPreviewText {
    fn render_text(&self, _data: &DataMap) -> Box<str> {
        render_receipt_page(0)
    }

    fn render_text_in_context(&self, render_ctx: &RenderContext<'_>) -> Box<str> {
        let page = render_ctx
            .context
            .widget_value_as::<usize>(self.scroll_id)
            .unwrap_or_default();
        render_receipt_page(page)
    }
}

fn receipt_page_count() -> usize {
    RECEIPT_LINES.len().div_ceil(RECEIPT_PAGE_SIZE)
}

fn render_receipt_page(page: usize) -> Box<str> {
    let page_count = receipt_page_count();
    let current_page = page.min(page_count.saturating_sub(1));
    let start = current_page * RECEIPT_PAGE_SIZE;
    let mut output = format!("Page {} of {}\n", current_page + 1, page_count);

    for (index, line) in RECEIPT_LINES
        .iter()
        .enumerate()
        .skip(start)
        .take(RECEIPT_PAGE_SIZE)
    {
        output.push_str(&format!("{}. {line}\n", index + 1));
    }

    output.into_boxed_str()
}

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
    let receipt_scroll = StubScroll::builder("receipt_page")
        .pages(receipt_page_count())
        .build();

    let dialog = dialog([
        window(
            START_STATE,
            [
                format_text(
                    "Cafe Menu\n\nBrowse a long drink list without flooding the chat.\nSelected \
                     drink: {selected_drink}\n\nThe catalog keeps pagination inside the drink \
                     grid and keeps the keyboard shape stable on the last page.\n\n[Pager] \
                     Built-in `ScrollingGroup` pager",
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
                                .action(|value| {
                                    ButtonAction::set_dialog_value("selected_drink", value)
                                })
                                .build(),
                        )
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::action(
                            "open_receipt",
                            "Open receipt preview",
                            ButtonAction::chain([
                                ButtonAction::set_widget_value("receipt_page", 0),
                                ButtonAction::switch_to("receipt"),
                            ]),
                        ))
                        .push(Button::done("close", "Close"))
                        .build(),
                ),
            ],
        ),
        window(
            "receipt",
            [
                text("Receipt Review\n"),
                text(ReceiptPreviewText::new("receipt_page")),
                text(
                    "[Pager] `StubScroll` does not render anything by itself. It stores the page \
                     in `widget_data`, exposes the page count, and lets `NumberedPager` control \
                     the custom receipt text below.\n\n",
                ),
                keyboard(
                    NumberedPager::builder(receipt_scroll)
                        .page_renderer(|page, _data| format!("{}", page + 1))
                        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                        .length(5)
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::switch_to(
                            "back_to_menu",
                            "Back to menu",
                            START_STATE,
                        ))
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
