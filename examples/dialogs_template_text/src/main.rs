//! `TemplateText` example for `telers-dialog`.
//!
//! Requires the `template` feature on `telers-dialog` (enabled in this
//! crate's `Cargo.toml`). Demonstrates:
//! - `TemplateText` with default `minijinja` environment.
//! - `TemplateEnvBuilder` for custom filters and globals.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_template_text
//! ```

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
    widgets::{keyboard, text, Button, InlineKeyboard, TemplateEnvBuilder, TemplateText},
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "summary";

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
    // Seed render data with values consumed by both templates below.
    manager
        .extend_dialog_data([
            (
                "user",
                json!({
                    "name": "alice",
                    "premium": true,
                    "items": ["espresso", "croissant", "oat milk"],
                }),
            ),
            ("price", json!(42.5)),
        ])
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

fn registry() -> DialogRegistry {
    // Custom environment exposes a `currency` filter and a `brand` global.
    let env = TemplateEnvBuilder::new()
        .add_filter("currency", |v: f64| format!("${:.2}", v))
        .add_global("brand", "North Roast")
        .build();
    let receipt = TemplateText::builder(
        "{{ brand }} receipt for {{ user.name | title }}\n{% if user.premium %}Status: premium \
         ({{ 10 }}% off applied){% else %}Status: regular{% endif %}\nTotal: {{ price | currency \
         }}\nItems:\n{% for item in user.items %}- {{ item }}\n{% endfor %}",
    )
    .env(env)
    .build();

    let dialog = dialog([
        window(
            START_STATE,
            [
                text(
                    "Template Text\n\nThe block below is rendered by a default `TemplateText`. \
                     Variables, filters, conditionals and loops all map to `minijinja` \
                     syntax.\n\n[Text] `TemplateText::builder(template).build()`\n",
                ),
                text(
                    TemplateText::builder(
                        "Hello, {{ user.name | upper }}!\n{% if user.premium %}You have premium \
                         access.{% else %}Upgrade for premium features.{% endif %}\nYou have {{ \
                         user.items | length }} items in your cart.",
                    )
                    .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::next("next", "Custom env")])
                        .row([Button::done("close", "Close")])
                        .build(),
                ),
            ],
        ),
        window(
            "custom_env",
            [
                text(
                    "Custom Environment\n\nA `TemplateEnvBuilder` registers a `currency` filter \
                     and a `brand` global, then hands the environment to \
                     `TemplateText::builder(..).env(env)`.\n\n[Text] `TemplateEnvBuilder` + \
                     `TemplateText::builder(template).env(env).build()`\n",
                ),
                text(receipt),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
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

    let router = Router::new("dialogs_template_text")
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
