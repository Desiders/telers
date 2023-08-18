//! This example shows how to create a simple echo bot, which will repeat all messages, which it receives.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package axum_and_echo_bot
//! ```

use axum::{routing::get, Router as AxumRouter, Server};
use telers::{
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    methods::CopyMessage,
    types::Message,
    Bot, Dispatcher, Router as TelersRouter,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(
        &CopyMessage::new(message.chat.id, message.chat.id, message.message_id),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

async fn hello_world_handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RUST_LOG"))
        .init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut router = TelersRouter::new("main");
    router.message.register(echo_handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    let dispatcher = dispatcher.to_service_provider_default().unwrap();

    let app = AxumRouter::new().route("/", get(hello_world_handler));

    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap());

    let dispatcher_handle = tokio::spawn(dispatcher.run_polling());
    let server_handle = tokio::spawn(server.serve(app.into_make_service()));

    tokio::join!(
        async {
            match dispatcher_handle.await {
                Ok(Ok(_)) => {}
                Ok(Err(err)) => {
                    event!(Level::ERROR, "Error in dispatcher: {:?}", err);
                }
                Err(err) => {
                    event!(Level::ERROR, "Dispatcher panicked: {:?}", err);
                }
            }
        },
        async {
            match server_handle.await {
                Ok(Ok(_)) => {}
                Ok(Err(err)) => {
                    event!(Level::ERROR, "Error in server: {:?}", err);
                }
                Err(err) => {
                    event!(Level::ERROR, "Server panicked: {:?}", err);
                }
            }
        }
    );
}
