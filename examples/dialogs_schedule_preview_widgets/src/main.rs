//! TimeSelect + ScrollingText example for `telers-dialog`.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_schedule_preview_widgets
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
    widgets::{keyboard, text, Button, InlineKeyboard, NumberedPager, ScrollingText, TimeSelect},
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "editor";
const PREVIEW_PAGE_SIZE: usize = 220;
const DIGEST_PREVIEW: &str =
    "Roastery Morning Digest\n\nThis week starts with a bright washed Ethiopia landing on bar. \
     Expect bergamot, peach tea, and a softer finish than last month’s lot. For guests who want \
     something deeper, the Brazil house espresso is staying on as the steady option for milk \
     drinks.\n\nFriday’s brew class is now open for twelve seats. We will cover bloom control, \
     tighter bypass ratios, and the difference between sweet extraction and flat strength in \
     small batch brewers. Returning guests can bring any hand grinder for a quick calibration \
     check at the end of the session.\n\nWeekend counter note: reusable cup rewards are doubled \
     on Saturday, and the retail shelf has new filters, brew scales, and spare servers ready for \
     pickup. Staff can point regulars to the single-origin subscription if they want the same \
     release mailed monthly.\n\nUse this preview to check long campaign copy before the bot sends \
     the digest to subscribers.";

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
    let preview = ScrollingText::builder("digest_copy_page")
        .text(DIGEST_PREVIEW)
        .page_size(PREVIEW_PAGE_SIZE)
        .build();
    let dialog = dialog([window(
        START_STATE,
        [
            text(
                "Daily Digest Setup\n\nPick the send time for the subscriber digest, then page \
                 through the full draft before it goes live.\n",
            ),
            text(preview.clone()),
            text(
                "\n[Widgets] `TimeSelect` stores the delivery slot in `widget_data`, and the \
                 pager now reads scroll state from `ScrollingText` instead of making the caller \
                 repeat page-count logic by hand.",
            ),
            keyboard(TimeSelect::builder("delivery_time").build()),
            keyboard(
                NumberedPager::builder(preview.clone())
                    .page_renderer(|page, _data| format!("{}", page + 1))
                    .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                    .length(5)
                    .build(),
            ),
            keyboard(InlineKeyboard::new().push(Button::done("close", "Close"))),
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

    let router = Router::new("dialogs_schedule_preview_widgets")
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
