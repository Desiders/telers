//! This example shows how to create an echo bot, which will repeat all messages, which it receives.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package echo_bot
//! ```

use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    types::Message,
    Bot, Dispatcher, Router,
};

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(message.to_copy_message(message.chat().id()))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let mut router = Router::new("main");
    router.message.register(Handler::new(echo_handler));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!( error = %err, "Bot stopped"),
    }
}
