//! Roastery broadcast text widget examples for `telers-dialog`.
//!
//! Shows how static text, formatted data, computed text, lists, and
//! `ScrollingText` can build a campaign preview flow.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_text_widgets
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
    widgets::{
        fn_text, format_text, keyboard, text, Button, ButtonAction, InlineKeyboard, ListText,
        NumberedPager, ScrollingText,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "draft_intro";
const PREVIEW_PAGE_SIZE: usize = 240;
const BROADCAST_COPY: &str =
    "Weekend Espresso Sale\n\nHouse Blend bags are 15% off through Sunday for customers who want \
     the same chocolate-forward profile they tasted on bar. The offer works well for takeout \
     regulars because it gives staff a clear, short pitch at checkout.\n\nSaturday at 12:00 we \
     also have a cupping session for the new washed Ethiopia. The note card should mention peach \
     tea, bergamot, and a soft floral finish, but avoid promising limited seats after the class \
     is full.\n\nReusable cup rewards are doubled for the whole weekend. Put this after the bean \
     and class notes so the message still reads like a cafe update, not only a discount blast.";

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
    let broadcast_preview = ScrollingText::builder("broadcast_preview_scroll")
        .text(BROADCAST_COPY)
        .page_size(PREVIEW_PAGE_SIZE)
        .build();

    let dialog = dialog([
        window(
            START_STATE,
            [
                text(
                    "Roastery Broadcast Draft\n\nOpen a ready-to-send promo preview built from \
                     several text blocks in one message.\n\n[Text] The next screen combines \
                     static text, formatted values, computed text, and a rendered list.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::action(
                            "show_preview",
                            "Open preview",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("cafe_name", "North Roast"),
                                ButtonAction::set_dialog_value(
                                    "campaign_title",
                                    "Weekend Espresso Sale",
                                ),
                                ButtonAction::set_dialog_value("week_label", "April 8-14"),
                                ButtonAction::set_dialog_value("bonus", "free oat milk upgrade"),
                                ButtonAction::next(),
                            ]),
                        ))
                        .build(),
                ),
            ],
        ),
        window(
            "preview",
            [
                text("Roastery Broadcast Preview\n"),
                format_text("Cafe: {cafe_name}\nCampaign: {campaign_title}\nWeek: {week_label}\n"),
                fn_text(|data: &DataMap| {
                    let bonus = data.get("bonus").and_then(Value::as_str).unwrap();
                    format!("This message highlights three offers and a {bonus}.\n")
                }),
                text(
                    ListText::builder()
                        .items_getter(|_data| {
                            [
                                "Espresso beans at 15% off",
                                "Saturday cupping at 12:00",
                                "Reusable cup reward for takeout orders",
                            ]
                        })
                        .item_renderer(|&item, _data| format!("- {item}"))
                        .build(),
                ),
                text(
                    "\n[Text] This window is assembled from `text`, `FormatText`, `FnText`, and \
                     `ListText`.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::back("back", "Back"))
                        .push(Button::switch_to(
                            "show_long_copy_preview",
                            "Long copy preview",
                            "long_copy",
                        ))
                        .push(Button::done("close", "Close"))
                        .build(),
                ),
            ],
        ),
        window(
            "long_copy",
            [
                text(
                    "Roastery Broadcast Long Copy\n\nUse this screen when message text is too \
                     long to review comfortably in one chunk.\n",
                ),
                text(broadcast_preview.clone()),
                text(
                    "\n[Text] `ScrollingText` renders only the current page, and `NumberedPager` \
                     controls the same scroll state.",
                ),
                keyboard(
                    NumberedPager::builder(broadcast_preview)
                        .page_renderer(|page, _data| format!("{}", page + 1))
                        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                        .length(5)
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

    let router = Router::new("dialogs_text_widgets")
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
