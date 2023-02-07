use aiogram_rs::{
    client::Bot,
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

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult {
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
    log::set_logger(&SimpleLogger)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .unwrap();

    let bot = Bot::builder()
        .token("5645341478:AAERH8MzJYL8zacQ_ht5oeg4tjYx_ZhTmxA")
        .build();

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
