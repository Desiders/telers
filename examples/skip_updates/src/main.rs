//! This example shows how to create a bot that skips updates and only processes new updates.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package skip_updates
//! ```

use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::DeleteWebhook,
    types::Update,
    Bot, Dispatcher, Router,
};

async fn handler(update: Update) -> HandlerResult<()> {
    tracing::info!(?update, "Received update");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main").on_update(|observer| observer.register(Handler::new(handler)));

    bot.send(DeleteWebhook::new().drop_pending_updates(true))
        .await
        .unwrap();

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_updates(UpdateType::all())
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
