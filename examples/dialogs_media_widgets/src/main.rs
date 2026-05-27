//! Media widget examples for `telers-dialog`.
//!
//! Shows three flows in one dialog:
//! - `StaticMedia` from a fixed URL/file id with a caption.
//! - `DynamicMedia` that reads the photo URL from `dialog_data`.
//! - `MediaScroll` paged through a `NumberedPager`.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_media_widgets
//! ```
//!
//! The file-id and URL constants below are placeholders. Replace them with
//! values your bot can actually send (Telegram only accepts a file id that
//! was uploaded by the same bot, or a public URL it can fetch).

use serde_json::{json, Value};
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
        format_text, keyboard, media, text, Button, ButtonAction, DynamicMedia, InlineKeyboard,
        MediaContentType, MediaId, MediaScroll, NumberedPager, StaticMedia,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "static_url";
const STATIC_URL: &str = "https://placehold.co/600x400/EEE/31343C?text=Static+URL";
const STATIC_FILE_ID: &str = "AgACAgQAAxk_replace_with_a_real_photo_file_id";
const DYNAMIC_URL_A: &str = "https://placehold.co/600x400/EEE/31343C?text=Dynamic+A";
const DYNAMIC_URL_B: &str = "https://placehold.co/600x400/EEE/31343C?text=Dynamic+B";
const GALLERY: &[&str] = &[
    "https://placehold.co/600x400/EEE/31343C?text=Gallery+1",
    "https://placehold.co/600x400/EEE/31343C?text=Gallery+2",
    "https://placehold.co/600x400/EEE/31343C?text=Gallery+3",
    "https://placehold.co/600x400/EEE/31343C?text=Gallery+4",
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
    // Seed `dialog_data` so the dynamic and scroll widgets have data to read.
    manager
        .extend_dialog_data([
            ("photo_url", json!(DYNAMIC_URL_A)),
            ("gallery_urls", json!(GALLERY)),
        ])
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

fn registry() -> DialogRegistry {
    let dialog = dialog([
        window(
            START_STATE,
            [
                text(
                    "Static Media\n\nFixed photo from URL with a caption rendered by a text \
                     widget.\n\n[Media] `StaticMedia::builder(content_type).url(...).caption(...)`",
                ),
                media(
                    StaticMedia::builder(MediaContentType::Photo)
                        .url(STATIC_URL)
                        .caption("Photo loaded from a public URL")
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::next("next", "Static file id")])
                        .row([Button::done("close", "Close")])
                        .build(),
                ),
            ],
        ),
        window(
            "static_file_id",
            [
                text(
                    "Static Media\n\nSame builder, but the source is a Telegram `file_id`. \
                     Replace the constant in `main.rs` with a real id your bot owns before \
                     running this window.\n\n[Media] \
                     `StaticMedia::builder(content_type).file_id(MediaId::new(...))`",
                ),
                media(
                    StaticMedia::builder(MediaContentType::Photo)
                        .file_id(MediaId::new(STATIC_FILE_ID))
                        .caption("Photo reused via file_id")
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
                        .row([Button::switch_to(
                            "open_dynamic",
                            "Dynamic media",
                            "dynamic",
                        )])
                        .build(),
                ),
            ],
        ),
        window(
            "dynamic",
            [
                format_text(
                    "Dynamic Media\n\nThe photo URL is read from `dialog_data[photo_url]`. Use \
                     the buttons below to write different URLs; the same widget re-renders \
                     against the new value.\n\nCurrent: {photo_url}\n\n[Media] \
                     `DynamicMedia::from_url_field(content_type, field)`",
                ),
                media(DynamicMedia::from_url_field(
                    MediaContentType::Photo,
                    "photo_url",
                )),
                keyboard(
                    InlineKeyboard::builder()
                        .row([
                            Button::action(
                                "use_a",
                                "Use A",
                                ButtonAction::set_dialog_value("photo_url", DYNAMIC_URL_A),
                            ),
                            Button::action(
                                "use_b",
                                "Use B",
                                ButtonAction::set_dialog_value("photo_url", DYNAMIC_URL_B),
                            ),
                        ])
                        .row([Button::switch_to("open_scroll", "Open gallery", "scroll")])
                        .row([Button::switch_to("back_to_url", "Back", START_STATE)])
                        .build(),
                ),
            ],
        ),
        window(
            "scroll",
            [
                text(
                    "Media Scroll\n\nPaginate a fixed list of URLs. `NumberedPager` writes the \
                     current page into `widget_data[gallery]`; `MediaScroll` renders the \
                     attachment for that page.\n\n[Media] `MediaScroll::from_url_array_field` + \
                     `NumberedPager`",
                ),
                media(MediaScroll::from_url_array_field(
                    "gallery",
                    MediaContentType::Photo,
                    "gallery_urls",
                )),
                keyboard(
                    NumberedPager::builder(MediaScroll::from_url_array_field(
                        "gallery",
                        MediaContentType::Photo,
                        "gallery_urls",
                    ))
                    .page_renderer(|page, _data| format!("{}", page + 1))
                    .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                    .length(5)
                    .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("back_to_dynamic", "Back", "dynamic")])
                        .row([Button::done("close", "Close")])
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

    let router = Router::new("dialogs_media_widgets")
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
