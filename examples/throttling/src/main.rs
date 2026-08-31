use std::time::Duration;
use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    fsm::Strategy,
    methods::SendMessage,
    middlewares::inner::Throttling,
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

    let router = Router::new("main").on_message(|observer| {
        observer
            // Process at most one message per chat-user pair in 5 seconds.
            // Peer IDs are resolved from the context by UserContextMiddleware.
            .register_inner_middleware(
                Throttling::new(Duration::from_secs(5))
                    .strategy(Strategy::UserInChat)
                    .on_throttled(|request, info| {
                        tracing::warn!(
                            exceeded_count = info.exceeded_count,
                            time_left = ?info.time_left,
                            "Request is throttled"
                        );
                        // Notify the user only for the first throttled requests,
                        // like the aiogram v2 antiflood example.
                        if info.exceeded_count > 2 {
                            return;
                        }
                        let bot = request.bot.clone();
                        let chat_id = request.update.message().unwrap().chat().id();
                        tokio::spawn(async move {
                            let _ = bot
                                .send(SendMessage::new(chat_id, "Too many requests!"))
                                .await;
                        });
                    }),
            )
            .register(Handler::new(echo_handler))
    });

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
