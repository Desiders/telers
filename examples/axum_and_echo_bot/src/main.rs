//! This example shows how to create an echo bot and how to run it concurrently with polling `axum` server.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package axum_and_echo_bot
//! ```

use axum::{routing, Router as AxumRouter};
use telers::{
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn},
    methods::CopyMessage,
    types::Message,
    utils::shutdown_signal,
    Bot, Dispatcher, Router as TelersRouter,
};
use tokio::{
    net::TcpListener,
    sync::broadcast::{channel, Receiver, Sender},
};
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

    let bot = Bot::from_env_by_key("BOT_TOKEN");

    let mut router = TelersRouter::new("main");
    router.message.register(echo_handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    let app = AxumRouter::new().route("/", routing::get(hello_world_handler));

    let (shutdown_tx, _) = channel(1);

    let _ = tokio::join!(
        tokio::spawn(run_server(app, shutdown_tx.subscribe())),
        tokio::spawn(run_dispatcher(dispatcher, shutdown_tx.subscribe())),
        tokio::spawn(handle_shutdown(shutdown_tx))
    );
}

async fn run_server(app: axum::Router, mut shutdown_rx: Receiver<()>) {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
        })
        .await
        .unwrap();
}

async fn run_dispatcher(dispatcher: Dispatcher, mut shutdown_rx: Receiver<()>) {
    dispatcher
        .run_polling()
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
        })
        .await
        .unwrap();
}

async fn handle_shutdown(shutdown_tx: Sender<()>) {
    let () = shutdown_signal().await;
    let _ = shutdown_tx.send(());
}
