//! This example shows how to create an echo bot and how to run it concurrently with polling `axum` server.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package axum_and_echo_bot
//! ```

use axum::{routing, Router as AxumRouter};
use telers::{
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    methods::CopyMessage,
    types::Message,
    Bot, Dispatcher, Router as TelersRouter,
};
use tokio::net::TcpListener;
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(CopyMessage::new(
        message.chat().id(),
        message.chat().id(),
        message.id(),
    ))
    .await?;

    Ok(EventReturn::Finish)
}

#[allow(clippy::unused_async)]
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
        .build()
        .to_service_provider_default()
        .unwrap();

    let app = AxumRouter::new()
        .route("/", routing::get(hello_world_handler))
        .into_make_service();

    // `tokio::spawn` is used to run polling and server in different threads.
    // You can also don't use `tokio::spawn` and run them in the same thread.
    tokio::join!(
        async {
            match tokio::spawn(dispatcher.run_polling()).await {
                Ok(Ok(())) => {}
                Ok(Err(err)) => {
                    event!(Level::ERROR, "Error in dispatcher: {:?}", err);
                }
                Err(err) => {
                    event!(Level::ERROR, "Dispatcher panicked: {:?}", err);
                }
            }
        },
        async {
            // Check graceful shutdown example of axum server:
            // https://github.com/tokio-rs/axum/tree/main/examples/graceful-shutdown
            // Telers provides graceful shutdown out of the box, so you don't need to do anything special.
            match tokio::spawn(async {
                let listener = TcpListener::bind("0.0.0.0:3000").await?;

                axum::serve(listener, app).await
            })
            .await
            {
                Ok(Ok(())) => {}
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
