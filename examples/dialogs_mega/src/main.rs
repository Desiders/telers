mod common;
mod dialogs;

use serde_json::Value;
use telers::{
    enums::UpdateType,
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    filters::CommandStart,
    fsm::{MemoryStorage, Strategy::UserInChat},
    middlewares::outer::FSMContext as FSMContextMiddleware,
    Bot, Dispatcher, Router,
};
use telers_dialog::{DialogManager, DialogObserverExt, DialogRegistry, StartMode};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

type Manager = DialogManager<MemoryStorage>;

/// `/start` resets the whole stack and shows the main menu.
async fn handle_start(bot: Bot, manager: Manager) -> HandlerResult<()> {
    let _ = manager
        .start(
            &bot,
            common::MAIN_MENU_STATE.to_owned(),
            Value::Null,
            StartMode::ResetStack,
        )
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

fn registry() -> DialogRegistry {
    DialogRegistry::new()
        .register(dialogs::main_menu::dialog())
        .and_then(|r| r.register(dialogs::text_widgets::dialog()))
        .and_then(|r| r.register(dialogs::template::dialog()))
        .and_then(|r| r.register(dialogs::scrolls::dialog()))
        .and_then(|r| r.register(dialogs::layouts::dialog()))
        .and_then(|r| r.register(dialogs::selects::dialog()))
        .and_then(|r| r.register(dialogs::multiwidget::dialog()))
        .and_then(|r| r.register(dialogs::counter::dialog()))
        .and_then(|r| r.register(dialogs::calendar::dialog()))
        .and_then(|r| r.register(dialogs::switch::dialog()))
        .and_then(|r| r.register(dialogs::reply_kbd::dialog()))
        .and_then(|r| r.register(dialogs::inputs::dialog()))
        .and_then(|r| r.register(dialogs::buttons::dialog()))
        .and_then(|r| r.register(dialogs::button_actions::dialog()))
        .and_then(|r| r.register(dialogs::link_preview::dialog()))
        .and_then(|r| r.register(dialogs::media::dialog()))
        .expect("all mega dialogs register without duplicate states")
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

    let router = Router::new("dialogs_mega")
        .on_update(|observer| {
            observer
                .register_outer_middleware(FSMContextMiddleware::new(storage).strategy(UserInChat))
        })
        .on_message(|observer| {
            observer
                .register(Handler::new(handle_start).filter(CommandStart::default()))
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
