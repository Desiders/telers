use aiogram_rs::{
    client::Bot,
    dispatcher::{event::service::ServiceFactory, Dispatcher, Router},
    types::Message,
    utils::backoff::ExponentialBackoff,
};
use log::{self, Level, LevelFilter, Log, Metadata, Record};
use tokio;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

async fn echo_handler(message: Message) {
    log::info!("Message: {:?}", message);

    // todo!("Send message back to the user with the same text as the user sent to the bot");
}

#[tokio::main]
async fn main() {
    log::set_logger(&SimpleLogger)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    let bot = Bot::default();

    let mut main_router = Router::new("main");
    main_router.message.register(echo_handler, vec![]);

    let dispatcher = Dispatcher::new(main_router).new_service(()).await.unwrap();

    log::info!("Starting bot");
    match dispatcher
        .run_polling(vec![bot], ExponentialBackoff::default())
        .await
    {
        Ok(_) => log::warn!("Bot stopped"),
        Err(err) => log::error!("Bot stopped with error: {err}"),
    }
}
