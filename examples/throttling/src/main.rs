use std::time::Duration;
use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    middlewares::inner::{Key, Throttling},
    types::{Chat, Message},
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
            .register_inner_middleware(
                Throttling::new(Duration::from_secs(5))
                    .key(Key::UserInChat)
                    .on_throttled(|request, info| {
                        let bot = request.bot.clone();
                        let chat_id = request.context.get::<Chat>("event_chat").map(Chat::id);
                        async move {
                            tracing::warn!(
                                exceeded_count = info.exceeded_count,
                                time_left = ?info.time_left,
                                "Request is throttled"
                            );
                            if info.exceeded_count > 2 {
                                return;
                            }
                            if let Some(chat_id) = chat_id {
                                let _ = bot
                                    .send(SendMessage::new(chat_id, "Too many requests!"))
                                    .await;
                            }
                        }
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
