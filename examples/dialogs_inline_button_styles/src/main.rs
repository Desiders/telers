//! Inline button addition examples for `telers-dialog`.
//!
//! Shows the styling and dynamic-payload features on top of the standard
//! `Button`:
//! - `danger()`, `success()`, `primary()` (and the generic `style()` setter).
//! - `icon_custom_emoji_id()` for prepending a custom emoji to the label.
//! - Dynamic URL / copy text / switch-inline / web-app payloads rendered
//!   from data via `*_dynamic` constructors.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_inline_button_styles
//! ```
//!
//! `icon_custom_emoji_id` only works for bots in premium-enabled chats and
//! requires a real custom emoji id; the constant below is a placeholder.

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
        format_text, keyboard, text, Button, ButtonAction, ButtonStyle, FormatText, InlineKeyboard,
    },
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "styles";
const CUSTOM_EMOJI_ID: &str = "5377592449929163392";

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
    // Seed dynamic payload values so the dynamic window renders meaningful data.
    manager
        .extend_dialog_data([
            ("docs_url", json!("https://core.telegram.org/bots/api")),
            ("copy_payload", json!("td_promo_2026")),
            ("inline_query", json!("@gif weekend")),
            ("web_app_url", json!("https://example.com/mini-app")),
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
                    "Inline Button Styles\n\nThree style helpers map to Telegram's coloured \
                     callback buttons. The shared `style(ButtonStyle::...)` setter is also \
                     available for parameterized cases.\n\n[Buttons] `Button::danger`, \
                     `Button::success`, `Button::primary`, `Button::style`",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "delete",
                            "Delete draft",
                            ButtonAction::switch_to("destructive"),
                        )
                        .danger()])
                        .row([Button::action(
                            "confirm",
                            "Confirm changes",
                            ButtonAction::switch_to("confirmed"),
                        )
                        .success()])
                        .row([Button::action(
                            "primary",
                            "Continue",
                            ButtonAction::switch_to("emoji"),
                        )
                        .primary()])
                        .row([Button::action(
                            "explicit_style",
                            "Continue (explicit style)",
                            ButtonAction::switch_to("emoji"),
                        )
                        .style(ButtonStyle::Primary)])
                        .row([Button::done("close", "Close")])
                        .build(),
                ),
            ],
        ),
        window(
            "destructive",
            [
                text("Confirm Deletion\n\nThe red button on the previous screen led here."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
                        .build(),
                ),
            ],
        ),
        window(
            "confirmed",
            [
                text("Changes Confirmed\n\nThe green success button led here."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
                        .build(),
                ),
            ],
        ),
        window(
            "emoji",
            [
                text(
                    "Custom Emoji Icon\n\nThe button below uses `icon_custom_emoji_id` to prepend \
                     a premium emoji. The id is a placeholder; replace the constant with a real \
                     custom emoji id to see the icon in a premium-enabled chat.\n\n[Buttons] \
                     `Button::action(..).icon_custom_emoji_id(id)`",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "with_emoji",
                            "Open dynamic payloads",
                            ButtonAction::switch_to("dynamic"),
                        )
                        .icon_custom_emoji_id(CUSTOM_EMOJI_ID)])
                        .row([Button::back("back", "Back")])
                        .build(),
                ),
            ],
        ),
        window(
            "dynamic",
            [
                format_text(
                    "Dynamic Payloads\n\nEach button below renders its payload from dialog data, \
                     so the same widget produces different Telegram URLs/queries on each \
                     render.\n\nDocs URL: {docs_url}\nCopy payload: {copy_payload}\nInline query: \
                     {inline_query}\nWeb app URL: {web_app_url}\n\n[Buttons] \
                     `Button::url_dynamic`, `Button::copy_text_dynamic`, \
                     `Button::switch_inline_query_dynamic`, `Button::web_app_dynamic`",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::url_dynamic(
                            "Open docs",
                            FormatText::new("{docs_url}"),
                        )])
                        .row([Button::copy_text_dynamic(
                            "Copy promo code",
                            FormatText::new("{copy_payload}"),
                        )])
                        .row([Button::switch_inline_query_dynamic(
                            "Share inline",
                            FormatText::new("{inline_query}"),
                        )])
                        .row([Button::web_app_dynamic(
                            "Open mini-app",
                            FormatText::new("{web_app_url}"),
                        )])
                        .row([Button::back("back", "Back")])
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

    let router = Router::new("dialogs_inline_button_styles")
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
