use aiogram_rs::{
    client::{Bot, Reqwest, Session},
    dispatcher::{
        event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
        middlewares::inner::Logging as LoggingMiddleware,
        Dispatcher, Router,
    },
    enums::UpdateType,
    methods::CopyMessage,
    types::Message,
};

use log::{self, LevelFilter, Log, Metadata, Record};
use std::env;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

async fn echo_handler<Client: Session + Send + Sync>(
    bot: Bot<Client>,
    message: Message,
) -> HandlerResult {
    bot.send(
        &CopyMessage::new(message.chat.id, message.chat.id, message.message_id)
            .allow_sending_without_reply(true),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Ok(bot_token) = env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    log::set_logger(&SimpleLogger)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .unwrap();

    let bot = Bot::<Reqwest>::builder().token(bot_token).build();

    let mut router = Router::new("main");
    router
        .message
        .inner_middlewares
        .register(LoggingMiddleware::default());
    router.message.register_no_filters(echo_handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher
        .to_service_provider(())
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => log::info!("Bot stopped"),
        Err(err) => log::error!("Bot stopped with error: {err}"),
    }
}
