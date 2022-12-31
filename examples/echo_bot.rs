use aiogram_rs::{
    client::Bot,
    dispatcher::{event::service::ToServiceProvider as _, Dispatcher, Router},
    enums::UpdateType,
    types::Message,
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
    log::info!("Received message: {message:?}");

    // todo!("Send message back to the user with the same text as the user sent to the bot");
}

#[tokio::main]
async fn main() {
    log::set_logger(&SimpleLogger)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();

    let bot = Bot::builder().token("TOKEN").build();

    let mut router = Router::new("main");
    router.message.register_no_filters(echo_handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    if let Err(err) = dispatcher
        .to_service_provider(())
        .unwrap()
        .run_polling()
        .await
    {
        log::error!("Bot stopped with error: {err}");
    }
}
