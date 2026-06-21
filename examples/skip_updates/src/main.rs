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
